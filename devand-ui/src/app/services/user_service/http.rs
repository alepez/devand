use super::FetchCallback;
use devand_core::User;
use gloo::timers::callback::Timeout;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const DELAY_MS: u32 = 5_000;
const API_URL: &'static str = "/api/settings";

pub struct UserService {
    // put_handler is wrapped in Arc<Mutex> so it can be passed to Timeout
    put_handler: Arc<Mutex<PutHandler>>,
    get_handler: GetHandler,
}

struct PutHandler {
    service: FetchService,
    callback: FetchCallback,
    task: Option<FetchTask>,
    debouncer: Option<Timeout>,
}

struct GetHandler {
    service: FetchService,
    callback: FetchCallback,
    task: Option<FetchTask>,
}

fn request<R>(
    service: &mut FetchService,
    callback: Callback<Result<User, anyhow::Error>>,
    r: http::request::Request<R>,
) -> Result<FetchTask, anyhow::Error>
where
    R: std::convert::Into<std::result::Result<std::string::String, anyhow::Error>>,
{
    let handler = move |response: Response<Json<Result<User, anyhow::Error>>>| {
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

impl PutHandler {
    fn put(&mut self, user: User) {
        let json = serde_json::to_string(&user).map_err(|_| anyhow::anyhow!("bo!"));
        let req = Request::put(API_URL).body(json).unwrap();
        self.task = request(&mut self.service, self.callback.clone(), req).ok();
    }
}

impl UserService {
    pub fn new(callback: FetchCallback) -> Self {
        let put_handler = PutHandler {
            service: FetchService::new(),
            callback: callback.clone(),
            task: None,
            debouncer: None,
        };

        let put_handler = Arc::new(Mutex::new(put_handler));

        let get_handler = GetHandler {
            service: FetchService::new(),
            callback: callback.clone(),
            task: None,
        };

        Self {
            put_handler,
            get_handler,
        }
    }

    pub fn restore(&mut self) {
        self.get_handler.get();
    }

    pub fn store(&mut self, user: &User) {
        let user: User = user.clone();

        let delayed_put_handler = self.put_handler.clone();

        if let Ok(mut put_handler) = self.put_handler.lock() {
            // Only if not already locked, clear previous timeout and set
            // a new delayed action. Note: overwriting debouncer clear the
            // previous timeout.
            put_handler.deref_mut().debouncer = Some(Timeout::new(DELAY_MS, move || {
                if let Ok(mut put_handler) = delayed_put_handler.lock() {
                    put_handler.put(user);
                }
            }));
        }
    }
}
