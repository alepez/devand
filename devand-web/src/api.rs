use crate::auth::{AuthData, LoggedUser};
use crate::{CodeNowUsers, Mailer, PgDevandConn, WeekScheduleMatrix};
use chrono::prelude::*;
use chrono::Duration;
use devand_core::schedule_matcher::AvailabilityMatch;
use devand_core::{User, UserAffinity, UserId};
use rocket::http::Status;
use rocket::{Route, State};
use rocket_contrib::json::Json;

const BASE_URL: Option<&'static str> = option_env!("DEVAND_BASE_URL");
const DEFAULT_BASE_URL: &str = "http://localhost:8000";

pub fn routes() -> Vec<Route> {
    routes![
        user,
        user_put,
        verify_email,
        affinities,
        code_now,
        availability_match,
        chats,
        chat,
        chat_messages_get,
        chat_messages_post,
        chat_messages_poll,
        user_public_profile,
        user_public_profile_by_id,
        password_edit,
        password_check,
    ]
}

/// Retrieve user settings
#[get("/user")]
fn user(user: LoggedUser) -> Json<User> {
    Json(user.into())
}

/// Update user settings
#[put("/user", data = "<user>")]
fn user_put(
    auth_data: AuthData,
    user: Json<User>,
    conn: PgDevandConn,
    wsmc: State<WeekScheduleMatrix>,
) -> Option<Json<User>> {
    // Note: here we don't need LoggedUser (needs db access) but only
    // auth_data to check if we are modifiyng the right user.
    if !auth_data.matches_user(&user) {
        return None;
    }

    // Update immediately the week schedule matrix
    wsmc.0
        .write()
        .unwrap()
        .update(user.id, &user.settings.schedule);

    // Save new settings in db
    devand_db::save_user(user.0, &conn.0).map(Json)
}

/// Send a verification email to the logged user
#[post("/verify_email")]
fn verify_email(user: LoggedUser, mailer: State<Mailer>) -> Json<()> {
    if let Err(e) = mailer.verify_address(user.email.clone()) {
        log::error!("Cannot send email: {:?}", e);
    }
    Json(())
}

/// Retrieve user's affinities
#[get("/affinities")]
fn affinities(user: LoggedUser, conn: PgDevandConn) -> Option<Json<Vec<UserAffinity>>> {
    // TODO Optimize query for public profiles
    let users = devand_db::load_users(&conn.0)?;
    let user: User = user.into();
    let users = users.into_iter().map(|u| u.into());
    let affinities = devand_core::calculate_affinities(&user.into(), users);
    Some(Json(affinities.collect()))
}

/// Retrieve user's affinities who are online. When an user access this
/// endpoint, it is considered online for some time (see CodeNowUserMap::TTL)
#[post("/code-now")]
fn code_now(user: LoggedUser, code_now_users: State<CodeNowUsers>) -> Json<devand_core::CodeNow> {
    let user: User = user.into();

    let all_users: devand_core::CodeNowUsers = {
        // We always need a write lock, because we are updating cache ttl
        let mut cache = code_now_users.0.write().unwrap();
        cache.touch(user.clone());
        cache.clone().into()
    };

    Json(devand_core::CodeNow {
        current_user: user,
        all_users: all_users.0,
    })
}

/// Retrieve possible matching for the next week, considered user's schedule
/// and affinities
#[get("/availability-match")]
fn availability_match(user: LoggedUser, wsm: State<WeekScheduleMatrix>) -> Json<AvailabilityMatch> {
    let now = Utc::now();
    let start = now.checked_add_signed(Duration::hours(2)).unwrap();
    let User { settings, id, .. } = user.into();
    let availability = settings.schedule;
    let wsm = wsm.0.read().unwrap();
    let wsm = wsm.get();
    let res = wsm.find_all_users_matching_in_week(id, start, availability);
    // TODO [optimization] users with same availability time can bee lots. Sort by affinity and keep only first n
    Json(res)
}

