CREATE TABLE audits_invitations (
    id VARCHAR PRIMARY KEY,
    audit_id VARCHAR NOT NULL,
    client_email VARCHAR NOT NULL,
    token VARCHAR NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    is_canceled BOOLEAN NOT NULL DEFAULT FALSE,
    is_used BOOLEAN NOT NULL DEFAULT FALSE,
    used_at TIMESTAMP NULL,
    created_by VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    
    FOREIGN KEY (audit_id) REFERENCES audits(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id)
);

CREATE INDEX idx_audits_invitations_audit_id ON audits_invitations(audit_id);
CREATE INDEX idx_audits_invitations_token ON audits_invitations(token);
CREATE INDEX idx_audits_invitations_email ON audits_invitations(client_email);
