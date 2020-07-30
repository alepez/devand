table! {
    auth (user_id) {
        user_id -> Int4,
        enc_password -> Varchar,
    }
}

table! {
    chats (id) {
        id -> Uuid,
        members -> Array<Int4>,
    }
}

table! {
    messages (id) {
        id -> Uuid,
        chat_id -> Uuid,
        created_at -> Timestamp,
        txt -> Varchar,
        author -> Int4,
    }
}

table! {
    unread_messages (message_id, user_id) {
        message_id -> Uuid,
        user_id -> Int4,
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
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    auth,
    chats,
    messages,
    unread_messages,
    users,
);
