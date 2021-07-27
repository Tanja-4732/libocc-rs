/*!
This is the initial implementation of the `libocc` versioning system.

While the new implementation stores its data in a tree structure, this one
uses event logs and projectors to project the history of events.

It's generally recommended to use the newer implementation instead of this one.
*/

mod core;

pub use crate::events::core::*;
