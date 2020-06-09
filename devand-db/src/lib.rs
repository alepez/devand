#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod auth;
mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::convert::TryInto;
use std::env;
#[macro_use] extern crate diesel_migrations;

embed_migrations!();


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub enum Error {
    Unknown,
}

pub fn load_user_by_id(id: i32, conn: &PgConnection) -> Option<devand_core::User> {
    let user: models::User = schema::users::table
        .filter(schema::users::dsl::id.eq(id))
        .first(conn)
        .ok()?;

    user.try_into().ok()
}

pub fn save_user(user: devand_core::User, conn: &PgConnection) -> Option<devand_core::User> {
    let settings = serde_json::to_value(user.settings).unwrap();
    diesel::update(schema::users::table.filter(schema::users::dsl::id.eq(user.id)))
        .set(schema::users::dsl::settings.eq(settings))
        .get_result(conn)
        .ok()
        .and_then(|x: models::User| x.try_into().ok())
}

pub fn is_username_available(username: &str, conn: &PgConnection) -> bool {
    let count: i64 = schema::users::table
        .filter(schema::users::dsl::username.eq(username))
        .select(diesel::dsl::count(schema::users::dsl::id))
        .first(conn)
        .expect("Checking for username availability");

    count == 0
}

pub fn is_email_available(email: &str, conn: &PgConnection) -> bool {
    let count: i64 = schema::users::table
        .filter(schema::users::dsl::email.eq(email))
        .select(diesel::dsl::count(schema::users::dsl::id))
        .first(conn)
        .expect("Checking for email availability");

    count == 0
}

pub fn run_migrations(conn: &PgConnection) -> Result<(), diesel_migrations::RunMigrationsError>{
    embedded_migrations::run(&*conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_db() -> PgConnection {
        let conn = establish_connection();
        diesel::delete(schema::users::table).execute(&conn).unwrap();
        diesel::delete(schema::auth::table).execute(&conn).unwrap();
        conn
    }

    #[test]
    #[ignore]
    fn join_valid_user() {
        let conn = fresh_db();

        let join_data = auth::JoinData {
            username: "foo".to_string(),
            email: "example@example.com".to_string(),
            password: "ZXokdUB6dWplaW5nYXU3am".to_string(),
        };

        let credentials = auth::Credentials {
            username: join_data.username.clone(),
            password: join_data.password.clone(),
        };

        assert!(auth::join(join_data, &conn).is_ok());

        assert!(auth::login(credentials, &conn).is_ok())
    }

    #[test]
    #[ignore]
    fn username_available() {
        let conn = fresh_db();

        let username = "bar";

        let join_data = auth::JoinData {
            username: username.to_string(),
            email: "example@example.com".to_string(),
            password: "ZXokdUB6dWplaW5nYXU3am".to_string(),
        };

        assert!(is_username_available(username, &conn));
        assert!(auth::join(join_data, &conn).is_ok());
        assert!(!is_username_available(username, &conn));
    }

    #[test]
    #[ignore]
    fn email_available() {
        let conn = fresh_db();

        let email = "example@example.com";

        let join_data = auth::JoinData {
            username: "foo".to_string(),
            email: email.to_string(),
            password: "ZXokdUB6dWplaW5nYXU3am".to_string(),
        };

        assert!(is_email_available(email, &conn));
        assert!(auth::join(join_data, &conn).is_ok());
        assert!(!is_email_available(email, &conn));
    }
}
