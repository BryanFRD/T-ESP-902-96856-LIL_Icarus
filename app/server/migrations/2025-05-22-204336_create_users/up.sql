-- Your SQL goes here
CREATE TABLE "users"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"username" VARCHAR NOT NULL,
	"first_name" VARCHAR NOT NULL,
	"last_name" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL,
	"role" VARCHAR NOT NULL,
	"password_hash" VARCHAR NOT NULL,
	"is_connected" BOOL NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	"last_connected_at" TIMESTAMP NOT NULL
);

-- PostgreSQL trigger function to update updated_at
CREATE OR REPLACE FUNCTION update_users_updated_at_func()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to call the function after update
CREATE TRIGGER update_users_updated_at
BEFORE UPDATE OF
    username,
    first_name,
    last_name,
    email,
    role,
    password_hash
ON users
FOR EACH ROW
EXECUTE FUNCTION update_users_updated_at_func();