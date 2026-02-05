// @generated automatically by Diesel CLI.

diesel::table! {
    audits (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
        team_id -> Varchar,
        company_id -> Varchar,
        end_date -> Nullable<Timestamp>,
        is_archived -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    audits_clients (audit_id, user_id) {
        audit_id -> Varchar,
        user_id -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    audits_invitations (id) {
        id -> Varchar,
        audit_id -> Varchar,
        client_email -> Varchar,
        token -> Varchar,
        expires_at -> Timestamp,
        is_canceled -> Bool,
        is_used -> Bool,
        used_at -> Nullable<Timestamp>,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    companies (id) {
        id -> Varchar,
        team_id -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
        siret -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    events (id) {
        id -> Varchar,
        user_id -> Nullable<Varchar>,
        action -> Varchar,
        resource_type -> Varchar,
        resource_id -> Nullable<Varchar>,
        request_method -> Varchar,
        request_path -> Varchar,
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Varchar>,
        metadata -> Nullable<Jsonb>,
        changes -> Nullable<Jsonb>,
        metadata_nonce -> Nullable<Bytea>,
        changes_nonce -> Nullable<Bytea>,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

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
    teams_invitations (id) {
        id -> Varchar,
        team_id -> Varchar,
        token -> Varchar,
        max_uses -> Int4,
        used_count -> Int4,
        expires_at -> Nullable<Timestamp>,
        created_by -> Varchar,
        is_canceled -> Bool,
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
        password_hash_version -> Nullable<Int2>,
        is_connected -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_connected_at -> Timestamp,
        avatar_url -> Nullable<Varchar>,
    }
}

diesel::joinable!(audits -> companies (company_id));
diesel::joinable!(audits -> teams (team_id));
diesel::joinable!(audits_clients -> audits (audit_id));
diesel::joinable!(audits_clients -> users (user_id));
diesel::joinable!(audits_invitations -> audits (audit_id));
diesel::joinable!(audits_invitations -> users (created_by));
diesel::joinable!(companies -> teams (team_id));
diesel::joinable!(events -> users (user_id));
diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(teams_invitations -> teams (team_id));
diesel::joinable!(teams_invitations -> users (created_by));
diesel::joinable!(teams_members -> teams (team_id));
diesel::joinable!(teams_members -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    audits,
    audits_clients,
    audits_invitations,
    companies,
    events,
    password_reset_tokens,
    teams,
    teams_invitations,
    teams_members,
    users,
);
