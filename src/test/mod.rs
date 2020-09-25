mod book;
mod person;

mod tests {
  use crate::{core, typings};

  use super::{book, person};

  use std::{thread, time};

  #[test]
  fn test_repo() {
    // Create a new book
    let mut my_book = book::Book {
      uuid: String::from("some-uuid"),

      some_number: 1234,

      another_number: 234,

      author: person::Person {
        first_name: String::from(""),
        last_name: String::from(""),
      },

      hidden_property: String::from("sneaky af"),
    };

    // Create a new projector of type `Book`
    let mut books = core::Repository::<book::Book>::new(vec![]);

    println!("Empty repository:");
    println!("{:?}\n", books.get_projection());

    // Add a new book
    books.create(my_book.clone());

    println!("Repository after creating new book:");
    println!("{:?}\n", books.get_projection());

    let old_date = time::Instant::now();

    // Simulated delay
    thread::sleep(time::Duration::from_secs(1));

    // Modify the book
    my_book.another_number = 42;
    books.update(my_book.clone());

    println!("Repository after updating the book:");
    println!("{:?}\n", books.get_projection());

    println!("Repository before the book was updated:");
    println!("{:?}\n", books.project_at(&old_date));
  }
}
