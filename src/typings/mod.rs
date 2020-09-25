//! This crate contains the [`CRUD`] enum, and the [`Event`] struct. They are used internally.
//!
//! Unless when working with the [`Projector`] directly, they won't be of any relevance to you,
//! as the [`Repository`] struct abstracts them away.
//!
//! [`Repository`]:../core/repository/struct.Repository.html
//! [`Projector`]: ../core/projector/struct.Projector.html
//! [`CRUD`]: enum.CRUD.html
//! [`Event`]: struct.Event.html

mod crud;
mod event;

pub use crud::*;
pub use event::*;
