use crate::core;
use crate::typings::{self, CRUD};
use std::time;

pub struct Repository<T: Clone + PartialEq> {
  projector: core::Projector<T>,
}

impl<T: Clone + PartialEq> Repository<T> {
  pub fn new(event_log: Vec<typings::Event<T>>) -> Repository<T> {
    Repository {
      projector: core::Projector::new(event_log),
    }
  }

  pub fn create(&mut self, entity: T) {
    self.projector.add_event(typings::Event::<T> {
      date: time::Instant::now(),
      operation: CRUD::Create,
      data: entity,
    });
  }

  pub fn update(&mut self, entity: T) {
    self.projector.add_event(typings::Event::<T> {
      date: time::Instant::now(),
      operation: CRUD::Update,
      data: entity,
    });
  }

  pub fn delete(&mut self, entity: T) {
    self.projector.add_event(typings::Event::<T> {
      date: time::Instant::now(),
      operation: CRUD::Delete,
      data: entity,
    });
  }

  pub fn get_projection(&self) -> &Vec<T> {
    &self.projector.projection
  }

  pub fn project_at(&self, at: time::Instant) -> Vec<T> {
    self.projector.project_at(at)
  }

  pub fn persist_data<R>(&mut self, persist_fn: fn(event_log: &Vec<typings::Event<T>>) -> R) -> R {
    persist_fn(&self.projector.event_log)
  }
}
