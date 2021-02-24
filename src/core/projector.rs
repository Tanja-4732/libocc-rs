use crate::{Event, Timestamp};
use chrono::Utc;
use serde::{de::DeserializeOwned, Serialize};
use std::unimplemented;

/**
Projects events from an event log
*/
pub struct Projector<T>
where
    T: Clone + PartialEq + Serialize + DeserializeOwned,
{
    current_projection: Vec<T>,
    segments: Vec<Segment<T>>,
}

impl<T> Projector<T>
where
    T: Clone + PartialEq + Serialize + DeserializeOwned,
{
    /// Generates a new projector for a given type
    pub fn new() -> Projector<T> {
        Self {
            current_projection: vec![],
            segments: vec![Segment::new()],
        }
    }

    pub fn project_at(&self, timestamp: &Timestamp) -> Option<&Vec<T>> {
        let latest_segment = self
            .segments
            .iter()
            .rev()
            .find(|s| s.timestamp <= *timestamp)?;

        unimplemented!()
    }

    pub fn push(&mut self, event: Event<T>) {
        self.segments.last_mut().unwrap().events.push(event);
    }
}

struct Segment<T>
where
    T: Clone + PartialEq + Serialize + DeserializeOwned,
{
    pub timestamp: Timestamp,
    pub snapshot: Vec<T>,
    pub events: Vec<Event<T>>,
}

impl<T> Segment<T>
where
    T: Clone + PartialEq + Serialize + DeserializeOwned,
{
    fn new() -> Segment<T> {
        Self::from_projection(vec![], vec![])
    }

    fn from_projection(projection: Vec<T>, events: Vec<Event<T>>) -> Segment<T> {
        Self {
            timestamp: Utc::now(),
            snapshot: projection,
            events,
        }
    }

    fn project_at(&self, timestamp: Timestamp) -> Option<&Vec<T>> {
        unimplemented!()
    }
}
