// `schema.rs` is generated on migration, here we write schema-like code
// that is not automatically generated, like PostgreSQL views.
table! {
    login (user_id) {
        user_id -> Int4,
        username -> Varchar,
        enc_password -> Varchar,
    }
}

table! {
    chat_members (user_id) {
        chat_id -> Uuid,
        user_id -> Int4,
        username -> Varchar,
        visible_name -> Varchar,
        bio -> Varchar,
        languages -> Jsonb,
        spoken_languages -> Nullable<Jsonb>,
        projects -> Array<Text>,
    }
}

table! {
    unread_messages_full (message_id) {
        message_id -> Uuid,
        chat_id -> Uuid,
        user_id -> Int4,
    }
}
