-- Your SQL goes here
CREATE TABLE teams (
    "id" VARCHAR NOT NULL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "description" VARCHAR NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL
);

-- PostgreSQL trigger function to update updated_at
CREATE OR REPLACE FUNCTION update_teams_updated_at_func()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to call the function after update
CREATE TRIGGER update_teams_updated_at
BEFORE UPDATE OF
    name,
    description
ON teams
FOR EACH ROW
EXECUTE FUNCTION update_teams_updated_at_func();