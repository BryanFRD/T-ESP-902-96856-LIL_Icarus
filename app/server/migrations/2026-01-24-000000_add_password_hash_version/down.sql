-- Recreate original trigger without password_hash_version
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
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

-- Remove index
DROP INDEX IF EXISTS idx_users_password_hash_version;

-- Remove column
ALTER TABLE users DROP COLUMN IF EXISTS password_hash_version;
