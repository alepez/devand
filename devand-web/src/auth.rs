mod captcha;

use self::captcha::CaptchaFile;
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
const JOIN_CAPTCHA_COOKIE_KEY: &'static str = "join_captcha";

#[derive(FromForm)]
pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    fn normalize(self) -> Self {
        Self {
            username: trimlow(self.username),
            password: self.password,
        }
    }
}

impl Into<db::auth::Credentials> for Credentials {
    fn into(self) -> db::auth::Credentials {
        db::auth::Credentials {
            username: self.username,
            password: self.password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub user_id: i32,
}

impl AuthData {
    pub fn matches_user(&self, user: &devand_core::User) -> bool {
        user.id == self.user_id
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
        Cookie::build(LOGIN_COOKIE_KEY, json)
            .http_only(true)
            .max_age(time::Duration::days(30))
            .finish()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthData {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<AuthData, ()> {
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
    let credentials = credentials.normalize().into();

    db::auth::login(credentials, &conn.0)
        .map(|user_id| {
            let auth_data = AuthData { user_id };
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
    pub captcha: String,
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
        Cookie::build(JOIN_COOKIE_KEY, json)
            .http_only(true)
            .max_age(time::Duration::minutes(10))
            .path("/join")
            .finish()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for JoinData {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<JoinData, ()> {
        request
            .cookies()
            .get_private(JOIN_COOKIE_KEY)
            .and_then(|cookie| JoinData::try_from(cookie).ok())
            .or_forward(())
    }
}

impl JoinData {
    fn normalize(self) -> Self {
        Self {
            username: trimlow(self.username),
            email: trimlow(self.email),
            password: self.password,
            captcha: trimlow(self.captcha),
        }
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

fn validate_captcha(input: String, expected: String) -> Result<(), validator::ValidationError> {
    let input = trimlow(input);
    let expected = trimlow(expected);

    if input == expected {
        Ok(())
    } else {
        let mut err = ValidationError::new("captcha");
        err.message = Some("Wrong captcha".into());
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
    expected_captcha: ExpectedCaptcha,
    conn: &PgDevandConn,
) -> Result<(), JoinError> {
    let mut join_data = join_data.normalize();

    let valid = join_data.validate();

    let captcha_input = join_data.captcha;
    join_data.captcha = String::new();

    // Remove old cookie
    cookies.remove_private(Cookie::named(JOIN_COOKIE_KEY));
    cookies.remove_private(Cookie::named(JOIN_CAPTCHA_COOKIE_KEY));

    let valid = valid
        .and_then(|_| check_for_uniqueness(&join_data, conn))
        .and_then(|_| {
            let mut errors = validator::ValidationErrors::new();

            if let Err(e) = validate_captcha(captcha_input, expected_captcha.value) {
                errors.add("captcha", e);
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors)
            }
        });

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

    let result = db::auth::join(join_data, &conn.0);

    if result.is_ok() {
        cookies.remove_private(Cookie::named(JOIN_COOKIE_KEY));
        cookies.remove_private(Cookie::named(JOIN_CAPTCHA_COOKIE_KEY));
    }

    result.map(|_| ()).map_err(|_| JoinError::UnknownError)
}

pub(crate) fn captcha(cookies: &mut Cookies) -> Result<CaptchaFile, ()> {
    let captcha = CaptchaFile::new();

    let captcha_value = ExpectedCaptcha {
        value: captcha.value(),
    };

    cookies.add_private(captcha_value.into());

    Ok(captcha)
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

fn trimlow(s: String) -> String {
    // We help the user, trimming spaces and converting to lowercase
    let s = s.to_lowercase();
    // Note: this allocates a new string, in place trimming does not exist
    s.trim().to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedCaptcha {
    pub value: String,
}

impl TryFrom<rocket::http::Cookie<'_>> for ExpectedCaptcha {
    type Error = ();
    fn try_from(cookie: rocket::http::Cookie<'_>) -> Result<Self, Self::Error> {
        let json = cookie.value();
        serde_json::from_str(json).or(Err(()))
    }
}

impl<'a> Into<rocket::http::Cookie<'a>> for ExpectedCaptcha {
    fn into(self) -> rocket::http::Cookie<'a> {
        let json = serde_json::to_string(&self).unwrap();
        Cookie::build(JOIN_CAPTCHA_COOKIE_KEY, json)
            .http_only(true)
            .max_age(time::Duration::minutes(10))
            .path("/join")
            .finish()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ExpectedCaptcha {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<ExpectedCaptcha, ()> {
        request
            .cookies()
            .get_private(JOIN_CAPTCHA_COOKIE_KEY)
            .and_then(|cookie| ExpectedCaptcha::try_from(cookie).ok())
            .or_forward(())
    }
}

pub struct RealIp(pub std::net::IpAddr);

impl<'a, 'r> FromRequest<'a, 'r> for RealIp {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<RealIp, ()> {
        let real_ip_from_header = request.real_ip();
        let remote_ip_from_sock = request.remote().map(|x| x.ip());

        real_ip_from_header
            .or(remote_ip_from_sock)
            .map(|x| RealIp(x))
            .or_forward(())
    }
}

pub struct LoggedUser(devand_core::User);

impl<'a, 'r> FromRequest<'a, 'r> for LoggedUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<LoggedUser, ()> {
        let conn = request.guard::<PgDevandConn>()?;
        let auth_data = request.guard::<AuthData>()?;
        let user = devand_db::load_user_by_id(auth_data.user_id, &conn.0);
        user.map(|x| LoggedUser(x)).or_forward(())
    }
}

impl Into<devand_core::User> for LoggedUser {
    fn into(self) -> devand_core::User {
        self.0
    }
}

impl std::ops::Deref for LoggedUser {
    type Target = devand_core::User;

    #[inline(always)]
    fn deref(&self) -> &devand_core::User {
        &self.0
    }
}
