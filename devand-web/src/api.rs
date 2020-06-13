use crate::auth::{AuthData, LoggedUser};
use crate::PgDevandConn;
use devand_core::{User, UserAffinity};
use rocket::Route;
use rocket_contrib::json::Json;

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
    let affinities = devand_core::calculate_affinities(user.into(), users);
    Some(Json(affinities.collect()))
}

#[get("/code-now")]
fn code_now(user: LoggedUser, conn: PgDevandConn) -> Option<Json<Vec<UserAffinity>>> {
    let users = devand_db::load_users(&conn.0)?;
    // TODO Return all online users
    let affinities = devand_core::calculate_affinities(user.into(), users);
    Some(Json(affinities.collect()))
}

pub fn routes() -> Vec<Route> {
    routes![settings, settings_put, affinities, code_now]
}
