use crate::auth::{AuthData, LoggedUser};
use crate::{CodeNowUsers, PgDevandConn, WeekScheduleMatrix};
use chrono::prelude::*;
use chrono::Duration;
use devand_core::schedule_matcher::AvailabilityMatch;
use devand_core::{User, UserAffinity, UserId};
use rocket::{Route, State};
use rocket_contrib::json::Json;

pub fn routes() -> Vec<Route> {
    routes![
        settings,
        settings_put,
        affinities,
        code_now,
        availability_match,
        chat_messages_get,
        chat_messages_post,
        chat_messages_poll,
        user_public_profile,
        user_public_profile_by_id,
        password_edit,
        password_check,
    ]
}

#[get("/settings")]
fn settings(user: LoggedUser) -> Json<User> {
    Json(user.into())
}

#[put("/settings", data = "<user>")]
fn settings_put(
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

    wsmc.0
        .write()
        .unwrap()
        .update(user.id, &user.settings.schedule);

    devand_db::save_user(user.0, &conn.0).map(|x| Json(x))
}

#[get("/affinities")]
fn affinities(user: LoggedUser, conn: PgDevandConn) -> Option<Json<Vec<UserAffinity>>> {
    let users = devand_db::load_users(&conn.0)?;
    let affinities = devand_core::calculate_affinities(&user.into(), users);
    Some(Json(affinities.collect()))
}

#[get("/code-now")]
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

#[get("/availability-match")]
fn availability_match(user: LoggedUser, wsm: State<WeekScheduleMatrix>) -> Json<AvailabilityMatch> {
    let now = Utc::now();
    let next_week = now.checked_add_signed(Duration::days(7)).unwrap();
    let User { settings, id, .. } = user.into();
    let availability = settings.schedule;
    let wsm = wsm.0.read().unwrap();
    let wsm = wsm.get();
    let res = wsm.find_all_users_matching_in_week(id, next_week, availability);
    Json(res)
}

fn parse_members(s: &str) -> Vec<UserId> {
    s.split("-")
        .filter_map(|x| x.parse().ok())
        .map(UserId)
        .collect()
}

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
    Some(Json(result))
}

#[post("/chat/<members>/messages", data = "<txt>")]
fn chat_messages_post(
    user: LoggedUser,
    members: String,
    txt: Json<String>,
    conn: PgDevandConn,
) -> Option<Json<Vec<devand_core::chat::ChatMessage>>> {
    let new_message = devand_core::chat::ChatMessage {
        author: user.id,
        txt: txt.0,
        created_at: Utc::now(),
    };

    let members = parse_members(&members);

    // TODO [refactoring] Authorize using request guard
    let authorized = members.contains(&user.id);
    if !authorized {
        return None;
    }

    if let Some(new_message) = devand_db::add_chat_message_by_members(&members, new_message, &conn)
    {
        Some(Json(vec![new_message]))
    } else {
        None
    }
}

#[get("/chat/<members>/messages/poll/<after>")]
fn chat_messages_poll(
    user: LoggedUser,
    members: String,
    after: i64,
    conn: PgDevandConn,
) -> Option<Json<Vec<devand_core::chat::ChatMessage>>> {
    // Note: Rocket 0.16.2 does not support websocket, so we just poll for new messages
    let members = parse_members(&members);

    // TODO [refactoring] Authorize using request guard
    let authorized = members.contains(&user.id);
    if !authorized {
        return None;
    }

    // TODO [optimization] It could be better loading from db only messages created after the
    // threshold, instead of filtering here.
    let result = devand_db::load_chat_history_by_members(&members, &conn)
        .into_iter()
        .filter(|x| x.created_at.timestamp() > after)
        .collect();
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

#[post("/password-edit", data = "<passwords>")]
fn password_edit(
    auth_data: AuthData,
    conn: PgDevandConn,
    passwords: Json<devand_core::PasswordEdit>,
) -> Option<()> {
    let ok = devand_db::auth::check_password(auth_data.user_id, &passwords.0.old_password, &conn)
        .ok()?;

    if !ok {
        return None;
    }

    devand_db::auth::set_password(auth_data.user_id, &passwords.0.new_password, &conn).ok()
}

#[cfg(test)]
mod tests {
    // use super::*;
}
