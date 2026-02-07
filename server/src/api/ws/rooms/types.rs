use serde::{Deserialize, Serialize};

/// Payload sent by the client when joining or leaving a room.
///
/// - `{ "type": "me" }` — the user's personal room (server maps to `user:{user_id}`)
/// - `{ "type": "audit", "id": "<audit_id>" }` — an audit room
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RoomPayload {
    Me,
    Audit { id: String },
}

/// Internal representation of a room.
#[derive(Debug, Clone)]
pub enum Room {
    /// Personal user room — `user:{user_id}`
    User(String),
    /// Audit room — `audit:{audit_id}`
    Audit(String),
}

impl Room {
    /// Build a `Room` from the client payload, injecting the authenticated user id.
    pub fn from_payload(payload: &RoomPayload, user_id: &str) -> Self {
        match payload {
            RoomPayload::Me => Self::User(user_id.to_string()),
            RoomPayload::Audit { id } => Self::Audit(id.clone()),
        }
    }

    /// The Socket.IO room identifier used internally.
    pub fn room_id(&self) -> String {
        match self {
            Self::User(id) => format!("user:{id}"),
            Self::Audit(id) => format!("audit:{id}"),
        }
    }

    /// The display name returned to the client.
    ///
    /// For the personal room we return `"me"` so the user id is never exposed.
    pub fn display_name(&self) -> String {
        match self {
            Self::User(_) => "me".to_string(),
            Self::Audit(id) => format!("audit:{id}"),
        }
    }
}
