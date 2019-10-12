use std::collections::BTreeMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};

use crate::err::{ErrorMessageExt, Fail};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    url: String,
    mode: Mode,
}

impl Target {
    pub fn load_list<P: AsRef<Path>>(path: P) -> Result<Vec<Target>, Fail> {
        let f = File::open(path).err_msg("can't open target list file")?;
        let list = from_reader(f).err_msg("can't parse target list file")?;
        Ok(list)
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn name(&self) -> Result<&str, Fail> {
        self.url()
            .split('/')
            .last()
            .err_msg("target URL should have name part, but not")
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }
}

#[derive(Debug, Clone)]
pub struct EtagStoreage {
    path: PathBuf,
}

impl EtagStoreage {
    pub fn new<P: AsRef<Path>>(path: P) -> EtagStoreage {
        EtagStoreage {
            path: path.as_ref().to_owned(),
        }
    }

    pub fn get(&self, target: &Target) -> Result<Option<String>, Fail> {
        if self.path.exists() {
            let f = File::open(&self.path).err_msg(format!("can't open file: {:?}", self.path))?;
            let mut table: BTreeMap<String, String> =
                from_reader(f).err_msg("can't parse ETag file")?;

            Ok(table.remove(target.url()))
        } else {
            Ok(None)
        }
    }

    pub fn save(&self, target: &Target, etag: &str) -> Result<(), Fail> {
        let mut table: BTreeMap<String, String> = if self.path.exists() {
            let f = File::open(&self.path).err_msg(format!("can't open file: {:?}", self.path))?;
            from_reader(f).err_msg("can't parse ETag file")?
        } else {
            BTreeMap::new()
        };

        table.insert(target.url().to_owned(), etag.to_owned());

        let mut f =
            File::create(&self.path).err_msg(format!("can't create file: {:?}", self.path))?;
        to_writer_pretty(&mut f, &table).err_msg("can't encode ETag file")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Small,
    Normal,
    Full,
}

impl Mode {
    pub fn parse(s: &str) -> Result<Mode, Fail> {
        let s = s.trim().to_lowercase();

        if s == "" || s == "s" || s == "small" {
            Ok(Mode::Small)
        } else if s == "n" || s == "normal" {
            Ok(Mode::Normal)
        } else if s == "f" || s == "full" {
            Ok(Mode::Full)
        } else {
            Err(Fail::new(format!("unknown mode: {}", s)))
        }
    }
}
