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

#[derive(Debug)]
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
}
