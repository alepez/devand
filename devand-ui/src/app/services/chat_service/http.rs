use super::{NewMessagesCallback, OtherUserLoadedCallback};
use devand_core::chat::ChatMessage;
use devand_core::UserId;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

fn encode_chat_members(chat_members: &[UserId]) -> String {
    chat_members
        .into_iter()
        .map(|x| x.0.to_string())
        .collect::<Vec<_>>()
        .join("-")
}

fn api_url_get_user(u: &str) -> String {
    format!("/api/u/{}", u)
}

fn api_url_get(chat_members: &[UserId]) -> String {
    format!("/api/chat/{}/messages", encode_chat_members(chat_members))
}

fn api_url_post(chat_members: &[UserId]) -> String {
    format!("/api/chat/{}/messages", encode_chat_members(chat_members))
}

fn api_url_poll(chat_members: &[UserId], last_message: Option<&ChatMessage>) -> String {
    format!(
        "/api/chat/{}/messages/poll/{}",
        encode_chat_members(chat_members),
        last_message.map(|x| x.created_at.timestamp()).unwrap_or(0)
    )
}

pub struct ChatService {
    chat_members: Option<Vec<UserId>>,
    new_messages_callback: NewMessagesCallback,
    other_user_loaded_callback: OtherUserLoadedCallback,

    task: Option<FetchTask>,
}

impl ChatService {
    pub fn new(
        new_messages_callback: NewMessagesCallback,
        other_user_loaded_callback: OtherUserLoadedCallback,
    ) -> Self {
        Self {
            chat_members: None,
            new_messages_callback,
            other_user_loaded_callback,
            task: None,
        }
    }

    pub fn load_other_user(&mut self, username: &str) {
        let url = api_url_get_user(username);
        let req = Request::get(url).body(Nothing).unwrap();

        let callback = self.other_user_loaded_callback.clone();

        let handler = move |response: Response<
            Json<Result<devand_core::PublicUserProfile, anyhow::Error>>,
        >| {
            let (meta, Json(data)) = response.into_parts();
            if let Ok(data) = data {
                callback.emit(Some(data));
            } else {
                log::error!("{:?}", &meta);
                callback.emit(None);
            }
        };

        self.task = FetchService::fetch(req, handler.into()).ok();
    }

    pub fn load_history(&mut self, mut chat_members: Vec<UserId>) {
        chat_members.sort();
        self.chat_members = Some(chat_members.clone());
        let url = api_url_get(&chat_members);
        let req = Request::get(url).body(Nothing).unwrap();
        self.task = request(self.new_messages_callback.clone(), req).ok();
    }

    pub fn send_message(&mut self, txt: String) {
        if let Some(chat_members) = &self.chat_members {
            let url = api_url_post(chat_members);
            let json = serde_json::to_string(&txt).map_err(|_| anyhow::anyhow!("Cannot serialize"));
            let req = Request::post(url).body(json).unwrap();
            self.task = request(self.new_messages_callback.clone(), req).ok();
        } else {
            log::error!("Cannot send message without knowing chat members");
        }
    }

    pub fn poll(&mut self, last_message: Option<&ChatMessage>) {
        if let Some(chat_members) = &self.chat_members {
            let url = api_url_poll(chat_members, last_message);
            let req = Request::get(url).body(Nothing).unwrap();
            self.task = request(self.new_messages_callback.clone(), req).ok();
        }
    }
}

fn request<R>(
    callback: Callback<Vec<ChatMessage>>,
    r: http::request::Request<R>,
) -> Result<FetchTask, anyhow::Error>
where
    R: std::convert::Into<std::result::Result<std::string::String, anyhow::Error>>,
{
    let handler = move |response: Response<Json<Result<Vec<ChatMessage>, anyhow::Error>>>| {
        let (meta, Json(data)) = response.into_parts();
        if let Ok(data) = data {
            callback.emit(data);
        } else {
            log::error!("{:?}", &meta);
        }
    };

    FetchService::fetch(r, handler.into())
}
