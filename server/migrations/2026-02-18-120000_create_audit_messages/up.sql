CREATE TABLE audit_messages (
    id VARCHAR PRIMARY KEY,
    audit_id VARCHAR NOT NULL REFERENCES audits(id) ON DELETE CASCADE,
    author_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    content_nonce BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX audit_messages_audit_id_idx ON audit_messages(audit_id);
CREATE INDEX audit_messages_created_at_idx ON audit_messages(created_at);

CREATE TABLE audit_message_attachments (
    id VARCHAR PRIMARY KEY,
    message_id VARCHAR NOT NULL REFERENCES audit_messages(id) ON DELETE CASCADE,
    uploaded_by VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    file_name VARCHAR NOT NULL,
    file_path VARCHAR NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE INDEX audit_message_attachments_message_id_idx ON audit_message_attachments(message_id);
