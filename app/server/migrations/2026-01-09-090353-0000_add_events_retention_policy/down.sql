DROP INDEX IF EXISTS idx_events_expires_at;
ALTER TABLE events DROP COLUMN IF EXISTS expires_at;
