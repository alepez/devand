use super::{FetchCallback, ScheduleServiceContent};
use devand_core::schedule_matcher::AvailabilityMatch;
use devand_core::{PublicUserProfile, UserId};

use chrono::offset::TimeZone;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub struct ScheduleService {
    callback: FetchCallback,
}

impl ScheduleService {
    pub fn new(callback: FetchCallback) -> Self {
        Self { callback }
    }

    pub fn load(&mut self) {
        self.callback.emit(Ok(ScheduleServiceContent::AvailabilityMatch(
            fake_availability_match(),
        )))
    }

    pub fn load_public_profile(&mut self, user_id: UserId) {
        self.callback.emit(Ok(ScheduleServiceContent::PublicUserProfile(
            fake_public_profile(user_id),
        )))
    }
}

fn fake_public_profile(id: UserId) -> PublicUserProfile {
    PublicUserProfile {
        id,
        languages: devand_core::Languages::default(),
        username: format!("user{}", id.0),
        visible_name: format!("User {}", id.0),
    }
}

fn fake_availability_match() -> AvailabilityMatch {
    let mut rng = StdRng::seed_from_u64(42);

    let start_t: i64 = 1592475298;

    let mut slots = Vec::new();

    for i in 0..(7 * 24) {
        let t_diff: i64 = i * 60 * 60;
        let available = rng.gen_range(0, 10) < 1;
        if available {
            let t = chrono::Utc.timestamp(start_t + t_diff, 0);
            let users_count = rng.gen_range(0, 7);
            let mut users = Vec::new();
            for _ in 0..users_count {
                users.push(devand_core::UserId(rng.gen_range(0, 5)));
            }
            slots.push((t, users));
        }
    }

    AvailabilityMatch { slots }
}
