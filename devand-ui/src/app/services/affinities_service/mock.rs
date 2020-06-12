use super::FetchCallback;
use devand_core::UserAffinity;
use devand_core::Affinity;

pub struct AffinitiesService {
    callback: FetchCallback,
}

impl AffinitiesService {
    pub fn new(callback: FetchCallback) -> Self {
        Self { callback }
    }

    pub fn restore(&mut self) {
        self.callback.emit(Ok(mock_affinities()))
    }
}

fn mock_affinities() -> Vec<UserAffinity> {
    vec![
        UserAffinity::new(mock_user(1, "aaaaa"), Affinity::from_number(0.1)),
        UserAffinity::new(mock_user(2, "bbbbb"), Affinity::from_number(0.2)),
        UserAffinity::new(mock_user(3, "ccccc"), Affinity::from_number(0.5)),
        UserAffinity::new(mock_user(4, "ddddd"), Affinity::from_number(1.0)),
    ]
}

fn mock_user(id: i32, username: &str) -> devand_core::User {
    use devand_core::*;
    use std::collections::BTreeMap;
    use std::convert::TryFrom;

    let mut languages = BTreeMap::default();

    languages.insert(
        Language::C,
        LanguagePreference {
            level: Level::Expert,
            priority: Priority::Low,
        },
    );
    languages.insert(
        Language::Javascript,
        LanguagePreference {
            level: Level::Proficient,
            priority: Priority::Low,
        },
    );
    languages.insert(
        Language::CPlusPlus,
        LanguagePreference {
            level: Level::Expert,
            priority: Priority::Low,
        },
    );
    languages.insert(
        Language::Rust,
        LanguagePreference {
            level: Level::Proficient,
            priority: Priority::High,
        },
    );
    languages.insert(
        Language::Go,
        LanguagePreference {
            level: Level::Novice,
            priority: Priority::No,
        },
    );

    User {
        id,
        username: username.into(),
        visible_name: "Alessandro Pezzato".into(),
        email: "alessandro@pezzato.net".into(),
        settings: UserSettings {
            languages,
            vacation_mode: false,
            schedule: Schedule::Weekly(WeekSchedule {
                mon: DaySchedule::try_from("21,22,23").unwrap(),
                tue: DaySchedule::never(),
                wed: DaySchedule::never(),
                thu: DaySchedule::never(),
                fri: DaySchedule::never(),
                sat: DaySchedule::always(),
                sun: DaySchedule::never(),
            }),
        },
    }
}
