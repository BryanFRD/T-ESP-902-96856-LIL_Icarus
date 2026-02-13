CREATE TABLE audit_agents (
    id VARCHAR PRIMARY KEY,
    audit_id VARCHAR NOT NULL REFERENCES audits(id) ON DELETE CASCADE,
    name VARCHAR NOT NULL,
    token_hash VARCHAR NOT NULL,
    is_revoked BOOLEAN NOT NULL DEFAULT FALSE,
    last_connected_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(audit_id, name)
);

CREATE INDEX idx_audit_agents_audit_id ON audit_agents(audit_id);
