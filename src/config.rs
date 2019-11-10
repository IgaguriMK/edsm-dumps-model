use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tiny_fail::{ErrorMessageExt, Fail};
use toml::from_slice;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    dumps_dir: Option<PathBuf>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, Fail> {
        let path = path.as_ref();
        let mut f = File::open(path).err_msg(format!("failed load config file '{:?}'", path))?;

        let mut buf = Vec::new();
        f.read_to_end(&mut buf)
            .err_msg("error caused while reading config file")?;

        let cfg: Config = from_slice(&buf).err_msg("failed parse config file")?;
        Ok(cfg)
    }

    pub fn dumps_dir(&self) -> Cow<'_, Path> {
        match self.dumps_dir {
            Some(ref v) => Cow::Borrowed(v),
            None => Cow::Owned(".".into()),
        }
    }
}
