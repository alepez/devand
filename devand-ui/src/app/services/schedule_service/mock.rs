use super::{FetchCallback, ScheduleServiceContent};
use chrono::offset::TimeZone;
use devand_core::schedule_matcher::AvailabilityMatch;
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

pub struct ScheduleService {
    callback: FetchCallback,
    rng: StdRng,
}

impl ScheduleService {
    pub fn new(callback: FetchCallback) -> Self {
        let rng = StdRng::seed_from_u64(42);
        Self { callback, rng }
    }

    pub fn load(&mut self) {
        self.callback
            .emit(Ok(ScheduleServiceContent::AvailabilityMatch(
                fake_availability_match(&mut self.rng),
            )))
    }

    pub fn load_public_profile(&mut self, user_id: UserId) {
        self.callback
            .emit(Ok(ScheduleServiceContent::PublicUserProfile(
                fake_public_profile(&mut self.rng, user_id),
            )))
    }
}

fn fake_public_profile(rng: &mut StdRng, user_id: UserId) -> PublicUserProfile {
    let name: String = Name(EN).fake_with_rng(rng);
    let username: String = Username(EN).fake_with_rng(rng);

    PublicUserProfile {
        id: user_id,
        languages: fake_languages(rng),
        username,
        visible_name: name,
        bio: "This is the bio".to_string(),
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

fn fake_availability_match(rng: &mut StdRng) -> AvailabilityMatch {
    let start_t: i64 = 1592474400;

    let mut slots = Vec::new();

    for i in 0..(7 * 24) {
        let t_diff: i64 = i * 60 * 60;
        let available = rng.gen_bool(0.1);
        if available {
            let t = chrono::Utc.timestamp(start_t + t_diff, 0);
            let users_count = rng.gen_range(1, 10);
            let mut users = Vec::new();
            for _ in 0..users_count {
                users.push(devand_core::UserId(rng.gen_range(0, 1_000_000_000)));
            }
            slots.push((t, users));
        }
    }

    AvailabilityMatch { slots }
}
