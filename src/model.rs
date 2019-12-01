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

use serde::de::DeserializeOwned;

pub trait RootEntry: DeserializeOwned {
    fn pre_filter(s: &str) -> Cow<'_, str> {
        Cow::Borrowed(s)
    }
}

impl RootEntry for serde_json::Value {}
