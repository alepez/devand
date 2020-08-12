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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spoken_lang_serde() {
        let x = vec![SpokenLanguage::English, SpokenLanguage::Italian];
        let j = serde_json::to_string(&x).unwrap();
        assert_eq!("[\"english\",\"italian\"]", j);
    }

    #[test]
    fn spoken_lang_display() {
        let x = SpokenLanguage::Italian;
        let f = format!("{}", x);
        assert_eq!("Italian", f);
    }
}
