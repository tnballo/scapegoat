use super::types::{Idx, SortSwapVec};

// Tree Node -----------------------------------------------------------------------------------------------------------

/// Binary tree node.
#[derive(Clone)]
pub struct Node<K, V> {
    pub key: K,
    pub val: V,
    pub left_idx: Option<Idx>,
    pub right_idx: Option<Idx>,

    #[cfg(feature = "fast_rebalance")]
    pub subtree_size: Idx,
}

impl<K, V> Node<K, V> {
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
pub struct NodeGetHelper {
    pub node_idx: Option<Idx>,
    pub parent_idx: Option<Idx>,
    pub is_right_child: bool,
}

impl NodeGetHelper {
    /// Constructor.
    pub fn new(node_idx: Option<Idx>, parent_idx: Option<Idx>, is_right_child: bool) -> Self {
        NodeGetHelper {
            node_idx,
            parent_idx,
            is_right_child,
        }
    }
}

// Tree Rebuild Helper -------------------------------------------------------------------------------------------------

/// Helper for in-place iterative rebuild.
pub struct NodeRebuildHelper {
    pub low_idx: Idx,
    pub high_idx: Idx,
    pub mid_idx: Idx,
}

impl NodeRebuildHelper {
    /// Constructor.
    pub fn new(low_idx: Idx, high_idx: Idx) -> Self {
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
pub struct NodeSwapHistHelper {
    /// Map `original_idx` -> `current_idx`
    history: SortSwapVec,
}

impl NodeSwapHistHelper {
    /// Constructor.
    pub fn new() -> NodeSwapHistHelper {
        NodeSwapHistHelper {
            history: SortSwapVec::new(),
        }
    }

    /// Log the swap of elements at two indexes.
    /// Every swap performed must be logged with this method for the cache to remain accurate.
    pub fn add(&mut self, pos_1: Idx, pos_2: Idx) {
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
    pub fn curr_idx(&self, orig_pos: Idx) -> Idx {
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

    #[cfg(feature = "high_assurance")]
    use crate::MAX_ELEMS;

    #[test]
    fn test_node_packing() {
        // No features
        #[cfg(target_pointer_width = "64")]
        #[cfg(not(feature = "high_assurance"))]
        #[cfg(not(feature = "fast_rebalance"))]
        {
            assert_eq!(size_of::<Node<u32, u32>>(), 40);
        }

        // All features
        #[cfg(target_pointer_width = "64")]
        #[cfg(feature = "high_assurance")]
        #[cfg(feature = "fast_rebalance")]
        {
            // Assumes `SG_MAX_STACK_ELEMS == 1024` (default)
            if MAX_ELEMS < u16::MAX.into() {
                assert_eq!(size_of::<Node<u32, u32>>(), 20);
            }
        }

        // fast_rebalance only
        #[cfg(target_pointer_width = "64")]
        #[cfg(not(feature = "high_assurance"))]
        #[cfg(feature = "fast_rebalance")]
        {
            assert_eq!(size_of::<Node<u32, u32>>(), 48);
        }

        // high_assurance only
        #[cfg(target_pointer_width = "64")]
        #[cfg(feature = "high_assurance")]
        #[cfg(not(feature = "fast_rebalance"))]
        {
            // Assumes `SG_MAX_STACK_ELEMS == 1024` (default)
            if MAX_ELEMS < u16::MAX.into() {
                assert_eq!(size_of::<Node<u32, u32>>(), 16);
            }
        }
    }
}
