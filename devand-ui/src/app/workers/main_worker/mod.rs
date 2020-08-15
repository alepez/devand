#[cfg(not(feature = "mock_http"))]
mod http;

#[cfg(feature = "mock_http")]
mod mock;

use serde_derive::{Deserialize, Serialize};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::BeforeUnloadEvent;
use yew::services::fetch::FetchTask;
use yew::services::interval::IntervalService;
use yew::services::timeout::TimeoutService;
use yew::services::Task;
use yew::worker::*;

const INTERVAL_MS: u64 = 5_000;
const LAZY_REQUEST_DELAY_MS: u64 = 2_000;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Init,
    Lazy(Box<Request>),
    SaveSelfUser(devand_core::User),
    VerifyEmail,
    LoadCodeNow,
}

impl Request {
    pub fn lazy(self) -> Self {
        Request::Lazy(Box::new(self))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    SelfUserFetched(devand_core::User),
    CodeNowFetched(devand_core::CodeNow),
    Done(()),
    Error(String),
}

pub enum Msg {
    CodeNowUpdate,
    Request(Request),
}

pub struct MainWorker {
    link: AgentLink<MainWorker>,

    // TODO Prevent overwriting (canceling) of fetch task
    _fetch_task: Option<FetchTask>,
    _code_now_task: Box<dyn Task>,
    _timeout_task: Option<Box<dyn Task>>,

    _on_unload: Closure<dyn FnMut(BeforeUnloadEvent) -> ()>,

    pending: Arc<AtomicBool>,
}

impl Agent for MainWorker {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let pending = Arc::new(AtomicBool::new(false));

        let code_now_task = make_code_now_task(link.clone());

        MainWorker {
            link,
            _fetch_task: None,
            _code_now_task: code_now_task,
            _timeout_task: None,
            _on_unload: make_on_unload_callback(pending.clone()),
            pending,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::CodeNowUpdate => {
                log::debug!("CodeNowUpdate");
                let req = Request::LoadCodeNow;
                self.link.send_input(req);
            }
            Msg::Request(req) => {
                self.link.send_input(req);
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        #[cfg(feature = "mock_http")]
        use self::mock::request;

        #[cfg(not(feature = "mock_http"))]
        use self::http::request;

        match msg {
            Request::Lazy(req) => {
                // Overwrites current timeout task
                self._timeout_task = Some(lazy_request(self, *req));
            }
            _ => request(self, msg, who),
        }
    }
}

fn make_code_now_task(link: AgentLink<MainWorker>) -> Box<dyn Task> {
    let duration = Duration::from_millis(INTERVAL_MS);
    let callback = link.callback(|_| Msg::CodeNowUpdate);
    let task = IntervalService::spawn(duration, callback);
    Box::new(task)
}

/// Delay this request by some seconds, so it can be overridden
/// Example: users edit a field in theirs settings. Only after some seconds
/// of "inactivity" it is actually sent.
fn lazy_request(main_worker: &MainWorker, req: Request) -> Box<dyn Task> {
    let pending = main_worker.pending.clone();

    pending.store(true, Ordering::SeqCst);

    let duration = Duration::from_millis(LAZY_REQUEST_DELAY_MS);

    // TODO [optimization] avoid cloning (or make cloning cheap)
    let callback = main_worker.link.callback(move |_| {
        pending.store(false, Ordering::SeqCst);
        Msg::Request(req.clone())
    });

    Box::new(TimeoutService::spawn(duration, callback))
}

/// _on_unload callback exists because we warn user of unsaved changes
/// Changes are saved automatically, but only after DELAY_MS. User
/// may leave the page before this delay has passed or the consequent
/// request has finished. on_unload is triggered when the user leave
/// the page and triggers an alert about unsaved changes. This is how GMail
/// handles this case.
fn make_on_unload_callback(
    pending: Arc<AtomicBool>,
) -> Closure<dyn FnMut(BeforeUnloadEvent) -> ()> {
    use wasm_bindgen::JsCast;

    let window = yew::utils::window();

    let on_unload = Box::new(move |e: BeforeUnloadEvent| {
        let pending = pending.load(Ordering::SeqCst);
        if pending {
            e.set_return_value("Changes you made may not be saved.");
        }
    }) as Box<dyn FnMut(BeforeUnloadEvent)>;

    let on_unload = Closure::wrap(on_unload);

    window.set_onbeforeunload(Some(&on_unload.as_ref().unchecked_ref()));

    on_unload
}
