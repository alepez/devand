use super::{ChatServiceCallback, ChatServiceContent};
use devand_core::chat::ChatMessage;
use devand_core::{UserChats, UserId};
use yew::format::{Json, Nothing};
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

    pub fn load_other_user(&mut self, username: &str) {
        let url = api_url_get_user(username);
        let req = Request::get(url).body(Nothing).unwrap();
        let callback = self.callback.clone();

        let handler = move |response: Response<
            Json<Result<devand_core::PublicUserProfile, anyhow::Error>>,
        >| {
            let (meta, Json(data)) = response.into_parts();
            if let Ok(data) = data {
                callback.emit(ChatServiceContent::OtherUser(data));
            } else {
                log::error!("{:?}", &meta);
            }
        };

        self.task = FetchService::fetch(req, handler.into()).ok();
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

    pub fn load_history(&mut self, mut chat_members: Vec<UserId>) {
        chat_members.sort();
        self.chat_members = Some(chat_members.clone());
        let url = api_url_get(&chat_members);
        let req = Request::get(url).body(Nothing).unwrap();
        let callback = self.callback.clone();
        let handler = move |response: Response<
            Json<Result<devand_core::chat::ChatInfo, anyhow::Error>>,
        >| {
            let (meta, Json(data)) = response.into_parts();
            if let Ok(data) = data {
                callback.emit(ChatServiceContent::NewMessagess(data.messages));
                for member in data.members_info {
                    callback.emit(ChatServiceContent::OtherUserExtended(member))
                }
            } else {
                log::error!("{:?}", &meta);
            }
        };

        self.task = FetchService::fetch(req, handler.into()).ok();
    }

    pub fn send_message(&mut self, txt: String) {
        if let Some(chat_members) = &self.chat_members {
            let url = api_url_post(chat_members);
            let json = serde_json::to_string(&txt).map_err(|_| anyhow::anyhow!("Cannot serialize"));
            let req = Request::post(url).body(json).unwrap();
            let callback = self.callback.clone();
            let handler = move |response: Response<
                Json<Result<Vec<devand_core::chat::ChatMessage>, anyhow::Error>>,
            >| {
                let (meta, Json(data)) = response.into_parts();
                if let Ok(data) = data {
                    callback.emit(ChatServiceContent::NewMessagess(data));
                } else {
                    log::error!("{:?}", &meta);
                }
            };

            self.task = FetchService::fetch(req, handler.into()).ok();
        } else {
            log::error!("Cannot send message without knowing chat members");
        }
    }

    pub fn poll(&mut self, last_message: Option<&ChatMessage>) {
        if let Some(chat_members) = &self.chat_members {
            let url = api_url_poll(chat_members, last_message);
            let req = Request::get(url).body(Nothing).unwrap();
            let callback = self.callback.clone();
            let handler = move |response: Response<
                Json<Result<Vec<devand_core::chat::ChatMessage>, anyhow::Error>>,
            >| {
                let (meta, Json(data)) = response.into_parts();
                if let Ok(data) = data {
                    callback.emit(ChatServiceContent::NewMessagess(data));
                } else {
                    log::error!("{:?}", &meta);
                }
            };

            self.task = FetchService::fetch(req, handler.into()).ok();
        }
    }
}
