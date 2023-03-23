use chrono::NaiveDate;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

const FORMAT: &'static str = "%F";

pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(date) => Serialize::serialize(&date.format(FORMAT).to_string(), serializer),
        None => Serialize::serialize("", serializer),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let data: String = Deserialize::deserialize(deserializer)?;
    match data.is_empty() {
        true => Ok(None),
        false => Ok(Some(
            NaiveDate::parse_from_str(&data, FORMAT)
                .map_err(|err| Error::custom(err.to_string()))?,
        )),
    }
}
