use super::FetchCallback;
use devand_core::UserAffinity;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const API_URL: &str = "/api/affinities";

pub struct AffinitiesService {
    get_handler: GetHandler,
}

struct GetHandler {
    callback: FetchCallback,
    task: Option<FetchTask>,
}

fn request<R>(
    callback: Callback<Result<Vec<UserAffinity>, anyhow::Error>>,
    r: http::request::Request<R>,
) -> Result<FetchTask, anyhow::Error>
where
    R: std::convert::Into<std::result::Result<std::string::String, anyhow::Error>>,
{
    let handler = move |response: Response<Json<Result<Vec<UserAffinity>, anyhow::Error>>>| {
        let (meta, Json(data)) = response.into_parts();
        if meta.status.is_success() {
            callback.emit(data)
        } else {
            callback.emit(Err(anyhow::anyhow!(meta.status)))
        }
    };

    FetchService::fetch(r, handler.into())
}

impl GetHandler {
    fn get(&mut self) {
        let req = Request::get(API_URL).body(Nothing).unwrap();
        self.task = request(self.callback.clone(), req).ok();
    }
}

impl AffinitiesService {
    pub fn new(callback: FetchCallback) -> Self {
        let get_handler = GetHandler {
            callback,
            task: None,
        };

        Self { get_handler }
    }

    pub fn restore(&mut self) {
        self.get_handler.get();
    }
}
