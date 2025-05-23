// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        role -> Varchar,
        password_hash -> Varchar,
        is_connected -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_connected_at -> Timestamp,
    }
}
