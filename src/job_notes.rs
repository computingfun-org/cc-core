#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct JobNotes {
    pub tags: Tags,
    pub price: usize,
    pub spaces: usize,
    pub access: Access,
    pub tear_out: TearOut,
}

impl JobNotes {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for JobNotes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}${}, {} spaces, {}, {}",
            self.tags, self.price, self.spaces, self.access, self.tear_out
        ))
    }
}

#[derive(Debug, derive_more::Display, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tag(Box<str>);

impl Tag {
    pub fn new(tag: non_empty_string::NonEmptyString) -> Self {
        Self::from(tag)
    }
}

impl From<non_empty_string::NonEmptyString> for Tag {
    fn from(value: non_empty_string::NonEmptyString) -> Self {
        Self(value.into_boxed_str())
    }
}

impl TryFrom<&str> for Tag {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.is_empty() {
            true => Err(()),
            false => Ok(Self(value.into())),
        }
    }
}

impl TryFrom<String> for Tag {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match non_empty_string::NonEmptyString::new(value) {
            Ok(non_empty) => Ok(Self::from(non_empty)),
            Err(empty) => Err(empty),
        }
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tags(Vec<Tag>);

impl Tags {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, tag: Tag) {
        self.0.push(tag);
    }

    pub fn remove<T>(&mut self, tag: &Tag) {
        self.0.retain(|i| i != tag);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tag> {
        self.0.iter()
    }
}

impl std::fmt::Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter()
            .try_for_each(|t| f.write_fmt(format_args!("{}, ", t)))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Access {
    House(Floors),
    TownHouse(Floors),
    Condo(CondoName),
    Custom(Box<str>),
}

pub type Floors = std::num::NonZeroUsize;

pub type CondoName = Box<str>;

impl Default for Access {
    fn default() -> Self {
        Self::House(Floors::MIN)
    }
}

impl std::fmt::Display for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Access::House(floors) => f.write_fmt(format_args!("{} floor house", floors)),
            Access::TownHouse(floors) => f.write_fmt(format_args!("{} floor townhouse", floors)),
            Access::Condo(name) => f.write_fmt(format_args!("{} condo", name)),
            Access::Custom(access) => f.write_str(access),
        }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub enum TearOut {
    #[default]
    None,
    LightWire,
    Wire,
    HeavyWire,
    VentilatedWood,
    CCSystem,
    Custom(String),
}

impl std::fmt::Display for TearOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TearOut::None => f.write_str("no tear out"),
            TearOut::LightWire => f.write_str("light wire tear out"),
            TearOut::Wire => f.write_str("wire tear out"),
            TearOut::HeavyWire => f.write_str("heavy wire tear out"),
            TearOut::VentilatedWood => f.write_str("ventilated wood tear out"),
            TearOut::CCSystem => f.write_str("CC tear out"),
            TearOut::Custom(tear_out) => f.write_str(&tear_out),
        }
    }
}
