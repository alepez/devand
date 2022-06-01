use crate::Language;
use crate::LanguagePreference;
use crate::Languages;
use crate::Level;
use crate::Priority;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct PairPriority(i32);

impl PairPriority {
    const MAX: Self = PairPriority(4);

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

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct PairLevel(i32);

impl PairLevel {
    const MAX: Self = PairLevel(3);

    #[allow(dead_code)]
    const MEDIUM: Self = PairLevel(2);
    #[allow(dead_code)]
    const MIN: Self = PairLevel(1);

    fn new(a: Level, b: Level) -> Self {
        let diff = ((a.as_number() as i32) - (b.as_number() as i32)).abs();
        let score = Self::MAX.0 - (diff as i32);
        Self(score)
    }
}

#[derive(Default, Clone, Debug)]
pub struct AffinityParams {
    languages: Languages,
}

impl AffinityParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_languages(mut self, languages: Languages) -> Self {
        self.languages = languages;
        self
    }
}

impl From<Vec<(Language, LanguagePreference)>> for AffinityParams {
    fn from(v: Vec<(Language, LanguagePreference)>) -> Self {
        let languages = Languages(v.into_iter().collect());
        Self { languages }
    }
}

struct MatchingLanguages(BTreeMap<Language, (LanguagePreference, LanguagePreference)>);

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct LanguageAffinity(i32);

impl LanguageAffinity {
    const MAX: Self = Self(PairPriority::MAX.0 * PairLevel::MAX.0);

    fn new(a: &LanguagePreference, b: &LanguagePreference) -> Self {
        let pair_prio = PairPriority::new(a.priority, b.priority);
        let pair_level = PairLevel::new(a.level, b.level);
        LanguageAffinity(pair_prio.0 * pair_level.0)
    }
}

