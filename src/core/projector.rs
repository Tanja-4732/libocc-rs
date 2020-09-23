use crate::typings::{self, At};

struct Projector<T> {
  pub event_log: Vec<typings::Event<T>>,
  projection: Vec<T>,
}

impl<T> Projector<T> {
  pub fn new(event_log: Vec<typings::Event<T>>) -> Projector<T> {
    let ret = Projector {
      event_log,
      projection: vec![],
    };

    Self::parse(&ret.event_log);

    ret
  }

  /// Applies an event to the projection and adds
  /// it to the event log of this instance.
  pub fn add_event(&mut self, event: typings::Event<T>) {
    self.event_log.push(event);
    Self::parse_one(&mut self.projection, &event);
  }

  /// Projects an event log onto a returned list of entities
  ///
  /// The `at` parameter specifies until when the event log should be
  /// projected from. Using `Latest` will return the cached projection instead.
  pub fn project(&self, at: typings::At) -> &Vec<T> {
    match at {
      At::Latest => &self.projection,
      At::Date(d) => vec![],
    }
  }

  fn parse(event_log: &Vec<typings::Event<T>>) -> Vec<T> {
    let list = vec![];

    // TODO parse the event log

    list
  }

  fn parse_one(list: &mut Vec<T>, event: &typings::Event<T>) {}
}
