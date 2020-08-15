use super::MainWorker;
use devand_core::User;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch;
use yew::worker::*;

fn api_url_self_user() -> &'static str {
    "/api/user"
}

fn api_url_verify_email() -> &'static str {
    "/api/verify_email"
}

pub fn request(worker: &mut MainWorker, msg: super::Request, who: HandlerId) {
    use super::Request;
    use super::Response;

    match msg {
        Request::Init => {
            let url = api_url_self_user();
            let req = fetch::Request::get(url).body(Nothing).unwrap();
            worker._fetch_task = make_task(worker, who, req, Response::SelfUserFetched);
        }

        Request::SaveSelfUser(user) => {
            let url = api_url_self_user();
            let req = fetch::Request::put(url).body(make_json_body(user)).unwrap();
            worker._fetch_task = make_task(worker, who, req, Response::SelfUserFetched);
        }

        Request::VerifyEmail => {
            let url = api_url_verify_email();
            let req = fetch::Request::post(url).body(Nothing).unwrap();
            worker._fetch_task = make_task(worker, who, req, Response::Done);
        }

        _ => {
            log::debug!("ignored");
        }
    }
}

fn make_json_body<T: serde::ser::Serialize>(data: T) -> Result<String, anyhow::Error> {
    serde_json::to_string(&data).map_err(|e| anyhow::anyhow!(e))
}

fn make_task<T, IN>(
    worker: &MainWorker,
    who: HandlerId,
    req: fetch::Request<IN>,
    ctor: impl Fn(T) -> super::Response + 'static,
) -> Option<fetch::FetchTask>
where
    IN: Into<Text>,
    T: serde::de::DeserializeOwned + 'static,
{
    let link = worker.link.clone();

    let handler = move |response: fetch::Response<Json<Result<T, anyhow::Error>>>| {
        let (meta, Json(data)) = response.into_parts();

        if let Ok(data) = data {
            link.respond(who, ctor(data));
        } else {
            let error = anyhow::anyhow!(meta.status);
            link.respond(who, super::Response::Error(error.to_string()));
        }
    };

    fetch::FetchService::fetch(req, handler.into()).ok()
}
