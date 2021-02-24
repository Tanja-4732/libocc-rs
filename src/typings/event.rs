use crate::typings;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/**
The `Event` struct usually is part of a list of events called an "event log".
Each event represents an atomic change in an entity (including its creation or destruction).
Every event log only contains one entity type:
Two entity types should have two separate event logs.
*/
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Event
// where
//     T: Clone + PartialEq + Serialize,
{
    /// The moment in time the event occurred
    pub date: DateTime<Utc>,

    /// The [`CRUD`] operation type of this event
    ///
    /// Specifies if this event creates, updates or deletes an entity
    ///
    /// [`CRUD`]: crud/enum.CRUD.html
    pub operation: typings::CRUD,

    /// The entity after the occurrence of this event
    ///
    /// This can be any type of data, as long as the traits
    /// `Clone` and `PartialEq` are implemented.
    pub data: String,
}
