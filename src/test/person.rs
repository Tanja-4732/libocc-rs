use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
}

// Implemented manually to distinguish between people based on their UUIDs
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
