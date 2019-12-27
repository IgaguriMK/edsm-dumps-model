#![allow(unused)]

use std::env;
use std::io::BufRead;
use std::path::Path;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};
use std::thread::{Builder, JoinHandle};
use std::vec::IntoIter;

use filebuffer::FileBuffer;
use serde_json::from_str;
use tiny_fail::{ErrorMessageExt, Fail};

use super::Progress;
use crate::model::RootEntry;

const INPUT_CHUNK_SIZE: usize = 8 * 1024 * 1024;
const MAX_INPUT_CHUNK_SIZE: usize = 12 * 1024 * 1024;

const INPUT_CHANNEL_MASTER_BUFFER: usize = 1;
const INPUT_CHANNEL_SUB_BUFFER: usize = 1;
const OUTPUT_CHANNEL_MASTER_BUFFER: usize = 16;
const OUTPUT_CHANNEL_SUB_BUFFER: usize = 16;

const WORKER_MULT: usize = 2;

pub struct ParallelDecoder<D> {
    handles: Vec<JoinHandle<()>>,
    reading: Option<IntoIter<D>>,
    recv: Receiver<Option<Vec<D>>>,
}

impl<D: 'static + Send + RootEntry> ParallelDecoder<D> {
    pub fn start(
        path: impl AsRef<Path>,
        progress: impl 'static + Send + Progress,
    ) -> Result<ParallelDecoder<D>, Fail> {
        let bytes = FileBuffer::open(path).err_msg("failed open file")?.leak();

        let worker_cnt = get_worker_cnt();

        let mut sends = Vec::with_capacity(worker_cnt);
        let mut recvs = Vec::with_capacity(worker_cnt);
        let mut handles = Vec::with_capacity(worker_cnt + 3);

        let (bytes_recv, h) = spawn_splitter(bytes, progress);
        handles.push(h);

        for i in 0..worker_cnt {
            let (s, r, h) = spawn_decoder::<D>(i);
            sends.push(s);
            recvs.push(r);
            handles.push(h);
        }

        let h = spawn_send_mux(bytes_recv, sends);
        handles.push(h);

        let (send, recv) = sync_channel(OUTPUT_CHANNEL_MASTER_BUFFER);
        let h = spawn_recv_mux(send, recvs);

        Ok(ParallelDecoder {
            handles,
            reading: None,
            recv,
        })
    }

    pub fn read_entry(&mut self) -> Result<Option<D>, Fail> {
        loop {
            if let Some(ref mut iter) = self.reading {
                if let Some(x) = iter.next() {
                    return Ok(Some(x));
                } else {
                    self.reading = None;
                }
            } else {
                match self.recv.recv().unwrap() {
                    Some(chunk) => self.reading = Some(chunk.into_iter()),
                    None => {
                        for h in self.handles.drain(..) {
                            h.join().unwrap();
                        }
                        return Ok(None);
                    }
                }
            }
        }
    }
}

fn spawn_splitter(
    bytes: &'static [u8],
    mut progress: impl 'static + Send + Progress,
) -> (Receiver<Option<&'static [u8]>>, JoinHandle<()>) {
    let (send, recv) = sync_channel(INPUT_CHANNEL_MASTER_BUFFER);

    let handle = Builder::new()
        .name("splitter".to_owned())
        .spawn(move || {
            let mut bytes = bytes;

            if bytes.starts_with(b"[\n") {
                bytes = bytes.split_at(2).1;
                progress.inc(2);
            }

            let reset_eta_at = (4 * 1024 * 1024 * 1024) / INPUT_CHUNK_SIZE;
            for i in 0.. {
                if bytes.len() < MAX_INPUT_CHUNK_SIZE {
                    break;
                }
                if i % reset_eta_at == 0 {
                    progress.reset_eta();
                }

                let pos = find_newline(bytes, INPUT_CHUNK_SIZE);
                if pos + 1 < bytes.len() {
                    let (bs, bss) = bytes.split_at(pos + 1);
                    progress.inc(bs.len());
                    send.send(Some(bs)).unwrap();
                    bytes = bss;
                }
            }

            if bytes.ends_with(b"]") {
                bytes = bytes.split_at(bytes.len() - 1).0;
                progress.inc(1);
            }
            progress.inc(bytes.len());
            send.send(Some(bytes)).unwrap();
            send.send(None).unwrap();
        })
        .unwrap();
    (recv, handle)
}

