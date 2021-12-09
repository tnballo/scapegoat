// TODO: remove this!
mod types;
pub(crate) use types::{ElemRefIter, ElemRefVec};

mod node_dispatch;
mod arena_dispatch;

#[cfg(test)]
mod test;

mod arena;
#[cfg(fuzzing)]
pub use arena::Arena;

mod node;
#[cfg(fuzzing)]
pub use node::{Node, NodeGetHelper, NodeRebuildHelper};

mod iter;
pub use iter::{IntoIter, Iter, IterMut};

mod error;
pub use error::SGErr;

#[allow(clippy::module_inception)]
mod tree;
pub use tree::SGTree;
