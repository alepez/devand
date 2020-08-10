use super::FetchCallback;
use devand_core::CodeNow;
use gloo::timers::callback::Interval;
use std::sync::{Arc, Mutex};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const INTERVAL_MS: u32 = 5_000;

const API_URL: &str = "/api/code-now";

pub struct CodeNowService {
    post_handler: Arc<Mutex<PostHandler>>,

    #[allow(dead_code)]
    pinger: Interval,
}

struct PostHandler {
    callback: FetchCallback,
    task: Option<FetchTask>,
}

fn request<R>(
    callback: Callback<Result<CodeNow, anyhow::Error>>,
    r: http::request::Request<R>,
) -> Result<FetchTask, anyhow::Error>
where
    R: std::convert::Into<std::result::Result<std::string::String, anyhow::Error>>,
{
    let handler = move |response: Response<Json<Result<CodeNow, anyhow::Error>>>| {
        let (meta, Json(data)) = response.into_parts();
        if meta.status.is_success() {
            callback.emit(data)
        } else {
            callback.emit(Err(anyhow::anyhow!(meta.status)))
        }
    };

    FetchService::fetch(r, handler.into())
}

impl PostHandler {
    fn post(&mut self) {
        let req = Request::post(API_URL).body(Nothing).unwrap();
        self.task = request(self.callback.clone(), req).ok();
    }
}

impl CodeNowService {
    pub fn new(callback: FetchCallback) -> Self {
        let post_handler = PostHandler {
            callback: callback.clone(),
            task: None,
        };

        let post_handler = Arc::new(Mutex::new(post_handler));

        let interval_post_handler = post_handler.clone();

        let pinger = Interval::new(INTERVAL_MS, move || {
            let mut post_handler = interval_post_handler.lock().unwrap();
            post_handler.post();
        });

        Self {
            post_handler,
            pinger,
        }
    }

    pub fn restore(&mut self) {
        self.post_handler.lock().unwrap().post();
    }
}
