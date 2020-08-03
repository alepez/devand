#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod auth;
mod models;
mod schema;
mod schema_view;

use chrono::prelude::*;
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

#[derive(Debug)]
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

pub fn load_user_by_email(email: &str, conn: &PgConnection) -> Option<devand_core::User> {
    log::debug!("Load user by email {}", email);
    let user: models::User = schema::users::table
        .filter(schema::users::dsl::email.eq(email))
        .first(conn)
        .ok()?;

    user.try_into().map_err(|e| dbg!(e)).ok()
}

// TODO [optimization] create a psql view, call db once
pub fn load_full_user_by_id(
    id: devand_core::UserId,
    conn: &PgConnection,
) -> Option<devand_core::User> {
    let user: models::User = schema::users::table
        .filter(schema::users::dsl::id.eq(id.0))
        .first(conn)
        .ok()?;

    let unread_messages: i64 = schema::unread_messages::table
        .filter(schema::unread_messages::dsl::user_id.eq(user.id))
        .select(diesel::dsl::count(schema::unread_messages::dsl::message_id))
        .first(conn)
        .unwrap_or(0);

    let mut user: devand_core::User = user.try_into().map_err(|e| dbg!(e)).ok()?;

    user.unread_messages = unread_messages as usize;

    Some(user)
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

    let (current_email, current_email_verified): (String, bool) = schema::users::table
        .filter(schema::users::dsl::id.eq(user.id.0))
        .select((schema::users::email, schema::users::email_verified))
        .first(conn)
        .ok()?;

    let email_changed = &current_email != &email;
    let email_verified = current_email_verified && !email_changed;

    if email_verified != current_email_verified {
        log::warn!("Email changed, it now needs to be verified again");
    }

    // visible_name cannot be empty. Set it to be equal to username as fallback
    let visible_name = if visible_name.is_empty() {
        &user.username
    } else {
        &visible_name
    };

    diesel::update(schema::users::table.filter(schema::users::dsl::id.eq(user.id.0)))
        .set((
            schema::users::dsl::settings.eq(settings),
            schema::users::dsl::visible_name.eq(visible_name),
            schema::users::dsl::email.eq(email),
            schema::users::dsl::email_verified.eq(email_verified),
        ))
        .get_result(conn)
        .ok()
        .and_then(|x: models::User| x.try_into().ok())
}

pub fn is_username_available(username: &str, conn: &PgConnection) -> bool {
    // We have a username blocklist. Instead of check if the name is valid,
    // just pretend it is not available
    if !username_blocklist::validate(username) {
        return false;
    }

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
    schema::messages::table
        .filter(schema::messages::dsl::chat_id.eq(chat_id.0))
        .load(conn)
        .map(|v: Vec<models::ChatMessage>| v.into_iter().map(|x| x.into()).collect())
        .unwrap_or(Vec::default())
}

