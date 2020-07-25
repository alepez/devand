use super::OtherUserLoadedCallback;
use devand_core::*;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::Rng;
use rand::SeedableRng;
use strum::IntoEnumIterator;

pub struct UserProfileService {
    other_user_loaded_callback: OtherUserLoadedCallback,
    rng: StdRng,
}

impl UserProfileService {
    pub fn new(other_user_loaded_callback: OtherUserLoadedCallback) -> Self {
        let rng = StdRng::seed_from_u64(42);
        Self {
            rng,
            other_user_loaded_callback,
        }
    }

    pub fn load_other_user(&mut self, username: &str) {
        self.other_user_loaded_callback
            .emit(Some(devand_core::PublicUserProfile {
                id: UserId(2),
                languages: fake_languages(&mut self.rng),
                username: username.into(),
                visible_name: "Foo Bar".into(),
            }))
    }
}

fn fake_languages(rng: &mut StdRng) -> Languages {
    let mut languages = std::collections::BTreeMap::default();

    for lang in Language::iter() {
        if rng.gen_bool(0.2) {
            let level = Level::iter().choose(rng).unwrap();
            let priority = Priority::iter().choose(rng).unwrap();
            languages.insert(lang, LanguagePreference { level, priority });
        }
    }

    Languages(languages)
}

