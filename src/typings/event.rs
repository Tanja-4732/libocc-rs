use crate::typings;
use std::time;

pub struct Event<T> {
  pub date: time::Instant,
  pub operation: typings::CRUD,
  pub data: T,
}
