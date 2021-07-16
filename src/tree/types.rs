use super::node::NodeGetHelper;
use crate::MAX_ELEMS;

use smallvec::SmallVec;

// TODO: centralize type definitions here!
// TODO: remove OptNode type all together?

/// TODO: documentation
pub type SortMetaVec = SmallVec<[NodeGetHelper; MAX_ELEMS]>;

/// TODO: documentation
pub type SortSwapVec = SmallVec<[(usize, usize); MAX_ELEMS]>;