impl MatchingLanguages {
    fn find_max_affinity(&self) -> Option<(Language, LanguageAffinity)> {
        self.0
            .iter()
            .map(|(lang, (a, b))| {
                let aff = LanguageAffinity::new(a, b);
                (lang, aff)
            })
            .max_by(|(_, l), (_, r)| l.cmp(r))
            .map(|(&lang, aff)| (lang, aff))
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

/// Find the intersection between the two collections a and b, extracting
/// only items with the same key
fn find_matching_languages(a: &Languages, b: &Languages) -> MatchingLanguages {
    let matching = a
        .iter()
        .filter_map(|(lang, a_pref)| b.get(lang).map(|b_pref| (lang, (a_pref, b_pref))))
        .filter(|(_, (a_pref, b_pref))| {
            a_pref.priority > Priority::No && b_pref.priority > Priority::No
        })
        .map(|(&lang, (a_pref, b_pref))| (lang, (a_pref.clone(), b_pref.clone())))
        .collect();

    MatchingLanguages(matching)
}

// NOTE Use i32 instead of f64 so it can derive Eq
// TODO Normalize to [0..1] when serializing
#[derive(Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Affinity(i32);

#[derive(Copy, Clone)]
pub enum AffinityLevel {
    Low,
    Medium,
    High,
}

impl Affinity {
    pub const NONE: Self = Affinity(Self::MIN);
    pub const FULL: Self = Affinity(Self::MAX);

    const MIN: i32 = 0;
    const MAX: i32 = 1000;

    const BEST_LANG_SCORE_WEIGHT: f64 = 0.8;

    pub fn from_params(a: &AffinityParams, b: &AffinityParams) -> Self {
        let matching_languages = find_matching_languages(&a.languages, &b.languages);
        let best_lang = matching_languages.find_max_affinity();

        if let Some((_, best_lang_score)) = best_lang {
            let best_lang_score = (best_lang_score.0 as f64) / (LanguageAffinity::MAX.0 as f64);
            let best_lang_score = best_lang_score * Self::BEST_LANG_SCORE_WEIGHT;

            let matching_languages_count = matching_languages.len();
            let total_languages_count = a.languages.union(&b.languages).len();
            let matching_ratio = (matching_languages_count as f64) / (total_languages_count as f64);
            let matching_ratio = matching_ratio * (1.0 - Self::BEST_LANG_SCORE_WEIGHT);

            let score = best_lang_score + matching_ratio;

            Self((score * (Self::MAX as f64)) as i32)
        } else {
            Self::NONE
        }
    }

    pub fn normalize(&self) -> f64 {
        (self.0 as f64) / (Self::MAX as f64)
    }

    pub fn from_number(n: f64) -> Self {
        Affinity(match n {
            n if n < 0.0 => Self::MIN,
            n if n > 1.0 => Self::MAX,
            n => (n * (Self::MAX as f64)) as i32,
        })
    }

    pub fn is_zero(&self) -> bool {
        self.0 <= Self::MIN
    }
}

impl From<Affinity> for AffinityLevel {
    fn from(affinity: Affinity) -> Self {
        let x = affinity.normalize();
        match x {
            x if x >= 0.6 => AffinityLevel::High,
            x if x >= 0.3 => AffinityLevel::Medium,
            _ => AffinityLevel::Low,
        }
    }
}

impl ToString for Affinity {
    fn to_string(&self) -> String {
        format!("{:.0}%", self.normalize() * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_params_have_no_affinity() {
        let a = AffinityParams::new();
        let b = AffinityParams::new();

        assert!(Affinity::from_params(&a, &b) == Affinity::NONE);
    }

    #[test]
    fn same_params_with_high_priority_have_full_affinity() {
        let mut languages = Languages::default();
        languages.insert(
            Language::Rust,
            LanguagePreference {
                level: Level::Expert,
                priority: Priority::High,
            },
        );
        let a = AffinityParams::new().with_languages(languages.clone());
        let b = AffinityParams::new().with_languages(languages.clone());

        let affinity = Affinity::from_params(&a, &b);

        assert!(affinity == Affinity::FULL);
    }

    #[test]
    fn same_level_with_low_priority_have_low_affinity() {
        let mut languages = Languages::default();
        languages.insert(
            Language::Rust,
            LanguagePreference {
                level: Level::Expert,
                priority: Priority::Low,
            },
        );
        let a = AffinityParams::new().with_languages(languages.clone());
        let b = AffinityParams::new().with_languages(languages.clone());

        let affinity = Affinity::from_params(&a, &b);

        assert!(affinity < Affinity::FULL);
        assert!(affinity > Affinity::NONE);
        assert!(affinity.0 == 399);
    }

    #[test]
    fn distant_level_with_high_priority_low_affinity() {
        let a = {
            let mut languages = Languages::default();
            languages.insert(
                Language::Rust,
                LanguagePreference {
                    level: Level::Expert,
                    priority: Priority::High,
                },
            );
            AffinityParams::new().with_languages(languages)
        };

        let b = {
            let mut languages = Languages::default();
            languages.insert(
                Language::Rust,
                LanguagePreference {
                    level: Level::Novice,
                    priority: Priority::High,
                },
            );
            AffinityParams::new().with_languages(languages)
        };

        let affinity = Affinity::from_params(&a, &b);

        assert!(affinity < Affinity::FULL);
        assert!(affinity > Affinity::NONE);
        // assert!(affinity.0 == 12);
    }

    #[test]
    fn find_matching_languages_ok() {
        let mut languages = Languages::default();
        languages.insert(
            Language::Rust,
            LanguagePreference {
                level: Level::Expert,
                priority: Priority::Low,
            },
        );
        let a = languages.clone();
        let b = languages.clone();

        let matching = find_matching_languages(&a, &b);

        assert!(matching.0.len() == 1);
        assert!(matching.0.get(&Language::Rust).is_some());
    }

    #[test]
    fn find_matching_languages_none() {
        let languages = Languages::default();
        let a = languages.clone();
        let b = languages;

        let matching = find_matching_languages(&a, &b);

        assert!(matching.0.is_empty());
    }

    #[test]
    fn normalize_full_affinity_to_one() {
        assert!(Affinity::FULL.normalize() == 1.0);
    }

    #[test]
    fn normalize_no_affinity_to_zero() {
        assert!(Affinity::NONE.normalize() == 0.0);
    }

    #[test]
    fn same_level_is_max() {
        assert!(PairLevel::new(Level::Expert, Level::Expert) == PairLevel::MAX);
        assert!(PairLevel::new(Level::Proficient, Level::Proficient) == PairLevel::MAX);
        assert!(PairLevel::new(Level::Novice, Level::Novice) == PairLevel::MAX);
    }

    #[test]
    fn distant_level_is_min() {
        assert!(PairLevel::new(Level::Expert, Level::Novice) == PairLevel::MIN);
        assert!(PairLevel::new(Level::Novice, Level::Expert) == PairLevel::MIN);
    }

    #[test]
    fn convert_user_to_affinity_params() {
        let user = crate::mock::user();
        let _aff_params = AffinityParams::from(&user);
    }

    #[test]
    fn convert_pub_user_prof_to_affinity_params() {
        let user = crate::mock::user();
        let public_user_profile: crate::PublicUserProfile = user.into();
        let _aff_params = AffinityParams::from(&public_user_profile);
    }
}
