ALTER TABLE events ADD COLUMN expires_at TIMESTAMP;

UPDATE events SET expires_at = created_at + INTERVAL '365 days';

ALTER TABLE events ALTER COLUMN expires_at SET NOT NULL;

CREATE INDEX idx_events_expires_at ON events(expires_at) WHERE expires_at IS NOT NULL;
