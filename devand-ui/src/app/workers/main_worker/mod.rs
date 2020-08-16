#[cfg(not(feature = "mock_http"))]
mod http;

#[cfg(feature = "mock_http")]
mod mock;

use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;
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

const CODE_NOW_INTERVAL_MS: u64 = 5_000;
const LAZY_REQUEST_DELAY_MS: u64 = 2_000;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Init,
    Lazy(Box<Request>),
    SaveSelfUser(Box<devand_core::User>),
    VerifyEmail,
    LoadCodeNow,
    LoadPublicUserProfileByUsername(String),
    LoadPublicUserProfile(devand_core::UserId),
    LoadAffinities,
    LoadAvailabilityMatch,
    CheckOldPassword(String),
    EditPassword(String, String),
    ChatSendMessage(Vec<devand_core::UserId>, String),
    ChatPoll(
        Vec<devand_core::UserId>,
        Option<chrono::DateTime<chrono::Utc>>,
    ),
    ChatLoadHistory(Vec<devand_core::UserId>),
    LoadAllChats,
}

impl Request {
    pub fn lazy(self) -> Self {
        Request::Lazy(Box::new(self))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    SelfUserFetched(Box<devand_core::User>),
    CodeNowFetched(Box<devand_core::CodeNow>),
    PublicUserProfileFetched(Box<devand_core::PublicUserProfile>),
    AffinitiesFetched(Vec<devand_core::UserAffinity>),
    AvailabilityMatchFetched(Box<devand_core::schedule_matcher::AvailabilityMatch>),
    OldPasswordChecked(bool),
    PasswordEdited(()),
    Done(()),
    Error(String),
    ChatNewMessagesLoaded(Vec<devand_core::chat::ChatMessage>),
    ChatHistoryLoaded(devand_core::chat::ChatInfo),
    AllChatsLoaded(devand_core::UserChats),
}

pub enum Msg {
    CodeNowUpdate,
    Request(Request),
    Response(Response),
}

impl From<Response> for Msg {
    fn from(res: Response) -> Self {
        Msg::Response(res)
    }
}

pub struct MainWorker {
    link: AgentLink<MainWorker>,
    subscribers: HashSet<HandlerId>,

    // TODO Prevent memory leak
    // TODO Add cache for resources like public profile
    fetch_tasks: Vec<FetchTask>,
    _code_now_task: Box<dyn Task>,
    _timeout_task: Option<Box<dyn Task>>,

    _on_unload: Closure<dyn FnMut(BeforeUnloadEvent)>,

    pending: Arc<AtomicBool>,
}

impl Agent for MainWorker {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        log::info!("MainWorker created");

        let pending = Arc::new(AtomicBool::new(false));

        let code_now_task = make_code_now_task(link.clone());

        MainWorker {
            link,
            subscribers: HashSet::default(),
            fetch_tasks: Vec::default(),
            _code_now_task: code_now_task,
            _timeout_task: None,
            _on_unload: make_on_unload_callback(pending.clone()),
            pending,
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
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

            Msg::Response(res) => {
                self.publish(res);
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _who: HandlerId) {
        #[cfg(feature = "mock_http")]
        use self::mock::request;

        #[cfg(not(feature = "mock_http"))]
        use self::http::request;

        match msg {
            Request::Lazy(req) => {
                // Overwrites current timeout task
                self._timeout_task = Some(lazy_request(self, *req));
            }

            _ => request(self, msg),
        }
    }
}

impl MainWorker {
    fn publish(&self, res: Response) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, res.clone());
        }
    }
}

fn make_code_now_task(link: AgentLink<MainWorker>) -> Box<dyn Task> {
    let duration = Duration::from_millis(CODE_NOW_INTERVAL_MS);
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
fn make_on_unload_callback(pending: Arc<AtomicBool>) -> Closure<dyn FnMut(BeforeUnloadEvent)> {
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
