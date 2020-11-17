pub mod bgs;
pub mod body;
pub mod powerplay;
pub mod station;
pub mod system;
pub mod system_populated;

#[macro_use]
mod util;
mod dec;

use std::borrow::Cow;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_slice;

pub trait RootEntry: 'static + Send + Sync + DeserializeOwned + Serialize {
    fn entry_id(&self) -> u64;
    fn type_name() -> &'static str;
    fn time(&self) -> DateTime<Utc>;

    fn parse_dump_json(bs: &[u8]) -> Result<Self> {
        from_slice(bs).context("parsing entry")
    }

    #[deprecated = "use parse_dump_json()"]
    fn pre_filter(s: &str) -> Cow<'_, str> {
        Cow::Borrowed(s)
    }
}
