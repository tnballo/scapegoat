use core::slice::{Iter, IterMut};

use super::node::{Node, NodeGetHelper, NodeSwapHistHelper};

use smallnum::SmallUnsigned;
use smallvec::SmallVec;

// Note: structures in this file generic for `U` in a *subset* of the set `(u8, u16, u32, u64, u128)`.
// All members in subset are <= host pointer width in size.
// If caller obeys contract, `U` will be smallest unsigned capable of representing const `N` (e.g. static capacity).

/// An arena allocator, meta programmable for low memory footprint.
/// Users of it's APIs only need to declare `U` type or trait bounds at construction.
/// Method APIs take/return `usize` and normalize to `U` internally.
/// Sole associated function, `gen_idx_vec`, has return type that uses `U` - to a void duplicating `Vec` API here.
#[derive(Clone)]
pub struct NodeArena<K, V, U, const N: usize> {
    arena: SmallVec<[Option<Node<K, V, U>>; N]>,

    #[cfg(not(feature = "low_mem_insert"))]
    free_list: SmallVec<[U; N]>,
}

impl<K, V, U: Default + SmallUnsigned + Ord + PartialEq + PartialOrd, const N: usize>
    NodeArena<K, V, U, N>
{
    // Public API ------------------------------------------------------------------------------------------------------

    // TODO: get rid of this function? Bad encapsulation?
    // TODO: make const with tinyvec::ArrayVec::default()?
    /// Associated constructor for index scratch vector.
    pub const fn new_idx_vec() -> SmallVec<[U; N]> {
        SmallVec::<[U; N]>::default()
    }

    /// Constructor.
    pub fn new() -> Self {
        let na = NodeArena {
            arena: SmallVec::<[Option<Node<K, V, U>>; N]>::new(),

            #[cfg(not(feature = "low_mem_insert"))]
            free_list: SmallVec::<[U; N]>::new(),
        };

        // Verify const generic invariants
        debug_assert_eq!(N, na.free_list.len());
        debug_assert_eq!(N, na.arena.len());
        debug_assert_eq!(N, na.len());

        na
    }

    /// `#![no_std]`: total capacity, e.g. maximum number of items.
    /// Attempting to insert items beyond capacity will panic.
    ///
    /// If using `std`: fast capacity, e.g. number of map items stored on the stack.
    /// Items inserted beyond capacity will be stored on the heap.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns an iterator over immutable arena elements.
    pub fn iter(&self) -> Iter<'_, Option<Node<K, V, U>>> {
        self.arena.iter()
    }

    /// Returns an iterator over arena elements that allows modifying each value.
    pub fn iter_mut(&mut self) -> IterMut<'_, Option<Node<K, V, U>>> {
        self.arena.iter_mut()
    }

    /// Add node to area, growing if necessary, and return addition index.
    pub fn add(&mut self, node: Node<K, V, U>) -> usize {
        // O(1) find, constant time
        #[cfg(not(feature = "low_mem_insert"))]
        let opt_free_idx = self.free_list.pop();

        // O(n) find, linear search
        #[cfg(feature = "low_mem_insert")]
        let opt_free_idx = self.arena.iter().position(|x| x.is_none()).map(|i| i as U);

        match opt_free_idx {
            Some(free_idx) => {
                debug_assert!(
                    self.arena[free_idx.usize()].is_none(),
                    "Internal invariant failed: overwrite of allocated node!"
                );
                self.arena[free_idx.usize()] = Some(node);
                free_idx.usize()
            }
            None => {
                self.arena.push(Some(node));
                self.arena.len() - 1
            }
        }
    }

    /// Remove node at a given index from area, return it.
    pub fn remove(&mut self, idx: usize) -> Option<Node<K, V, U>> {
        debug_assert!(
            idx < self.arena.len(),
            "API misuse: requested removal past last index!"
        );
        if idx < self.arena.len() {
            // Move node to back, replacing with None, preserving order
            self.arena.push(None);
            let len = self.arena.len();
            self.arena.swap(idx, len - 1);

            // Append removed index to free list
            #[cfg(not(feature = "low_mem_insert"))]
            self.free_list.push(U::checked_from(idx));

            // Retrieve node
            return match self.arena.pop() {
                Some(opt_node) => match opt_node {
                    Some(node) => Some(node),
                    None => {
                        debug_assert!(
                            false,
                            "Internal invariant failed: removal popped an empty node!"
                        );
                        None
                    }
                },
                None => None,
            };
        }

        None
    }

    /// Sort the arena in caller-requested order and update all tree metadata accordingly
    /// `unwraps` will never panic if caller invariants upheld (checked via `debug_assert`)
    pub fn sort(
        &mut self,
        root_idx: usize,
        sort_metadata: SmallVec<[NodeGetHelper<U>; N]>,
    ) -> usize {
        debug_assert!(sort_metadata.iter().all(|ngh| ngh.node_idx().is_some()));

        let mut swap_history = NodeSwapHistHelper::<U, N>::new(); //  TODO: node shouldn't know N!

        // Sort as requested
        for (sorted_idx, ngh) in sort_metadata.iter().enumerate() {
            let curr_idx = swap_history.curr_idx(ngh.node_idx().unwrap());
            if curr_idx != sorted_idx {
                self.arena.swap(curr_idx, sorted_idx);
                swap_history.add(curr_idx, sorted_idx);

                #[cfg(not(feature = "low_mem_insert"))]
                self.free_list.retain(|i| (*i).usize() != sorted_idx);
            }
        }

        // Update all parent-child relationships
        for ngh in sort_metadata {
            if let Some(parent_idx) = ngh.parent_idx() {
                let curr_parent_idx = swap_history.curr_idx(parent_idx);
                let curr_child_idx = swap_history.curr_idx(ngh.node_idx().unwrap());
                let parent_node = self.hard_get_mut(curr_parent_idx);
                if ngh.is_right_child() {
                    parent_node.set_right_idx(Some(curr_child_idx));
                } else {
                    parent_node.set_left_idx(Some(curr_child_idx));
                }
            }
        }

        // Report new root
        swap_history.curr_idx(root_idx)
    }

    /// Remove node at a known-good index (simpler callsite and error handling) from area.
    /// This function can panic. If the index might be invalid, use `remove` instead.
    pub fn hard_remove(&mut self, idx: usize) -> Node<K, V, U> {
        match self.remove(idx) {
            Some(node) => node,
            None => {
                panic!("Internal invariant failed: attempted removal of node from invalid index.")
            }
        }
    }

    /// Get a reference to a node.
    pub fn get(&self, idx: usize) -> Option<&Node<K, V, U>> {
        match self.arena.get(idx) {
            Some(Some(node)) => Some(node),
            _ => None,
        }
    }

    /// Get mutable reference to a node.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Node<K, V, U>> {
        match self.arena.get_mut(idx) {
            Some(Some(node)) => Some(node),
            _ => None,
        }
    }

    /// Get reference to a node at a known-good index (simpler callsite and error handling).
    /// This function can panic. If the index might be invalid, use `get` instead.
    pub fn hard_get(&self, idx: usize) -> &Node<K, V, U> {
        match self.get(idx) {
            Some(node) => node,
            None => {
                panic!("Internal invariant failed: attempted retrieval of node from invalid index.")
            }
        }
    }

    /// Get mutable reference to a node at a known-good index (simpler callsite and error handling).
    /// This function can panic. If the index might be invalid, use `get_mut` instead.
    pub fn hard_get_mut(&mut self, idx: usize) -> &mut Node<K, V, U> {
        match self.get_mut(idx) {
            Some(node) => node,
            None => panic!("Internal invariant failed: attempted mutable retrieval of node from invalid index."),
        }
    }

    /// Returns the number of entries in the arena, some of which may be `None`.
    pub fn len(&self) -> usize {
        self.arena.len()
    }
}

