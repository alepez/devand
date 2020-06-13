use super::FetchCallback;
use devand_core::UserAffinity;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const API_URL: &'static str = "/api/affinities";

pub struct CodeNowService {
    get_handler: GetHandler,
}

struct GetHandler {
    service: FetchService,
    callback: FetchCallback,
    task: Option<FetchTask>,
}

fn request<R>(
    service: &mut FetchService,
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
            callback.emit(Err(anyhow::anyhow!("Error {} restoring user", meta.status)))
        }
    };

    service.fetch(r, handler.into())
}

impl GetHandler {
    fn get(&mut self) {
        let req = Request::get(API_URL).body(Nothing).unwrap();
        self.task = request(
            &mut self.service,
            self.callback.clone(),
            req,
        )
        .ok();
    }
}

impl CodeNowService {
    pub fn new(callback: FetchCallback) -> Self {
        let get_handler = GetHandler {
            service: FetchService::new(),
            callback: callback.clone(),
            task: None,
        };

        Self {
            get_handler,
        }
    }

    pub fn restore(&mut self) {
        self.get_handler.get();
    }
}
