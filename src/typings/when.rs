use std::time;

pub enum At {
  Latest,
  Date(time::Instant),
}
