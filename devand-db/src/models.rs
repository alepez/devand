use super::schema::{auth, chats, messages, unread_messages, users};
use chrono::{DateTime, Utc};
use serde_json;
use std::convert::TryInto;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub settings: serde_json::Value,
    pub visible_name: Option<String>,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub settings: serde_json::Value,
    pub visible_name: Option<String>,
    pub email_verified: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "auth"]
pub struct NewAuth {
    pub user_id: i32,
    pub enc_password: String,
}

#[derive(Queryable)]
pub struct Auth {
    pub user_id: i32,
    pub username: String,
    pub enc_password: String,
}

impl TryInto<devand_core::User> for User {
    type Error = Error;

    fn try_into(self) -> Result<devand_core::User, Error> {
        let settings = serde_json::from_value(self.settings)
            .map_err(|e| Error::CannotDeserializeUserSettings(e.to_string()))?;

        let visible_name = self.visible_name.unwrap_or(self.username.clone());

        let user = devand_core::User {
            id: devand_core::UserId(self.id),
            username: self.username,
            email: self.email,
            email_verified: self.email_verified,
            visible_name,
            settings,
            unread_messages: 0,
        };

        Ok(user)
    }
}

#[derive(Queryable)]
pub struct ChatMessage {
    pub id: uuid::Uuid,
    pub chat_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub txt: String,
    pub author: i32,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewChatMessage {
    pub chat_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub txt: String,
    pub author: i32,
}

#[derive(Queryable)]
pub struct Chat {
    pub id: uuid::Uuid,
    pub members: Vec<i32>,
}

impl TryInto<devand_core::chat::Chat> for Chat {
    type Error = Error;
    fn try_into(self) -> Result<devand_core::chat::Chat, Self::Error> {
        let members: Vec<devand_core::UserId> =
            self.members.into_iter().map(devand_core::UserId).collect();

        let chat = devand_core::chat::Chat {
            id: devand_core::chat::ChatId(self.id),
            members,
        };

        Ok(chat)
    }
}

#[derive(Insertable)]
#[table_name = "chats"]
pub struct NewChat {
    pub members: Vec<i32>,
}

impl Into<devand_core::chat::ChatMessage> for ChatMessage {
    fn into(self) -> devand_core::chat::ChatMessage {
        devand_core::chat::ChatMessage {
            id: self.id,
            created_at: DateTime::from_utc(self.created_at, Utc),
            txt: self.txt,
            author: devand_core::UserId(self.author),
        }
    }
}

#[derive(Insertable, Queryable)]
#[table_name = "unread_messages"]
pub struct UnreadMessage {
    pub user_id: i32,
    pub message_id: uuid::Uuid,
}

#[derive(Debug)]
pub enum Error {
    CannotDeserializeUserSettings(String),
}
