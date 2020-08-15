// #[cfg(not(feature = "mock_http"))]
mod http;

// #[cfg(feature = "mock_http")]
mod mock;

use devand_core::User;
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use yew::services::fetch::FetchTask;
use yew::services::interval::IntervalService;
use yew::services::Task;
use yew::worker::*;

const INTERVAL_MS: u64 = 2_000;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Init,
    SaveSelfUser(User),
    VerifyEmail,
}

// TODO Add Error
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    SelfUserFetched(User),
}

pub enum Msg {
    Updating,
    Response(Result<Response, anyhow::Error>),
}

pub struct MainWorker {
    link: AgentLink<MainWorker>,
    fetch_task: Option<FetchTask>,
    _interval_task: Box<dyn Task>,
}

impl Agent for MainWorker {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let duration = Duration::from_millis(INTERVAL_MS);
        let callback = link.callback(|_| Msg::Updating);
        let task = IntervalService::spawn(duration, callback);
        MainWorker {
            link,
            fetch_task: None,
            _interval_task: Box::new(task),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Updating => {
                log::info!("Tick...");
            }
            Msg::Response(res) => match res {
                Ok(res) => {
                    // TODO
                    log::debug!("Response: {:?}", res);
                }
                Err(err) => {
                    log::error!("Error: {}", err);
                }
            },
        }
    }

    #[cfg(feature = "mock_http")]
    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        log::info!("Request: {:?}", msg);
        mock::handle_input(self, msg, who)
    }

    #[cfg(not(feature = "mock_http"))]
    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        log::info!("Request: {:?}", msg);
        http::handle_input(self, msg, who)
    }
}
