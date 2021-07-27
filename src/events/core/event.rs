use crate::events::Timestamp;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cmp::Ordering, ops::Deref};

/// The CRUD operation type
// TODO derive Deserialize
#[derive(Clone, PartialEq, Serialize, Debug, Deserialize)]
pub enum Event<'a, T>
where
    T: Clone + PartialEq,
{
    /// The operation type of an event creating a new entity
    Create(EventContent<'a, T>),

    /// The operation type of an event mutating an existing entity
    Update(EventContent<'a, T>),

    /// The operation type of an event deleting an entity
    Delete(EventContent<'a, T>),
}

/**
The `Event` struct usually is part of a list of events called an "event log".
Each event represents an atomic change in an entity (including its creation or destruction).
Every event log only contains one entity type:
Two entity types should have two separate event logs.
*/
#[derive(Clone, PartialEq, Serialize, Debug, Deserialize)]
pub struct EventContent<'a, T>
where
    T: Clone + PartialEq,
{
    /// The moment in time the event occurred
    timestamp: Timestamp,

    /// The entity after the occurrence of this event
    ///
    /// This can be any type of data, as long as the traits
    /// `Clone` and `PartialEq` are implemented.
    data: Cow<'a, T>,
}

impl<'a, T> Event<'a, T>
where
    T: Clone + PartialEq,
{
    /// Constructs a new create event
    pub fn create(data: Cow<'a, T>) -> Self {
        Self::Create(EventContent {
            timestamp: Utc::now(),
            data,
        })
    }

    /// Constructs a new update event
    pub fn update(data: Cow<'a, T>) -> Self {
        Self::Update(EventContent {
            timestamp: Utc::now(),
            data,
        })
    }

    /// Constructs a new delete event
    pub fn delete(data: Cow<'a, T>) -> Self {
        Self::Delete(EventContent {
            timestamp: Utc::now(),
            data,
        })
    }

    /// Borrow the contained data
    fn borrow_inner(&self) -> &T {
        &match self {
            Self::Create(ref data) | Self::Update(ref data) | Self::Delete(ref data) => data,
        }
        .data
    }

    /// Borrow the date of the event
    pub fn get_time(&self) -> &Timestamp {
        &match self {
            Self::Create(ref data) | Self::Update(ref data) | Self::Delete(ref data) => data,
        }
        .timestamp
    }

    /// Compare two events based on their timestamps
    fn compare_timestamps(&self, other: &Self) -> Ordering {
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
    pub fn take(self) -> Cow<'a, T> {
        match self {
            Self::Create(data) | Self::Update(data) | Self::Delete(data) => data,
        }
        .data
    }
}

impl<'a, T> Deref for Event<'a, T>
where
    T: Clone + PartialEq,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.borrow_inner()
    }
}

impl<'a, T> PartialOrd for Event<'a, T>
where
    T: Clone + PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare_timestamps(other))
    }
}
