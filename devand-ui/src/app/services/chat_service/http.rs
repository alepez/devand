use super::NewMessagesCallback;
use devand_core::chat::{ChatId, ChatMessage};
use devand_core::UserId;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

fn api_url_get(chat_id: ChatId) -> String {
    format!("/api/chat/{}/messages", chat_id)
}

fn api_url_post(chat_id: ChatId) -> String {
    format!("/api/chat/{}/messages", chat_id)
}

fn api_url_poll(chat_id: ChatId) -> String {
    format!("/api/chat/{}/messages/poll", chat_id)
}

pub struct ChatService {
    chat_id: ChatId,
    new_messages_callback: NewMessagesCallback,

    get_messages_service: FetchService,
    get_messages_task: Option<FetchTask>,
}

impl ChatService {
    pub fn new(chat_id: ChatId, new_messages_callback: NewMessagesCallback) -> Self {
        Self {
            chat_id,
            new_messages_callback,
            get_messages_service: FetchService::new(),
            get_messages_task: None,
        }
    }

    pub fn load_history(&mut self) {
        let url = api_url_get(self.chat_id);
        let req = Request::get(url).body(Nothing).unwrap();
        self.get_messages_task = request(
            &mut self.get_messages_service,
            self.new_messages_callback.clone(),
            req,
        )
        .ok();

        // self.new_messages_callback
        //     .emit(mock_history(self.chat_id.user_me, self.chat_id.user_other))
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

fn mock_history(_me: UserId, _other: UserId) -> Vec<ChatMessage> {
    Vec::default()
}
