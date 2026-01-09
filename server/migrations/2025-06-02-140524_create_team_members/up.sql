-- Your SQL goes here
CREATE TABLE teams_members (
    team_id VARCHAR NOT NULL,
    user_id VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    PRIMARY KEY (user_id, team_id),
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
-- PostgreSQL trigger function to update updated_at
CREATE OR REPLACE FUNCTION update_teams_members_updated_at_func()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to call the function after update
CREATE TRIGGER update_teams_members_updated_at
BEFORE UPDATE OF
    role
ON teams_members
FOR EACH ROW
EXECUTE FUNCTION update_teams_members_updated_at_func();