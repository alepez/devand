use devand_core::User;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use super::FetchCallback;

const API_URL: &'static str = "/api/settings";

pub struct UserService {
    service: FetchService,
    task: Option<FetchTask>,
    callback: FetchCallback,
}

impl UserService {
    pub fn new(callback: FetchCallback) -> Self {
        Self {
            service: FetchService::new(),
            task: None,
            callback,
        }
    }

    fn request<R>(
        &mut self,
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

        self.service.fetch(r, handler.into())
    }

    fn get(
        &mut self,
        callback: Callback<Result<User, anyhow::Error>>,
    ) -> Result<FetchTask, anyhow::Error> {
        let request = Request::get(API_URL).body(Nothing).unwrap();
        self.request(callback, request)
    }

    fn put(
        &mut self,
        callback: Callback<Result<User, anyhow::Error>>,
        user: User,
    ) -> Result<FetchTask, anyhow::Error> {
        let json = serde_json::to_string(&user).map_err(|_| anyhow::anyhow!("bo!"));
        let request = Request::put(API_URL).body(json).unwrap();
        self.request(callback, request)
    }

    pub fn restore(&mut self) {
        self.task = self.get(self.callback.clone()).ok();
    }

    pub fn store(&mut self, user: &User) {
        let user: User = user.clone();
        self.task = self.put(self.callback.clone(), user).ok();
    }
}


