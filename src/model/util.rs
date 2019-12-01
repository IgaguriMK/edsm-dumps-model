use std::fmt;

use serde::Serialize;
use serde_json::to_string;

pub trait DisplayViaSerde: Serialize {
    fn display_via_serde(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", s.trim_start_matches('"').trim_end_matches('"'))
    }
}

#[macro_export]
macro_rules! display_via_serde {
    ($t:ty) => {
        impl DisplayViaSerde for $t {}

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.display_via_serde(f)
            }
        }
    };
}
