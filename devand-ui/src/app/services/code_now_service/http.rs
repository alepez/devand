use super::FetchCallback;
use devand_core::CodeNow;
use gloo::timers::callback::Interval;
use std::sync::{Arc, Mutex};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const INTERVAL_MS: u32 = 5_000;

const API_URL: &'static str = "/api/code-now";

pub struct CodeNowService {
    get_handler: Arc<Mutex<GetHandler>>,

    #[allow(dead_code)]
    pinger: Interval,
}

struct GetHandler {
    service: FetchService,
    callback: FetchCallback,
    task: Option<FetchTask>,
}

fn request<R>(
    service: &mut FetchService,
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
            callback.emit(Err(anyhow::anyhow!("Error {} restoring user", meta.status)))
        }
    };

    service.fetch(r, handler.into())
}

impl GetHandler {
    fn get(&mut self) {
        let req = Request::get(API_URL).body(Nothing).unwrap();
        self.task = request(&mut self.service, self.callback.clone(), req).ok();
    }
}

impl CodeNowService {
    pub fn new(callback: FetchCallback) -> Self {
        let get_handler = GetHandler {
            service: FetchService::new(),
            callback: callback.clone(),
            task: None,
        };

        let get_handler = Arc::new(Mutex::new(get_handler));

        let interval_get_handler = get_handler.clone();

        let pinger = Interval::new(INTERVAL_MS, move || {
            let mut get_handler = interval_get_handler.lock().unwrap();
            get_handler.get();
        });

        Self {
            get_handler,
            pinger,
        }
    }

    pub fn restore(&mut self) {
        self.get_handler.lock().unwrap().get();
    }
}
