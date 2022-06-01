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

fn api_url_user_by_username(username: &str) -> String {
    format!("/api/u/{}", username)
}

fn api_url_user(user_id: devand_core::UserId) -> String {
    format!("/api/u/{}", user_id.0)
}

fn api_url_affinities() -> &'static str {
    "/api/affinities"
}

fn api_url_availability_match() -> &'static str {
    "/api/availability-match"
}

fn api_url_password_check() -> &'static str {
    "/api/password-check"
}
fn api_url_password_edit() -> &'static str {
    "/api/password-edit"
}

fn api_url_chat(chat_members: &[devand_core::UserId]) -> String {
    format!("/api/chat/{}", encode_chat_members(chat_members))
}

fn api_url_chat_messages(members: &[devand_core::UserId]) -> String {
    let members = encode_chat_members(members);
    format!("/api/chat/{}/messages", members)
}

fn api_url_chat_messages_poll(
    members: &[devand_core::UserId],
    from_created_at: Option<chrono::DateTime<chrono::Utc>>,
) -> String {
    let members = encode_chat_members(members);
    format!(
        "/api/chat/{}/messages/poll/{}",
        members,
        from_created_at.map(|x| x.timestamp()).unwrap_or(0)
    )
}

fn api_url_chats() -> &'static str {
    "/api/chats"
}

fn encode_chat_members(chat_members: &[devand_core::UserId]) -> String {
    chat_members
        .iter()
        .map(|x| x.0.to_string())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn request(worker: &mut MainWorker, msg: Request) {
    let task = match msg {
        Request::Init => {
            let req = get(api_url_self_user());
            task(worker, req, Response::SelfUserFetched)
        }

        Request::SaveSelfUser(user) => {
            let req = put(api_url_self_user(), json(user));
            task(worker, req, Response::SelfUserFetched)
        }

        Request::VerifyEmail => {
            let req = post(api_url_verify_email(), Nothing);
            task(worker, req, Response::Done)
        }

        Request::LoadCodeNow => {
            let req = post(api_url_code_now(), Nothing);
            task(worker, req, Response::CodeNowFetched)
        }

        Request::LoadPublicUserProfile(user_id) => {
            let req = get(&api_url_user(user_id));
            task(worker, req, Response::PublicUserProfileFetched)
        }

        Request::LoadPublicUserProfileByUsername(username) => {
            let req = get(&api_url_user_by_username(&username));
            task(worker, req, Response::PublicUserProfileFetched)
        }

        Request::LoadAffinities => {
            let req = get(api_url_affinities());
            task(worker, req, Response::AffinitiesFetched)
        }

        Request::LoadAvailabilityMatch => {
            let req = get(api_url_availability_match());
            task(worker, req, Response::AvailabilityMatchFetched)
        }

        Request::CheckOldPassword(old_password) => {
            let body = devand_core::PasswordEdit {
                old_password,
                new_password: String::default(),
            };

            let req = post(api_url_password_check(), json(body));
            task(worker, req, Response::OldPasswordChecked)
        }

        Request::EditPassword(old_password, new_password) => {
            let body = devand_core::PasswordEdit {
                old_password,
                new_password,
            };

            let req = post(api_url_password_edit(), json(body));
            task(worker, req, Response::PasswordEdited)
        }

        Request::ChatSendMessage(members, txt) => {
            let req = post(&api_url_chat_messages(&members), json(txt));
            task(worker, req, Response::ChatNewMessagesLoaded)
        }

        Request::ChatPoll(members, from_created_at) => {
            let req = get(&api_url_chat_messages_poll(&members, from_created_at));
            task(worker, req, Response::ChatNewMessagesLoaded)
        }

        Request::ChatLoadHistory(members) => {
            let req = get(&api_url_chat(&members));
            task(worker, req, Response::ChatHistoryLoaded)
        }

        Request::LoadAllChats => {
            let req = get(api_url_chats());
            task(worker, req, Response::AllChatsLoaded)
        }

        // Program should never hit this
        Request::Lazy(_) => unimplemented!(),
    };

    if let Some(task) = task {
        worker.fetch_tasks.push(task);
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
