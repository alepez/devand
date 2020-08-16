use super::{ChatServiceCallback, ChatServiceContent};
use devand_core::chat::ChatMessage;
use devand_core::{UserChats, UserId};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

fn encode_chat_members(chat_members: &[UserId]) -> String {
    chat_members
        .iter()
        .map(|x| x.0.to_string())
        .collect::<Vec<_>>()
        .join("-")
}

fn api_url_get_user(u: &str) -> String {
    format!("/api/u/{}", u)
}

fn api_url_get(chat_members: &[UserId]) -> String {
    format!("/api/chat/{}", encode_chat_members(chat_members))
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

fn api_url_chats() -> String {
    format!("/api/chats")
}

pub struct ChatService {
    chat_members: Option<Vec<UserId>>,
    callback: ChatServiceCallback,
    task: Option<FetchTask>,
}

impl ChatService {
    pub fn new(callback: ChatServiceCallback) -> Self {
        Self {
            chat_members: None,
            callback,
            task: None,
        }
    }

    pub fn load_all_chats(&mut self) {
        let url = api_url_chats();
        let req = Request::get(url).body(Nothing).unwrap();
        let callback = self.callback.clone();
        let handler = move |response: Response<Json<Result<UserChats, anyhow::Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if let Ok(data) = data {
                callback.emit(ChatServiceContent::AllChats(data));
            } else {
                log::error!("{:?}", &meta);
            }
        };

        self.task = FetchService::fetch(req, handler.into()).ok();
    }
}
