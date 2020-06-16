use crate::{Affinity, AffinityParams, DaySchedule, UserId, WeekSchedule};

#[derive(Debug)]
struct AffinityMatrix(Vec<Option<Affinity>>);

impl std::ops::Index<(UserId, UserId)> for AffinityMatrix {
    type Output = Option<Affinity>;

    fn index(&self, pair: (UserId, UserId)) -> &Self::Output {
        let (UserId(i), UserId(j)) = pair;
        let p = (((j * (j - 1)) / 2) + i) as usize;
        &self.0[p]
    }
}

/// Creates an AffinityMatrix from a Vec of (UserId,AffinityParams), where the
/// UserId can be unsorted and with holes.
/// The resulting matrix size is ((n * (n-1)) / 2), where n is the max
/// UserId.
impl From<Vec<(UserId, AffinityParams)>> for AffinityMatrix {
    fn from(ua: Vec<(UserId, AffinityParams)>) -> Self {
        if let Some((UserId(max_user_id), _)) = ua.iter().max_by_key(|x| x.0) {
            let size = (((max_user_id + 1) * max_user_id) / 2) as usize;

            let mut data = Vec::with_capacity(size);
            data.resize_with(size, Default::default);

            for (s, (UserId(j), y)) in ua.iter().enumerate().skip(1) {
                for (UserId(i), x) in ua.iter().take(s) {
                    let affinity = Affinity::from_params(x, y);
                    let p = (((j * (j - 1)) / 2) + i) as usize;
                    unsafe {
                        *data.get_unchecked_mut(p) = Some(affinity);
                    }
                }
            }
            Self(data)
        } else {
            Self(Vec::default())
        }
    }
}

struct Hour(i32);

#[derive(Debug)]
struct ScheduleMatrix {
    data: Vec<bool>,
    max_user_id: UserId,
}

impl std::ops::Index<(UserId, Hour)> for ScheduleMatrix {
    type Output = bool;

    fn index(&self, pair: (UserId, Hour)) -> &Self::Output {
        let (UserId(i), Hour(h)) = pair;
        let p = (i as usize) * DaySchedule::HOURS_IN_DAY + (h as usize);
        &self.data[p]
    }
}

impl From<Vec<(UserId, DaySchedule)>> for ScheduleMatrix {
    fn from(us: Vec<(UserId, DaySchedule)>) -> Self {
        if let Some((UserId(max_user_id), _)) = us.iter().max_by_key(|x| x.0) {
            let size = (1 + (*max_user_id as usize)) * DaySchedule::HOURS_IN_DAY;

            let mut data = Vec::with_capacity(size);
            data.resize_with(size, Default::default);

            for h in 0..24 {
                for (UserId(i), day) in us.iter() {
                    let in_schedule = day.hours[h];
                    let p = (*i as usize) * DaySchedule::HOURS_IN_DAY + h;
                    let cell = unsafe { data.get_unchecked_mut(p) };
                    *cell = in_schedule;
                }
            }
            Self {
                data,
                max_user_id: UserId(*max_user_id),
            }
        } else {
            Self {
                data: Vec::default(),
                max_user_id: UserId(0),
            }
        }
    }
}

impl ScheduleMatrix {
    fn get_available_at_hour(&self, h: Hour) -> Vec<UserId> {
        self.data
            .iter()
            .skip(h.0 as usize)
            .step_by(DaySchedule::HOURS_IN_DAY)
            .enumerate()
            .filter(|(_, in_schedule)| **in_schedule)
            .map(|(id, _)| UserId(id as i32))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Language, LanguagePreference, Level, Priority};
    use std::convert::TryFrom;

    #[test]
    fn calculate_affinity_matrix() {
        let params = vec![
            (
                UserId(0),
                AffinityParams::from(vec![(
                    Language::CPlusPlus,
                    LanguagePreference {
                        level: Level::Novice,
                        priority: Priority::High,
                    },
                )]),
            ),
            (
                UserId(1),
                AffinityParams::from(vec![
                    (
                        Language::Rust,
                        LanguagePreference {
                            level: Level::Novice,
                            priority: Priority::High,
                        },
                    ),
                    (
                        Language::CPlusPlus,
                        LanguagePreference {
                            level: Level::Expert,
                            priority: Priority::Low,
                        },
                    ),
                ]),
            ),
            (
                UserId(3),
                AffinityParams::from(vec![(
                    Language::CPlusPlus,
                    LanguagePreference {
                        level: Level::Novice,
                        priority: Priority::No,
                    },
                )]),
            ),
            (
                UserId(2),
                AffinityParams::from(vec![(
                    Language::Rust,
                    LanguagePreference {
                        level: Level::Novice,
                        priority: Priority::High,
                    },
                )]),
            ),
        ];

        let expected_len = params.len() * (params.len() - 1) / 2;
        let matrix = AffinityMatrix::from(params);

        assert!(matrix.0.len() == expected_len);
        assert!(matrix[(UserId(0), UserId(2))] == Some(Affinity::NONE));
        assert!(matrix[(UserId(0), UserId(3))] == Some(Affinity::NONE));
        assert!(matrix[(UserId(1), UserId(2))] == Some(Affinity::FULL));
    }

    #[test]
    fn calculate_schedule_matrix() {
        let params = vec![
            (UserId(0), DaySchedule::try_from("1,2").unwrap()),
            (UserId(1), DaySchedule::try_from("1,2,3").unwrap()),
            (UserId(2), DaySchedule::try_from("2,3,5,6").unwrap()),
            (UserId(3), DaySchedule::try_from("3,4,5").unwrap()),
        ];

        let expected_len = params.len() * 24;
        let matrix = ScheduleMatrix::from(params);

        assert!(matrix.data.len() == expected_len);
        assert!(matrix[(UserId(0), Hour(1))] == true);
        assert!(matrix[(UserId(2), Hour(3))] == true);
        assert!(matrix[(UserId(2), Hour(5))] == true);
        assert!(matrix[(UserId(2), Hour(1))] == false);
        assert!(matrix[(UserId(1), Hour(5))] == false);
        assert!(matrix[(UserId(3), Hour(6))] == false);

        assert!(matrix.get_available_at_hour(Hour(1)) == vec![UserId(0), UserId(1)]);
        assert!(matrix.get_available_at_hour(Hour(3)) == vec![UserId(1), UserId(2), UserId(3)]);

        // assert!(matrix[(UserId(0), Hour(1))] == vec![UserId(1)]);
        // assert!(matrix[(UserId(2), Hour(3))] == vec![UserId(1), UserId(3)]);
        // assert!(matrix[(UserId(2), Hour(5))] == vec![UserId(3)]);
    }
}
