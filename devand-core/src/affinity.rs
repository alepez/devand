#![allow(unused_imports)]
#![allow(dead_code)]

use crate::Language;
use crate::LanguagePreference;
use crate::Languages;
use crate::Level;
use crate::Priority;
use std::collections::BTreeMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Affinity(i32);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct PairPriority(i32);

impl PairPriority {
    fn new(a: Priority, b: Priority) -> Self {
        let a = Self::priority_score(a);
        let b = Self::priority_score(b);
        Self(a * b)
    }

    fn priority_score(p: Priority) -> i32 {
        match p {
            Priority::No => 0,
            Priority::Low => 1,
            Priority::High => 2,
        }
    }
}

#[derive(Debug, Default)]
struct AffinityParams {
    languages: Languages,
}

impl AffinityParams {
    fn new() -> Self {
        Self::default()
    }

    fn with_languages(mut self, languages: Languages) -> Self {
        self.languages = languages;
        self
    }
}

type MatchingLanguage = (Language, (LanguagePreference, LanguagePreference));
struct MatchingLanguages(BTreeMap<Language, (LanguagePreference, LanguagePreference)>);

impl MatchingLanguages {
    fn find_max_affinity(&self) -> (&MatchingLanguage, Affinity) {
        todo!()
    }
}

fn find_matching_languages(mut a: Languages, mut b: Languages) -> MatchingLanguages {
    use strum::IntoEnumIterator;

    // TODO Algorithm can be optimized
    let matching = Language::iter()
        .map(|lang| -> Option<MatchingLanguage> {
            let a_lang = a.remove(&lang)?;
            let b_lang = b.remove(&lang)?;
            Some((lang, (a_lang, b_lang)))
        })
        .filter_map(|x| x)
        .collect();

    MatchingLanguages(matching)
}

impl Affinity {
    pub const NONE: Self = Affinity(Self::MIN);
    pub const FULL: Self = Affinity(Self::MAX);

    const MIN: i32 = 0;
    const MAX: i32 = 100;

    fn from_params(a: AffinityParams, b: AffinityParams) -> Self {
        let matching_languages = find_matching_languages(a.languages, b.languages);

        if matching_languages.0.is_empty() {
            return Self::NONE;
        }

        Self::FULL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_params_have_no_affinity() {
        let a = AffinityParams::new();
        let b = AffinityParams::new();

        assert!(Affinity::from_params(a, b) == Affinity::NONE);
    }

    #[test]
    fn same_params_with_high_priority_have_full_affinity() {
        let mut languages = Languages::new();
        languages.insert(
            Language::Rust,
            LanguagePreference {
                level: Level::Expert,
                priority: Priority::High,
            },
        );
        let a = AffinityParams::new().with_languages(languages.clone());
        let b = AffinityParams::new().with_languages(languages.clone());

        assert!(Affinity::from_params(a, b) == Affinity::FULL);
    }

    #[test]
    fn same_params_with_low_priority_have_low_affinity() {
        let mut languages = Languages::new();
        languages.insert(
            Language::Rust,
            LanguagePreference {
                level: Level::Expert,
                priority: Priority::Low,
            },
        );
        let a = AffinityParams::new().with_languages(languages.clone());
        let b = AffinityParams::new().with_languages(languages.clone());

        let affinity = Affinity::from_params(a, b);

        // assert!(affinity < Affinity::FULL);
        assert!(affinity > Affinity::NONE);
    }

    #[test]
    fn find_matching_languages_ok() {
        let mut languages = Languages::new();
        languages.insert(
            Language::Rust,
            LanguagePreference {
                level: Level::Expert,
                priority: Priority::Low,
            },
        );
        let a = languages.clone();
        let b = languages.clone();

        let matching = find_matching_languages(a, b);

        assert!(matching.0.len() == 1);
        assert!(matching.0.get(&Language::Rust).is_some());
    }

    #[test]
    fn find_matching_languages_none() {
        let languages = Languages::new();
        let a = languages.clone();
        let b = languages.clone();

        let matching = find_matching_languages(a, b);

        assert!(matching.0.is_empty());
    }
}
