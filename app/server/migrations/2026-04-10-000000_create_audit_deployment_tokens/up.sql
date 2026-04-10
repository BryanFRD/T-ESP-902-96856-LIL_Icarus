CREATE TABLE audit_deployment_tokens (
    id VARCHAR PRIMARY KEY,
    audit_id VARCHAR NOT NULL REFERENCES audits(id) ON DELETE CASCADE,
    token_hash VARCHAR NOT NULL,
    created_by VARCHAR NOT NULL REFERENCES users(id),
    is_revoked BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_deployment_tokens_audit ON audit_deployment_tokens(audit_id);
