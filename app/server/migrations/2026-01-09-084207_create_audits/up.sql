-- audits table stores audit information linked to teams and companies
CREATE TABLE audits (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    team_id VARCHAR NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    company_id VARCHAR NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    end_date TIMESTAMP,
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Junction table for audit clients (many-to-many)
CREATE TABLE audits_clients (
    audit_id VARCHAR NOT NULL REFERENCES audits(id) ON DELETE CASCADE,
    user_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL,
    PRIMARY KEY (audit_id, user_id)
);

-- Indexes for performance
CREATE INDEX idx_audits_team_id ON audits(team_id);
CREATE INDEX idx_audits_company_id ON audits(company_id);
CREATE INDEX idx_audits_is_archived ON audits(is_archived);
CREATE INDEX idx_audits_end_date ON audits(end_date) WHERE end_date IS NOT NULL;
CREATE INDEX idx_audits_clients_audit_id ON audits_clients(audit_id);
CREATE INDEX idx_audits_clients_user_id ON audits_clients(user_id);

-- Trigger to auto-update updated_at timestamp
CREATE OR REPLACE FUNCTION update_audits_updated_at_func()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_audits_updated_at
BEFORE UPDATE OF name, description, end_date, is_archived
ON audits
FOR EACH ROW
EXECUTE FUNCTION update_audits_updated_at_func();
