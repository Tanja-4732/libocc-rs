#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! This crate offers versioned CRUD operations for entities.
//!
//! Any entity is modified using events. Every event is either of type create, update or delete.
//!
//! You can use the [`Repository`] struct to use a nice abstraction for the underlying event logs,
//! or use the [`Projector`] struct to access the lower-level operations of event projection.
//!
//! [`Repository`]: core/repository/struct.Repository.html
//! [`Projector`]: core/projector/struct.Projector.html

#[cfg(test)]
mod test;

pub mod core;
pub mod typings;
