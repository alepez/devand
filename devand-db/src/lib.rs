#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod auth;
mod models;
mod schema;
mod schema_view;

#[cfg(feature = "mock")]
pub mod fake_data;

use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::convert::TryInto;
use std::env;

#[macro_use]
extern crate diesel_migrations;

embed_migrations!();

fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn establish_connection() -> PgConnection {
    let url = database_url();
    PgConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to {}", &url))
}

#[derive(Debug)]
pub enum Error {
    Unknown,
    Generic(String),
}

// TODO This is very expensive. Should return an iterator and should be cached
// somewhere. Or we could use a custom database when this is needed, like
// when searching for user affinity
/// Load all users
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

/// Load single user by username (unique)
pub fn load_user_by_username(username: &str, conn: &PgConnection) -> Option<devand_core::User> {
    let user: models::User = schema::users::table
        .filter(schema::users::dsl::username.eq(username))
        .first(conn)
        .ok()?;

    user.try_into().map_err(|e| dbg!(e)).ok()
}

/// Load single user by email (unique)
pub fn load_user_by_email(email: &str, conn: &PgConnection) -> Option<devand_core::User> {
    log::debug!("Load user by email {}", email);
    let user: models::User = schema::users::table
        .filter(schema::users::dsl::email.eq(email))
        .first(conn)
        .ok()?;

    user.try_into().map_err(|e| dbg!(e)).ok()
}

// TODO [optimization] create a psql view, call db once
// TODO unread_messages should be outside User struct
// TODO This functions smells of shortcut
/// Load single user by id (unique) with more informations
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

/// Load single user by id (unique)
pub fn load_user_by_id(id: devand_core::UserId, conn: &PgConnection) -> Option<devand_core::User> {
    let user: models::User = schema::users::table
        .filter(schema::users::dsl::id.eq(id.0))
        .first(conn)
        .ok()?;

    user.try_into().map_err(|e| dbg!(e)).ok()
}

/// Save the given user
/// If the email has changed, it set as not verified
/// If the visible_name is set to empty, it is set to username instead
pub fn save_user(user: devand_core::User, conn: &PgConnection) -> Option<devand_core::User> {
    let devand_core::User {
        settings,
        visible_name,
        email,
        bio,
        ..
    } = user;

    let settings = serde_json::to_value(settings).unwrap();

    let (current_email, current_email_verified) = has_verified_email(user.id, &conn).ok()?;

    let email_changed = current_email != email;
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
            schema::users::dsl::bio.eq(bio),
        ))
        .get_result(conn)
        .ok()
        .and_then(|x: models::User| x.try_into().ok())
}

// TODO Instead of panicing, return a Result
/// Return true if username is available
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

// TODO Instead of panicing, return a Result
/// Return true if email is available
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
        .unwrap_or_default()
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

