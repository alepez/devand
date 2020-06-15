use crate::{Affinity, AffinityParams, DaySchedule, UserId, WeekSchedule};

#[derive(Debug)]
struct AffinityMatrix(Vec<Affinity>);

impl std::ops::Index<(UserId, UserId)> for AffinityMatrix {
    type Output = Affinity;

    fn index(&self, pair: (UserId, UserId)) -> &Self::Output {
        let (UserId(i), UserId(j)) = pair;
        let p = (((j * (j - 1)) / 2) + i) as usize;
        &self.0[p]
    }
}

/// Creates an AffinityMatrix from a Vec of AffinityParams, where the index
/// is the associated UserId
impl From<Vec<AffinityParams>> for AffinityMatrix {
    fn from(ua: Vec<AffinityParams>) -> Self {
        let n = ua.len();
        let n2 = n * n;
        let mut data = Vec::with_capacity(n2);

        for (j, y) in ua.iter().enumerate().skip(1) {
            for x in ua.iter().take(j) {
                let affinity = Affinity::from_params(x, y);
                data.push(affinity);
            }
        }

        Self(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Language, LanguagePreference, Level, Priority};

    use super::*;
    #[test]
    fn calculate_affinity_matrix() {
        let params = vec![
            AffinityParams::from(vec![(
                Language::CPlusPlus,
                LanguagePreference {
                    level: Level::Novice,
                    priority: Priority::High,
                },
            )]),
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
            AffinityParams::from(vec![(
                Language::Rust,
                LanguagePreference {
                    level: Level::Novice,
                    priority: Priority::High,
                },
            )]),
            AffinityParams::from(vec![(
                Language::CPlusPlus,
                LanguagePreference {
                    level: Level::Novice,
                    priority: Priority::No,
                },
            )]),
        ];

        let expected_len = params.len() * (params.len() - 1) / 2;
        let matrix = AffinityMatrix::from(params);

        assert!(matrix.0.len() == expected_len);
        assert!(matrix[(UserId(0), UserId(2))] == Affinity::NONE);
        assert!(matrix[(UserId(0), UserId(3))] == Affinity::NONE);
        assert!(matrix[(UserId(1), UserId(2))] == Affinity::FULL);
    }
}
