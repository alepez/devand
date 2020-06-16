use crate::{Affinity, AffinityParams, Availability, DaySchedule, User, UserId, WeekSchedule};
use chrono::prelude::*;
use chrono::Duration;
use serde::{Deserialize, Serialize};

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

impl AffinityMatrix {
    fn find_best_match<'a, I>(&self, u: &UserId, o: I) -> Option<UserId>
    where
        I: IntoIterator<Item = &'a UserId>,
    {
        o.into_iter()
            .map(|x| (*x, self[(*u, *x)]))
            .max_by_key(|x| x.1)
            .map(|x| x.0)
    }
}

#[derive(Serialize, Deserialize)]
struct Hour(i32);

impl ToString for Hour {
    fn to_string(&self) -> String {
        format!("{:02}", self.0)
    }
}

#[derive(Debug)]
pub struct DayScheduleMatrix {
    data: Vec<bool>,
    max_user_id: UserId,
}

impl std::ops::Index<(UserId, Hour)> for DayScheduleMatrix {
    type Output = bool;

    fn index(&self, pair: (UserId, Hour)) -> &Self::Output {
        let (UserId(i), Hour(h)) = pair;
        let p = (i as usize) * DaySchedule::HOURS_IN_DAY + (h as usize);
        &self.data[p]
    }
}

impl From<Vec<(UserId, DaySchedule)>> for DayScheduleMatrix {
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

impl DayScheduleMatrix {
    /// Return a Vec of all users available in a given hour
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

    /// Return a Vec of all users available in a given dayly schedule
    fn get_available_at_day(&self, day: &DaySchedule) -> Vec<UserId> {
        use std::collections::BTreeSet;

        let mut set: BTreeSet<UserId> = BTreeSet::new();

        day.hours.iter().enumerate().for_each(|(h, is_available)| {
            if *is_available {
                self.get_available_at_hour(Hour(h as i32))
                    .iter()
                    .for_each(|&u| {
                        set.insert(u);
                    });
            }
        });

        set.into_iter().collect()
    }
}

#[derive(Debug)]
pub struct WeekScheduleMatrix {
    pub mon: DayScheduleMatrix,
    pub tue: DayScheduleMatrix,
    pub wed: DayScheduleMatrix,
    pub thu: DayScheduleMatrix,
    pub fri: DayScheduleMatrix,
    pub sat: DayScheduleMatrix,
    pub sun: DayScheduleMatrix,
}

impl std::ops::Index<chrono::Weekday> for WeekScheduleMatrix {
    type Output = DayScheduleMatrix;
    fn index(&self, index: chrono::Weekday) -> &Self::Output {
        match index {
            chrono::Weekday::Mon => &self.mon,
            chrono::Weekday::Tue => &self.tue,
            chrono::Weekday::Wed => &self.wed,
            chrono::Weekday::Thu => &self.thu,
            chrono::Weekday::Fri => &self.fri,
            chrono::Weekday::Sat => &self.sat,
            chrono::Weekday::Sun => &self.sun,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct DayMatching {
    pub hours: [Vec<UserId>; 24],
}

fn get_day_schedule(date: Date<Utc>, week_schedule: &WeekSchedule) -> (Date<Utc>, DaySchedule) {
    (date, week_schedule[date.weekday()].clone())
}

/// Return a vector with an item for each hour of the next week, each one
/// containing all available users at that moment.
fn match_all_week(
    target: &Vec<(Date<Utc>, DaySchedule)>,
    week_sched_matrix: &WeekScheduleMatrix,
) -> Vec<(DateTime<Utc>, Vec<UserId>)> {
    todo!()
}

fn attach_schedule(
    days: Vec<Date<Utc>>,
    availability: Availability,
) -> Vec<(Date<Utc>, DaySchedule)> {
    match availability {
        Availability::Never => days
            .iter()
            .map(|day| (*day, DaySchedule::default()))
            .collect(),
        Availability::Weekly(week_schedule) => days
            .iter()
            .map(|day| get_day_schedule(*day, &week_schedule))
            .collect(),
    }
}

fn days_from(n: usize, from: DateTime<Utc>) -> Vec<Date<Utc>> {
    (0..n)
        .into_iter()
        .filter_map(|x| from.checked_add_signed(Duration::days(x as i64)))
        .map(|x| x.date())
        .collect()
}

pub fn find_all_users_matching_in_week(
    date: DateTime<Utc>,
    availability: Availability,
    week_sched_matrix: WeekScheduleMatrix,
) -> Vec<(DateTime<Utc>, Vec<UserId>)> {
    let days = days_from(7, date);
    let future_availability = attach_schedule(days, availability);
    match_all_week(&future_availability, &week_sched_matrix)
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

        let u = UserId(1);
        let o = vec![UserId(2), UserId(3)];
        let best_match = matrix.find_best_match(&u, &o);
        assert!(best_match == Some(UserId(2)));
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
        let matrix = DayScheduleMatrix::from(params);

        assert!(matrix.data.len() == expected_len);
        assert!(matrix[(UserId(0), Hour(1))] == true);
        assert!(matrix[(UserId(2), Hour(3))] == true);
        assert!(matrix[(UserId(2), Hour(5))] == true);
        assert!(matrix[(UserId(2), Hour(1))] == false);
        assert!(matrix[(UserId(1), Hour(5))] == false);
        assert!(matrix[(UserId(3), Hour(6))] == false);

        assert!(matrix.get_available_at_hour(Hour(1)) == vec![UserId(0), UserId(1)]);
        assert!(matrix.get_available_at_hour(Hour(3)) == vec![UserId(1), UserId(2), UserId(3)]);

        assert!(
            matrix.get_available_at_day(&DaySchedule::try_from("2,3,5,6").unwrap())
                == vec![UserId(0), UserId(1), UserId(2), UserId(3)]
        );

        assert!(
            matrix.get_available_at_day(&DaySchedule::try_from("4,5,6").unwrap())
                == vec![UserId(2), UserId(3)]
        );
    }

    #[test]
    fn availability_match_ok() {
        let _days = days_from(7, Utc::now());
        // dbg!(days);
    }

    #[test]
    fn future_availability_ok() {
        let now = Utc::now();
        let next_week = now.checked_add_signed(Duration::days(7)).unwrap();
        let days = days_from(7, next_week);
        let User { settings, .. } = crate::mock::user();
        let availability = settings.schedule;
        let _uture_availability = attach_schedule(days, availability);
        // dbg!(future_availability);
    }
}
