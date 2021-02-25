use crate::{Event, Timestamp};
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use serde::{de::DeserializeOwned, Serialize};
use std::{mem::replace, unimplemented};

/**
Projects events from an event log
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
            .find(|s| s.timestamp <= *timestamp)?;

        unimplemented!()
    }

    pub fn push(&mut self, event: Event<T>) {
        self.segments.last_mut().unwrap().events.push(event);
    }
}

/**
A segment is a part of an event log.

It's a list of events followed by a projection, called a
"snapshot". Creating a new segment will make the former
segments snapshot immutable, and a new (mutable) segment
is created.

Snapshots allow for faster history traversal, as not the
entire event log needs to be replayed in order to project
an earlier state, unlike a single-segment event log.
*/
pub struct Segment<T>
where
    T: Clone + PartialEq + Serialize + DeserializeOwned,
{
    /// The earliest data captured by this segment
    timestamp: Timestamp,

    /// The latest projection from this segment
    snapshot: Vec<T>,

    /// The event log of this segment
    events: Vec<Event<T>>,
}

impl<T> Segment<T>
where
    T: Clone + PartialEq + Serialize + DeserializeOwned,
{
    /**
    Creates a new segment

    The new segment will have a timestamp of the current time,
    and won't have any prior history associated with it.
    */
    pub fn new() -> Segment<T> {
        Self::from_projection(vec![], vec![])
    }

    pub fn from_projection(projection: Vec<T>, events: Vec<Event<T>>) -> Segment<T> {
        Self {
            timestamp: Utc::now(),
            snapshot: projection,
            events,
        }
    }

    pub fn project_at(&self, timestamp: Timestamp) -> Option<&Vec<T>> {
        // Check for timestamps before the segment started
        if timestamp < self.timestamp {
            return None;
        };

        unimplemented!()
    }

    pub fn push(&mut self, event: Event<T>) -> Result<()> {
        unimplemented!()
    }

    pub fn push_unchecked(&mut self, event: Event<T>) -> Result<()> {
        // Apply the event tto the snapshot
        self.apply_event(event.clone());

        // Push the event onto the event log
        self.events.push(event);

        Ok(())
    }

    fn apply_event(&mut self, event: Event<T>) -> Result<()> {
        // The pre-existing element
        let prev_position = self.snapshot.iter_mut().position(|e| e == event.borrow());

        match &event {
            Event::Create(_) => {
                // Avoid collisions
                if prev_position.is_some() {
                    bail!("Cannot create pre-existing data")
                }

                // Insert the new element
                self.snapshot.push(event.take());

                // Return Ok
                Ok(())
            }
            Event::Update(_) => {
                if let Some(index) = prev_position {
                    // Perform the replacement
                    *self.snapshot.get_mut(index).unwrap() = event.take();

                    // Return Ok
                    Ok(())
                } else {
                    bail!("Cannot modify non-existent data")
                }
            }
            Event::Delete(_) => {
                if let Some(index) = prev_position {
                    // Perform the deletion
                    self.snapshot.remove(index);

                    // Return Ok
                    Ok(())
                } else {
                    bail!("Cannot delete non-existent data")
                }
            }
        }
    }
}
