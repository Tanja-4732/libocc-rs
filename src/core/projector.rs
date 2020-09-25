use crate::typings::{self, CRUD};
use std::time;

/// The `Projector` struct uses a list of [`Event`s] to generate a projection of entities.
///
/// The projection is simply a `Vector` containing all entities of the type `T` at a specified moment in time.
///
/// You can also use the [`Repository`] struct to abstract the [`Event`s] away.
///
/// [`Event`s]: ../typings/event/struct.Event.html
/// [`Repository`]: repository/struct.Repository.html
pub struct Projector<T: Clone + PartialEq> {
  /// The ordered list of events used to generate the projection
  pub event_log: Vec<typings::Event<T>>,

  /// The resulting projection from the event log
  pub projection: Vec<T>,
}

impl<T: Clone + PartialEq> Projector<T> {
  /// Constructs a new `Projector` given an event log (may be `vec![]`)
  pub fn new(event_log: Vec<typings::Event<T>>) -> Projector<T> {
    Projector {
      projection: Self::project_all(&event_log),
      event_log,
    }
  }

  /// Applies an event to the projection and adds
  /// it to the event log of this instance.
  pub fn add_event(&mut self, event: typings::Event<T>) {
    Self::project_one(&mut self.projection, &event);
    self.event_log.push(event);
  }

  /// Projects an event log onto a returned list of entities
  pub fn project_at(&self, at: &time::Instant) -> Vec<T> {
    // Create a new list
    let mut list: Vec<T> = Vec::new();

    // Iterate over all events in teh event log until the target date
    // & project the event onto the list
    self
      .event_log
      .iter()
      .take_while(|event| &event.date <= at)
      .for_each(|event| Self::project_one(&mut list, &event));

    // Return the entity-list from the projected events
    list
  }

  fn project_all(event_log: &Vec<typings::Event<T>>) -> Vec<T> {
    // Create a new list
    let mut list: Vec<T> = Vec::new();

    // Iterate over all events in teh event log until the target date
    event_log
      .iter()
      .for_each(|event| Self::project_one(&mut list, &event));

    // Return the entity-list from the projected events
    list
  }

  fn project_one(list: &mut Vec<T>, event: &typings::Event<T>) {
    let maybe_i = list.iter().position(|el| el == &event.data);

    match event.operation {
      CRUD::Create => {
        if let Some(_) = maybe_i {
          // TODO return an error
        } else {
          list.push(event.data.clone());
        }
      }
      CRUD::Update => {
        if let Some(i) = maybe_i {
          list.splice(i..i, vec![event.data.clone()]);
        } else {
          // TODO return an error
        }
      }
      CRUD::Delete => {
        if let Some(i) = maybe_i {
          list.splice(i..i, vec![]);
        } else {
          // TODO return an error
        }
      }
    };
  }
}
