use self::note::Note;

pub mod note;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    #[serde(
        serialize_with = "serialize_naive_date",
        deserialize_with = "deserialize_naive_date"
    )]
    pub install: chrono::NaiveDate,
    pub deposit: bool,
    pub confirmation: bool,
    pub marked_ready: bool,
    #[serde(flatten)]
    pub note: Note,
    pub lead_items: Vec<LeadItem>,
    pub process: usize,
    pub paperwork: usize,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            install: Default::default(),
            deposit: false,
            confirmation: false,
            marked_ready: false,
            note: Default::default(),
            lead_items: vec![LeadItem {
                name: "EMC".into(),
                lead: 15,
            }],
            process: 5,
            paperwork: 5,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LeadItem {
    pub name: String,
    pub lead: usize,
}

pub const NAIVE_DATE_FORMAT: &'static str = "%F";

fn serialize_naive_date<S>(date: &chrono::NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.format(NAIVE_DATE_FORMAT).to_string())
}

fn deserialize_naive_date<'de, D>(deserializer: D) -> Result<chrono::NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    chrono::NaiveDate::parse_from_str(
        &<String as serde::Deserialize>::deserialize(deserializer)?,
        NAIVE_DATE_FORMAT,
    )
    .map_err(serde::de::Error::custom)
}
