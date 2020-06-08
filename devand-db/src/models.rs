use super::schema::auth;
use super::schema::users;
use serde_json;
use std::convert::TryInto;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub settings: serde_json::Value,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub settings: serde_json::Value,
}

#[derive(Insertable)]
#[table_name = "auth"]
pub struct NewAuth {
    pub user_id: i32,
    pub enc_password: String,
}

#[derive(Queryable)]
pub struct Auth {
    pub user_id: i32,
    pub username: String,
    pub enc_password: String,
}

impl TryInto<devand_core::User> for User {
    type Error = ();

    fn try_into(self) -> Result<devand_core::User, ()> {
        let settings = serde_json::from_value(self.settings).map_err(|_| ())?;

        let user = devand_core::User {
            id: self.id,
            username: self.username,
            email: self.email,
            visible_name: "".to_string(),
            settings,
        };

        Ok(user)
    }
}
