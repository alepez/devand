use super::*;
use devand_core::*;
use maplit::btreeset;

pub fn user1() -> (User, String) {
    let user = User {
        id: UserId(1),
        username: "user1".into(),
        email: "user1@test.devand.dev".into(),
        email_verified: true,
        visible_name: "User One".into(),
        settings: UserSettings {
            languages: Languages::default(),
            schedule: Availability::default(),
            vacation_mode: false,
            spoken_languages: SpokenLanguages(btreeset![SpokenLanguage::English]),
        },
        unread_messages: 5,
        bio: "This is the bio".to_string(),
    };

    let password = "qwertyuiop1";

    (user, password.into())
}

pub fn populate_with_just_one_user(conn: &PgConnection) {
    let (user, password) = user1();

    let join_data = super::auth::JoinData {
        username: user.username,
        email: user.email,
        password: password.into(),
    };

    // Result is ignored (an error is be generated if the user already exist,
    // but it's expected to exist if database is not reset)
    super::auth::join(join_data, &conn).ok();
}
