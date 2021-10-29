use core::slice::{Iter, IterMut};

use super::node::{Node, NodeSwapHistHelper};
use super::types::{ArenaVec, Idx, IdxVec, SortMetaVec};

use crate::MAX_ELEMS;

use smallnum::SmallUnsigned;

/// A simple arena allocator.
#[derive(Clone)]
pub struct NodeArena<K, V> {
    arena: ArenaVec<K, V>,
    free_list: IdxVec,
}

impl<K, V> NodeArena<K, V> {
    // Public API ------------------------------------------------------------------------------------------------------

    /// Constructor.
    pub fn new() -> Self {
        NodeArena {
            arena: ArenaVec::new(),
            free_list: IdxVec::new(),
        }
    }

    /// `#![no_std]`: total capacity, e.g. maximum number of items.
    /// Attempting to insert items beyond capacity will panic.
    ///
    /// If using `std`: fast capacity, e.g. number of map items stored on the stack.
    /// Items inserted beyond capacity will be stored on the heap.
    pub fn capacity(&self) -> usize {
        MAX_ELEMS
    }

    /// Returns an iterator over immutable arena elements.
    pub fn iter(&self) -> Iter<'_, Option<Node<K, V>>> {
        self.arena.iter()
    }

    /// Returns an iterator over arena elements that allows modifying each value.
    pub fn iter_mut(&mut self) -> IterMut<'_, Option<Node<K, V>>> {
        self.arena.iter_mut()
    }

    /// Add node to area, growing if necessary, and return addition index.
    pub fn add(&mut self, node: Node<K, V>) -> Idx {
        match self.free_list.pop() {
            Some(free_idx) => {
                debug_assert!(
                    self.arena[free_idx.usize()].is_none(),
                    "Internal invariant failed: overwrite of allocated node!"
                );
                self.arena[free_idx.usize()] = Some(node);
                free_idx
            }
            None => {
                self.arena.push(Some(node));
                (self.arena.len() - 1) as Idx
            }
        }
    }

    /// Remove node at a given index from area, return it.
    pub fn remove(&mut self, idx: Idx) -> Option<Node<K, V>> {
        debug_assert!(
            idx < self.arena.len() as Idx,
            "API misuse: requested removal past last index!"
        );
        if idx < self.arena.len() as Idx {
            // Move node to back, replacing with None, preserving order
            self.arena.push(None);
            let len = self.arena.len();
            self.arena.swap(idx.usize(), len - 1);

            // Append removed index to free list
            self.free_list.push(idx);

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
    pub fn sort(&mut self, root_idx: Idx, sort_metadata: SortMetaVec) -> Idx {
        debug_assert!(sort_metadata.iter().all(|ngh| ngh.node_idx.is_some()));

        let mut swap_history = NodeSwapHistHelper::new();

        // Sort as requested
        for (sorted_idx, ngh) in sort_metadata.iter().enumerate() {
            let curr_idx = swap_history.curr_idx(ngh.node_idx.unwrap());
            let sorted_idx = sorted_idx as Idx;
            if curr_idx != sorted_idx {
                self.arena.swap(curr_idx.usize(), sorted_idx.usize());
                swap_history.add(curr_idx, sorted_idx);
                self.free_list.retain(|i| *i != sorted_idx);
            }
        }

        // Update all parent-child relationships
        for ngh in sort_metadata {
            if let Some(parent_idx) = ngh.parent_idx {
                let curr_parent_idx = swap_history.curr_idx(parent_idx);
                let curr_child_idx = swap_history.curr_idx(ngh.node_idx.unwrap());
                let parent_node = self.hard_get_mut(curr_parent_idx);
                if ngh.is_right_child {
                    parent_node.right_idx = Some(curr_child_idx);
                } else {
                    parent_node.left_idx = Some(curr_child_idx);
                }
            }
        }

        // Report new root
        swap_history.curr_idx(root_idx)
    }

    /// Remove node at a known-good index (simpler callsite and error handling) from area.
    /// This function can panic. If the index might be invalid, use `remove` instead.
    pub fn hard_remove(&mut self, idx: Idx) -> Node<K, V> {
        match self.remove(idx) {
            Some(node) => node,
            None => {
                panic!("Internal invariant failed: attempted removal of node from invalid index.")
            }
        }
    }

    /// Get a reference to a node.
    pub fn get(&self, idx: Idx) -> Option<&Node<K, V>> {
        match self.arena.get(idx.usize()) {
            Some(Some(node)) => Some(node),
            _ => None,
        }
    }

    /// Get mutable reference to a node.
    pub fn get_mut(&mut self, idx: Idx) -> Option<&mut Node<K, V>> {
        match self.arena.get_mut(idx.usize()) {
            Some(Some(node)) => Some(node),
            _ => None,
        }
    }

    /// Get reference to a node at a known-good index (simpler callsite and error handling).
    /// This function can panic. If the index might be invalid, use `get` instead.
    pub fn hard_get(&self, idx: Idx) -> &Node<K, V> {
        match self.get(idx) {
            Some(node) => node,
            None => {
                panic!("Internal invariant failed: attempted retrieval of node from invalid index.")
            }
        }
    }

    /// Get mutable reference to a node at a known-good index (simpler callsite and error handling).
    /// This function can panic. If the index might be invalid, use `get_mut` instead.
    pub fn hard_get_mut(&mut self, idx: Idx) -> &mut Node<K, V> {
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

impl<K: Ord, V> Default for NodeArena<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, NodeArena};
    use crate::tree::arena::MAX_ELEMS;
    use crate::tree::node::NodeGetHelper;
    use smallvec::smallvec;

    #[test]
    fn test_add_and_remove() {
        let n_1 = Node::new(1, "n/a");
        let n_2 = Node::new(2, "n/a");
        let n_3 = Node::new(3, "n/a");

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
        let n_1 = Node::new(1, "n/a");
        let mut arena = NodeArena::new();
        let n_1_idx = arena.add(n_1);
        assert_eq!(arena.get(n_1_idx).unwrap().val, "n/a");
        let n_1_mut_ref = arena.get_mut(n_1_idx).unwrap();
        n_1_mut_ref.val = "This is a value. There are many like it but this one is mine.";
        assert_ne!(arena.get(n_1_idx).unwrap().val, "n/a");
    }

    #[test]
    fn test_hard_get_1() {
        let n_1 = Node::new(0xD00DFEED_u64, "n/a");
        let mut arena = NodeArena::new();
        let n_1_idx = arena.add(n_1);
        let n_1_ref = arena.hard_get(n_1_idx);
        assert_eq!(n_1_ref.key, 0xD00DFEED_u64);
    }

    #[test]
    #[should_panic]
    fn test_hard_get_2() {
        let n_1 = Node::new(0xD00DFEED_u64, "n/a");
        let mut arena = NodeArena::new();
        arena.add(n_1);
        arena.hard_get(1); // OOB
    }

    #[test]
    fn test_capacity() {
        let arena = NodeArena::<i32, &str>::new();
        assert_eq!(arena.capacity(), MAX_ELEMS);
    }

    #[test]
    fn test_sort() {
        let mut arena = NodeArena::<usize, &str>::new();

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

        n_2.left_idx = Some(2);
        n_2.right_idx = Some(0);

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
