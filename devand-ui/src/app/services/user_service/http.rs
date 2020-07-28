use super::FetchCallback;
use devand_core::User;
use gloo::timers::callback::Timeout;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::BeforeUnloadEvent;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const DELAY_MS: u32 = 2_000;
const API_URL: &'static str = "/api/user";
const API_URL_VERIFY_EMAIL: &'static str = "/api/verify_email";

pub struct UserService {
    // put_handler is wrapped in Arc<Mutex> so it can be passed to Timeout
    put_handler: Arc<Mutex<PutHandler>>,
    get_handler: GetHandler,

    // on_unload callback exists because we warn user of unsaved changes
    // Changes are saved automatically, but only after DELAY_MS. User
    // may leave the page before this delay has passed or the consequent
    // request has finished. on_unload is triggered when the user leave
    // the page and triggers an alert about unsaved changes. This is how GMail
    // handles this case.
    #[allow(dead_code)]
    on_unload: Closure<dyn FnMut(BeforeUnloadEvent) -> ()>,
}

struct PutHandler {
    callback: FetchCallback,
    task: Option<FetchTask>,
    debouncer: Option<Timeout>,
    pending: Arc<Mutex<bool>>,
}

struct GetHandler {
    callback: FetchCallback,
    task: Option<FetchTask>,
    pending: Arc<Mutex<bool>>,
}

fn request<R>(
    callback: Callback<Result<User, anyhow::Error>>,
    pending: Arc<Mutex<bool>>,
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
            callback.emit(Err(anyhow::anyhow!(meta.status)))
        }

        if let Ok(mut pending) = pending.lock() {
            *pending.deref_mut() = false;
        }
    };

    FetchService::fetch(r, handler.into())
}

impl GetHandler {
    fn get(&mut self) {
        let req = Request::get(API_URL).body(Nothing).unwrap();
        self.task = request(self.callback.clone(), self.pending.clone(), req).ok();
    }
}

impl PutHandler {
    fn put(&mut self, user: User) {
        let json = serde_json::to_string(&user).map_err(|_| anyhow::anyhow!("bo!"));
        let req = Request::put(API_URL).body(json).unwrap();
        self.task = request(self.callback.clone(), self.pending.clone(), req).ok();
    }
}

impl UserService {
    pub fn new(callback: FetchCallback) -> Self {
        let put_handler = PutHandler {
            callback: callback.clone(),
            task: None,
            debouncer: None,
            pending: Arc::new(Mutex::new(false)),
        };

        let put_handler = Arc::new(Mutex::new(put_handler));

        let on_unload = make_on_unload_callback(put_handler.clone());

        let get_handler = GetHandler {
            callback: callback.clone(),
            task: None,
            pending: Arc::new(Mutex::new(false)),
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
            if let Ok(mut pending) = put_handler.pending.lock() {
                log::debug!("Start timer...");
                *pending.deref_mut() = true;
            }

            // Only if not already locked, clear previous timeout and set
            // a new delayed action. Note: overwriting debouncer clear the
            // previous timeout.
            put_handler.deref_mut().debouncer = Some(Timeout::new(DELAY_MS, move || {
                log::debug!("Start sending...");
                let mut put_handler = delayed_put_handler.lock().unwrap();
                put_handler.put(user);
            }));
        }
    }

    pub fn verify_email(&mut self) {
        let url = API_URL_VERIFY_EMAIL;
        let req = Request::post(url).body(Nothing).unwrap();

        let handler = move |_response: Response<Json<Result<(), anyhow::Error>>>| {
            // Just ignore the response
        };

        self.get_handler.task = FetchService::fetch(req, handler.into()).ok();
    }
}

fn make_on_unload_callback(
    put_handler: Arc<Mutex<PutHandler>>,
) -> Closure<dyn FnMut(BeforeUnloadEvent) -> ()> {
    let window = yew::utils::window();

    let on_unload = Box::new(move |e: BeforeUnloadEvent| {
        if let Ok(put_handler) = put_handler.lock() {
            if *put_handler.pending.lock().unwrap() {
                e.set_return_value("Changes you made may not be saved.");
            }
        }
    }) as Box<dyn FnMut(BeforeUnloadEvent)>;

    let on_unload = Closure::wrap(on_unload);

    window.set_onbeforeunload(Some(&on_unload.as_ref().unchecked_ref()));

    on_unload
}
