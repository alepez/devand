use crate::PgDevandConn;
use core::convert::TryFrom;
use devand_db as db;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use validator_derive::Validate;

const LOGIN_COOKIE_KEY: &'static str = "login";
const JOIN_COOKIE_KEY: &'static str = "join";

#[derive(FromForm)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub username: String,
    pub user_id: i32,
}

impl AuthData {
    pub fn matches_user(&self, user: &devand_core::User) -> bool {
        dbg!(&user.username);
        dbg!(&self.username);
        dbg!(&user.id);
        dbg!(&self.user_id);
        user.username == self.username && user.id == self.user_id
    }
}

impl TryFrom<rocket::http::Cookie<'_>> for AuthData {
    type Error = ();
    fn try_from(cookie: rocket::http::Cookie<'_>) -> Result<Self, Self::Error> {
        let json = cookie.value();
        serde_json::from_str(json).or(Err(()))
    }
}

impl<'a> Into<rocket::http::Cookie<'a>> for AuthData {
    fn into(self) -> rocket::http::Cookie<'a> {
        let json = serde_json::to_string(&self).unwrap();
        Cookie::new(LOGIN_COOKIE_KEY, json)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthData {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<AuthData, !> {
        request
            .cookies()
            .get_private(LOGIN_COOKIE_KEY)
            .and_then(|cookie| AuthData::try_from(cookie).ok())
            .or_forward(())
    }
}

pub(crate) fn login(
    cookies: &mut Cookies,
    credentials: Credentials,
    conn: &PgDevandConn,
) -> Result<(), ()> {
    let credentials = db::auth::Credentials {
        username: credentials.username,
        password: credentials.password,
    };

    db::auth::login(credentials, &conn.0)
        .map(|(user_id, username)| {
            let auth_data = AuthData { user_id, username };
            cookies.add_private(auth_data.into());
        })
        .map_err(|_| ())
}

pub fn logout(cookies: &mut Cookies) {
    cookies.remove_private(Cookie::named(LOGIN_COOKIE_KEY));
}

#[derive(FromForm, Debug, Validate, Serialize, Deserialize, Clone)]
pub struct JoinData {
    #[validate(custom = "validate_username")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validate_password")]
    pub password: String,
}

impl TryFrom<rocket::http::Cookie<'_>> for JoinData {
    type Error = ();
    fn try_from(cookie: rocket::http::Cookie<'_>) -> Result<Self, Self::Error> {
        let json = cookie.value();
        serde_json::from_str(json).or(Err(()))
    }
}

impl<'a> Into<rocket::http::Cookie<'a>> for JoinData {
    fn into(self) -> rocket::http::Cookie<'a> {
        let json = serde_json::to_string(&self).unwrap();
        Cookie::new(JOIN_COOKIE_KEY, json)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for JoinData {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<JoinData, !> {
        request
            .cookies()
            .get_private(JOIN_COOKIE_KEY)
            .and_then(|cookie| JoinData::try_from(cookie).ok())
            .or_forward(())
    }
}

fn validate_password(s: &str) -> Result<(), validator::ValidationError> {
    if devand_core::auth::is_valid_password(s) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_password");
        err.message = Some("Make sure it's at least 15 characters OR at least 8 characters including a number and a lowercase letter".into());
        Err(err)
    }
}

fn validate_username(s: &str) -> Result<(), validator::ValidationError> {
    if devand_core::auth::is_valid_username(s) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_password");
        err.message = Some("Must be at least 3 characters, only lowercase and digits".into());
        Err(err)
    }
}

fn is_unique_email(s: &str, conn: &PgDevandConn) -> Result<(), validator::ValidationError> {
    if devand_db::is_email_available(s, conn) {
        Ok(())
    } else {
        let mut err = ValidationError::new("email_unavailable");
        err.message = Some("Email is already used".into());
        Err(err)
    }
}

fn is_unique_username(s: &str, conn: &PgDevandConn) -> Result<(), validator::ValidationError> {
    if devand_db::is_username_available(s, conn) {
        Ok(())
    } else {
        let mut err = ValidationError::new("username_unavailable");
        err.message = Some("Username is already used".into());
        Err(err)
    }
}

fn check_for_uniqueness(
    join_data: &JoinData,
    conn: &PgDevandConn,
) -> Result<(), validator::ValidationErrors> {
    let mut errors = validator::ValidationErrors::new();

    if let Err(e) = is_unique_username(&join_data.username, conn) {
        errors.add("username", e);
    }

    if let Err(e) = is_unique_email(&join_data.email, conn) {
        errors.add("email", e);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(crate) fn join(
    cookies: &mut Cookies,
    join_data: JoinData,
    conn: &PgDevandConn,
) -> Result<(), JoinError> {
    let valid = join_data.validate();

    // Remove old cookie
    cookies.remove_private(Cookie::named(JOIN_COOKIE_KEY));

    let valid = valid.and_then(|_| check_for_uniqueness(&join_data, conn));

    // Add new join data if there is an error
    if valid.is_err() {
        // Update with new data
        cookies.add_private(join_data.clone().into());
    }

    // Early exit if not valid
    valid?;

    let join_data = db::auth::JoinData {
        username: join_data.username,
        password: join_data.password.into(),
        email: join_data.email,
    };

    db::auth::join(join_data, &conn.0)
        .map(|_| ())
        .map_err(|_| JoinError::UnknownError)
}

#[derive(Debug)]
pub enum JoinError {
    UnknownError,
    ValidationError(String),
}

impl From<validator::ValidationErrors> for JoinError {
    fn from(e: validator::ValidationErrors) -> Self {
        let msg = e
            .field_errors()
            .into_iter()
            .map(|(field, errors)| {
                let msg = errors
                    .iter()
                    .filter_map(|x| x.message.clone())
                    .collect::<Vec<_>>()
                    .concat();
                if msg.is_empty() {
                    format!("Invalid {}. ", field)
                } else {
                    format!("Invalid {}: {}. ", field, msg)
                }
            })
            .collect::<Vec<_>>()
            .concat();
        JoinError::ValidationError(msg)
    }
}

impl ToString for JoinError {
    fn to_string(&self) -> String {
        match self {
            JoinError::UnknownError => "UnknownError".into(),
            JoinError::ValidationError(msg) => msg.into(),
        }
    }
}
