ALTER TABLE events
ADD COLUMN metadata_nonce BYTEA,
ADD COLUMN changes_nonce BYTEA;
