use crate::auth::{AuthData, LoggedUser};
use crate::PgDevandConn;
use devand_core::{Affinity, AffinityParams, User, UserAffinity};
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
    let logged_user_params = AffinityParams::new().with_languages(user.settings.languages.clone());

    dbg!(&logged_user_params);

    let users = devand_db::load_users(&conn.0)?;

    let affinities = users.into_iter().filter(|u| u.id != user.id).map(|u| {
        let languages = u.settings.languages.clone();
        let u_params = AffinityParams::new().with_languages(languages);
        // TODO Avoid cloning logged user params
        let affinity = Affinity::from_params(logged_user_params.clone(), u_params);
        UserAffinity::new(u, affinity)
    });

    Some(Json(affinities.collect()))
}

pub fn routes() -> Vec<Route> {
    routes![settings, settings_put, affinities]
}
