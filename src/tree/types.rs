use super::node::{Node, NodeGetHelper};
use crate::MAX_ELEMS;

use smallvec::SmallVec;

// TODO: centralize SmallVec usage here!

// Arena Internals -----------------------------------------------------------------------------------------------------

pub type ArenaVec<K, V> = SmallVec<[Option<Node<K, V>>; MAX_ELEMS]>;

// Sorting -------------------------------------------------------------------------------------------------------------

/// Working set of arena indexes
pub type IdxVec = SmallVec<[usize; MAX_ELEMS]>;

/// Metadata for sorting the arena
pub type SortMetaVec = SmallVec<[NodeGetHelper; MAX_ELEMS]>;

/// Map of a sort's swap history
pub type SortSwapVec = SmallVec<[(usize, usize); MAX_ELEMS]>;
