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

use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait RootEntry: 'static + Send + Sync + DeserializeOwned + Serialize {
    fn entry_id(&self) -> u64;
    fn type_name() -> &'static str;
    fn time(&self) -> DateTime<Utc>;

    fn pre_filter(s: &str) -> Cow<'_, str> {
        Cow::Borrowed(s)
    }
}
