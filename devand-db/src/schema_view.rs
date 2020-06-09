// `schema.rs` is generated on migration, here we write schema-like code
// that is not automatically generated, like PostgreSQL views.
table! {
    login (user_id) {
        user_id -> Int4,
        username -> Varchar,
        enc_password -> Varchar,
    }
}