/// Retrieve all chats
#[get("/chats")]
fn chats(user: LoggedUser, conn: PgDevandConn) -> Option<Json<devand_core::UserChats>> {
    let result = devand_db::load_chats_by_member(user.id, &conn);
    Some(Json(result))
}

/// Retrieve all messages in a chat, given its members
#[get("/chat/<members>/messages")]
fn chat_messages_get(
    user: LoggedUser,
    members: String,
    conn: PgDevandConn,
) -> Option<Json<Vec<devand_core::chat::ChatMessage>>> {
    let members = parse_members(&members);

    // TODO [refactoring] Authorize using request guard
    let authorized = members.contains(&user.id);
    if !authorized {
        return None;
    }

    let result = devand_db::load_chat_history_by_members(&members, &conn);

    if !result.is_empty() {
        devand_db::mark_messages_as_read_by(user.id, &result, &conn);
    }

    Some(Json(result))
}

/// Retrieve all messages in a chat, given its members
#[get("/chat/<members>")]
fn chat(
    user: LoggedUser,
    members: String,
    conn: PgDevandConn,
) -> Option<Json<devand_core::chat::ChatInfo>> {
    let members = parse_members(&members);

    // TODO [refactoring] Authorize using request guard
    let authorized = members.contains(&user.id);
    if !authorized {
        return None;
    }

    let messages = devand_db::load_chat_history_by_members(&members, &conn);

    if !messages.is_empty() {
        devand_db::mark_messages_as_read_by(user.id, &messages, &conn);
    }

    let members_info = members
        .iter()
        .filter_map(|&user_id| {
            // TODO [optimization] do not load full user info, only needed
            // TODO [optimization] for multiple users, just do only one call to db
            let user = devand_db::load_full_user_by_id(user_id, &conn)?;

            Some(devand_core::chat::ChatMemberInfo {
                user_id,
                verified_email: user.email_verified,
            })
        })
        .collect();

    let result = devand_core::chat::ChatInfo {
        members_info,
        messages,
    };

    Some(Json(result))
}

/// Post a new message in a chat
#[post("/chat/<members>/messages", data = "<txt>")]
fn chat_messages_post(
    user: LoggedUser,
    members: String,
    txt: Json<String>,
    mailer: State<Mailer>,
    conn: PgDevandConn,
) -> Option<Json<Vec<devand_core::chat::ChatMessage>>> {
    let author = user.id;
    let txt = txt.0;
    let members = parse_members(&members);

    // TODO [refactoring] Authorize using request guard
    let authorized = members.contains(&user.id);
    if !authorized {
        return None;
    }

    crate::notifications::notify_chat_members(
        BASE_URL.unwrap_or(DEFAULT_BASE_URL),
        &mailer,
        &conn,
        &user,
        &members,
    );

    if let Some(new_message) = devand_db::add_chat_message_by_members(&members, author, txt, &conn)
    {
        Some(Json(vec![new_message]))
    } else {
        None
    }
}

/// Retrieve new messages
#[get("/chat/<members>/messages/poll/<after>")]
fn chat_messages_poll(
    user: LoggedUser,
    members: String,
    after: i64,
    conn: PgDevandConn,
) -> Option<Json<Vec<devand_core::chat::ChatMessage>>> {
    // Note: Rocket 0.4 does not support websocket, so we just poll for new messages
    let members = parse_members(&members);

    // TODO [refactoring] Authorize using request guard
    let authorized = members.contains(&user.id);
    if !authorized {
        return None;
    }

    // TODO [optimization] It could be better loading from db only messages created after the
    // threshold, instead of filtering here.
    let result: Vec<_> = devand_db::load_chat_history_by_members(&members, &conn)
        .into_iter()
        .filter(|x| x.created_at.timestamp() > after)
        .collect();

    if !result.is_empty() {
        devand_db::mark_messages_as_read_by(user.id, &result, &conn);
    }

    Some(Json(result))
}

