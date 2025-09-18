// @generated automatically by Diesel CLI.

diesel::table! {
    password_reset_tokens (user_id) {
        user_id -> Varchar,
        token_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    teams (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    teams_members (user_id, team_id) {
        team_id -> Varchar,
        user_id -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
        avatar_url -> Nullable<Varchar>,
    }
}

diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(teams_members -> teams (team_id));
diesel::joinable!(teams_members -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    password_reset_tokens,
    teams,
    teams_members,
    users,
);
