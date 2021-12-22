mod node_dispatch;

#[cfg(test)]
mod test;

mod arena;
#[cfg(fuzzing)]
pub use arena::Arena;

pub(super) mod node;
#[cfg(fuzzing)]
pub use node::{Node, NodeGetHelper, NodeRebuildHelper};

mod iter;
pub use iter::{IntoIter, Iter, IterMut};

mod error;
pub use error::SgError;

#[allow(clippy::module_inception)]
mod tree;
pub use tree::{Idx, SgTree};
