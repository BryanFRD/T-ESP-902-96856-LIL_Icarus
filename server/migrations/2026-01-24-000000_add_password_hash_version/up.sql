-- Add password_hash_version column (nullable for backward compatibility)
ALTER TABLE users ADD COLUMN password_hash_version SMALLINT DEFAULT NULL;

-- Create index for quick version lookups during migration monitoring
CREATE INDEX idx_users_password_hash_version ON users(password_hash_version);

-- Update trigger to include new column
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
CREATE TRIGGER update_users_updated_at
BEFORE UPDATE OF
    username,
    first_name,
    last_name,
    email,
    role,
    password_hash,
    password_hash_version
ON users
FOR EACH ROW
EXECUTE FUNCTION update_users_updated_at_func();

COMMENT ON COLUMN users.password_hash_version IS 'NULL or 1: double SHA-256 (legacy), 2: Argon2id';
