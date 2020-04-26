pub mod parallel;

use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

use anyhow::{Context, Error};
use detect_compression::{DetectReader, ReadWrapperBuilder};
use serde_json::from_str;

use crate::model::RootEntry;

const ERROR_COLOR_LEN: usize = 20;

pub struct ArrayDecoder {
    r: DetectReader,
    line: usize,
    buf: String,
}

impl ArrayDecoder {
    pub fn open<P: 'static + Progress>(
        path: impl AsRef<Path>,
        progress: P,
    ) -> Result<ArrayDecoder, Error> {
        let builder = ProgressReaderBuilder::new(progress);
        let inner = DetectReader::open_with_wrapper(path, builder).context("open input file")?;

        Ok(ArrayDecoder {
            r: inner,
            line: 0,
            buf: String::new(),
        })
    }
}

impl ArrayDecoder {
    fn read_line(&mut self) -> Result<Option<&str>, Error> {
        self.buf.truncate(0);
        self.r
            .read_line(&mut self.buf)
            .context(format!("failed read dump file at line {}", self.line))?;
        self.line += 1;

        if self.buf.trim() == "[" {
            self.buf.truncate(0);
            self.r
                .read_line(&mut self.buf)
                .context(format!("failed read dump file at line {}", self.line))?;
            self.line += 1;
        }

        match self.buf.trim() {
            "" => Ok(None),
            "]" => Ok(None),
            s => Ok(Some(s.trim_end_matches(','))),
        }
    }

    pub fn read_entry<D: RootEntry>(&mut self) -> Result<Option<D>, Error> {
        let line_num = self.line;

        if let Some(line) = self.read_line()? {
            let line = D::pre_filter(line);
            let v = from_str(line.as_ref()).map_err(|e| {
                let err_pos = if e.column() > 0 { e.column() - 1 } else { 0 };

                let (line_before, line_after) = line.split_at(err_pos);
                let (line_mid, line_after) = if line_after.len() > ERROR_COLOR_LEN {
                    line_after.split_at(ERROR_COLOR_LEN)
                } else {
                    (line_after, "")
                };
                Error::msg(format!(
                    "at line {}: {}\n\twith line: {}\x1B[31m{}\x1B[m{}",
                    line_num, e, line_before, line_mid, line_after
                ))
            })?;
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }
}

pub trait Progress {
    fn inc(&mut self, delta: usize);
    fn reset_eta(&mut self) {}
}

pub struct NopProgress;

impl Progress for NopProgress {
    fn inc(&mut self, _delta: usize) {}
}

struct ProgressReaderBuilder<P: Progress> {
    progress: P,
}

impl<P: Progress> ProgressReaderBuilder<P> {
    fn new(progress: P) -> ProgressReaderBuilder<P> {
        ProgressReaderBuilder { progress }
    }
}

impl<P: 'static + Progress> ReadWrapperBuilder for ProgressReaderBuilder<P> {
    type Wrapper = ProgressReader<P>;
    fn new_wrapped_reader(self, f: File) -> ProgressReader<P> {
        ProgressReader {
            inner: f,
            progress: self.progress,
        }
    }
}

struct ProgressReader<P: Progress> {
    inner: File,
    progress: P,
}

impl<P: Progress> Read for ProgressReader<P> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.progress.inc(n);
        Ok(n)
    }
}
