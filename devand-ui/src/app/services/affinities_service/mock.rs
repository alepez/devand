use super::FetchCallback;
use devand_core::Affinity;
use devand_core::UserAffinity;

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
        UserAffinity::new(mock_user("Albert Einstein"), Affinity::from_number(0.1)),
        UserAffinity::new(mock_user("Isaac Newton"), Affinity::from_number(0.2)),
        UserAffinity::new(mock_user("James Clerk Maxwell"), Affinity::from_number(0.5)),
        UserAffinity::new(mock_user("Max Plank"), Affinity::from_number(1.0)),
    ]
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

    PublicUserProfile {
        username: name.to_string().to_lowercase().chars().filter(|x| x.is_alphabetic()).collect(),
        visible_name: name.to_string(),
        languages,
    }
}
