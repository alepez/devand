use super::{MainWorker, Request, Response};
use yew::format::{Json, Nothing, Text};
use yew::services::fetch;

fn api_url_self_user() -> &'static str {
    "/api/user"
}

fn api_url_verify_email() -> &'static str {
    "/api/verify_email"
}

fn api_url_code_now() -> &'static str {
    "/api/code-now"
}

fn api_url_user(username: &str) -> String {
    format!("/api/u/{}", username)
}

fn api_url_affinities() -> &'static str {
    "/api/affinities"
}

pub fn request(worker: &mut MainWorker, msg: Request) {
    match msg {
        Request::Init => {
            let url = api_url_self_user();
            let req = fetch::Request::get(url).body(Nothing).unwrap();
            worker._fetch_task = make_task(worker, req, Response::SelfUserFetched);
        }

        Request::SaveSelfUser(user) => {
            let url = api_url_self_user();
            let req = fetch::Request::put(url).body(make_json_body(user)).unwrap();
            worker._fetch_task = make_task(worker, req, Response::SelfUserFetched);
        }

        Request::VerifyEmail => {
            let url = api_url_verify_email();
            let req = fetch::Request::post(url).body(Nothing).unwrap();
            worker._fetch_task = make_task(worker, req, Response::Done);
        }

        Request::LoadCodeNow => {
            let url = api_url_code_now();
            let req = fetch::Request::post(url).body(Nothing).unwrap();
            worker._fetch_task = make_task(worker, req, Response::CodeNowFetched);
        }

        Request::LoadPublicUserProfileByUsername(username) => {
            let url = api_url_user(&username);
            let req = fetch::Request::get(url).body(Nothing).unwrap();
            worker._fetch_task = make_task(worker, req, Response::PublicUserProfileFetched);
        }

        Request::LoadAffinities => {
            let url = api_url_affinities();
            let req = fetch::Request::get(url).body(Nothing).unwrap();
            worker._fetch_task = make_task(worker, req, Response::AffinitiesFetched);
        }

        // Program should never hit this
        Request::Lazy(_) => unimplemented!(),
    }
}

fn make_json_body<T: serde::ser::Serialize>(data: T) -> Result<String, anyhow::Error> {
    serde_json::to_string(&data).map_err(|e| anyhow::anyhow!(e))
}

fn make_task<T, IN>(
    worker: &MainWorker,
    req: fetch::Request<IN>,
    ctor: impl Fn(T) -> Response + 'static,
) -> Option<fetch::FetchTask>
where
    IN: Into<Text>,
    T: serde::de::DeserializeOwned + 'static,
{
    let link = worker.link.clone();

    let handler = move |response: fetch::Response<Json<Result<T, anyhow::Error>>>| {
        let (meta, Json(data)) = response.into_parts();

        if let Ok(data) = data {
            let res = ctor(data);
            link.send_message(res);
        } else {
            let error = anyhow::anyhow!(meta.status);
            let res = Response::Error(error.to_string());
            link.send_message(res);
        }
    };

    fetch::FetchService::fetch(req, handler.into()).ok()
}
