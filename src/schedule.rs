use non_empty_string::NonEmptyString;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Notes {
    pub tags: Vec<NonEmptyString>,
    pub price: String,
    pub spaces: usize,
    pub access: String,
    pub tear_out: String,
}

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct State {
    pub deposit: bool,
    pub confirmation: bool,
    pub marked_ready: bool,
}

#[derive(
    Debug,
    Default,
    derive_more::Constructor,
    derive_more::From,
    derive_more::Into,
    Clone,
    Copy,
    derive_more::FromStr,
    derive_more::Add,
    derive_more::Sub,
    derive_more::Display,
    derive_more::Not,
    derive_more::Sum,
    derive_more::AddAssign,
    derive_more::SubAssign,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(transparent)]
pub struct LeadTime(isize);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LeadItem {
    pub name: NonEmptyString,
    pub lead: LeadTime,
}
