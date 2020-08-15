use super::MainWorker;
use devand_core::User;
use yew::format::{Json, Nothing};
use yew::services::fetch;
use yew::worker::*;

fn api_url_get_self_user() -> &'static str {
    "/api/user"
}

pub fn handle_input(worker: &mut MainWorker, msg: super::Request, who: HandlerId) {
    log::info!("Request: {:?}", msg);
    use super::Request;

    match msg {
        Request::Init => {
            log::info!("Initializing...");

            let url = api_url_get_self_user();

            let req = fetch::Request::get(url).body(Nothing).unwrap();

            let callback = worker.link.callback(super::Msg::Response);

            let handler = move |response: fetch::Response<Json<Result<User, anyhow::Error>>>| {
                let (meta, Json(data)) = response.into_parts();

                if let Ok(data) = data {
                    let res = super::Response::SelfUserFetched(data);
                    callback.emit(Ok(res));
                } else {
                    let error = anyhow::anyhow!(meta.status);
                    callback.emit(Err(error));
                }
            };

            let task = fetch::FetchService::fetch(req, handler.into()).ok();

            worker.fetch_task = task;
        }
        Request::SaveSelfUser(user) => {
            log::info!("Saving user...");
        }
        Request::VerifyEmail => {
            log::info!("Verifing email...");
        }
    }
}
