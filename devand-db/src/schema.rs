table! {
    auth (user_id) {
        user_id -> Int4,
        enc_password -> Varchar,
    }
}

table! {
    chats (id) {
        id -> Int4,
        members -> Jsonb,
    }
}

table! {
    messages (id) {
        id -> Int4,
        chat_id -> Int4,
        created_at -> Timestamp,
        txt -> Varchar,
        author -> Int4,
    }
}

table! {
    password_reset (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        settings -> Jsonb,
        visible_name -> Nullable<Varchar>,
        email_verified -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    auth,
    chats,
    messages,
    password_reset,
    users,
);
