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
    fn from(mut ua: Vec<(UserId, AffinityParams)>) -> Self {
        ua.sort_unstable_by_key(|x| x.0);

        if let Some((UserId(last), _)) = ua.last() {
            let max_user_id = (1 + last) as usize;
            let size = (max_user_id * (max_user_id - 1)) / 2;
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


#[cfg(test)]
mod tests {
    use crate::{Language, LanguagePreference, Level, Priority};

    use super::*;
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
                UserId(2),
                AffinityParams::from(vec![(
                    Language::Rust,
                    LanguagePreference {
                        level: Level::Novice,
                        priority: Priority::High,
                    },
                )]),
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
        ];

        let expected_len = params.len() * (params.len() - 1) / 2;
        let matrix = AffinityMatrix::from(params);

        assert!(matrix.0.len() == expected_len);
        assert!(matrix[(UserId(0), UserId(2))] == Some(Affinity::NONE));
        assert!(matrix[(UserId(0), UserId(3))] == Some(Affinity::NONE));
        assert!(matrix[(UserId(1), UserId(2))] == Some(Affinity::FULL));
    }
}
