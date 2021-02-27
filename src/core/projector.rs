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

    /// Returns the current (cached) projection as a shared reference
    pub fn get_projection(&self) -> &Vec<T> {
        // Unwraps safely because there's always at least one segment
        self.segments.last().unwrap().get_projection()
    }

    /// Performs a projection using a copy of the previous segments' snapshot if available
    pub fn project_at(&self, timestamp: &Timestamp) -> Option<Vec<T>> {
        // Find the segment containing the timestamp and the snapshot before it (if available)
        let (containing_segment, snapshot) = {
            // The position of the segment containing the requested timestamp
            let latest_segment_pos = self
                .segments
                .iter()
                .rev()
                .position(|s| s.get_time() <= timestamp)?;

            (
                // The segment containing the timestamp
                self.segments.get(latest_segment_pos)?,
                // Check if another segment exists which could provide a snapshot for projection
                if let Some(prev_segment) = self.segments.get(latest_segment_pos - 1) {
                    // Return a copy of the snapshot of the previous segment
                    prev_segment.get_projection().clone()
                } else {
                    // If no such snapshot exists (containing segment is the first or only one segment in total),
                    // make a new vector on which to project the events of the containing segment onto
                    vec![]
                },
            )
        };

        // Perform the projection
        containing_segment.project_at_onto(timestamp, snapshot)
    }

    /// Pushes an event onto the latest segment, updating the projection
    pub fn push(&mut self, event: Event<T>) -> Result<()> {
        // Unwraps safely because there's always at least one segment
        self.segments.last_mut().unwrap().push(event)
    }

    /// Makes a new snapshot of the projector by creating a new segment
    pub fn make_snapshot(&mut self) {
        // Get the latest segment
        // Unwraps safely because there's always at least one segment
        let latest_segment = self.segments.last().unwrap();

        // Make a new segment with the previously-latest segments snapshot
        let new_segment = Segment::from_projection(latest_segment.get_projection().clone(), vec![]);

        // Push the new segment onto the segments vector of this projector
        self.segments.push(new_segment);
    }
}
