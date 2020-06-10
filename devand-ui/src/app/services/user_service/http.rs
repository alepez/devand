use super::FetchCallback;
use devand_core::User;
use gloo::timers::callback::Timeout;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const API_URL: &'static str = "/api/settings";

pub struct UserService {
    service: Arc<Mutex<FetchService>>,
    callback: FetchCallback,
    put_task: Arc<Mutex<Option<FetchTask>>>,
    put_debouncer: Option<Timeout>,
    get_task: Option<FetchTask>,
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

fn get(
    service: &mut FetchService,
    callback: Callback<Result<User, anyhow::Error>>,
) -> Result<FetchTask, anyhow::Error> {
    let req = Request::get(API_URL).body(Nothing).unwrap();
    request(service, callback, req)
}

fn put(
    service: &mut FetchService,
    callback: Callback<Result<User, anyhow::Error>>,
    user: User,
) -> Result<FetchTask, anyhow::Error> {
    let json = serde_json::to_string(&user).map_err(|_| anyhow::anyhow!("bo!"));
    let req = Request::put(API_URL).body(json).unwrap();
    request(service, callback, req)
}

impl UserService {
    pub fn new(callback: FetchCallback) -> Self {
        Self {
            service: Arc::new(Mutex::new(FetchService::new())),
            callback,
            put_task: Arc::new(Mutex::new(None)),
            put_debouncer: None,
            get_task: None,
        }
    }

    pub fn restore(&mut self) {
        let mut service = self.service.lock().unwrap();
        self.get_task = get(&mut service, self.callback.clone()).ok();
    }

    pub fn store(&mut self, user: &User) {
        let user: User = user.clone();
        let callback = self.callback.clone();
        let put_task = self.put_task.clone();
        let service = self.service.clone();

        self.put_debouncer = Some(Timeout::new(1_000, move || {
            let mut service = service.lock().unwrap();
            let mut put_task = put_task.lock().unwrap();
            *put_task.deref_mut() = put(&mut service, callback, user).ok();
        }));
    }
}
