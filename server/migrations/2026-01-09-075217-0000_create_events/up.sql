CREATE TABLE events (
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR NOT NULL,
    resource_type VARCHAR NOT NULL,
    resource_id VARCHAR,
    request_method VARCHAR NOT NULL,
    request_path VARCHAR NOT NULL,
    ip_address VARCHAR,
    user_agent VARCHAR,
    metadata JSONB,
    changes JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_events_user_id ON events(user_id);
CREATE INDEX idx_events_resource_type ON events(resource_type);
CREATE INDEX idx_events_resource_id ON events(resource_id);
CREATE INDEX idx_events_action ON events(action);
CREATE INDEX idx_events_created_at ON events(created_at DESC);
CREATE INDEX idx_events_resource_type_resource_id ON events(resource_type, resource_id);
CREATE INDEX idx_events_user_action_created ON events(user_id, action, created_at DESC);
