use std::io::BufRead;

use serde::de::DeserializeOwned;
use serde_json::from_str;
use tiny_fail::{ErrorMessageExt, Fail};


#[derive(Debug)]
pub struct ArrayDecoder<R: BufRead> {
    r: R,
    line: usize,
    buf: String,
}

impl<R: BufRead> ArrayDecoder<R> {
    pub fn new(r: R) -> ArrayDecoder<R> {
        ArrayDecoder {
            r,
            line: 0,
            buf: String::new(),
        }
    }

    fn read_line(&mut self) -> Result<Option<&str>, Fail> {
        self.buf.truncate(0);
        self.r
            .read_line(&mut self.buf)
            .err_msg(format!("failed read dump file at line {}", self.line))?;
        self.line += 1;

        if self.buf.trim() == "[" {
            self.buf.truncate(0);
            self.r
                .read_line(&mut self.buf)
                .err_msg(format!("failed read dump file at line {}", self.line))?;
            self.line += 1;
        }

        match self.buf.trim() {
            "" => return Ok(None),
            "]" => return Ok(None),
            s => return Ok(Some(s.trim_end_matches(','))),
        }
    }

    pub fn next<D: DeserializeOwned>(&mut self) -> Result<Option<D>, Fail> {
        let line_num = self.line;

        if let Some(line) = self.read_line()? {
            let v = from_str(line).map_err(|e| Fail::new(format!("at line {}: {}\n\twith line: {}", line_num, e, line)))?;
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }
}
