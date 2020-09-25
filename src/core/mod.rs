//! This is the core module, containing both the  [`Repository`] and the [`Projector`] struct.
//!
//! You can use the [`Repository`] struct to use a nice abstraction for the underlying event logs,
//! or use the [`Projector`] struct to access the lower-level operations of event projection.
//!
//! [`Repository`]: repository/struct.Repository.html
//! [`Projector`]: projector/struct.Projector.html

mod projector;
mod repository;

pub use projector::*;
pub use repository::*;
