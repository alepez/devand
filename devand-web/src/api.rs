use crate::auth::AuthData;
use crate::PgDevandConn;
use devand_core::User;
use rocket::Route;
use rocket_contrib::json::Json;

#[get("/settings")]
fn settings(auth_data: AuthData, conn: PgDevandConn) -> Option<Json<User>> {
    devand_db::load_user_by_id(auth_data.user_id, &conn.0).map(|x| Json(x))
}

#[put("/settings", data = "<user>")]
fn settings_put(auth_data: AuthData, user: Json<User>, conn: PgDevandConn) -> Option<Json<User>> {
    if !auth_data.matches_user(&user) {
        return None;
    }

    devand_db::save_user(user.0, &conn.0).map(|x| Json(x))
}

pub fn routes() -> Vec<Route> {
    routes![settings, settings_put]
}
