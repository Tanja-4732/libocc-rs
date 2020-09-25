use crate::core;
use crate::typings::{self, CRUD};
use std::time;

/// The `Repository` struct uses a [`Projector`] internally to abstract a list of [`Event`s],
/// offering simple CRUD methods instead of requiring the user to create and manage events themselves.
///
/// [`Event`s]: ../typings/event/struct.Event.html
/// [`Projector`]: projector/struct.Projector.html
pub struct Repository<T: Clone + PartialEq> {
  projector: core::Projector<T>,
}

impl<T: Clone + PartialEq> Repository<T> {
  /// Constructs a new `Repository` with a given event log (may be `vec![]`)
  pub fn new(event_log: Vec<typings::Event<T>>) -> Repository<T> {
    Repository {
      projector: core::Projector::new(event_log),
    }
  }

  /// Creates a new Entity
  pub fn create(&mut self, entity: T) {
    self.projector.add_event(typings::Event::<T> {
      date: time::Instant::now(),
      operation: CRUD::Create,
      data: entity,
    });
  }

  /// Mutates an existing Entity
  pub fn update(&mut self, entity: T) {
    self.projector.add_event(typings::Event::<T> {
      date: time::Instant::now(),
      operation: CRUD::Update,
      data: entity,
    });
  }

  /// Deletes an Entity
  pub fn delete(&mut self, entity: T) {
    self.projector.add_event(typings::Event::<T> {
      date: time::Instant::now(),
      operation: CRUD::Delete,
      data: entity,
    });
  }

  /// Returns the latest projection
  pub fn get_projection(&self) -> &Vec<T> {
    &self.projector.projection
  }

  /// Generates a new projection at a specified moment in time
  pub fn project_at(&self, at: &time::Instant) -> Vec<T> {
    self.projector.project_at(&at)
  }

  /// Persists the event log using a given persistance strategy
  pub fn persist_data<R>(&mut self, persist_fn: fn(event_log: &Vec<typings::Event<T>>) -> R) -> R {
    persist_fn(&self.projector.event_log)
  }
}
