use super::NewMessagesCallback;
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
    chat_members: Vec<UserId>,
    new_messages_callback: NewMessagesCallback,

    service: FetchService,
    task: Option<FetchTask>,
}

impl ChatService {
    pub fn new(chat_members: Vec<UserId>, new_messages_callback: NewMessagesCallback) -> Self {
        Self {
            chat_members,
            new_messages_callback,
            service: FetchService::new(),
            task: None,
        }
    }

    pub fn load_history(&mut self) {
        let url = api_url_get(&self.chat_members);
        let req = Request::get(url).body(Nothing).unwrap();
        self.task = request(&mut self.service, self.new_messages_callback.clone(), req).ok();
    }

    pub fn send_message(&mut self, txt: String) {
        let url = api_url_post(&self.chat_members);
        let json = serde_json::to_string(&txt).map_err(|_| anyhow::anyhow!("Cannot serialize"));
        let req = Request::post(url).body(json).unwrap();
        self.task = request(&mut self.service, self.new_messages_callback.clone(), req).ok();
    }

    pub fn poll(&mut self, last_message: Option<&ChatMessage>) {
        let url = api_url_poll(&self.chat_members, last_message);
        let req = Request::get(url).body(Nothing).unwrap();
        self.task = request(&mut self.service, self.new_messages_callback.clone(), req).ok();
    }
}

fn request<R>(
    service: &mut FetchService,
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

    service.fetch(r, handler.into())
}
