use super::FetchCallback;
use devand_core::CodeNowUsers;

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

fn mock_code_now_users() -> CodeNowUsers {
    CodeNowUsers(vec![
        mock_user("Albert Einstein"),
        mock_user("Isaac Newton"),
        mock_user("James Clerk Maxwell"),
        mock_user("Max Plank"),
    ])
}

fn mock_user(name: &str) -> devand_core::PublicUserProfile {
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

    PublicUserProfile {
        username: name
            .to_string()
            .to_lowercase()
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect(),
        visible_name: name.to_string(),
        languages,
    }
}
