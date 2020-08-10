use crate::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub author: UserId,
    pub txt: String,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ChatId(pub uuid::Uuid);

impl std::fmt::Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: ChatId,
    pub members: Vec<UserId>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Chats(pub Vec<Chat>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberInfo {
    pub user_id: UserId,
    pub verified_email: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatInfo {
    pub members_info: Vec<ChatMemberInfo>,
    pub messages: Vec<ChatMessage>,
}
