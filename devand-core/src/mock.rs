use super::*;
use maplit::btreemap;
use maplit::btreeset;
use std::convert::TryFrom;

pub fn user() -> User {
    User {
        id: UserId(1),
        username: "alepez".into(),
        visible_name: "Alessandro Pezzato".into(),
        email: "alessandro@pezzato.net".into(),
        email_verified: true,
        settings: UserSettings {
            languages: Languages(btreemap! {
                Language::C => LanguagePreference { level: Level::Expert, priority: Priority::Low },
                Language::JavaScript => LanguagePreference { level: Level::Proficient, priority: Priority::No },
                Language::CPlusPlus => LanguagePreference { level: Level::Expert, priority: Priority::Low },
                Language::Rust => LanguagePreference { level: Level::Novice, priority: Priority::High },
            }),
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
            spoken_languages: SpokenLanguages(btreeset! {SpokenLanguage::English}),
        },
        bio: "Ciao".to_string(),
        projects: vec!["https://github.com/alepez/devand".into()],
    }
}

pub fn user_with_username(username: &str) -> User {
    let mut user = user();
    user.username = username.to_string();
    user
}
