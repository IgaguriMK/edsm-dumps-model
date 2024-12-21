use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let date_naive = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
    Ok(date_naive.and_utc())
}
