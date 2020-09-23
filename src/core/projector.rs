use crate::typings::{self, At};

struct Projector<T> {
  pub event_log: Vec<typings::Event<T>>,
}

impl<T> Projector<T> {
  pub fn new(event_log: Vec<typings::Event<T>>) -> Projector<T> {
    Projector { event_log }
  }

  pub fn project(&self, at: typings::At) -> Vec<T> {
    match at {
      At::Latest => (),
      At::Date(d) => (),
    }

    vec![]
  }
}
