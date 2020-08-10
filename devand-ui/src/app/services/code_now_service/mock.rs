use super::FetchCallback;
use devand_core::*;
use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::Rng;
use rand::SeedableRng;
use strum::IntoEnumIterator;

pub struct CodeNowService {
    callback: FetchCallback,
    rng: StdRng,
}

impl CodeNowService {
    pub fn new(callback: FetchCallback) -> Self {
        let rng = StdRng::seed_from_u64(100);
        Self { callback, rng }
    }

    pub fn restore(&mut self) {
        self.callback.emit(Ok(fake_code_now_users(&mut self.rng)))
    }
}

fn fake_code_now_users(rng: &mut StdRng) -> CodeNow {
    let current_user = fake_user(rng);

    let mut all_users = Vec::new();
    let n = rng.gen_range(0, 20);

    for _ in 0..n {
        all_users.push(fake_user(rng).into());
    }

    CodeNow {
        current_user: current_user.clone(),
        all_users,
    }
}

fn fake_user(rng: &mut StdRng) -> devand_core::User {
    let name: String = Name(EN).fake_with_rng(rng);
    let user_id: i32 = rng.gen_range(1, 1_000_000_000);

    let mut languages = std::collections::BTreeMap::default();

    for lang in Language::iter() {
        if rng.gen_bool(0.2) {
            let level = Level::iter().choose(rng).unwrap();
            let priority = Priority::iter().choose(rng).unwrap();
            languages.insert(lang, LanguagePreference { level, priority });
        }
    }

    let email: String = SafeEmail(EN).fake_with_rng(rng);
    let email_verified = rng.gen_bool(0.7);

    User {
        id: UserId(user_id),
        username: name
            .to_string()
            .to_lowercase()
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect(),
        email,
        email_verified,
        visible_name: name,
        settings: devand_core::UserSettings {
            languages: Languages(languages),
            schedule: Availability::default(),
            vacation_mode: false,
            spoken_languages: SpokenLanguages(vec![devand_core::SpokenLanguage::English]),
        },
        unread_messages: 5,
        bio: "This is the bio".to_string(),
    }
}
