use serde::{Deserialize, Serialize};

/// The CRUD operation type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CRUD {
    /// The operation type of an event creating a new entity
    Create,

    /// The operation type of an event mutating an existing entity
    Update,

    /// The operation type of an event deleting an entity
    Delete,
}
