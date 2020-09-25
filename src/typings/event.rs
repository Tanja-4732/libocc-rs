use crate::typings;
use std::time;

/// The `Event` struct usually is part of a list of events called an "event log".
///
/// Each event represents an atomic change in an entity (including its creation or destruction).
///
/// Every event log only contains one entity type:
/// Two entity types should have two separate event logs.
pub struct Event<T: Clone + PartialEq> {
  /// The moment in time the event occurred
  pub date: time::Instant,

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
  pub data: T,
}
