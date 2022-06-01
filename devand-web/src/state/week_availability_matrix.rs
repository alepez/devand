use devand_core::schedule_matcher::WeekScheduleMatrix;
use devand_core::{Availability, UserId};
use rocket_contrib::databases::diesel;

#[derive(Default)]
pub struct WeekScheduleMatrixCache {
    data: Option<WeekScheduleMatrix>,
}

impl WeekScheduleMatrixCache {
    pub fn get(&self) -> &WeekScheduleMatrix {
        self.data.as_ref().unwrap()
    }

    pub fn init(&mut self, conn: &diesel::PgConnection) {
        // TODO [optimization] load only needed data
        let users = devand_db::load_users(conn).expect("Cannot load users from database");

        let schedules: Vec<_> = users
            .into_iter()
            .filter_map(|u| {
                let user_id = u.id;
                let schedule = u.settings.schedule;
                if let Availability::Weekly(week_schedule) = schedule {
                    Some((user_id, week_schedule))
                } else {
                    None
                }
            })
            .collect();

        let wsm = WeekScheduleMatrix::from(schedules);
        self.data = Some(wsm);
    }

    pub fn update(&mut self, user: UserId, availability: &Availability) {
        if let Some(wsm) = &mut self.data {
            wsm.update(user, availability);
        }
    }
}
