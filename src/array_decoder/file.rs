use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use either::Either;
use flate2::bufread::GzDecoder;

const BUFFER_SIZE: usize = 16 * 1024;

type DetectReader = Either<BufReader<GzDecoder<BufReader<File>>>, BufReader<File>>;

pub fn open_detect(path: &Path) -> Result<DetectReader> {
    let file_name = path
        .file_name()
        .context("file has no name")?
        .to_string_lossy();

    let f = File::open(path).context("open file")?;
    let r = BufReader::with_capacity(BUFFER_SIZE, f);
    if file_name.ends_with(".gz") {
        let d = GzDecoder::new(r);
        let r = BufReader::with_capacity(BUFFER_SIZE, d);
        Ok(Either::Left(r))
    } else {
        Ok(Either::Right(r))
    }
}
