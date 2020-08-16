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
            let req = get(api_url_self_user());
            worker._fetch_task = task(worker, req, Response::SelfUserFetched);
        }

        Request::SaveSelfUser(user) => {
            let req = put(api_url_self_user(), json(user));
            worker._fetch_task = task(worker, req, Response::SelfUserFetched);
        }

        Request::VerifyEmail => {
            let req = post(api_url_verify_email(), Nothing);
            worker._fetch_task = task(worker, req, Response::Done);
        }

        Request::LoadCodeNow => {
            let req = post(api_url_code_now(), Nothing);
            worker._fetch_task = task(worker, req, Response::CodeNowFetched);
        }

        Request::LoadPublicUserProfileByUsername(username) => {
            let req = get(&api_url_user(&username));
            worker._fetch_task = task(worker, req, Response::PublicUserProfileFetched);
        }

        Request::LoadAffinities => {
            let req = get(api_url_affinities());
            worker._fetch_task = task(worker, req, Response::AffinitiesFetched);
        }

        // Program should never hit this
        Request::Lazy(_) => unimplemented!(),
    }
}

fn get(url: &str) -> fetch::Request<Nothing> {
    fetch::Request::get(url).body(Nothing).unwrap()
}

fn post<IN>(url: &str, body: IN) -> fetch::Request<IN>
where
    IN: Into<Text>,
{
    fetch::Request::post(url).body(body).unwrap()
}

fn put<IN>(url: &str, body: IN) -> fetch::Request<IN>
where
    IN: Into<Text>,
{
    fetch::Request::put(url).body(body).unwrap()
}

fn json<T: serde::ser::Serialize>(data: T) -> Result<String, anyhow::Error> {
    serde_json::to_string(&data).map_err(|e| anyhow::anyhow!(e))
}

fn task<T, IN>(
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
