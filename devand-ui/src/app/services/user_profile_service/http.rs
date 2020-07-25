use super::OtherUserLoadedCallback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

fn api_url_get_user(u: &str) -> String {
    format!("/api/u/{}", u)
}

pub struct UserProfileService {
    other_user_loaded_callback: OtherUserLoadedCallback,
    task: Option<FetchTask>,
}

impl UserProfileService {
    pub fn new(other_user_loaded_callback: OtherUserLoadedCallback) -> Self {
        Self {
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
}
