DROP TRIGGER IF EXISTS update_ticket_comments_updated_at ON ticket_comments;
DROP FUNCTION IF EXISTS update_ticket_comments_updated_at_func();
DROP INDEX IF EXISTS idx_ticket_comments_ticket_id;
DROP TABLE IF EXISTS ticket_comments;
