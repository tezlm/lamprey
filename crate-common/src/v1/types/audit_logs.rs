use crate::v1::types::{AuditLogId, MessageSync, RoomId, UserId};

use serde::{Deserialize, Serialize};
#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

// TODO(#239): redesign audit log schema, since recursion
// also causes some issues when trying to load old data, need to add migrations or #[serde(default)] attrs
// TODO: rename to AuditLogEntry and AuditLogEntryId

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct AuditLog {
    /// Unique id idenfitying this entry
    pub id: AuditLogId,

    /// Room this happened in
    pub room_id: RoomId,

    /// User who caused this entry to be created
    pub user_id: UserId,

    /// User supplied reason why this happened
    pub reason: Option<String>,

    #[cfg_attr(feature = "utoipa", schema(no_recursion))]
    /// Generated sync payload (sent in websocket)
    pub payload: Box<MessageSync>,
    // pub payload: Box<Value>,
    #[cfg_attr(feature = "utoipa", schema(no_recursion))]
    /// The previous payload, or None if this resource is newly created
    // theres probably a better way to do this, but its the best solution i could think of for now
    pub payload_prev: Option<Box<MessageSync>>,
    // pub payload_prev: Option<Box<Value>>,
}
