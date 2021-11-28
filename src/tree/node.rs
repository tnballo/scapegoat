use smallvec::SmallVec;
use smallnum::SmallUnsigned;

// Tree Node -----------------------------------------------------------------------------------------------------------

/// Binary tree node.
#[derive(Clone)]
pub struct Node<K, V, I> {
    pub key: K,
    pub val: V,
    pub left_idx: Option<I>,
    pub right_idx: Option<I>,

    #[cfg(feature = "fast_rebalance")]
    pub subtree_size: I,
}

impl<K, V, I> Node<K, V, I> {
    /// Constructor.
    pub fn new(key: K, val: V) -> Self {
        Node {
            key,
            val,
            left_idx: None,
            right_idx: None,

            #[cfg(feature = "fast_rebalance")]
            subtree_size: 1,
        }
    }
}

// Retrieval Helper ----------------------------------------------------------------------------------------------------

/// Helper for node retrieval, usage eliminates the need a store parent pointer in each node.
pub struct NodeGetHelper<I> {
    pub node_idx: Option<I>,
    pub parent_idx: Option<I>,
    pub is_right_child: bool,
}

impl<I> NodeGetHelper<I> {
    /// Constructor.
    pub fn new(node_idx: Option<I>, parent_idx: Option<I>, is_right_child: bool) -> Self {
        NodeGetHelper {
            node_idx,
            parent_idx,
            is_right_child,
        }
    }
}

// Tree Rebuild Helper -------------------------------------------------------------------------------------------------

/// Helper for in-place iterative rebuild.
pub struct NodeRebuildHelper<I> {
    pub low_idx: I,
    pub high_idx: I,
    pub mid_idx: I,
}

impl<I: SmallUnsigned + Ord> NodeRebuildHelper<I> {
    /// Constructor.
    pub fn new(low_idx: I, high_idx: I) -> Self {
        debug_assert!(
            high_idx >= low_idx,
            "Node rebuild helper low/high index reversed!"
        );
        NodeRebuildHelper {
            low_idx,
            high_idx,
            mid_idx: low_idx + ((high_idx - low_idx) / 2),
        }
    }
}

// Swap History Cache --------------------------------------------------------------------------------------------------

/// A helper "cache" for swap operation history.
/// If every index swap is logged, tracks mapping of original to current indexes.
pub struct NodeSwapHistHelper<I, const C: usize> {
    /// Map `original_idx` -> `current_idx`
    history: SmallVec<[(I, I); C]>,
}

impl<I: Ord, const C: usize> NodeSwapHistHelper<I, C> {
    /// Constructor.
    pub fn new() -> Self {
        NodeSwapHistHelper {
            history: SmallVec::<[(I, I); C]>::new(),
        }
    }

    /// Log the swap of elements at two indexes.
    /// Every swap performed must be logged with this method for the cache to remain accurate.
    pub fn add(&mut self, pos_1: I, pos_2: I) {
        debug_assert_ne!(pos_1, pos_2);

        let mut known_pos_1 = false;
        let mut known_pos_2 = false;

        // Update existing
        for (_, curr_idx) in self.history.iter_mut() {
            if *curr_idx == pos_1 {
                *curr_idx = pos_2;
                known_pos_1 = true;
            } else if *curr_idx == pos_2 {
                *curr_idx = pos_1;
                known_pos_2 = true;
            }
        }

        // Add new mapping
        if !known_pos_1 {
            self.history.push((pos_1, pos_2));
        }

        // Add new mapping
        if !known_pos_2 {
            self.history.push((pos_2, pos_1));
        }
    }

    /// Retrieve the current value of an original index from the map.
    pub fn curr_idx(&self, orig_pos: I) -> I {
        debug_assert!(self.history.iter().filter(|(k, _)| *k == orig_pos).count() <= 1);

        match self
            .history
            .iter()
            .filter(|(k, _)| *k == orig_pos)
            .map(|(_, curr)| *curr)
            .next()
        {
            Some(curr_idx) => curr_idx,
            None => orig_pos,
        }
    }
}

// Note: low_mem_insert feature doesn't affect node size, only arena size.
#[cfg(not(feature = "low_mem_insert"))]
#[cfg(test)]
mod tests {
    use super::Node;
    use std::mem::size_of;
    use smallnum::small_unsigned;

    #[cfg(feature = "high_assurance")]
    use crate::MAX_ELEMS;

    #[test]
    fn test_node_packing() {
        // No features
        #[cfg(target_pointer_width = "64")]
        #[cfg(not(feature = "high_assurance"))]
        #[cfg(not(feature = "fast_rebalance"))]
        {
            assert_eq!(size_of::<Node<u32, u32, small_unsigned!(1024)>>(), 40);
        }

        // All features
        #[cfg(target_pointer_width = "64")]
        #[cfg(feature = "high_assurance")]
        #[cfg(feature = "fast_rebalance")]
        {
            // Assumes `SG_MAX_STACK_ELEMS == 1024` (default)
            if MAX_ELEMS < u16::MAX.into() {
                assert_eq!(size_of::<Node<u32, u32, small_unsigned!(1024)>>(), 20);
            }
        }

        // fast_rebalance only
        #[cfg(target_pointer_width = "64")]
        #[cfg(not(feature = "high_assurance"))]
        #[cfg(feature = "fast_rebalance")]
        {
            assert_eq!(size_of::<Node<u32, u32, small_unsigned!(1024)>>(), 48);
        }

        // high_assurance only
        #[cfg(target_pointer_width = "64")]
        #[cfg(feature = "high_assurance")]
        #[cfg(not(feature = "fast_rebalance"))]
        {
            // Assumes `SG_MAX_STACK_ELEMS == 1024` (default)
            if MAX_ELEMS < u16::MAX.into() {
                assert_eq!(size_of::<Node<u32, u32, small_unsigned!(1024)>>(), 16);
            }
        }
    }
}
