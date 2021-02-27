use super::person::Person;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub uuid: Uuid,
    pub some_number: usize,
    pub author: Person,
    // // TODO demonstrate some kind of field hider
    // pub hidden_property: String,
}

// Implemented manually to distinguish between books based on their UUIDs
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
