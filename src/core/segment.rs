use crate::{Event, Timestamp};
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};

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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Segment<T>
where
    T: Clone + PartialEq,
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
    T: Clone + PartialEq,
{
    /// Creates a new segment
    ///
    /// The new segment will have a timestamp of the current time,
    /// and won't have any prior history associated with it.
    pub fn new() -> Segment<T> {
        Self::from_projection(vec![], vec![])
    }

    /// Creates a new segment from a given projection and event log at the current time
    pub fn from_projection(projection: Vec<T>, events: Vec<Event<T>>) -> Segment<T> {
        Self {
            timestamp: Utc::now(),
            snapshot: projection,
            events,
        }
    }

    /// Returns a shared reference to the timestamp of this segment
    pub fn get_time(&self) -> &Timestamp {
        &self.timestamp
    }

    /// Returns the current projection
    pub fn get_projection(&self) -> &Vec<T> {
        &self.snapshot
    }

    /// Projects the segments events predating a specified timestamp onto a given snapshot
    pub fn project_at_onto(&self, timestamp: &Timestamp, snapshot: Vec<T>) -> Option<Vec<T>> {
        // Check for timestamps before the segment started
        if timestamp < &self.timestamp {
            return None;
        };

        // A new vector to store the projection to be created
        // TODO maybe use the segments snapshot (this means it needs another snapshot at its beginning)
        let mut projection = snapshot;

        // Project all events up to (and including) the specified timestamp
        for event in &self.events {
            // Check if the specified predates the next event
            if event.get_time() > timestamp {
                break;
            }

            // Apply the event to the projection
            Self::apply_event_to(&mut projection, event.clone()).ok()?;
        }

        // Return the projection
        Some(projection)
    }

    /// Applies and appends an event to the segments snapshot and log, respectively (checked)
    pub fn push(&mut self, event: Event<T>) -> Result<()> {
        // Get the time of the new event
        let new_event_time = event.get_time();

        // Check if the new event predates the segments timestamp
        if new_event_time < &self.timestamp {
            bail!("Cannot accept events before the segment started")
        } else
        // Check if the new event predates the latest event stored in this segment (if it exists)
        if let Some(last_event_time) = self.events.last().map(|event| event.get_time()) {
            if new_event_time < last_event_time {
                bail!("Cannot accept events predating the lastest logged event")
            }
        }

        // Perform the push using push_unchecked
        self.push_unchecked(event)?;

        // Return Ok
        Ok(())
    }

    /// Applies and appends an event to the segments snapshot and log, respectively (unchecked)
    fn push_unchecked(&mut self, event: Event<T>) -> Result<()> {
        // Apply the event to the snapshot
        self.apply_event(event.clone())?;

        // Push the event onto the event log
        self.events.push(event);

        Ok(())
    }

    /// Modifies the segments snapshot to reflect the changes of the event
    fn apply_event(&mut self, event: Event<T>) -> Result<()> {
        Self::apply_event_to(&mut self.snapshot, event)
    }

    /// Modifies a given snapshot to reflect the changes of the event
    fn apply_event_to(snapshot: &mut Vec<T>, event: Event<T>) -> Result<()> {
        // The pre-existing element
        let prev_position = snapshot.iter_mut().position(|e| e == event.borrow());

        match &event {
            Event::Create(_) => {
                // Avoid collisions
                if prev_position.is_some() {
                    bail!("Cannot create pre-existing data")
                }

                // Insert the new element
                snapshot.push(event.take());

                // Return Ok
                Ok(())
            }
            Event::Update(_) => {
                if let Some(index) = prev_position {
                    // Perform the replacement
                    *snapshot.get_mut(index).unwrap() = event.take();

                    // Return Ok
                    Ok(())
                } else {
                    bail!("Cannot modify non-existent data")
                }
            }
            Event::Delete(_) => {
                if let Some(index) = prev_position {
                    // Perform the deletion
                    snapshot.remove(index);

                    // Return Ok
                    Ok(())
                } else {
                    bail!("Cannot delete non-existent data")
                }
            }
        }
    }

    /// Merges two consecutive segments by prepending the other before this one (checked)
    pub fn prepend(&mut self, other: Self) -> Result<()> {
        // Avoid a panic in append()
        self.events
            .len()
            .checked_add(other.events.len())
            .ok_or(anyhow!("Cannot merge segments exceeding usize"))?;

        // Check if this segment predates the newest event of the other one
        if self.get_time()
            < other
                .events
                .last()
                .map(|e| e.get_time())
                .unwrap_or(other.get_time())
        {
            bail!("Cannot prepend another segment if this one predates it")
        }

        // Perform the prepend
        self.prepend_unchecked(other);

        // Return Ok
        Ok(())
    }

    /// Merges two consecutive segments by prepending the other before this one (unchecked)
    pub fn prepend_unchecked(&mut self, mut other: Self) {
        // Append the new events onto the old ones
        other.events.append(&mut self.events);

        // Replace the current (now empty) list with the old (appended) one
        self.events = other.events;

        // Replace the timestamp of this segment with the one from the other
        self.timestamp = other.timestamp;
    }
}
