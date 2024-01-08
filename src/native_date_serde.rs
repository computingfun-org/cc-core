pub const NAIVE_DATE_FORMAT: &'static str = "%F";

pub fn serialize<S>(date: &chrono::NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.format(NAIVE_DATE_FORMAT).to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<chrono::NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    chrono::NaiveDate::parse_from_str(
        &<String as serde::Deserialize>::deserialize(deserializer)?,
        NAIVE_DATE_FORMAT,
    )
    .map_err(serde::de::Error::custom)
}
