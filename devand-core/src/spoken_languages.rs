use isolang::Language as IsoLang;
use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use strum_macros::{EnumIter, EnumString};

#[derive(
    Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, EnumIter, EnumString,
)]
#[serde(rename_all = "snake_case")]
pub enum SpokenLanguage {
    Ara,
    Ben,
    Cmn,
    Deu,
    Fra,
    Hin,
    Jpn,
    Pan,
    Por,
    Rus,
    Spa,
    // TODO Change to Ita, only for back compatibility
    Italian,
    // TODO Change to Eng, only for back compatibility
    English,
}

impl SpokenLanguage {
    pub fn iso(&self) -> IsoLang {
        match self {
            SpokenLanguage::Ara => IsoLang::Ara,
            SpokenLanguage::Ben => IsoLang::Ben,
            SpokenLanguage::Cmn => IsoLang::Cmn,
            SpokenLanguage::Deu => IsoLang::Deu,
            SpokenLanguage::English => IsoLang::Eng,
            SpokenLanguage::Fra => IsoLang::Fra,
            SpokenLanguage::Hin => IsoLang::Hin,
            SpokenLanguage::Italian => IsoLang::Ita,
            SpokenLanguage::Jpn => IsoLang::Jpn,
            SpokenLanguage::Pan => IsoLang::Pan,
            SpokenLanguage::Por => IsoLang::Por,
            SpokenLanguage::Rus => IsoLang::Rus,
            SpokenLanguage::Spa => IsoLang::Spa,
        }
    }
}

impl std::fmt::Display for SpokenLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iso())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spoken_lang_serde() {
        let x = vec![SpokenLanguage::Spa, SpokenLanguage::Fra];
        let j = serde_json::to_string(&x).unwrap();
        assert_eq!("[\"spa\",\"fra\"]", j);
    }

    #[test]
    fn spoken_lang_serde_back_compatibility() {
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
