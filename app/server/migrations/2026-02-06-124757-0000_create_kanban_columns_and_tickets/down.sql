DROP TABLE IF EXISTS tickets_labels;

DROP TRIGGER IF EXISTS update_ticket_labels_updated_at ON ticket_labels;
DROP FUNCTION IF EXISTS update_ticket_labels_updated_at_func();
DROP TABLE IF EXISTS ticket_labels;

DROP TRIGGER IF EXISTS check_ticket_column_audit ON tickets;
DROP FUNCTION IF EXISTS check_ticket_column_same_audit();
DROP TRIGGER IF EXISTS check_ticket_parent_audit ON tickets;
DROP FUNCTION IF EXISTS check_ticket_parent_same_audit();
DROP TRIGGER IF EXISTS update_tickets_updated_at ON tickets;
DROP FUNCTION IF EXISTS update_tickets_updated_at_func();
DROP INDEX IF EXISTS idx_tickets_due_date;
DROP INDEX IF EXISTS idx_tickets_column_id_position;
DROP INDEX IF EXISTS idx_tickets_assignee_id;
DROP INDEX IF EXISTS idx_tickets_parent_id;
DROP INDEX IF EXISTS idx_tickets_column_id;
DROP INDEX IF EXISTS idx_tickets_audit_id;
DROP TABLE IF EXISTS tickets;

DROP TRIGGER IF EXISTS update_kanban_columns_updated_at ON kanban_columns;
DROP FUNCTION IF EXISTS update_kanban_columns_updated_at_func();
DROP INDEX IF EXISTS idx_kanban_columns_audit_id_position;
DROP TABLE IF EXISTS kanban_columns;
