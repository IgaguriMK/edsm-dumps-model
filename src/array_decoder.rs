pub mod parallel;

use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

use anyhow::{Context, Error};
use detect_compression::{DetectReader, ReadWrapperBuilder};

use crate::model::RootEntry;

pub struct ArrayDecoder {
    r: DetectReader,
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
            buf: String::new(),
        })
    }
}

impl ArrayDecoder {
    fn read_line(&mut self) -> Result<Option<&str>, Error> {
        self.buf.truncate(0);
        self.r
            .read_line(&mut self.buf)
            .context("failed read dump file at line")?;

        if self.buf.trim() == "[" {
            self.buf.truncate(0);
            self.r
                .read_line(&mut self.buf)
                .context("failed read dump file at line")?;
        }

        match self.buf.trim().trim_end_matches(',') {
            "" => Ok(None),
            "]" => Ok(None),
            s => Ok(Some(s)),
        }
    }

    pub fn read_entry<D: RootEntry>(&mut self) -> Result<Option<D>, Error> {
        if let Some(line) = self.read_line()? {
            let v = D::parse_dump_json(line.as_bytes())
                .with_context(|| format!("failed parse line:\"{}\"", line))?;
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
