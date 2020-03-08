use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::mem::{drop, swap};
use std::path::{Path, PathBuf};
use std::thread::Builder;
use std::vec::IntoIter;

use anyhow::{Context, Error};
use crossbeam_channel::{bounded, Receiver, Sender};
use serde_json::from_str;

use super::Progress;
use crate::model::RootEntry;

const INPUT_CHUNK_SIZE: usize = 8 * 1024 * 1024;
const INPUT_BYTES_CHANNEL_BUF: usize = 1024;
const INPUT_LINE_BUFFER_INITIAL_SIZE: usize = 1024;
const PARSED_CHANNEL_BUF: usize = 256;
const RESULT_CHANNEL_BUF: usize = 256;
const WORKER_MULT: usize = 2;

pub struct ParallelDecoder<D> {
    reading: IntoIter<D>,
    recv: Receiver<Result<Vec<D>, Error>>,
}

impl<D: 'static + Send + RootEntry> ParallelDecoder<D> {
    pub fn start(
        path: impl AsRef<Path>,
        progress: impl 'static + Send + Progress,
    ) -> Result<ParallelDecoder<D>, Error> {
        let path = path.as_ref().to_owned();

        let (input_send, input_recv) = bounded(INPUT_BYTES_CHANNEL_BUF);
        let (parsed_send, parsed_recv) = bounded(PARSED_CHANNEL_BUF);
        let (result_send, result_recv) = bounded(RESULT_CHANNEL_BUF);

        Builder::new()
            .name("input reader".to_owned())
            .spawn(move || {
                read(path, input_send, progress);
            })
            .context("failed spawn input reader")?;

        for i in 0..get_worker_cnt() {
            let r = input_recv.clone();
            let s = parsed_send.clone();

            Builder::new()
                .name(format!("parser({})", i))
                .spawn(move || {
                    parse(r, s);
                })
                .with_context(|| format!("failed spawn parser({})", i))?;
        }
        drop(input_recv);
        drop(parsed_send);

        Builder::new()
            .name("collector".to_owned())
            .spawn(move || {
                collect(parsed_recv, result_send);
            })
            .context("failed spawn collector")?;

        Ok(ParallelDecoder {
            reading: Vec::new().into_iter(),
            recv: result_recv,
        })
    }

    pub fn read_entry(&mut self) -> Result<Option<D>, Error> {
        loop {
            if let Some(v) = self.reading.next() {
                return Ok(Some(v));
            }

            match self.recv.recv() {
                Ok(Ok(vs)) => {
                    self.reading = vs.into_iter();
                }
                Ok(Err(e)) => {
                    return Err(e);
                }
                Err(_) => {
                    return Ok(None);
                }
            }
        }
    }
}

fn read(
    path: PathBuf,
    send: Sender<(usize, Result<Vec<u8>, Error>)>,
    mut progress: impl 'static + Send + Progress,
) {
    let f = match File::open(&path).context("failed to open input file") {
        Ok(v) => v,
        Err(e) => {
            send.send((0, Err(e))).expect("failed to send input value");
            return;
        }
    };
    let mut chunk_reader = ChunkReader::new(f, INPUT_CHUNK_SIZE);

    for idx in 0usize.. {
        match chunk_reader
            .read_chunk()
            .context("failed to read input chunk")
        {
            Ok(Some(bs)) => {
                progress.inc(bs.len());

                send.send((idx, Ok(bs)))
                    .expect("failed to send input value");
            }
            Ok(None) => {
                break;
            }
            Err(e) => {
                send.send((idx, Err(e))).expect("failed to send read error");
                break;
            }
        }
    }
}

fn parse<D: 'static + Send + RootEntry>(
    recv: Receiver<(usize, Result<Vec<u8>, Error>)>,
    send: Sender<(usize, Result<Vec<D>, Error>)>,
) {
    let mut chunk_parser = ChunkParser::new();

    while let Ok((idx, r)) = recv.recv() {
        match r {
            Ok(bs) => {
                match chunk_parser
                    .parse_chunk(bs.as_slice())
                    .context("failed to parse input chunk")
                {
                    Ok(v) => {
                        send.send((idx, Ok(v)))
                            .expect("failed to send parsed value");
                    }
                    Err(e) => {
                        send.send((idx, Err(e)))
                            .expect("failed to send parse error");
                        return;
                    }
                }
            }
            Err(e) => {
                send.send((idx, Err(e)))
                    .expect("failed to pass error in parser");
                return;
            }
        }
    }
}

