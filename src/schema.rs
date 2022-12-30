// @generated automatically by Diesel CLI.

diesel::table! {
    auth_user (email) {
        email -> Varchar,
        id -> Uuid,
        password -> Varchar,
        expires_at -> Timestamp,
    }
}

embed_migrations!();