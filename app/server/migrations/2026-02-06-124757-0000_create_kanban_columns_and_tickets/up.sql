-- Kanban columns per audit
CREATE TABLE kanban_columns (
    id VARCHAR PRIMARY KEY,
    audit_id VARCHAR NOT NULL REFERENCES audits(id) ON DELETE CASCADE,
    name VARCHAR NOT NULL,
    position INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    UNIQUE(audit_id, name)
);

CREATE INDEX idx_kanban_columns_audit_id_position ON kanban_columns(audit_id, position);

CREATE OR REPLACE FUNCTION update_kanban_columns_updated_at_func()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_kanban_columns_updated_at
BEFORE UPDATE OF name, position
ON kanban_columns
FOR EACH ROW
EXECUTE FUNCTION update_kanban_columns_updated_at_func();

-- Tickets
CREATE TABLE tickets (
    id VARCHAR PRIMARY KEY,
    audit_id VARCHAR NOT NULL REFERENCES audits(id) ON DELETE CASCADE,
    column_id VARCHAR NOT NULL REFERENCES kanban_columns(id) ON DELETE CASCADE,
    parent_id VARCHAR REFERENCES tickets(id) ON DELETE CASCADE,
    title VARCHAR NOT NULL,
    description TEXT,
    priority VARCHAR NOT NULL DEFAULT 'medium',
    assignee_id VARCHAR REFERENCES users(id) ON DELETE SET NULL,
    position INTEGER NOT NULL,
    due_date TIMESTAMP,
    created_by VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_tickets_audit_id ON tickets(audit_id);
CREATE INDEX idx_tickets_column_id ON tickets(column_id);
CREATE INDEX idx_tickets_parent_id ON tickets(parent_id);
CREATE INDEX idx_tickets_assignee_id ON tickets(assignee_id);
CREATE INDEX idx_tickets_column_id_position ON tickets(column_id, position);
CREATE INDEX idx_tickets_due_date ON tickets(due_date);

CREATE OR REPLACE FUNCTION update_tickets_updated_at_func()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_tickets_updated_at
BEFORE UPDATE OF title, description, priority, assignee_id, column_id, position, due_date, parent_id
ON tickets
FOR EACH ROW
EXECUTE FUNCTION update_tickets_updated_at_func();

-- Constraint: parent_id must belong to the same audit
CREATE OR REPLACE FUNCTION check_ticket_parent_same_audit()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.parent_id IS NOT NULL THEN
        IF NOT EXISTS (
            SELECT 1 FROM tickets WHERE id = NEW.parent_id AND audit_id = NEW.audit_id
        ) THEN
            RAISE EXCEPTION 'Parent ticket must belong to the same audit';
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_ticket_parent_audit
BEFORE INSERT OR UPDATE OF parent_id ON tickets
FOR EACH ROW
EXECUTE FUNCTION check_ticket_parent_same_audit();

-- Constraint: column_id must belong to the same audit
CREATE OR REPLACE FUNCTION check_ticket_column_same_audit()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM kanban_columns WHERE id = NEW.column_id AND audit_id = NEW.audit_id
    ) THEN
        RAISE EXCEPTION 'Column must belong to the same audit';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_ticket_column_audit
BEFORE INSERT OR UPDATE OF column_id ON tickets
FOR EACH ROW
EXECUTE FUNCTION check_ticket_column_same_audit();

-- Ticket labels per audit
CREATE TABLE ticket_labels (
    id VARCHAR PRIMARY KEY,
    audit_id VARCHAR NOT NULL REFERENCES audits(id) ON DELETE CASCADE,
    name VARCHAR NOT NULL,
    color VARCHAR NOT NULL DEFAULT '#6B7280',
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    UNIQUE(audit_id, name)
);

CREATE OR REPLACE FUNCTION update_ticket_labels_updated_at_func()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_ticket_labels_updated_at
BEFORE UPDATE OF name, color
ON ticket_labels
FOR EACH ROW
EXECUTE FUNCTION update_ticket_labels_updated_at_func();

-- Junction table: tickets <-> labels (M2M)
CREATE TABLE tickets_labels (
    ticket_id VARCHAR NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    label_id VARCHAR NOT NULL REFERENCES ticket_labels(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL,
    PRIMARY KEY (ticket_id, label_id)
);
