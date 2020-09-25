# libocc-rs

This library aims to provide a simple interface for developing event-sourced occasionally-connected-computing experiences.

A port of [libocc-ts](https://github.com/Bernd-L/libocc-ts) (the TypeScript version of libocc)

See the docs at <https://docs.rs/libocc/>

## Example

```Rust
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
  assert_eq!(books.get_projection().len(), 0);

  // Add a new book
  books.create(my_book.clone());

  println!("Repository after creating new book:");
  println!("{:?}\n", books.get_projection());
  assert_eq!(books.get_projection().get(0).unwrap().another_number, 234);

  let old_date = time::Instant::now();

  // Simulated delay
  thread::sleep(time::Duration::from_secs(1));

  // Modify the book
  my_book.another_number = 42;
  books.update(my_book.clone());

  println!("Repository after updating the book:");
  println!("{:?}\n", books.get_projection());
  assert_eq!(books.get_projection().get(0).unwrap().another_number, 42);

  println!("Repository before the book was updated:");
  println!("{:?}\n", books.project_at(&old_date));
  assert_eq!(
    books.project_at(&old_date).get(0).unwrap().another_number,
    234
  );
}
```
