use super::node::{Node, NodeGetHelper, NodeRebuildHelper};
use crate::MAX_ELEMS;

use smallvec::{IntoIter, SmallVec};
use smallnum::small_unsigned;

// Index Variable ------------------------------------------------------------------------------------------------------

#[cfg(not(feature = "high_assurance"))]
pub type Idx = small_unsigned!(usize::MAX);

#[cfg(feature = "high_assurance")]
pub type Idx = small_unsigned!(MAX_ELEMS);

// Arena Internals -----------------------------------------------------------------------------------------------------

pub type ArenaVec<K, V> = SmallVec<[Option<Node<K, V>>; MAX_ELEMS]>;

// Sorting Internals ---------------------------------------------------------------------------------------------------

/// Working set of arena indexes
pub type IdxVec = SmallVec<[Idx; MAX_ELEMS]>;

/// Metadata for sorting the arena
pub type SortMetaVec = SmallVec<[NodeGetHelper; MAX_ELEMS]>;

/// Map of a sort's swap history
pub type SortSwapVec = SmallVec<[(Idx, Idx); MAX_ELEMS]>;

// List of node references
pub type SortNodeRefVec<'a, K, V> = SmallVec<[&'a Node<K, V>; MAX_ELEMS]>;

// List of (node reference, index) pairs
pub type SortNodeRefIdxPairVec<'a, K, V> = SmallVec<[(&'a Node<K, V>, Idx); MAX_ELEMS]>;

// List of (index, node rebuild helper) pairs
pub type RebuildMetaVec = SmallVec<[(Idx, NodeRebuildHelper); MAX_ELEMS]>;

// Set -----------------------------------------------------------------------------------------------------------------

// List of element references
pub type ElemRefVec<'a, T> = SmallVec<[&'a T; MAX_ELEMS]>;

// Iterator for element references
pub type ElemRefIter<'a, T> = IntoIter<[&'a T; MAX_ELEMS]>;