// TODO This function is huge, needs refactoring
fn load_user_chat_by_id(
    user: devand_core::UserId,
    id: uuid::Uuid,
    conn: &PgConnection,
) -> Result<devand_core::UserChat, Error> {
    let members: Vec<devand_core::PublicUserProfile> = schema_view::chat_members::table
        .filter(schema_view::chat_members::chat_id.eq(id))
        .load(conn)
        .map_err(|e| Error::Generic(format!("Error loading members from database: {:?}", e)))?
        .into_iter()
        .filter_map(|chat_member: models::ChatMember| {
            let models::ChatMember {
                user_id,
                username,
                visible_name,
                bio,
                languages,
                spoken_languages,
                ..
            } = chat_member;

            // Ignore same user
            if user_id == user.0 {
                return None;
            }

            let languages = serde_json::from_value(languages).ok()?;

            let spoken_languages = spoken_languages
                .and_then(|x| serde_json::from_value(x).ok())
                .unwrap_or_default();

            let profile = devand_core::PublicUserProfile {
                id: devand_core::UserId(user_id),
                username,
                visible_name,
                languages,
                bio,
                spoken_languages,
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
        .map_err(|_| Error::Generic("Error loading unread messages from database".to_string()))?;

    let unread_messages = unread_messages as usize;

    let chat = devand_core::UserChat {
        chat: devand_core::chat::Chat {
            id: devand_core::chat::ChatId(id),
            members: members_ids,
        },
        unread_messages,
        members,
    };

    Ok(chat)
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
                .filter_map(|id| {
                    load_user_chat_by_id(member, id, conn)
                        .map_err(|e| log::error!("Error: {:?}", e))
                        .ok()
                })
                .collect()
        })
        .unwrap_or_default();

    devand_core::UserChats(chats)
}

pub fn load_chat_history_by_members(
    members: &[devand_core::UserId],
    conn: &PgConnection,
) -> Vec<devand_core::chat::ChatMessage> {
    find_chat_id_by_members(members, conn)
        .map(|chat_id| load_chat_history_by_id(chat_id, conn))
        .unwrap_or_default()
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
        .unwrap_or_default()
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

/// Return true if the given user has a verified email
pub fn has_verified_email(
    user_id: devand_core::UserId,
    conn: &PgConnection,
) -> Result<(String, bool), Error> {
    schema::users::table
        .filter(schema::users::dsl::id.eq(user_id.0))
        .select((schema::users::email, schema::users::email_verified))
        .first(conn)
        .map_err(|_| Error::Unknown)
}

/// Return true if the email is verified
pub fn is_verified_email(email_addr: &str, conn: &PgConnection) -> bool {
    schema::users::table
        .filter(schema::users::dsl::email.eq(email_addr))
        .select(schema::users::dsl::email_verified)
        .first(conn)
        .unwrap_or(false)
}

/// Mark the email address as verified
pub fn set_verified_email(email_addr: &str, conn: &PgConnection) -> Result<(), Error> {
    diesel::update(schema::users::table.filter(schema::users::dsl::email.eq(email_addr)))
        .set((schema::users::dsl::email_verified.eq(true),))
        .execute(conn)
        .map(|_| ())
        .map_err(|_| Error::Unknown)
}

/// List all unverified email addresses
pub fn list_unverified_emails(conn: &PgConnection) -> Result<Vec<String>, Error> {
    schema::users::table
        .filter(schema::users::dsl::email_verified.eq(false))
        .select(schema::users::dsl::email)
        .get_results(conn)
        .map_err(|_| Error::Unknown)
}

/// Run database migrations
pub fn run_migrations(conn: &PgConnection) -> Result<(), diesel_migrations::RunMigrationsError> {
    embedded_migrations::run(&*conn)
}

/// Clear table content and any auto-increment counter
fn clear_table(table: &str, conn: &PgConnection) -> Result<(), diesel::result::Error> {
    let q = format!("TRUNCATE TABLE {} RESTART IDENTITY;", table);
    diesel::sql_query(q).execute(conn).map(|_| ())
}

/// Clear all tables and their auto-increment counters
fn clear_all(conn: &PgConnection) -> Result<(), diesel::result::Error> {
    let tables = vec!["auth", "chats", "messages", "unread_messages", "users"];

    for table in tables {
        clear_table(table, conn)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn fresh_db() -> PgConnection {
        let conn = establish_connection();
        run_migrations(&conn).unwrap();
        clear_all(&conn).unwrap();
        conn
    }

    fn fake_join_data() -> auth::JoinData {
        auth::JoinData {
            username: "foo".to_string(),
            email: "foo@example.com".to_string(),
            password: "ZXokdUB6dWplaW5nYXU3am".to_string(),
        }
    }

    fn fresh_db_with_fake_user() -> (PgConnection, devand_core::User) {
        let conn = fresh_db();
        let join_data = fake_join_data();
        let username = join_data.username.clone();
        auth::join(join_data, &conn).unwrap();
        let user = load_user_by_username(&username, &conn).unwrap();
        (conn, user)
    }

    #[test]
    #[ignore]
    #[serial]
    fn join_valid_user() {
        let conn = fresh_db();

        let join_data = fake_join_data();

        let credentials = auth::Credentials {
            username: join_data.username.clone(),
            password: join_data.password.clone(),
        };

        assert!(auth::join(join_data, &conn).is_ok());
        assert!(auth::login(credentials, &conn).is_ok())
    }

    #[test]
    #[ignore]
    #[serial]
    fn username_available() {
        let conn = fresh_db();

        let join_data = fake_join_data();
        let username = join_data.username.clone();

        assert!(is_username_available(&username, &conn));
        assert!(auth::join(join_data, &conn).is_ok());
        assert!(!is_username_available(&username, &conn));
    }

    #[test]
    #[ignore]
    #[serial]
    fn blocklist_username_unavailable() {
        let conn = fresh_db();
        assert!(!is_username_available("root", &conn));
    }

    #[test]
    #[ignore]
    #[serial]
    fn email_available() {
        let conn = fresh_db();

        let join_data = fake_join_data();
        let email = join_data.email.clone();

        assert!(is_email_available(&email, &conn));
        assert!(auth::join(join_data, &conn).is_ok());
        assert!(!is_email_available(&email, &conn));
    }

    #[test]
    #[ignore]
    #[serial]
    fn load_no_users_ok() {
        let conn = fresh_db();
        let users = load_users(&conn).unwrap();
        assert!(users.is_empty());
    }

    #[test]
    #[ignore]
    #[serial]
    fn load_single_users_ok() {
        let conn = fresh_db();
        let join_data = fake_join_data();
        auth::join(join_data, &conn).unwrap();
        let users = load_users(&conn).unwrap();
        assert_eq!(users.len(), 1);
    }

    #[test]
    #[ignore]
    #[serial]
    fn load_user_ok() {
        let (conn, user) = fresh_db_with_fake_user();

        let user = load_user_by_username(&user.username, &conn).unwrap();
        let user = load_user_by_id(user.id, &conn).unwrap();
        let user = load_user_by_email(&user.email, &conn).unwrap();
        let user = load_full_user_by_id(user.id, &conn).unwrap();
        assert_eq!(user.email_verified, false);
        assert_eq!(user.unread_messages, 0);
    }

    #[test]
    #[ignore]
    #[serial]
    fn save_user_ok() {
        let (conn, mut user) = fresh_db_with_fake_user();
        let user_id = user.id;
        let email = user.email.clone();

        assert!(!user.email_verified);

        user.visible_name = "Foo Bar".to_string();
        save_user(user, &conn).unwrap();

        set_verified_email(&email, &conn).unwrap();
        let user = load_user_by_id(user_id, &conn).unwrap();
        assert!(user.email_verified);
    }

    #[test]
    #[ignore]
    #[serial]
    fn save_user_change_email_unverified() {
        let (conn, user) = fresh_db_with_fake_user();
        let user_id = user.id;
        let email = user.email.clone();

        set_verified_email(&email, &conn).unwrap();

        let mut user = load_user_by_id(user_id, &conn).unwrap();
        assert!(user.email_verified);

        user.email = format!("changed_{}", user.email);
        let user = save_user(user, &conn).unwrap();

        assert!(!user.email_verified);
    }

    #[test]
    #[ignore]
    #[serial]
    fn visible_name_is_equal_to_username_when_set_to_empty() {
        let (conn, mut user) = fresh_db_with_fake_user();
        let other_visible_name = "Gne Gne";

        user.visible_name = other_visible_name.to_string();
        let mut user = save_user(user, &conn).unwrap();
        assert_eq!(user.visible_name, other_visible_name);

        user.visible_name = String::default();
        let user = save_user(user, &conn).unwrap();

        assert_eq!(user.visible_name, user.username);
    }
}