#[derive(Debug)]
struct ChunkParser {
    buf: String,
    max_values_len: usize,
}

impl ChunkParser {
    fn new() -> ChunkParser {
        ChunkParser {
            buf: String::with_capacity(INPUT_LINE_BUFFER_INITIAL_SIZE),
            max_values_len: 0,
        }
    }

    fn parse_chunk<D: 'static + Send + RootEntry>(
        &mut self,
        mut bs: &[u8],
    ) -> Result<Vec<D>, Error> {
        let mut values = Vec::<D>::with_capacity(self.max_values_len);

        loop {
            self.buf.clear();
            let n = bs.read_line(&mut self.buf)?;
            if n == 0 {
                break;
            }

            let s = self.buf.trim();
            if s == "[" {
                continue;
            }
            if s == "]" {
                break;
            }
            let s = s.trim_end_matches(',');

            if s.is_empty() {
                continue;
            }

            let s = D::pre_filter(s);
            let v =
                from_str(s.as_ref()).with_context(|| format!("failed parse line: {}", self.buf))?;
            values.push(v);
        }
        if values.len() > self.max_values_len {
            self.max_values_len = values.len();
        }
        Ok(values)
    }
}

fn collect<T: Send>(recv: Receiver<(usize, Result<T, Error>)>, send: Sender<Result<T, Error>>) {
    let mut holding = BTreeMap::<usize, Result<T, Error>>::new();
    let mut next = 0;

    loop {
        match recv.recv() {
            Ok((idx, r)) => {
                holding.insert(idx, r);
            }
            Err(_) => {
                // Sender Closed
                return;
            }
        }

        // Send following values
        while let Some(r) = holding.remove(&next) {
            match r {
                Ok(v) => {
                    send.send(Ok(v)).expect("failed to send collected value");
                    next += 1;
                }
                Err(e) => {
                    send.send(Err(e)).expect("failed to send collected error");
                    return;
                }
            }
        }
    }
}

fn get_worker_cnt() -> usize {
    if let Ok(s) = env::var("EDM_THREADS") {
        s.parse().unwrap()
    } else {
        WORKER_MULT * num_cpus::get()
    }
}

struct ChunkReader<R: Read> {
    chunk_size: usize,
    inner: R,
    left_buf: Vec<u8>,
    left_size: usize,
}

impl<R: Read> ChunkReader<R> {
    fn new(inner: R, chunk_size: usize) -> ChunkReader<R> {
        ChunkReader {
            chunk_size,
            inner,
            left_buf: alloc_vec(chunk_size),
            left_size: 0,
        }
    }

    fn read_chunk(&mut self) -> Result<Option<Vec<u8>>, io::Error> {
        let mut buf = alloc_vec(self.chunk_size);
        swap(&mut buf, &mut self.left_buf);

        let mut total_read = self.left_size;

        // Read
        while total_read < self.chunk_size {
            let n = self.inner.read(&mut buf[total_read..])?;
            total_read += n;

            if n == 0 {
                break;
            }
        }

        // return `None` if empty
        if total_read == 0 {
            return Ok(None);
        }

        // return final chunk
        if total_read < self.chunk_size {
            buf.truncate(total_read);
            self.left_size = 0;
            return Ok(Some(buf));
        }

        // find newline
        let mut split_pos = total_read - 1;
        while split_pos > 0 {
            if buf[split_pos] == b'\n' {
                let left = &buf[split_pos + 1..];
                self.left_size = left.len();
                self.left_buf[0..left.len()].copy_from_slice(left);

                buf.truncate(split_pos + 1);
                return Ok(Some(buf));
            }

            split_pos -= 1;
        }

        // no newline in buffer
        self.left_size = 0;
        Ok(Some(buf))
    }
}

fn alloc_vec(len: usize) -> Vec<u8> {
    let layout = std::alloc::Layout::from_size_align(len, 1).expect("failed create layout");
    unsafe { Vec::from_raw_parts(std::alloc::alloc_zeroed(layout) as *mut _, len, len) }
}
