use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Availability {
    /// Disabled
    Never,
    /// Schedule every week in the future
    Weekly(WeekSchedule),
}

impl Default for Availability {
    fn default() -> Self {
        Availability::Never
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DaySchedule {
    pub hours: [bool; 24],
}

impl TryFrom<&str> for DaySchedule {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        if s == "*" {
            return Ok(Self::always());
        }

        let numbers: Vec<usize> = s.split(',').filter_map(|x| x.parse().ok()).collect();

        if numbers.is_empty() {
            return Ok(Self::never());
        }

        let mut hours: [bool; Self::HOURS_IN_DAY] = [false; Self::HOURS_IN_DAY];

        for n in numbers {
            if n < hours.len() {
                hours[n] = true;
            }
        }

        Ok(DaySchedule { hours })
    }
}

impl DaySchedule {
    pub const HOURS_IN_DAY: usize = 24;

    pub fn never() -> Self {
        DaySchedule {
            hours: [false; Self::HOURS_IN_DAY],
        }
    }

    pub fn always() -> Self {
        DaySchedule {
            hours: [true; Self::HOURS_IN_DAY],
        }
    }

    pub fn new(hours: [bool; Self::HOURS_IN_DAY]) -> Self {
        DaySchedule { hours }
    }
}

/// Week scheduling
#[derive(Debug, Serialize, Deserialize, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct WeekSchedule {
    pub mon: DaySchedule,
    pub tue: DaySchedule,
    pub wed: DaySchedule,
    pub thu: DaySchedule,
    pub fri: DaySchedule,
    pub sat: DaySchedule,
    pub sun: DaySchedule,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hourly_schedule_from_comma_separated_list() {
        let s = "5,7,21";
        let schedule = DaySchedule::try_from(s).unwrap();
        assert!(schedule.hours[4] == false);
        assert!(schedule.hours[5] == true);
        assert!(schedule.hours[6] == false);
        assert!(schedule.hours[7] == true);
        assert!(schedule.hours[21] == true);
        assert!(schedule.hours[22] == false);
    }
}
