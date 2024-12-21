use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(date) => {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;

    match s {
        Some(s) => {
            let date_naive =
                NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
            Ok(Some(date_naive.and_utc()))
        }
        None => Ok(None),
    }
}
