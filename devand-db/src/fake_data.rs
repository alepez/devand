use super::*;
use devand_core::*;

pub struct OneUser {
    pub user: User,
    pub password: String,
}

impl OneUser {
    pub fn new(conn: &PgConnection) -> Self {
        super::clear_all(conn).unwrap();

        let password = "qwertyuiop1".to_string();

        let join_data = super::auth::JoinData {
            username: "user1".into(),
            email: "user1@test.devand.dev".into(),
            password: password.clone(),
        };

        let user = super::auth::join(join_data, &conn).unwrap();

        Self { user, password }
    }
}
