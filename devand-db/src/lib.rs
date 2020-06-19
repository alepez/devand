#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod auth;
mod models;
mod schema;
mod schema_view;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::convert::TryInto;
use std::env;
#[macro_use]
extern crate diesel_migrations;

embed_migrations!();

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub enum Error {
    Unknown,
}

// TODO This is very expensive. Should return an iterator and should be cached
// somewhere. Or we could use a custom database when this is needed, like
// when searching for user affinity
pub fn load_users(conn: &PgConnection) -> Option<Vec<devand_core::User>> {
    schema::users::table
        .load(conn)
        .map(|results: Vec<models::User>| {
            results
                .into_iter()
                .filter_map(|u: models::User| u.try_into().ok())
                .collect::<Vec<devand_core::User>>()
        })
        .ok()
}

pub fn load_user_by_username(username: &str, conn: &PgConnection) -> Option<devand_core::User> {
    let user: models::User = schema::users::table
        .filter(schema::users::dsl::username.eq(username))
        .first(conn)
        .ok()?;

    user.try_into().map_err(|e| dbg!(e)).ok()
}

pub fn load_user_by_id(id: devand_core::UserId, conn: &PgConnection) -> Option<devand_core::User> {
    let user: models::User = schema::users::table
        .filter(schema::users::dsl::id.eq(id.0))
        .first(conn)
        .ok()?;

    user.try_into().map_err(|e| dbg!(e)).ok()
}

pub fn save_user(user: devand_core::User, conn: &PgConnection) -> Option<devand_core::User> {
    let devand_core::User {
        settings,
        visible_name,
        email,
        ..
    } = user;

    let settings = serde_json::to_value(settings).unwrap();

    diesel::update(schema::users::table.filter(schema::users::dsl::id.eq(user.id.0)))
        .set((
            schema::users::dsl::settings.eq(settings),
            schema::users::dsl::visible_name.eq(visible_name),
            schema::users::dsl::email.eq(email),
        ))
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

pub fn load_chat_history_by_id(
    chat_id: devand_core::chat::ChatId,
    conn: &PgConnection,
) -> Vec<devand_core::chat::ChatMessage> {
    let result: Option<Vec<devand_core::chat::ChatMessage>> = schema::messages::table
        .filter(schema::messages::dsl::chat_id.eq(chat_id.0))
        .load(conn)
        .ok()
        .map(|v: Vec<models::ChatMessage>| v.into_iter().map(|x| x.into()).collect());

    result.unwrap_or(Vec::default())
}

fn find_chat_id_by_members(
    members: &[devand_core::UserId],
    conn: &PgConnection,
) -> Option<devand_core::chat::ChatId> {
    let members = serde_json::to_value(members).unwrap();

    schema::chats::table
        .filter(schema::chats::dsl::members.eq(members))
        .select(schema::chats::id)
        .first(conn)
        .ok()
        .map(devand_core::chat::ChatId)
}

fn find_or_create_chat_by_members(
    members: &[devand_core::UserId],
    conn: &PgConnection,
) -> Result<devand_core::chat::ChatId, Error> {
    if let Some(chat_id) = find_chat_id_by_members(members, conn) {
        Ok(chat_id)
    } else {
        let new_chat = models::NewChat {
            members: serde_json::to_value(members).unwrap(),
        };
        diesel::insert_into(schema::chats::table)
            .values(new_chat)
            .get_result(conn)
            .map_err(|err| {
                dbg!(err);
                Error::Unknown
            })
            .map(|x: models::Chat| devand_core::chat::ChatId(x.id))
    }
}

pub fn load_chat_history_by_members(
    members: &[devand_core::UserId],
    conn: &PgConnection,
) -> Vec<devand_core::chat::ChatMessage> {
    if let Ok(chat_id) = find_or_create_chat_by_members(members, conn) {
        load_chat_history_by_id(chat_id, conn)
    } else {
        Vec::default()
    }
}

pub fn add_chat_message_by_id(
    chat_id: devand_core::chat::ChatId,
    new_message: devand_core::chat::ChatMessage,
    conn: &PgConnection,
) -> Result<devand_core::chat::ChatMessage, Error> {
    let new_message = models::NewChatMessage {
        chat_id: chat_id.0,
        created_at: new_message.created_at.naive_utc(),
        txt: new_message.txt,
        author: new_message.author.0,
    };

    diesel::insert_into(schema::messages::table)
        .values(new_message)
        .get_result(conn)
        .map_err(|err| {
            dbg!(err);
            Error::Unknown
        })
        .map(|x: models::ChatMessage| x.into())
}

pub fn add_chat_message_by_members(
    members: &[devand_core::UserId],
    new_message: devand_core::chat::ChatMessage,
    conn: &PgConnection,
) -> Option<devand_core::chat::ChatMessage> {
    if let Ok(chat_id) = find_or_create_chat_by_members(members, conn) {
        add_chat_message_by_id(chat_id, new_message, conn).ok()
    } else {
        None
    }
}

pub fn run_migrations(conn: &PgConnection) -> Result<(), diesel_migrations::RunMigrationsError> {
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
