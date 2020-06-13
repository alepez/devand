use crate::auth::{AuthData, LoggedUser};
use crate::CodeNowUsers;
use crate::PgDevandConn;
use devand_core::{User, UserAffinity};
use rocket::{Route, State};
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
    let affinities = devand_core::calculate_affinities(&user.into(), users);
    Some(Json(affinities.collect()))
}

// TODO Remove user from cache after inactivity
#[get("/code-now")]
fn code_now(user: LoggedUser, code_now_users: State<CodeNowUsers>) -> Json<devand_core::CodeNow> {
    let user: User = user.into();

    // First try a read lock, can be locked by  multiple readers
    let cache = code_now_users.0.read().unwrap();

    // Check if it already contains the current user
    let all_users: devand_core::CodeNowUsers = if cache.contains(&user) {
        cache.clone().into()
    } else {
        // If it does not contain it, lock with write access
        drop(cache);
        let mut cache = code_now_users.0.write().unwrap();
        // Now we can add the current user to the cache
        cache.add(user.clone());
        cache.clone().into()
    };

    Json(devand_core::CodeNow {
        current_user: user,
        all_users: all_users.0,
    })
}

pub fn routes() -> Vec<Route> {
    routes![settings, settings_put, affinities, code_now]
}
