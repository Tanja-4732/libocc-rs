# libocc-rs

This library aims to provide a simple interface for developing event-sourced occasionally-connected-computing experiences.

A port of [libocc-ts](https://github.com/Bernd-L/libocc-ts) (the TypeScript version of libocc)

See the docs at <https://docs.rs/libocc/>

See the [TODO section](#todo) below.

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

## TODO

- [ ] Data model
  - [ ] Select a serialization format suitable for both Rust and TypeScript data
    - Maybe JSON?
    - Or:
      - [ ] Implement a Serde serializer
      - [ ] Implement a Serde deserializer
  - [ ] Decide on how to replay events
    - Partial structs (incremental updates)
    - Whole-struct versioning (replacing updates)
  - [ ] Implement self-describing hashes
    - Probably use multiformats
- [ ] Implement some kind of sync-server
  - [ ] Decide on how to handle persistency
    - Maybe use a SQL database (PostgreSQL)
    - Maybe use SQLite
    - Maybe use a Rust-native storage format
    - ~~Maybe use a Git repository~~
  - [ ] Implement communication
    - [ ] RESTful API over HTTP
    - [ ] Maybe some WebSocket stuff?

## Licence & Copyright

Copyright (c) 2020-2021 Bernd-L. All rights reserved.

![AGPL v3: Free as in Freedom](https://www.gnu.org/graphics/agplv3-with-text-162x68.png)

libocc-rs is free software: you can redistribute it and/or modify it under the terms of the [GNU Affero General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

libocc-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Affero General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Affero General Public License](/LICENSE.md) along with libocc-rs. If not, see <https://www.gnu.org/licenses/>.

This project (including its source code and its documentation) is released under the terms of the [GNU Affero General Public License](/LICENSE.md).
