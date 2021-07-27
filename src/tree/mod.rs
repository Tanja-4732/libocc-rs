/*!
This is the newer implementation of the `libocc` versioning system.

Its tree data structure is generally preferred to the initial implementation
using projectors and event logs.
*/

mod node;

pub use node::Node;