impl<K: Ord, V, U: Default + SmallUnsigned + Ord + PartialEq + PartialOrd, const N: usize> Default
    for NodeArena<K, V, U, N>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, NodeArena};
    use crate::tree::node::NodeGetHelper;
    use smallnum::small_unsigned;
    use smallvec::smallvec;

    #[test]
    fn test_add_and_remove() {
        let n_1: Node<i32, &str, small_unsigned!(i32::MAX)> = Node::new(1, "n/a");
        let n_2: Node<i32, &str, small_unsigned!(i32::MAX)> = Node::new(2, "n/a");
        let n_3: Node<i32, &str, small_unsigned!(i32::MAX)> = Node::new(3, "n/a");

        let mut arena = NodeArena::new();

        let n_1_idx = arena.add(n_1);
        let n_2_idx = arena.add(n_2);
        let n_3_idx = arena.add(n_3);

        assert_eq!(n_1_idx, 0);
        assert_eq!(n_2_idx, 1);
        assert_eq!(n_3_idx, 2);

        let n_2_removed = arena.remove(n_2_idx).unwrap();
        assert_eq!(n_2_removed.key, 2);
        assert!(arena.arena[1].is_none());

        let n_4 = Node::new(4, "n/a");
        let n_4_idx = arena.add(n_4);
        assert_eq!(n_4_idx, 1);

        let n_5 = Node::new(5, "n/a");
        let n_5_idx = arena.add(n_5);
        assert_eq!(n_5_idx, 3);
    }

    #[test]
    fn test_get_mut() {
        let n_1: Node<i32, &str, small_unsigned!(i32::MAX)> = Node::new(1, "n/a");
        let mut arena = NodeArena::new();
        let n_1_idx = arena.add(n_1);
        assert_eq!(arena.get(n_1_idx).unwrap().val, "n/a");
        let n_1_mut_ref = arena.get_mut(n_1_idx).unwrap();
        n_1_mut_ref.val = "This is a value. There are many like it but this one is mine.";
        assert_ne!(arena.get(n_1_idx).unwrap().val, "n/a");
    }

    #[test]
    fn test_hard_get_1() {
        let n_1: Node<u64, &str, small_unsigned!(u64::MAX)> = Node::new(0xD00DFEED_u64, "n/a");
        let mut arena = NodeArena::new();
        let n_1_idx = arena.add(n_1);
        let n_1_ref = arena.hard_get(n_1_idx);
        assert_eq!(n_1_ref.key, 0xD00DFEED_u64);
    }

    #[test]
    #[should_panic]
    fn test_hard_get_2() {
        let n_1: Node<u64, &str, small_unsigned!(u64::MAX)> = Node::new(0xD00DFEED_u64, "n/a");
        let mut arena = NodeArena::new();
        arena.add(n_1);
        arena.hard_get(1); // OOB
    }

    #[test]
    fn test_capacity() {
        let arena = NodeArena::<i32, &str, small_unsigned!(i32::MAX), 1337>::new();
        assert_eq!(arena.capacity(), 1337);
    }

    #[test]
    fn test_sort() {
        let arena = NodeArena::<usize, &str, small_unsigned!(usize::MAX), 1024>::new();

        // Simple 3-node tree:
        //
        //     2
        //     |
        // ---------
        // |       |
        // 1       3
        //
        let n_1 = Node::new(1, "n/a");
        let mut n_2 = Node::new(2, "n/a");
        let n_3 = Node::new(3, "n/a");

        n_2.set_left_idx(Some(2));
        n_2.set_right_idx(Some(0));

        arena.add(n_3);
        arena.add(n_2);
        arena.add(n_1);

        // Unsorted (insertion/"physical" order)
        assert_eq!(arena.arena[0].as_ref().unwrap().key, 3);
        assert_eq!(arena.arena[1].as_ref().unwrap().key, 2);
        assert_eq!(arena.arena[2].as_ref().unwrap().key, 1);

        // Would be supplied for the above tree
        let sort_metadata = smallvec! {
            NodeGetHelper {
                node_idx: Some(2),
                parent_idx: Some(1),
                is_right_child: false,
            },
            NodeGetHelper {
                node_idx: Some(1),
                parent_idx: None,
                is_right_child: false,
            },
            NodeGetHelper {
                node_idx: Some(0),
                parent_idx: Some(1),
                is_right_child: true,
            },
        };

        arena.sort(1, sort_metadata);

        // Sorted ("logical" order)
        assert_eq!(arena.arena[0].as_ref().unwrap().key, 1);
        assert_eq!(arena.arena[1].as_ref().unwrap().key, 2);
        assert_eq!(arena.arena[2].as_ref().unwrap().key, 3);
    }
}
