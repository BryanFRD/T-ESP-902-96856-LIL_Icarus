CREATE TABLE ticket_attachments (
    id VARCHAR PRIMARY KEY,
    ticket_id VARCHAR NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    uploaded_by VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    file_name VARCHAR NOT NULL,
    file_path VARCHAR NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_ticket_attachments_ticket_id ON ticket_attachments(ticket_id);
