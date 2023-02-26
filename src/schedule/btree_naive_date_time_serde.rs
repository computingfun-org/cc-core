use std::collections::BTreeSet;

use chrono::NaiveDateTime;

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

const FORMAT: &'static str = "%c";
const SEPARATOR: &'static str = ", ";

pub fn serialize<S>(btree: &BTreeSet<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    Serialize::serialize(
        &btree
            .iter()
            .map(|dt| dt.format(FORMAT).to_string())
            .collect::<Vec<String>>()
            .join(SEPARATOR),
        serializer,
    )
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<BTreeSet<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut btree = BTreeSet::new();
    for result in String::deserialize(deserializer)?
        .split(SEPARATOR)
        .filter(|s| !s.is_empty())
        .map(|s| {
            NaiveDateTime::parse_from_str(s, FORMAT)
                .map_err(|err| D::Error::custom(err.to_string()))
        })
    {
        btree.insert(result?);
    }
    Ok(btree)
}
