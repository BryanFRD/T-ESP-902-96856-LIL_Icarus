// @generated automatically by Diesel CLI.

diesel::table! {
    audit_deployment_tokens (id) {
        id -> Varchar,
        audit_id -> Varchar,
        token_hash -> Varchar,
        created_by -> Varchar,
        is_revoked -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    audit_agents (id) {
        id -> Varchar,
        audit_id -> Varchar,
        name -> Varchar,
        token_hash -> Varchar,
        is_revoked -> Bool,
        last_connected_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

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
    audit_message_attachments (id) {
        id -> Varchar,
        message_id -> Varchar,
        uploaded_by -> Varchar,
        file_name -> Varchar,
        file_path -> Varchar,
        file_size -> Int8,
        mime_type -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    audit_messages (id) {
        id -> Varchar,
        audit_id -> Varchar,
        author_id -> Varchar,
        content -> Text,
        content_nonce -> Bytea,
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
        created_at -> Timestamp,
        expires_at -> Timestamp,
        metadata_nonce -> Nullable<Bytea>,
        changes_nonce -> Nullable<Bytea>,
    }
}

diesel::table! {
    kanban_columns (id) {
        id -> Varchar,
        audit_id -> Varchar,
        name -> Varchar,
        position -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    ticket_attachments (id) {
        id -> Varchar,
        ticket_id -> Varchar,
        uploaded_by -> Varchar,
        file_name -> Varchar,
        file_path -> Varchar,
        file_size -> Int8,
        mime_type -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ticket_comments (id) {
        id -> Varchar,
        ticket_id -> Varchar,
        author_id -> Varchar,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    ticket_labels (id) {
        id -> Varchar,
        audit_id -> Varchar,
        name -> Varchar,
        color -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tickets (id) {
        id -> Varchar,
        audit_id -> Varchar,
        column_id -> Varchar,
        parent_id -> Nullable<Varchar>,
        title -> Varchar,
        description -> Nullable<Text>,
        priority -> Varchar,
        assignee_id -> Nullable<Varchar>,
        position -> Int4,
        due_date -> Nullable<Timestamp>,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tickets_labels (ticket_id, label_id) {
        ticket_id -> Varchar,
        label_id -> Varchar,
        created_at -> Timestamp,
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
        password_hash_version -> Nullable<Int2>,
    }
}

diesel::joinable!(audit_agents -> audits (audit_id));
diesel::joinable!(audit_deployment_tokens -> audits (audit_id));
diesel::joinable!(audit_deployment_tokens -> users (created_by));
diesel::joinable!(audit_message_attachments -> audit_messages (message_id));
diesel::joinable!(audit_message_attachments -> users (uploaded_by));
diesel::joinable!(audit_messages -> audits (audit_id));
diesel::joinable!(audit_messages -> users (author_id));
diesel::joinable!(audits -> companies (company_id));
diesel::joinable!(audits -> teams (team_id));
diesel::joinable!(audits_clients -> audits (audit_id));
diesel::joinable!(audits_clients -> users (user_id));
diesel::joinable!(audits_invitations -> audits (audit_id));
diesel::joinable!(audits_invitations -> users (created_by));
diesel::joinable!(companies -> teams (team_id));
diesel::joinable!(events -> users (user_id));
diesel::joinable!(kanban_columns -> audits (audit_id));
diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(teams_invitations -> teams (team_id));
diesel::joinable!(teams_invitations -> users (created_by));
diesel::joinable!(teams_members -> teams (team_id));
diesel::joinable!(teams_members -> users (user_id));
diesel::joinable!(ticket_attachments -> tickets (ticket_id));
diesel::joinable!(ticket_attachments -> users (uploaded_by));
diesel::joinable!(ticket_comments -> tickets (ticket_id));
diesel::joinable!(ticket_comments -> users (author_id));
diesel::joinable!(ticket_labels -> audits (audit_id));
diesel::joinable!(tickets -> audits (audit_id));
diesel::joinable!(tickets -> kanban_columns (column_id));
diesel::joinable!(tickets_labels -> ticket_labels (label_id));
diesel::joinable!(tickets_labels -> tickets (ticket_id));

diesel::allow_tables_to_appear_in_same_query!(
    audit_agents,
    audit_deployment_tokens,
    audit_message_attachments,
    audit_messages,
    audits,
    audits_clients,
    audits_invitations,
    companies,
    events,
    kanban_columns,
    password_reset_tokens,
    teams,
    teams_invitations,
    teams_members,
    ticket_attachments,
    ticket_comments,
    ticket_labels,
    tickets,
    tickets_labels,
    users,
);
