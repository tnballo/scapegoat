use core::ops::{Sub, Div};

use smallvec::SmallVec;
use smallnum::SmallUnsigned;

// Note: structures in this file generic for `I` in a *subset* of the set `(u8, u16, u32, u64, u128)`.
// All members in subset are <= host pointer width in size.

// Tree Node -----------------------------------------------------------------------------------------------------------

/// Binary tree node.
/// Users of it's APIs only need to declare `I` type or trait bounds at construction.
/// All APIs take/return `usize` and normalize to `I` internally.
#[derive(Clone)]
pub struct Node<K, V, I> {
    pub key: K,
    pub val: V,
    left_idx: Option<I>,
    right_idx: Option<I>,

    #[cfg(feature = "fast_rebalance")]
    pub subtree_size: I,
}

impl<K, V, I: SmallUnsigned> Node<K, V, I> {
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

    /// Get left index as usize
    pub fn left_idx(&self) -> Option<usize> {
        self.left_idx.map(|i| i.usize())
    }

    /// Set left index
    pub fn set_left_idx(&mut self, opt_idx: Option<usize>) {
        match opt_idx {
            Some(idx) => self.left_idx = Some(I::checked_from(idx)),
            None => self.left_idx = None
        }
    }

    /// Get right index as usize
    pub fn right_idx(&self) -> Option<usize> {
        self.right_idx.map(|i| i.usize())
    }

    /// Set right index
    pub fn set_right_idx(&mut self, opt_idx: Option<usize>) {
        match opt_idx {
            Some(idx) => self.right_idx = Some(I::checked_from(idx)),
            None => self.right_idx = None
        }
    }
}

// Retrieval Helper ----------------------------------------------------------------------------------------------------

/// Helper for node retrieval, usage eliminates the need a store parent pointer in each node.
/// Users of it's APIs only need to declare `I` type or trait bounds at construction.
/// All APIs take/return `usize` and normalize to `I` internally.
pub struct NodeGetHelper<I> {
    node_idx: Option<I>,
    parent_idx: Option<I>,
    is_right_child: bool,
}

impl<I: SmallUnsigned> NodeGetHelper<I> {
    /// Constructor.
    pub fn new(node_idx: Option<usize>, parent_idx: Option<usize>, is_right_child: bool) -> Self {
        NodeGetHelper {
            node_idx: node_idx.map(|i| I::checked_from(i)),
            parent_idx: parent_idx.map(|i| I::checked_from(i)),
            is_right_child,
        }
    }

    /// Get node index as usize
    pub fn node_idx(&self) -> Option<usize> {
        self.node_idx.map(|i| i.usize())
    }

    /// Get parent index as usize
    pub fn parent_idx(&self) -> Option<usize> {
        self.node_idx.map(|i| i.usize())
    }

    // Tell if right or left child
    pub const fn is_right_child(&self) -> bool {
        self.is_right_child
    }
}

// TODO: impl a To<Node<K,V>> so tree's public APIs can hide the `I` by promoting to a `usize` for external consumption.

// Tree Rebuild Helper -------------------------------------------------------------------------------------------------

/// Helper for in-place iterative rebuild.
/// Users of it's APIs only need to declare `I` type or trait bounds at construction.
/// All APIs take/return `usize` and normalize to `I` internally.
pub struct NodeRebuildHelper<I> {
    pub low_idx: I,
    pub high_idx: I,
    pub mid_idx: I,
}

/*
TODO: this doesnt' work
struct NrhI(NodeRebuildHelper::I);

impl Div<<NrhI as core::ops::Sub>::Output> for NrhI
where
    NrhI: Div
{
    type Output = Self;

    fn div(self, rhs: <NrhI as core::ops::Sub>::Output) -> Self::Output {
        Self {
            self / (rhs as NrhI)
        }
    }
}
*/

impl<I: SmallUnsigned + Ord + Sub + Div> NodeRebuildHelper<I> {

    /// Constructor.
    pub fn new(low_idx: usize, high_idx: usize) -> Self {
        debug_assert!(
            high_idx >= low_idx,
            "Node rebuild helper low/high index reversed!"
        );

        let low_idx = I::checked_from(low_idx);
        let high_idx = I::checked_from(high_idx);

        NodeRebuildHelper {
            low_idx,
            high_idx,
            mid_idx: low_idx + ((high_idx - low_idx) / I::checked_from(2)),
        }
    }
}

// Swap History Cache --------------------------------------------------------------------------------------------------

/// A helper "cache" for swap operation history.
/// If every index swap is logged, tracks mapping of original to current indexes.
/// Users of it's APIs only need to declare `I` type or trait bounds at construction.
/// All APIs take/return `usize` and normalize to `I` internally.
pub struct NodeSwapHistHelper<I, const C: usize> {
    /// Map `original_idx` -> `current_idx`
    history: SmallVec<[(I, I); C]>,
}

impl<I: Ord + SmallUnsigned, const C: usize> NodeSwapHistHelper<I, C> {
    /// Constructor.
    pub fn new() -> Self {
        NodeSwapHistHelper {
            history: SmallVec::<[(I, I); C]>::new(),
        }
    }

    /// Log the swap of elements at two indexes.
    /// Every swap performed must be logged with this method for the cache to remain accurate.
    pub fn add(&mut self, pos_1: usize, pos_2: usize) {
        debug_assert_ne!(pos_1, pos_2);

        let mut known_pos_1 = false;
        let mut known_pos_2 = false;

        let pos_1 = I::checked_from(pos_1);
        let pos_2 = I::checked_from(pos_2);

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
    pub fn curr_idx(&self, orig_pos: usize) -> usize {
        debug_assert!(self.history.iter().filter(|(k, _)| (*k).usize() == orig_pos).count() <= 1);

        match self
            .history
            .iter()
            .filter(|(k, _)| (*k).usize() == orig_pos)
            .map(|(_, curr)| *curr)
            .next()
        {
            Some(curr_idx) => curr_idx.usize(),
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
