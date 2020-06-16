use devand_core::schedule_matcher::WeekScheduleMatrix;
use rocket_contrib::databases::diesel;

#[derive(Default)]
pub struct WeekScheduleMatrixCache {
    data: WeekScheduleMatrix,
}

impl WeekScheduleMatrixCache {
    pub fn get(&self) -> &WeekScheduleMatrix {
        &self.data
    }

    pub fn init(&mut self, _conn: &diesel::PgConnection) {
        // TODO Create from database data
        let wsm = WeekScheduleMatrix::default();

        self.data = wsm;
    }
}
