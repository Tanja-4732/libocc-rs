mod book;
mod person;
use chrono::Utc;
use std::{borrow::Cow, thread, time};
use uuid::Uuid;

#[test]
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
    let mut books = crate::events::Projector::<book::Book>::new();

    // So far, the projector is empty.
    println!("Empty projector:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().len(), 0);

    // Add a new book
    books
        .push(crate::events::Event::create(Cow::Owned(my_book.clone())))
        .unwrap();

    // The projector now contains the new book in its initial state
    println!("Projector after creating new book:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().get(0).unwrap().some_number, 42);

    // This timestamp will be used in the future to get a previous state of the book
    let timestamp: crate::events::Timestamp = Utc::now();

    // Some time later ... (simulated delay)
    thread::sleep(time::Duration::from_millis(1));

    // Modify the book and save it in the projector
    my_book.some_number = 123;
    books
        .push(crate::events::Event::update(Cow::Borrowed(&my_book)))
        .unwrap();

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

#[test]
fn test_snapshots() {
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
    let mut books = crate::events::Projector::<book::Book>::new();

    // So far, the projector is empty.
    println!("Empty projector:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().len(), 0);

    // Add a new book
    books
        .push(crate::events::Event::create(Cow::Owned(my_book.clone())))
        .unwrap();

    // The projector now contains the new book in its initial state
    println!("Projector after creating new book:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().get(0).unwrap().some_number, 42);

    // This timestamp will be used in the future to get a previous state of the book
    let timestamp: crate::events::Timestamp = Utc::now();

    // Some time later ... (simulated delay)
    thread::sleep(time::Duration::from_millis(1));

    // Make a new snapshot
    books.make_snapshot();

    // Modify the book and save it in the projector
    my_book.some_number = 123;
    books
        .push(crate::events::Event::update(Cow::Owned(my_book.clone())))
        .unwrap();

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

#[test]
fn test_many_change_after_snapshots() {
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
    let mut books = crate::events::Projector::<book::Book>::new();

    // So far, the projector is empty.
    println!("Empty projector:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().len(), 0);

    // Add a new book
    books
        .push(crate::events::Event::create(Cow::Owned(my_book.clone())))
        .unwrap();

    // The projector now contains the new book in its initial state
    println!("Projector after creating new book:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().get(0).unwrap().some_number, 42);

    // This timestamp will be used in the future to get a previous state of the book
    let timestamp: crate::events::Timestamp = Utc::now();

    // Some time later ... (simulated delay)
    thread::sleep(time::Duration::from_millis(1));

    // Make a new snapshot
    books.make_snapshot();

    // Modify the book and save it in the projector
    my_book.some_number = 123;
    books
        .push(crate::events::Event::update(Cow::Owned(my_book.clone())))
        .unwrap();

    // This timestamp will be used in the future to get a previous state of the book
    let timestamp_2: crate::events::Timestamp = Utc::now();

    // Some time later ... (simulated delay)
    thread::sleep(time::Duration::from_millis(1));

    // Some time later ... (simulated delay)
    thread::sleep(time::Duration::from_millis(1));

    // Make a new snapshot
    books.make_snapshot(); // Some time later ... (simulated delay)
    thread::sleep(time::Duration::from_millis(1));

    // Make a new snapshot
    books.make_snapshot();

    // Modify the book and save it in the projector
    my_book.some_number = 321;
    books
        .push(crate::events::Event::update(Cow::Owned(my_book.clone())))
        .unwrap();

    // The projector now contains the new version of the book
    println!("Projector after updating the book:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().get(0).unwrap().some_number, 321);

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

    // We can still retrieve the second version of the book (using the timestamp)
    println!("Projector before the book was updated again:");
    println!("{:?}\n", books.project_at(&timestamp));
    assert_eq!(
        books
            .project_at(&timestamp_2)
            .unwrap()
            .get(0)
            .unwrap()
            .some_number,
        123
    );
    // The projector now contains the new version of the book
    println!("Projector after updating the book:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().get(0).unwrap().some_number, 321);

    //
    //
    //
    // And again! (and again...)
    //
    //

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

    // We can still retrieve the second version of the book (using the timestamp)
    println!("Projector before the book was updated again:");
    println!("{:?}\n", books.project_at(&timestamp));
    assert_eq!(
        books
            .project_at(&timestamp_2)
            .unwrap()
            .get(0)
            .unwrap()
            .some_number,
        123
    );
    // The projector now contains the new version of the book
    println!("Projector after updating the book:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().get(0).unwrap().some_number, 321);

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

    // We can still retrieve the second version of the book (using the timestamp)
    println!("Projector before the book was updated again:");
    println!("{:?}\n", books.project_at(&timestamp));
    assert_eq!(
        books
            .project_at(&timestamp_2)
            .unwrap()
            .get(0)
            .unwrap()
            .some_number,
        123
    );
    // The projector now contains the new version of the book
    println!("Projector after updating the book:");
    println!("{:?}\n", books.get_projection());
    assert_eq!(books.get_projection().get(0).unwrap().some_number, 321);

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

    // We can still retrieve the second version of the book (using the timestamp)
    println!("Projector before the book was updated again:");
    println!("{:?}\n", books.project_at(&timestamp));
    assert_eq!(
        books
            .project_at(&timestamp_2)
            .unwrap()
            .get(0)
            .unwrap()
            .some_number,
        123
    );
}
