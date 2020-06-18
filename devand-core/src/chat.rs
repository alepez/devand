use crate::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub created_at: DateTime<Utc>,
    pub from: UserId,
    pub to: UserId,
    pub txt: String,
}

#[derive(Debug, Copy, Clone)]
pub struct ChatId {
    pub user_me: UserId,
    pub user_other: UserId,
}

impl ChatId {
    pub fn new(user_me: UserId, user_other: UserId) -> Self {
        Self {
            user_me,
            user_other,
        }
    }

    pub fn to_number(&self) -> i64 {
        let (min, max) = if self.user_me.0 < self.user_other.0 {
            (self.user_me.0, self.user_other.0)
        } else {
            (self.user_other.0, self.user_me.0)
        };

        (min as i64) * 1_000_000_000 + (max as i64)
    }
}

impl std::fmt::Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_number())
    }
}
