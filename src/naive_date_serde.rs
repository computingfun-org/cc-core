use chrono::NaiveDate;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

const FORMAT: &'static str = "%D";

pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    String::serialize(&date.format(FORMAT).to_string(), serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    NaiveDate::parse_from_str(&String::deserialize(deserializer)?, FORMAT)
        .map_err(|err| Error::custom(err.to_string()))
}
