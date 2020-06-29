use crate::{models, schema, schema_view, Error};
use argon2::{self, Config};
use devand_core::UserId;
use diesel::pg::PgConnection;
use diesel::prelude::*;

fn generate_salt() -> [u8; 16] {
    use rand::Rng;
    rand::thread_rng().gen()
}

fn encode_password(password: &str) -> String {
    let password = password.as_bytes();
    let salt = generate_salt();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify_password(hash: &str, password: &str) -> bool {
    let password = password.as_bytes();
    argon2::verify_encoded(&hash, password).unwrap()
}

pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub struct JoinData {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn join(join_data: JoinData, conn: &PgConnection) -> Result<(), Error> {
    use schema::users;

    let JoinData {
        username,
        email,
        password,
    } = join_data;

    let settings = devand_core::UserSettings::default();
    let settings = serde_json::to_value(settings).map_err(|_| Error::Unknown)?;

    let new_user = models::NewUser {
        username,
        email,
        settings,
        visible_name: None,
    };

    let user: models::User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .map_err(|err| {
            // TODO Use anyhow to propagate the error message
            dbg!(err);
            Error::Unknown
        })?;

    add_password(UserId(user.id), &password, conn)
}

pub fn login(credentials: Credentials, conn: &PgConnection) -> Result<UserId, Error> {
    let Credentials { username, password } = credentials;

    let auth: models::Auth = schema_view::login::table
        .filter(schema_view::login::dsl::username.like(username))
        .first(conn)
        .map_err(|_| Error::Unknown)?;

    if verify_password(&auth.enc_password, &password) {
        Ok(UserId(auth.user_id))
    } else {
        Err(Error::Unknown)
    }
}

pub fn check_password(user_id: UserId, password: &str, conn: &PgConnection) -> Result<bool, Error> {
    let enc_password: String = schema::auth::table
        .filter(schema::auth::dsl::user_id.eq(user_id.0))
        .select(schema::auth::enc_password)
        .first(conn)
        .map_err(|_| Error::Unknown)?;
    Ok(verify_password(&enc_password, &password))
}

fn add_password(
    user_id: devand_core::UserId,
    password: &str,
    conn: &PgConnection,
) -> Result<(), Error> {
    let enc_password = encode_password(password);

    let new_auth = models::NewAuth {
        user_id: user_id.0,
        enc_password,
    };

    let ok = diesel::insert_into(schema::auth::table)
        .values(&new_auth)
        .execute(conn)
        .map_err(|err| {
            // TODO Use anyhow to propagate the error message
            dbg!(err);
            Error::Unknown
        })?;

    // TODO Handle this error instead of panicing
    assert_eq!(ok, 1);

    Ok(())
}

pub fn set_password(
    user_id: devand_core::UserId,
    password: &str,
    conn: &PgConnection,
) -> Result<(), Error> {
    let enc_password = encode_password(password);

    let ok = diesel::update(schema::auth::table.filter(schema::auth::dsl::user_id.eq(user_id.0)))
        .set((schema::auth::dsl::enc_password.eq(enc_password),))
        .execute(conn)
        .map_err(|err| {
            // TODO Use anyhow to propagate the error message
            dbg!(err);
            Error::Unknown
        })?;

    // TODO Handle this error instead of panicing
    assert_eq!(ok, 1);

    Ok(())
}

pub struct PasswordResetToken(pub String);

pub fn create_password_reset_token(
    user: devand_core::UserId,
    conn: &PgConnection,
) -> Option<PasswordResetToken> {
    use chrono::Utc;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let token: String = { thread_rng().sample_iter(&Alphanumeric).take(176).collect() };

    let token_duration = chrono::Duration::hours(3);

    let expires_at = Utc::now()
        .checked_add_signed(token_duration)
        .unwrap()
        .naive_utc();

    let new_password_reset = models::NewPasswordReset {
        user_id: user.0,
        expires_at,
        token: token.clone(),
    };

    diesel::insert_into(schema::password_reset::table)
        .values(&new_password_reset)
        .execute(conn)
        .map_err(|err| {
            // TODO Use anyhow to propagate the error message
            dbg!(err);
        })
        .ok();

    Some(PasswordResetToken(token))
}

pub fn reset_password(
    token: PasswordResetToken,
    password: String,
    conn: &PgConnection,
) -> Result<(), Error> {
    use chrono::Utc;

    // TODO Test expiration
    let now = Utc::now();

    let (user_id, expires_at): (i32, chrono::NaiveDateTime) = schema::password_reset::table
        .filter(schema::password_reset::dsl::token.eq(token.0))
        .select((
            schema::password_reset::user_id,
            schema::password_reset::expires_at,
        ))
        .first(conn)
        .map_err(|_| Error::Unknown)?;

    if expires_at < now.naive_utc() {
        // TODO Return Expired error (or use anyhow)
        return Err(Error::Unknown);
    }

    // Token is ok, we can change the password
    set_password(UserId(user_id), &password, conn)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_ok() {
        let password = "password";
        let hash = encode_password(password);

        assert!(verify_password(&hash, password));
    }
}
