use super::FetchCallback;
use devand_core::User;
use gloo::timers::callback::Timeout;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const DELAY_MS: u32 = 5_000;
const API_URL: &'static str = "/api/settings";

pub struct UserService {
    // put_handler is wrapped in Arc<Mutex> so it can be passed to Timeout
    put_handler: Arc<Mutex<PutHandler>>,
    get_handler: GetHandler,
    on_unload: Closure<dyn FnMut() -> ()>,
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

        let on_unload = make_on_unload_callback(put_handler.clone());

        let get_handler = GetHandler {
            service: FetchService::new(),
            callback: callback.clone(),
            task: None,
        };

        Self {
            put_handler,
            get_handler,
            on_unload,
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

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn make_on_unload_callback(put_handler: Arc<Mutex<PutHandler>>) -> Closure<dyn FnMut() -> ()> {
    let window = yew::utils::window();

    let on_unload = Box::new(move || {
        log::debug!("Leaving...");
        if let Ok(put_handler) = put_handler.lock() {
            // if let Some(timeout) = put_handler.debouncer.take() {
            //     let cb = timeout.cancel();
            //     put_handler.debouncer = Some(Timeout::new(0, cb));
            // }

            if put_handler.debouncer.is_some() {
                log::error!("There are unsaved settings");
                // TODO This alert does not diplay
                alert("There are unsaved settings");
            }
        }
    }) as Box<dyn FnMut()>;

    let on_unload = Closure::wrap(on_unload);

    window.set_onbeforeunload(Some(&on_unload.as_ref().unchecked_ref()));

    on_unload
}
