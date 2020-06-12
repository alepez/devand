pub mod mock;
pub mod auth;
mod affinity;

use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use strum_macros::{Display, EnumIter, EnumString};

pub use affinity::{Affinity, AffinityParams};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct User {
    /// This is unique and cannot be changed
    pub id: i32,
    /// This is unique and cannot be changed
    pub username: String,
    /// This is unique
    pub email: String,
    /// This name is shown on human readable content (chat, email, ...)
    pub visible_name: String,
    /// All user settings are here
    pub settings: UserSettings,
}

pub type Languages = BTreeMap<Language, LanguagePreference>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserSettings {
    /// User can set language preferences
    pub languages: Languages,
    /// User must set a schedule
    pub schedule: Schedule,
    /// User can disable all activities without losing schedule
    pub vacation_mode: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Schedule {
    /// Disabled
    Never,
    /// Schedule every week in the future
    Weekly(WeekSchedule),
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule::Never
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DaySchedule {
    pub hours: [bool; 24],
}

impl TryFrom<&str> for DaySchedule {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        if s == "*" {
            return Ok(Self::always());
        }

        let numbers: Vec<usize> = s.split(',').filter_map(|x| x.parse().ok()).collect();

        if numbers.is_empty() {
            return Ok(Self::never());
        }

        let mut hours: [bool; Self::HOURS_IN_DAY] = [false; Self::HOURS_IN_DAY];

        for n in numbers {
            if n < hours.len() {
                hours[n] = true;
            }
        }

        Ok(DaySchedule { hours })
    }
}

impl DaySchedule {
    pub const HOURS_IN_DAY: usize = 24;

    pub fn never() -> Self {
        DaySchedule {
            hours: [false; Self::HOURS_IN_DAY],
        }
    }

    pub fn always() -> Self {
        DaySchedule {
            hours: [true; Self::HOURS_IN_DAY],
        }
    }

    pub fn new(hours: [bool; Self::HOURS_IN_DAY]) -> Self {
        DaySchedule { hours }
    }
}

/// Week scheduling
#[derive(Debug, Serialize, Deserialize, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct WeekSchedule {
    pub mon: DaySchedule,
    pub tue: DaySchedule,
    pub wed: DaySchedule,
    pub thu: DaySchedule,
    pub fri: DaySchedule,
    pub sat: DaySchedule,
    pub sun: DaySchedule,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, EnumIter, Display, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum Level {
    Novice,
    Proficient,
    Expert,
}

impl Level {
    pub fn as_number(&self) -> usize {
        match self {
            Level::Novice => 1,
            Level::Proficient => 2,
            Level::Expert => 3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, EnumIter, Display, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    /// No: because user can add a known language, but may not want to use it
    No,
    /// When a match is found, higher priority are chosen over low priority
    Low,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LanguagePreference {
    pub level: Level,
    pub priority: Priority,
}

impl Default for LanguagePreference {
    fn default() -> Self {
        Self {
            level: Level::Novice,
            priority: Priority::No,
        }
    }
}

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
pub enum Language {
    C,
    CPlusPlus,
    CSharp,
    Clojure,
    Dart,
    Elixir,
    Erlang,
    FSharp,
    Go,
    Java,
    Javascript,
    Kotlin,
    ObjectiveC,
    PHP,
    Python,
    R,
    Ruby,
    Rust,
    Scala,
    Swift,
    TypeScript,
    VBA,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_example() {
        let user = mock::user();
        let _json = serde_json::to_string(&user).unwrap();
    }

    #[test]
    fn hourly_schedule_from_comma_separated_list() {
        let s = "5,7,21";
        let schedule = DaySchedule::try_from(s).unwrap();
        assert!(schedule.hours[4] == false);
        assert!(schedule.hours[5] == true);
        assert!(schedule.hours[6] == false);
        assert!(schedule.hours[7] == true);
        assert!(schedule.hours[21] == true);
        assert!(schedule.hours[22] == false);
    }
}
