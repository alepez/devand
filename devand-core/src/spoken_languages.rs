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

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct SpokenLanguages(pub std::collections::BTreeSet<SpokenLanguage>);

impl std::ops::Deref for SpokenLanguages {
    type Target = std::collections::BTreeSet<SpokenLanguage>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SpokenLanguages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Check if users have compatible languages. Users without any language set
/// are compatible with everybody by default.
pub fn are_spoken_language_compatible(l: &SpokenLanguages, r: &SpokenLanguages) -> bool {
    l.0.is_empty() || r.0.is_empty() || l.0.iter().any(|x| r.0.iter().any(|y| x == y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::btreeset;

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

    #[test]
    fn are_spoken_language_compatible_empty_is_yes() {
        assert!(are_spoken_language_compatible(
            &SpokenLanguages(btreeset![]),
            &SpokenLanguages(btreeset![SpokenLanguage::English]),
        ));
        assert!(are_spoken_language_compatible(
            &SpokenLanguages(btreeset![SpokenLanguage::English]),
            &SpokenLanguages(btreeset![]),
        ));
    }

    #[test]
    fn are_spoken_language_compatible_yes() {
        assert!(are_spoken_language_compatible(
            &SpokenLanguages(btreeset![SpokenLanguage::English]),
            &SpokenLanguages(btreeset![SpokenLanguage::English])
        ));
    }

    #[test]
    fn are_spoken_language_compatible_no() {
        assert!(!are_spoken_language_compatible(
            &SpokenLanguages(btreeset![SpokenLanguage::Italian]),
            &SpokenLanguages(btreeset![SpokenLanguage::English])
        ));
    }
}
