#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! This crate offers versioned CRUD operations for entities.
//!
//! Any entity is modified using events. Every event is either of type create, update or delete.
//!
//! You can use the [`Repository`] struct to use a nice abstraction for the underlying event logs,
//! or use the [`Projector`] struct to access the lower-level operations of event projection.
//!
//!
//!
//!
//! ```
//!
//! # #[derive(Debug, PartialEq, Clone)]
//! # struct Person {
//! #   // #[property]
//! #   pub first_name: String,
//! #
//! #   // #[property]
//! #   pub last_name: String,
//! # }
//! #
//! #
//! #
//! # #[derive(Debug, Clone)]
//! #  struct Book {
//! #   pub uuid: String,
//! #
//! #   // #[property]
//! #   pub some_number: usize,
//! #
//! #   // #[property]
//! #   pub another_number: usize,
//! #
//! #   // #[property]
//! #   pub author: Person,
//! #
//! #   // TODO implement some kind of field hider
//! #   // // Does not have #[property], so it should not be included
//! #   pub hidden_property: String,
//! # }
//! #
//! # impl PartialEq for Book {
//! #   fn eq(&self, other: &Self) -> bool {
//! #     self.uuid == other.uuid
//! #   }
//! # }
//! #
//! #
//! # use std::{thread, time};
//! use libocc::core;
//!
//! fn test_repo() {
//!   // Create a new book
//!   let mut my_book = Book {
//!     uuid: String::from("some-uuid"),
//!
//!     some_number: 1234,
//!
//!     another_number: 234,
//!
//!     author: Person {
//!       first_name: String::from(""),
//!       last_name: String::from(""),
//!     },
//!
//!     hidden_property: String::from("sneaky af"),
//!   };
//!
//!   // Create a new projector of type `Book`
//!   let mut books = core::Repository::<Book>::new(vec![]);
//!
//!   println!("Empty repository:");
//!   println!("{:?}\n", books.get_projection());
//!   assert_eq!(books.get_projection().len(), 0);
//!
//!   // Add a new book
//!   books.create(my_book.clone());
//!
//!   println!("Repository after creating new book:");
//!   println!("{:?}\n", books.get_projection());
//!   assert_eq!(books.get_projection().get(0).unwrap().another_number, 234);
//!
//!   let old_date = time::Instant::now();
//!
//!   // Simulated delay
//!   thread::sleep(time::Duration::from_secs(1));
//!
//!   // Modify the book
//!   my_book.another_number = 42;
//!   books.update(my_book.clone());
//!
//!   println!("Repository after updating the book:");
//!   println!("{:?}\n", books.get_projection());
//!   assert_eq!(books.get_projection().get(0).unwrap().another_number, 42);
//!
//!   println!("Repository before the book was updated:");
//!   println!("{:?}\n", books.project_at(&old_date));
//!   assert_eq!(
//!     books.project_at(&old_date).get(0).unwrap().another_number,
//!     234
//!   );
//! }
//!
//! ```
//!
//!
//!
//! [`Repository`]: core/repository/struct.Repository.html
//! [`Projector`]: core/projector/struct.Projector.html

#[cfg(test)]
mod test;

pub mod core;
pub mod typings;
