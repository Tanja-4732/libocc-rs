use super::person::Person;

#[derive(Debug, Clone)]
pub struct Book {
  pub uuid: String,

  // #[property]
  pub some_number: usize,

  // #[property]
  pub another_number: usize,

  // #[property]
  pub author: Person,

  // TODO implement some kind of field hider
  // // Does not have #[property], so it should not be included
  pub hidden_property: String,
}

impl PartialEq for Book {
  fn eq(&self, other: &Self) -> bool {
    self.uuid == other.uuid
  }
}
