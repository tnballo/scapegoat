use super::node::{Node, NodeGetHelper, NodeRebuildHelper};
use crate::MAX_ELEMS;

use smallvec::{IntoIter, SmallVec};

// Arena Internals -----------------------------------------------------------------------------------------------------

pub type ArenaVec<K, V> = SmallVec<[Option<Node<K, V>>; MAX_ELEMS]>;

// Sorting Internals ---------------------------------------------------------------------------------------------------

/// Working set of arena indexes
pub type IdxVec = SmallVec<[usize; MAX_ELEMS]>;

/// Metadata for sorting the arena
pub type SortMetaVec = SmallVec<[NodeGetHelper; MAX_ELEMS]>;

/// Map of a sort's swap history
pub type SortSwapVec = SmallVec<[(usize, usize); MAX_ELEMS]>;

// List of node references
pub type SortNodeRefVec<'a, K, V> = SmallVec<[&'a Node<K, V>; MAX_ELEMS]>;

// List of (node reference, index) pairs
pub type SortNodeRefIdxPairVec<'a, K, V> = SmallVec<[(&'a Node<K, V>, usize); MAX_ELEMS]>;

// List of (index, node rebuild helper) pairs
pub type RebuildMetaVec = SmallVec<[(usize, NodeRebuildHelper); MAX_ELEMS]>;

// Set -----------------------------------------------------------------------------------------------------------------

// List of element references
pub type ElemRefVec<'a, T> = SmallVec<[&'a T; MAX_ELEMS]>;

// Iterator for element references
pub type ElemRefIter<'a, T> = IntoIter<[&'a T; MAX_ELEMS]>;
