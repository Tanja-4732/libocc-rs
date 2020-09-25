use super::person::Person;

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
  hidden_property: String,
}
