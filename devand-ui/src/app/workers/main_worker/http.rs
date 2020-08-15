use super::MainWorker;
use devand_core::User;
use yew::format::{Json, Nothing};
use yew::services::fetch;
use yew::worker::*;

fn api_url_self_user() -> &'static str {
    "/api/user"
}

pub fn request(worker: &mut MainWorker, msg: super::Request, who: HandlerId) {
    use super::Request;

    match msg {
        Request::Init => {
            let url = api_url_self_user();
            let req = fetch::Request::get(url).body(Nothing).unwrap();
            let link = worker.link.clone();

            let handler = move |response: fetch::Response<Json<Result<User, anyhow::Error>>>| {
                let (meta, Json(data)) = response.into_parts();

                if let Ok(data) = data {
                    link.respond(who, super::Response::SelfUserFetched(data));
                } else {
                    let error = anyhow::anyhow!(meta.status);
                    link.respond(who, super::Response::Error(error.to_string()));
                }
            };

            worker._fetch_task = fetch::FetchService::fetch(req, handler.into()).ok();
        }

        Request::SaveSelfUser(user) => {
            log::info!("Saving user...");
            let url = api_url_self_user();
            let json = serde_json::to_string(&user).map_err(|_| anyhow::anyhow!("bo!"));
            let req = fetch::Request::put(url).body(json).unwrap();
            let link = worker.link.clone();

            let handler = move |response: fetch::Response<Json<Result<User, anyhow::Error>>>| {
                let (meta, Json(data)) = response.into_parts();

                if let Ok(data) = data {
                    link.respond(who, super::Response::SelfUserFetched(data));
                } else {
                    let error = anyhow::anyhow!(meta.status);
                    link.respond(who, super::Response::Error(error.to_string()));
                }
            };

            worker._fetch_task = fetch::FetchService::fetch(req, handler.into()).ok();
        }

        Request::VerifyEmail => {
            log::info!("Verifing email...");
        }

        _ => {
            log::debug!("ignored");
        }
    }
}
