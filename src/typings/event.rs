use crate::typings;
use std::time;

pub struct Event<T> {
  date: time::Instant,
  operation: typings::CRUD,
  data: T,
}
