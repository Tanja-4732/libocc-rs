use crate::{Event, Segment, Timestamp};
use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

/**
Projects events from an event log

Manages several segments internally
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Projector<'a, T>
where
    T: Clone + PartialEq,
{
    segments: Vec<Segment<'a, T>>,
}

impl<'a, T> Projector<'_, T>
where
    T: Clone + PartialEq,
{
    /// Generates a new projector for a given type
    pub fn new() -> Projector<'a, T> {
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
        // Find the segment containing the timestamp (if available):
        // The position of the segment containing the requested timestamp
        let latest_segment_pos = self
            .segments
            .iter()
            .rposition(|s| s.get_time() <= timestamp)?;

        // The segment containing the timestamp
        // Unwraps safely because the index was found previously
        let containing_segment = self.segments.get(latest_segment_pos).unwrap();

        // Check if another segment exists which could provide a snapshot for projection
        let snapshot = if latest_segment_pos != 0 {
            // Return a copy of the snapshot of the previous segment
            // Unwraps safely because there're at least two segments (because != 0)
            self.segments
                .get(latest_segment_pos - 1)
                .unwrap()
                .get_projection()
                .clone()
        } else {
            // If no such snapshot exists (containing segment is the first or only one segment in total),
            // make a new vector on which to project the events of the containing segment onto
            vec![]
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

    /// Attempts to merge two segments/snapshots.  
    /// The one including the timestamp and the one before it (if any)
    pub fn merge_at(&mut self, timestamp: &Timestamp) -> Result<()> {
        // Find the segment containing the timestamp (if available):
        // The position of the segment containing the requested timestamp
        let latest_segment_pos = self
            .segments
            .iter()
            .rposition(|s| s.get_time() <= timestamp)
            .ok_or(anyhow!(
                "Cannot find segment containing the requested timestamp"
            ))?;

        // Find the snapshot before it the one containing the timestamp (if available)
        // Check if another segment exists which could provide a snapshot for projection
        let predating_segment = if latest_segment_pos != 0 {
            // Return the previous segment
            // Remove is safe because there're at least two segments (because != 0)
            self.segments.remove(latest_segment_pos - 1)
        } else {
            // If no such snapshot exists (containing segment is the first or only one segment in total),
            // return an error, as merging is impossible
            bail!("Cannot find a preceding segment")
        };

        // The segment containing the timestamp
        self.segments
            .get_mut(latest_segment_pos)
            // Unwraps safely because the index was found previously
            .unwrap()
            // Perform the merge
            .prepend(predating_segment)
    }
}
