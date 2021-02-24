use std::cmp::Ordering;

use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// The CRUD operation type
#[derive(Clone, PartialEq, Serialize)]
pub enum Event<T>
where
    T: Clone + PartialEq + Serialize + DeserializeOwned,
{
    /// The operation type of an event creating a new entity
    Create(EventContent<T>),

    /// The operation type of an event mutating an existing entity
    Update(EventContent<T>),

    /// The operation type of an event deleting an entity
    Delete(EventContent<T>),
}

/**
The `Event` struct usually is part of a list of events called an "event log".
Each event represents an atomic change in an entity (including its creation or destruction).
Every event log only contains one entity type:
Two entity types should have two separate event logs.
*/
#[derive(Serialize, PartialEq, Clone)]
pub struct EventContent<T>
where
    T: Clone + PartialEq + Serialize + DeserializeOwned,
{
    /// The moment in time the event occurred
    timestamp: DateTime<Utc>,

    /// The entity after the occurrence of this event
    ///
    /// This can be any type of data, as long as the traits
    /// `Clone` and `PartialEq` are implemented.
    data: T,
}

impl<T> Event<T>
where
    T: Clone + PartialEq + Serialize + DeserializeOwned,
{
    /// Constructs a new create event
    pub fn create(data: T) -> Self {
        Self::Create(EventContent {
            timestamp: Utc::now(),
            data,
        })
    }

    /// Constructs a new update event
    pub fn update(data: T) -> Self {
        Self::Update(EventContent {
            timestamp: Utc::now(),
            data,
        })
    }

    /// Constructs a new delete event
    pub fn delete(data: T) -> Self {
        Self::Delete(EventContent {
            timestamp: Utc::now(),
            data,
        })
    }

    /// Borrow the contained data
    pub fn borrow(&self) -> &T {
        &match self {
            Self::Create(ref data) => data,
            Self::Update(ref data) => data,
            Self::Delete(ref data) => data,
        }
        .data
    }

    /// Borrow the date of the event
    pub fn get_time(&self) -> &DateTime<Utc> {
        &match self {
            Self::Create(ref data) => data,
            Self::Update(ref data) => data,
            Self::Delete(ref data) => data,
        }
        .timestamp
    }

    /// Compare two events based on their timestamps
    pub fn compare(&self, other: &Self) -> Ordering {
        let (t1, t2) = (self.get_time(), other.get_time());

        if t1 < t2 {
            Ordering::Less
        } else if t1 > t2 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Consumes the event, returning the contained data
    pub fn take(self) -> T {
        match self {
            Self::Create(data) => data,
            Self::Update(data) => data,
            Self::Delete(data) => data,
        }
        .data
    }
}