/// Load user public profile, given the user id. Note that this api is
/// accessible only by authenticated users, this is why we have the LoggedUser
/// guard, even if it is unused.
#[get("/u/<user_id>")]
fn user_public_profile_by_id(
    _user: LoggedUser,
    user_id: i32,
    conn: PgDevandConn,
) -> Option<Json<devand_core::PublicUserProfile>> {
    // TODO [optimization] Load only public profile
    let user = devand_db::load_user_by_id(UserId(user_id), &conn.0)?;
    Some(Json(user.into()))
}

/// Load user public profile, given the username. Note that this api is
/// accessible only by authenticated users, this is why we have the LoggedUser
/// guard, even if it is unused.
#[get("/u/<username>", rank = 2)]
fn user_public_profile(
    _user: LoggedUser,
    username: String,
    conn: PgDevandConn,
) -> Option<Json<devand_core::PublicUserProfile>> {
    // TODO [optimization] Load only public profile
    let user = devand_db::load_user_by_username(&username, &conn.0)?;
    Some(Json(user.into()))
}

/// Check if user password is the right one. This enpoint is used when
/// user is changing password.  It  just give an immediate feedback about
/// password validity.
#[post("/password-check", data = "<passwords>")]
fn password_check(
    auth_data: AuthData,
    conn: PgDevandConn,
    passwords: Json<devand_core::PasswordEdit>,
) -> Option<Json<bool>> {
    let ok = devand_db::auth::check_password(auth_data.user_id, &passwords.0.old_password, &conn)
        .ok()?;
    Some(Json(ok))
}

/// Change user password. Both old and new password must be provided, even
/// if the user is authenticated
#[post("/password-edit", data = "<passwords>")]
fn password_edit(
    auth_data: AuthData,
    conn: PgDevandConn,
    passwords: Json<devand_core::PasswordEdit>,
) -> Result<(), Status> {
    let ok = devand_db::auth::check_password(auth_data.user_id, &passwords.0.old_password, &conn)
        .map_err(|_| Status::InternalServerError)?;

    if !ok {
        return Err(Status::Unauthorized);
    }

    if !devand_core::auth::is_valid_password(&passwords.0.new_password) {
        return Err(Status::BadRequest);
    }

    devand_db::auth::set_password(auth_data.user_id, &passwords.0.new_password, &conn)
        .map_err(|_| Status::InternalServerError)
}

/// Given a string with user ids separated by a dash, return a Vec of UserId
fn parse_members(s: &str) -> Vec<UserId> {
    s.split('-')
        .filter_map(|x| x.parse().ok())
        .map(UserId)
        .collect()
}

#[cfg(test)]
mod test {
    use super::super::ignite;
    use super::super::PgDevandConn;
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    #[test]
    fn parse_members_ok() {
        let members = parse_members("5-72");
        assert_eq!(
            members,
            vec![devand_core::UserId(5), devand_core::UserId(72)]
        );
    }

    fn make_client() -> rocket::local::Client {
        Client::new(ignite()).unwrap()
    }

    fn join_user(rocket: &rocket::Rocket, username: String, password: String) {
        let conn = PgDevandConn::get_one(rocket).unwrap();
        let join_data = devand_db::auth::JoinData {
            username,
            email: "user1@devand.dev".into(),
            password,
        };
        // Result is ignored (an error should be generated if already exist,
        // but it's expected to exist if database is not reset)
        devand_db::auth::join(join_data, &conn).ok();
    }

    fn make_authenticated_client() -> rocket::local::Client {
        let client = make_client();
        let username = "user1";
        let password = "qwertyuiop1";
        join_user(client.rocket(), username.to_string(), password.to_string());

        {
            let response = client
                .post("/login/%2F")
                .body(format!("username={}&password={}", username, password))
                .header(ContentType::Form)
                .dispatch();

            assert_eq!(response.status(), Status::SeeOther);
        }

        client
    }

    #[test]
    #[ignore]
    fn anonimous_is_unauthorized_to_get_api_user() {
        let client = make_client();
        let response = client.get("/api/user").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    #[ignore]
    fn authenticated_can_get_api_user() {
        let client = make_authenticated_client();
        let response = client.get("/api/user").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    // TODO Test other APIs
}
