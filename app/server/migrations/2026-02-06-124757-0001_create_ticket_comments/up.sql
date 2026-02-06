CREATE TABLE ticket_comments (
    id VARCHAR PRIMARY KEY,
    ticket_id VARCHAR NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    author_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_ticket_comments_ticket_id ON ticket_comments(ticket_id);

CREATE OR REPLACE FUNCTION update_ticket_comments_updated_at_func()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_ticket_comments_updated_at
BEFORE UPDATE OF content
ON ticket_comments
FOR EACH ROW
EXECUTE FUNCTION update_ticket_comments_updated_at_func();
