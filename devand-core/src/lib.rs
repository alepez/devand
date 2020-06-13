mod affinity;
pub mod auth;
pub mod mock;

use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use strum_macros::{Display, EnumIter, EnumString};

pub use affinity::{Affinity, AffinityParams};

pub type UserId = i32;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct User {
    /// This is unique and cannot be changed
    pub id: UserId,
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

#[derive(Serialize, Deserialize)]
pub struct UserAffinity {
    pub user: PublicUserProfile,
    pub affinity: Affinity,
}

impl UserAffinity {
    pub fn new(user: PublicUserProfile, affinity: Affinity) -> Self {
        Self { user, affinity }
    }
}

impl From<&User> for AffinityParams {
    fn from(user: &User) -> Self {
        let languages = user.settings.languages.clone();
        AffinityParams::new().with_languages(languages)
    }
}

/// Calculate affinities between `user` and all `users` passed
pub fn calculate_affinities(
    user: User,
    users: impl IntoIterator<Item = User>,
) -> impl Iterator<Item = UserAffinity> {
    let user_id = user.id;
    let user_params = AffinityParams::from(&user);

    users
        .into_iter()
        // There may be same user in the list, just skip it
        .filter(move |u| u.id != user_id)
        // Calculate the affinity
        .map(move |u| {
            let u_params = AffinityParams::from(&u);
            // TODO Avoid cloning logged user params
            let affinity = Affinity::from_params(&user_params, &u_params);
            UserAffinity::new(u.into(), affinity)
        })
        // Remove users who do not have any affinity
        .filter(|aff| aff.affinity != Affinity::NONE)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct PublicUserProfile {
    pub username: String,
    pub visible_name: String,
    pub languages: Languages,
}

impl From<User> for PublicUserProfile {
    fn from(user: User) -> Self {
        PublicUserProfile {
            username: user.username,
            visible_name: user.visible_name,
            languages: user.settings.languages,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct CodeNowUsers {
    users: std::collections::HashMap<UserId, PublicUserProfile>,
}

impl Serialize for CodeNowUsers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("code_now_users", 3)?;
        let users : Vec<_> = self.users.values().collect();
        state.serialize_field("users", &users)?;
        state.end()
    }
}

impl CodeNowUsers {
    pub fn add(&mut self, u: User) {
        let id = u.id;
        let profile = PublicUserProfile::from(u);
        self.users.insert(id, profile);
    }
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

    #[test]
    fn serialize_code_now_users() {
        let mut code_now_users = CodeNowUsers::default();
        let user = crate::mock::user();
        code_now_users.add(user);
        let json = serde_json::to_string(&code_now_users);
        dbg!(json);
    }
}
