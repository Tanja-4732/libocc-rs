use crate::{Event, Segment, Timestamp};
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use serde::{de::DeserializeOwned, Serialize};
use std::{mem::replace, unimplemented};

/**
Projects events from an event log

Manages several segments internally
*/
pub struct Projector<T>
where
    T: Clone + PartialEq + Serialize + DeserializeOwned,
{
    segments: Vec<Segment<T>>,
}

impl<T> Projector<T>
where
    T: Clone + PartialEq + Serialize + DeserializeOwned,
{
    /// Generates a new projector for a given type
    pub fn new() -> Projector<T> {
        Self {
            segments: vec![Segment::new()],
        }
    }

    pub fn project_at(&self, timestamp: &Timestamp) -> Option<&Vec<T>> {
        let latest_segment = self
            .segments
            .iter()
            .rev()
            .find(|s| s.get_time() <= timestamp)?;

        unimplemented!()
    }

    pub fn push(&mut self, event: Event<T>) {
        self.segments.last_mut().unwrap().events.push(event);
    }
}
