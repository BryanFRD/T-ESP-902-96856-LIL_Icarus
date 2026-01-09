CREATE TABLE companies (
    id VARCHAR PRIMARY KEY,
    team_id VARCHAR NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    name VARCHAR NOT NULL,
    description VARCHAR,
    siret VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    UNIQUE(team_id, siret)
);

CREATE INDEX idx_companies_team_id ON companies(team_id);

CREATE OR REPLACE FUNCTION update_companies_updated_at_func()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_companies_updated_at
BEFORE UPDATE OF name, description, siret
ON companies
FOR EACH ROW
EXECUTE FUNCTION update_companies_updated_at_func();
