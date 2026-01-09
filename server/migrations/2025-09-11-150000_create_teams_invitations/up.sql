CREATE TABLE teams_invitations (
    id VARCHAR PRIMARY KEY,
    team_id VARCHAR NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    token VARCHAR NOT NULL UNIQUE,
    max_uses INTEGER NOT NULL,
    used_count INTEGER NOT NULL DEFAULT 0,
    expires_at TIMESTAMP NULL,
    created_by VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_canceled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_teams_invitations_team_id ON teams_invitations(team_id);

CREATE OR REPLACE FUNCTION set_expires_at_on_cancel()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.is_canceled = TRUE AND (OLD.is_canceled IS DISTINCT FROM TRUE) THEN
        NEW.expires_at := COALESCE(NEW.expires_at, NOW());
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_set_expires_at_on_cancel
BEFORE UPDATE OF is_canceled ON teams_invitations
FOR EACH ROW
EXECUTE FUNCTION set_expires_at_on_cancel();

