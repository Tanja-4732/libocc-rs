# libocc-rs

[![dependency status](https://deps.rs/crate/libocc/0.4.0/status.svg)](https://deps.rs/crate/libocc/0.4.0)

This library aims to provide a simple interface for developing event-sourced occasionally-connected-computing experiences.

A port of [libocc-ts](https://github.com/Bernd-L/libocc-ts) (the TypeScript version of libocc)

See the docs at <https://docs.rs/libocc/>

See the [TODO section](#todo) below.

## Example

```Rust
fn test_projector() {
    // Create a new book
    let mut my_book = book::Book {
        uuid: Uuid::new_v4(),
        some_number: 42,
        author: person::Person {
            uuid: Uuid::new_v4(),
            first_name: String::from("Alex"),
            last_name: String::from("Example"),
        },
    };

    // Create a new projector of type `Book`
    let mut books = crate::Projector::<book::Book>::new();

    // So far, the projector is empty.
    println!("Empty projector:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().len(), 0);

    // Add a new book
    books.push(crate::Event::create(my_book.clone())).unwrap();

    // The projector now contains the new book in its initial state
    println!("Projector after creating new book:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().get(0).unwrap().some_number, 42);

    // This timestamp will be used in the future to get a previous state of the book
    let timestamp: crate::Timestamp = Utc::now();

    // Some time later ... (simulated delay)
    thread::sleep(time::Duration::from_millis(1));

    // Modify the book and save it in the projector
    my_book.some_number = 123;
    books.push(crate::Event::update(my_book.clone())).unwrap();

    // The projector now contains the new version of the book
    println!("Projector after updating the book:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().get(0).unwrap().some_number, 123);

    // We can still retrieve the old version of the book (using the timestamp)
    println!("Projector before the book was updated:");
    println!("{:?}\n", books.project_at(&timestamp));
    assert_eq!(
        books
            .project_at(&timestamp)
            .unwrap()
            .get(0)
            .unwrap()
            .some_number,
        42
    );
}
```

## TODO

- Data model
  - [ ] Implement self-describing hashes
    - Probably use multiformats
- [ ] Implement some kind of sync-server
  - [ ] Decide on how to handle persistency
    - Maybe use a SQL database (PostgreSQL)
    - Maybe use SQLite
    - Maybe use a Rust-native storage format
  - [ ] Implement communication
    - [ ] RESTful API over HTTP
- Future stuff
  - Custom data model (JSON alternative)
    - [ ] Implement a Serde serializer
    - [ ] Implement a Serde deserializer
  - [ ] Maybe some WebSocket stuff?
  - [ ] Persistance using a Git repository
  - [ ] Support incremental updates

## Licence & Copyright

Copyright (c) 2020-2021 Bernd-L. All rights reserved.

![AGPL v3: Free as in Freedom](https://www.gnu.org/graphics/agplv3-with-text-162x68.png)

libocc-rs is free software: you can redistribute it and/or modify it under the terms of the [GNU Affero General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

libocc-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Affero General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Affero General Public License](/LICENSE.md) along with libocc-rs. If not, see <https://www.gnu.org/licenses/>.

This project (including its source code and its documentation) is released under the terms of the [GNU Affero General Public License](/LICENSE.md).
