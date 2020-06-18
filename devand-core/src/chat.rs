use crate::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub created_at: DateTime<Utc>,
    pub from: UserId,
    pub to: UserId,
    pub txt: String,
}

pub struct ChatId {
    pub user_me: UserId,
    pub user_other: UserId,
}
