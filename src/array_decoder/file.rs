use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

use anyhow::{Context, Result};
use either::Either;
use flate2::bufread::GzDecoder;

use super::Progress;

type ProgressFile<P> = ProgressReader<File, P>;

pub struct DetectReader<P: Progress> {
    r: BufReader<Either<GzDecoder<ProgressFile<P>>, ProgressFile<P>>>,
}

impl<P: Progress> DetectReader<P> {
    pub fn open_detect(path: impl AsRef<Path>, progress: P) -> Result<DetectReader<P>> {
        let path = path.as_ref();
        let file_name = path
            .file_name()
            .context("file has no name")?
            .to_string_lossy();
        let f = File::open(path).context("open file")?;
        let p = ProgressReader::new(f, progress);
        if file_name.ends_with(".gz") {
            let d = GzDecoder::new(p);
            Ok(DetectReader {
                r: BufReader::new(Either::Left(d)),
            })
        } else {
            Ok(DetectReader {
                r: BufReader::new(Either::Right(p)),
            })
        }
    }
}

impl<P: Progress> Read for DetectReader<P> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.r.read(buf)
    }
}

impl<P: Progress> BufRead for DetectReader<P> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.r.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.r.consume(amt)
    }
}

pub struct ProgressReader<R: Read, P: Progress> {
    r: BufReader<R>,
    progress: P,
}

impl<R: Read, P: Progress> ProgressReader<R, P> {
    pub fn new(r: R, progress: P) -> ProgressReader<R, P> {
        ProgressReader {
            r: BufReader::new(r),
            progress,
        }
    }
}

impl<R: Read, P: Progress> Read for ProgressReader<R, P> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.r.read(buf)?;
        self.progress.inc(n);
        Ok(n)
    }
}

impl<R: Read, P: Progress> BufRead for ProgressReader<R, P> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.r.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.progress.inc(amt);
        self.r.consume(amt);
    }
}
