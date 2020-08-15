// #[cfg(not(feature = "mock_http"))]
mod http;

// #[cfg(feature = "mock_http")]
mod mock;

use devand_core::User;
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use yew::services::fetch::FetchTask;
use yew::services::interval::IntervalService;
use yew::services::timeout::TimeoutService;
use yew::services::Task;
use yew::worker::*;

const INTERVAL_MS: u64 = 2_000;
const LAZY_REQUEST_DELAY_MS: u64 = 2_000;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Init,
    Lazy(Box<Request>),
    SaveSelfUser(User),
    VerifyEmail,
}

impl Request {
    pub fn lazy(self) -> Self {
        Request::Lazy(Box::new(self))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    SelfUserFetched(User),
    Done(()),
    Error(String),
}

pub enum Msg {
    AutoUpdate,
    Request(Request),
}

pub struct MainWorker {
    link: AgentLink<MainWorker>,
    // TODO Prevent overwriting (canceling) of fetch task
    _fetch_task: Option<FetchTask>,
    _interval_task: Box<dyn Task>,
    _timeout_task: Option<Box<dyn Task>>,
}

impl Agent for MainWorker {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let duration = Duration::from_millis(INTERVAL_MS);
        let callback = link.callback(|_| Msg::AutoUpdate);
        let task = IntervalService::spawn(duration, callback);
        MainWorker {
            link,
            _fetch_task: None,
            _interval_task: Box::new(task),
            _timeout_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::AutoUpdate => {
                // TODO
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

fn lazy_request(main_worker: &MainWorker, req: Request) -> Box<dyn Task> {
    let duration = Duration::from_millis(LAZY_REQUEST_DELAY_MS);

    let callback = main_worker
        .link
        .callback(move |_| Msg::Request(req.clone()));

    Box::new(TimeoutService::spawn(duration, callback))
}
