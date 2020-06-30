use super::FetchCallback;
use devand_core::User;

pub struct UserService {
    callback: FetchCallback,
}

impl UserService {
    pub fn new(callback: FetchCallback) -> Self {
        Self { callback }
    }

    pub fn restore(&mut self) {
        self.callback.emit(Ok(mock_user()))
    }

    pub fn store(&mut self, _user: &User) {}
}

fn mock_user() -> devand_core::User {
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
        Language::JavaScript,
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

    let languages = Languages(languages);

    User {
        id: UserId(1),
        username: "alepez".into(),
        visible_name: "Alessandro Pezzato".into(),
        email: "alessandro@pezzato.net".into(),
        email_verified: false,
        settings: UserSettings {
            languages,
            vacation_mode: false,
            schedule: Availability::Weekly(WeekSchedule {
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
