use crate::typings;
use std::time;

pub struct Event<T: Clone + PartialEq> {
  pub date: time::Instant,
  pub operation: typings::CRUD,
  pub data: T,
}
