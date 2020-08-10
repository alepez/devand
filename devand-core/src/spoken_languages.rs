use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Copy,
    Clone,
    EnumIter,
    Display,
    EnumString,
)]
#[serde(rename_all = "snake_case")]
pub enum SpokenLanguage {
    English,
    Italian,
}
