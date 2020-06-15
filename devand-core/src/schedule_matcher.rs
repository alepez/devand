use crate::{Affinity, AffinityParams};

#[derive(Debug)]
struct AffinityMatrix(Vec<Affinity>);

impl From<Vec<AffinityParams>> for AffinityMatrix {
    fn from(ua: Vec<AffinityParams>) -> Self {
        let n = ua.len();
        let n2 = n * n;
        let mut data = Vec::with_capacity(n2);

        for (i, x) in ua.iter().enumerate() {
            for y in ua.iter().skip(i + 1) {
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
    }
}
