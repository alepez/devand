use crate::auth::{AuthData, LoggedUser};
use crate::{CodeNowUsers, PgDevandConn, WeekScheduleMatrix};
use chrono::prelude::*;
use chrono::Duration;
use devand_core::schedule_matcher::find_all_users_matching_in_week;
use devand_core::{User, UserAffinity, UserId};
use rocket::{Route, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

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
    ]
}

#[get("/settings")]
fn settings(user: LoggedUser) -> Json<User> {
    Json(user.into())
}

#[put("/settings", data = "<user>")]
fn settings_put(auth_data: AuthData, user: Json<User>, conn: PgDevandConn) -> Option<Json<User>> {
    // Note: here we don't need LoggedUser (needs db access) but only
    // auth_data to check if we are modifiyng the right user.
    if !auth_data.matches_user(&user) {
        return None;
    }

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
fn availability_match(
    user: LoggedUser,
    week_sched_matrix: State<WeekScheduleMatrix>,
) -> Json<AvailabilityMatch> {
    let now = Utc::now();
    let next_week = now.checked_add_signed(Duration::days(7)).unwrap();
    let User { settings, .. } = user.into();
    let availability = settings.schedule;
    let week_sched_mat = week_sched_matrix.0.read().unwrap();
    let slots = find_all_users_matching_in_week(next_week, availability, week_sched_mat.get());
    let res = AvailabilityMatch { slots };
    Json(res)
}

#[get("/chat/<members>/messages")]
fn chat_messages_get(user: LoggedUser, members: String) -> Json<Vec<devand_core::chat::ChatMessage>> {
    // TODO Load from db
    Json(vec![])
}

#[post("/chat/<members>/messages", data = "<txt>")]
fn chat_messages_post(user: LoggedUser, members: String, txt: Json<String>) -> Json<Vec<devand_core::chat::ChatMessage>> {
    let new_message = devand_core::chat::ChatMessage {
        author: user.id,
        txt: txt.0,
        created_at: Utc::now(),
    };
    // TODO Save to db
    Json(vec![new_message])
}

#[get("/chat/<members>/messages/poll/<from>")]
fn chat_messages_poll(user: LoggedUser, members: String, from: String) -> Json<Vec<devand_core::chat::ChatMessage>> {
    // Note: Rocket 0.16.2 does not support websocket, so we just poll for new messages
    // TODO Load from db from given point
    Json(vec![])
}

#[derive(Serialize, Deserialize)]
struct AvailabilityMatch {
    slots: Vec<(DateTime<Utc>, Vec<UserId>)>,
}

#[cfg(test)]
mod tests {
    // use super::*;
}
