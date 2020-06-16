use devand_core::schedule_matcher::WeekScheduleMatrix;

#[derive(Default)]
pub struct WeekScheduleMatrixCache {
    data: Option<WeekScheduleMatrix>,
}

impl WeekScheduleMatrixCache {
    pub fn get(&self) -> &WeekScheduleMatrix {
        self.data.as_ref().unwrap()
    }
}