fn spawn_decoder<D: 'static + Send + RootEntry>(
    idx: usize,
) -> (
    SyncSender<Option<&'static [u8]>>,
    Receiver<Option<Vec<D>>>,
    JoinHandle<()>,
) {
    let (bytes_send, bytes_recv) = sync_channel(INPUT_CHANNEL_SUB_BUFFER);
    let (chunk_send, chunk_recv) = sync_channel(OUTPUT_CHANNEL_SUB_BUFFER);

    let h = Builder::new()
        .name(format!("decoder[{}]", idx))
        .spawn(move || {
            if let Err(e) = decoder(bytes_recv, chunk_send) {
                eprintln!("\n\n{}\n\n", e);
                panic!("decode failed")
            }
        })
        .unwrap();

    (bytes_send, chunk_recv, h)
}

fn decoder<D: 'static + Send + RootEntry>(
    recv: Receiver<Option<&'static [u8]>>,
    send: SyncSender<Option<Vec<D>>>,
) -> Result<(), Fail> {
    let mut line = String::new();

    while let Some(mut bytes) = recv.recv().unwrap() {
        let mut chunk = Vec::new();
        loop {
            line.truncate(0);
            let n = bytes.read_line(&mut line)?;
            if n == 0 {
                break;
            }

            let s = line
                .trim_start_matches("    ")
                .trim_end_matches("\n")
                .trim_end_matches(",");
            if s.is_empty() {
                continue;
            }
            let s = D::pre_filter(s);

            let v = from_str(s.as_ref()).err_msg(format!("failed parse entry with: {}", s))?;
            chunk.push(v);
        }
        send.send(Some(chunk)).unwrap();
    }
    send.send(None).unwrap();
    Ok(())
}

fn spawn_send_mux<T: 'static + Send>(
    recv: Receiver<Option<T>>,
    sends: Vec<SyncSender<Option<T>>>,
) -> JoinHandle<()> {
    Builder::new()
        .name("send_mux".to_owned())
        .spawn(move || {
            while let Some(x) = recv.recv().unwrap() {
                let mut sending = Some(x);
                'send: loop {
                    for s in &sends {
                        match s.try_send(sending) {
                            Ok(_) => break 'send,
                            Err(TrySendError::Full(s)) => sending = s,
                            Err(TrySendError::Disconnected(_)) => {
                                panic!("unexpected disconnect in send_mux")
                            }
                        }
                    }
                }
            }

            for s in &sends {
                s.send(None).unwrap();
            }
        })
        .unwrap()
}

fn spawn_recv_mux<T: 'static + Send>(
    send: SyncSender<Option<T>>,
    recvs: Vec<Receiver<Option<T>>>,
) -> JoinHandle<()> {
    Builder::new()
        .name("recv_mux".to_owned())
        .spawn(move || {
            let mut recvs = recvs;
            loop {
                if recvs.is_empty() {
                    send.send(None).unwrap();
                    return;
                }

                let mut end_idx = Option::<usize>::None;
                for (i, r) in recvs.iter().enumerate() {
                    match r.try_recv() {
                        Ok(Some(x)) => {
                            send.send(Some(x)).unwrap();
                            break;
                        }
                        Ok(None) => {
                            end_idx = Some(i);
                            break;
                        }
                        Err(_) => {}
                    }
                }
                if let Some(i) = end_idx {
                    recvs.remove(i);
                }
            }
        })
        .unwrap()
}

fn get_worker_cnt() -> usize {
    if let Ok(s) = env::var("THREADS") {
        s.parse().unwrap()
    } else {
        WORKER_MULT * num_cpus::get()
    }
}

fn find_newline(buf: &[u8], start: usize) -> usize {
    for i in start..buf.len() {
        if buf[i] == b'\n' {
            return i;
        }
    }
    buf.len()
}
