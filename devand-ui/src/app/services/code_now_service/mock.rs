use super::FetchCallback;
use devand_core::CodeNow;

pub struct CodeNowService {
    callback: FetchCallback,
}

impl CodeNowService {
    pub fn new(callback: FetchCallback) -> Self {
        Self { callback }
    }

    pub fn restore(&mut self) {
        self.callback.emit(Ok(mock_code_now_users()))
    }
}

fn mock_code_now_users() -> CodeNow {
    let current_user = mock_user("Betrand Russel");
    CodeNow {
        current_user: current_user.clone(),
        all_users: vec![
            current_user.into(),
            mock_user("Albert Einstein").into(),
            mock_user("Isaac Newton").into(),
            mock_user("James Clerk Maxwell").into(),
            mock_user("Max Plank").into(),
        ],
    }
}

fn mock_user(name: &str) -> devand_core::User {
    use devand_core::*;
    use std::collections::BTreeMap;

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

    User {
        id: UserId(1),
        username: name
            .to_string()
            .to_lowercase()
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect(),
        email: "a@b.c".to_string(),
        visible_name: name.to_string(),
        settings: devand_core::UserSettings {
            languages: Languages(languages),
            schedule: Availability::default(),
            vacation_mode: false,
        },
    }
}
