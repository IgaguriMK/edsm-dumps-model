pub mod parallel;

use std::io::BufRead;

use anyhow::{Context, Error};
use serde_json::from_str;

use crate::model::RootEntry;

const ERROR_COLOR_LEN: usize = 20;

#[derive(Debug)]
pub struct ArrayDecoder<R: BufRead, P: Progress = NopProgress> {
    r: R,
    line: usize,
    buf: String,
    progress: P,
}

impl<R: BufRead> ArrayDecoder<R> {
    pub fn new(r: R) -> ArrayDecoder<R> {
        ArrayDecoder {
            r,
            line: 0,
            buf: String::new(),
            progress: NopProgress,
        }
    }

    pub fn set_progress<P: Progress>(self, progress: P) -> ArrayDecoder<R, P> {
        ArrayDecoder {
            r: self.r,
            line: self.line,
            buf: String::new(),
            progress,
        }
    }
}

impl<R: BufRead, P: Progress> ArrayDecoder<R, P> {
    fn read_line(&mut self) -> Result<Option<&str>, Error> {
        self.buf.truncate(0);
        let n = self
            .r
            .read_line(&mut self.buf)
            .context(format!("failed read dump file at line {}", self.line))?;
        self.progress.inc(n);
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