fn find_chat_id_by_members(
    members: &[devand_core::UserId],
    conn: &PgConnection,
) -> Option<devand_core::chat::ChatId> {
    let members: Vec<_> = members.iter().map(|x| x.0).collect();

    // TODO eq may not be good, what if members are in different order?
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
        let members: Vec<_> = members.iter().map(|x| x.0).collect();
        let new_chat = models::NewChat { members };

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

fn load_user_chat_by_id(
    user: devand_core::UserId,
    id: uuid::Uuid,
    conn: &PgConnection,
) -> Option<devand_core::UserChat> {
    // TODO [refactoring] Create PublicUserProfile model
    let members: Vec<devand_core::PublicUserProfile> = schema_view::chat_members::table
        .filter(schema_view::chat_members::chat_id.eq(id))
        .select((
            schema_view::chat_members::user_id,
            schema_view::chat_members::username,
            schema_view::chat_members::visible_name,
            schema_view::chat_members::languages,
        ))
        .load(conn)
        .unwrap_or(Vec::default())
        .into_iter()
        .filter(|(user_id, _, _, _)| *user_id != user.0)
        .filter_map(|(user_id, username, visible_name, languages)| {
            let languages: devand_core::Languages = serde_json::from_value(languages).ok()?;

            let profile = devand_core::PublicUserProfile {
                id: devand_core::UserId(user_id),
                username,
                visible_name,
                languages,
                bio: "".to_string(), // FIXME
            };

            Some(profile)
        })
        .collect();

    let members_ids = members.iter().map(|u| u.id).collect();

    let unread_messages: i64 = schema_view::unread_messages_full::table
        .filter(schema_view::unread_messages_full::chat_id.eq(id))
        .filter(schema_view::unread_messages_full::user_id.eq(user.0))
        .select(diesel::dsl::count(
            schema_view::unread_messages_full::dsl::message_id,
        ))
        .first(conn)
        .ok()?;

    let unread_messages = unread_messages as usize;

    let chat = devand_core::UserChat {
        chat: devand_core::chat::Chat {
            id: devand_core::chat::ChatId(id),
            members: members_ids,
        },
        unread_messages,
        members,
    };

    Some(chat)
}

pub fn load_chats_by_member(
    member: devand_core::UserId,
    conn: &PgConnection,
) -> devand_core::UserChats {
    let chats = schema::chats::table
        .filter(schema::chats::members.contains(vec![member.0]))
        .select(schema::chats::id)
        .load(conn)
        .ok()
        .map(|chats: Vec<uuid::Uuid>| {
            chats
                .into_iter()
                .filter_map(|id| load_user_chat_by_id(member, id, conn))
                .collect()
        })
        .unwrap_or(Vec::default());

    devand_core::UserChats(chats)
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

pub fn mark_messages_as_read_by(
    user_id: devand_core::UserId,
    messages: &[devand_core::chat::ChatMessage],
    conn: &PgConnection,
) {
    for message in messages {
        let res = diesel::delete(
            schema::unread_messages::table
                .filter(schema::unread_messages::user_id.eq(user_id.0))
                .filter(schema::unread_messages::message_id.eq(message.id)),
        )
        .execute(conn);

        if let Err(err) = res {
            log::warn!("Cannot mark as read message: {:?}", err);
        }
    }
}

fn mark_message_as_unread(message: &models::ChatMessage, conn: &PgConnection) {
    let models::ChatMessage {
        id,
        author,
        chat_id,
        ..
    } = message;

    let message_id = *id;

    schema::chats::table
        .filter(schema::chats::dsl::id.eq(chat_id))
        .first(conn)
        .ok()
        .and_then(|x: models::Chat| x.try_into().ok())
        .map(|chat: devand_core::chat::Chat| chat.members)
        .unwrap_or(Vec::default())
        .into_iter()
        .map(|x| x.0)
        .filter(|x| x != author)
        .for_each(|user_id| {
            let values = models::UnreadMessage {
                user_id,
                message_id,
            };

            let res = diesel::insert_into(schema::unread_messages::table)
                .values(values)
                .execute(conn);

            if let Err(err) = res {
                log::warn!("Error: {:?}", err);
            }
        });
}

pub fn add_chat_message_by_id(
    chat_id: devand_core::chat::ChatId,
    author: devand_core::UserId,
    txt: String,
    conn: &PgConnection,
) -> Result<devand_core::chat::ChatMessage, Error> {
    let new_message = models::NewChatMessage {
        chat_id: chat_id.0,
        created_at: Utc::now().naive_utc(),
        txt,
        author: author.0,
    };

    let message: models::ChatMessage = diesel::insert_into(schema::messages::table)
        .values(new_message)
        .get_result(conn)
        .map_err(|err| {
            dbg!(err);
            Error::Unknown
        })?;

    mark_message_as_unread(&message, conn);

    Ok(message.into())
}

pub fn add_chat_message_by_members(
    members: &[devand_core::UserId],
    author: devand_core::UserId,
    txt: String,
    conn: &PgConnection,
) -> Option<devand_core::chat::ChatMessage> {
    if let Ok(chat_id) = find_or_create_chat_by_members(members, conn) {
        add_chat_message_by_id(chat_id, author, txt, conn).ok()
    } else {
        None
    }
}

pub fn is_verified_email(email_addr: &str, conn: &PgConnection) -> bool {
    schema::users::table
        .filter(schema::users::dsl::email.eq(email_addr))
        .select(schema::users::dsl::email_verified)
        .first(conn)
        .unwrap_or(false)
}

pub fn set_verified_email(email_addr: &str, conn: &PgConnection) -> Result<(), Error> {
    diesel::update(schema::users::table.filter(schema::users::dsl::email.eq(email_addr)))
        .set((schema::users::dsl::email_verified.eq(true),))
        .execute(conn)
        .map(|_| ())
        .map_err(|_| Error::Unknown)
}

pub fn list_unverified_emails(conn: &PgConnection) -> Result<Vec<String>, Error> {
    schema::users::table
        .filter(schema::users::dsl::email_verified.eq(false))
        .select(schema::users::dsl::email)
        .get_results(conn)
        .map_err(|_| Error::Unknown)
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
