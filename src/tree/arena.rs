use core::slice::{Iter, IterMut};
use core::ops::{Index, IndexMut};

use super::node::{Node, NodeGetHelper, NodeSwapHistHelper};
use super::node_dispatch::{SmallNode, SmallNodeDispatch};
use super::arena_dispatch::SmallArena;

use smallnum::{small_unsigned_label, SmallUnsigned, SmallUnsignedLabel};
use smallvec::SmallVec;

// CRITICAL TODO: very that all use of "arena.capacity()" is correct and should not be "arena.len()"

/*
Note:

Structures in this file generic for `U` in a *subset* of the set `(u8, u16, u32, u64, u128)`.
All members in subset are <= host pointer width in size.
If caller obeys contract, `U` will be smallest unsigned capable of representing const `N` (e.g. static capacity).
*/

/// An arena allocator, meta programmable for low memory footprint.
/// Users of it's APIs only need to declare `U` type or trait bounds at construction.
/// Method APIs take/return `usize` and normalize to `U` internally.
/// Sole associated function, `gen_idx_vec`, has return type that uses `U` - to a void duplicating `Vec` API here.
#[derive(Clone)]
pub struct Arena<K: Default, V: Default, U, const N: usize> {
    arena: SmallVec<[Option<Node<K, V, U>>; N]>,

    #[cfg(not(feature = "low_mem_insert"))]
    free_list: SmallVec<[U; N]>,
}

impl<K: Default, V: Default, U: Default + Copy + SmallUnsigned + Ord + PartialEq + PartialOrd, const N: usize>
    Arena<K, V, U, N>
{
    // TODO: is this function necessary?
    /// Const associated constructor for index scratch vector.
    pub fn new_idx_vec() -> SmallVec<[U; N]> {
        SmallVec::<[U; N]>::default()
    }

    /// Constructor.
    pub fn new() -> Self {
        let na = Arena {
            arena: SmallVec::<[Option<Node<K, V, U>>; N]>::new(),

            #[cfg(not(feature = "low_mem_insert"))]
            free_list: SmallVec::<[U; N]>::new(),
        };

        debug_assert_eq!(0, na.free_list.len());
        debug_assert_eq!(0, na.arena.len());

        debug_assert_eq!(N, na.free_list.capacity());
        debug_assert_eq!(N, na.arena.capacity());

        na
    }
}

impl <K: Default, V: Default, U: Default + Copy + SmallUnsigned + Ord + PartialEq + PartialOrd, const N: usize> SmallArena<K, V, N> for Arena<K, V, U, N> {
    fn capacity(&self) -> usize {
        N
    }

    fn iter(&self) -> Iter<'_, Option<SmallNodeDispatch<K, V>>> {
        self.arena.iter() // TODO: add iterator converter
    }

    fn iter_mut(&mut self) -> IterMut<'_, Option<SmallNodeDispatch<K, V>>> {
        self.arena.iter_mut() // TODO: add iterator converter
    }

    // TODO: change API to take key and val
    fn add(&mut self, key: K, val: V) -> usize {
        // O(1) find, constant time
        #[cfg(not(feature = "low_mem_insert"))]
        let opt_free_idx = self.free_list.pop();

        // O(n) find, linear search
        #[cfg(feature = "low_mem_insert")]
        let opt_free_idx = self.arena.iter().position(|x| x.is_none()).map(|i| i as U);

        let node = Node::new(key, val);
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

    fn remove(&mut self, idx: usize) -> Option<SmallNodeDispatch<K, V>> {
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
                    Some(node) => Some(SmallNodeDispatch::<K,V>::new(node.take_key(), node.take_val(), small_unsigned_label!(N))),
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

    fn hard_remove(&mut self, idx: usize) -> SmallNodeDispatch<K, V> {
        match self.remove(idx) {
            Some(node) => node,
            None => {
                panic!("Internal invariant failed: attempted removal of node from invalid index.")
            }
        }
    }

    fn sort(
        &mut self,
        root_idx: usize,
        sort_metadata: SmallVec<[NodeGetHelper<usize>; N]>, // `usize` instead of `U` avoids `U` in tree iter sigs
    ) -> usize {
        debug_assert!(sort_metadata.iter().all(|ngh| ngh.node_idx().is_some()));

        let mut swap_history = NodeSwapHistHelper::<U, N>::new();

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
                let parent_node = &mut self[curr_parent_idx];
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

    fn len(&self) -> usize {
        self.arena.len()
    }

    fn node_size(&self) -> usize {
        let node = SmallNodeDispatch::new(K::default(), V::default(), small_unsigned_label!(N));
        core::mem::size_of_val(&node)
    }
}

// Convenience Traits --------------------------------------------------------------------------------------------------

/// Immutable indexing.
/// Indexed location MUST be occupied.
impl<K: Default, V: Default, U, const N: usize> Index<usize> for Arena<K, V, U, N> {
    type Output = Node<K, V, U>;

    fn index(&self, index: usize) -> &Self::Output {
        match &self.arena[index] {
            Some(node) => &node,
            None => unreachable!()
        }
    }
}

/// Mutable indexing
/// Indexed location MUST be occupied.
impl<K: Default, V: Default, U, const N: usize> IndexMut<usize> for Arena<K, V, U, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self.arena.index_mut(index) {
            Some(node) => node,
            None => unreachable!()
        }
    }
}

impl<K: Ord + Default, V: Default, U: Default + Copy + SmallUnsigned + Ord + PartialEq + PartialOrd, const N: usize> Default
    for Arena<K, V, U, N>
{
    fn default() -> Self {
        Self::new()
    }
}

// Test ----------------------------------------------------------------------------------------------------------------

/* CRITCAL TODO: re-write and re-enable

#[cfg(test)]
mod tests {
    use core::mem::size_of_val;
    use super::Arena;
    use crate::tree::node::NodeGetHelper;
    use crate::tree::node_dispatch::{SmallNode, SmallNodeDispatch};
    use crate::tree::arena_dispatch::SmallArena;
    use smallnum::{small_unsigned, small_unsigned_label, SmallUnsignedLabel};
    use smallvec::smallvec;

    const CAPACITY: usize = 1024;

    #[test]
    fn test_add_and_remove() {
        let n_1 = SmallNodeDispatch::new(1, "n/a", small_unsigned_label!(CAPACITY));
        let n_2 = SmallNodeDispatch::new(2, "n/a", small_unsigned_label!(CAPACITY) );
        let n_3 = SmallNodeDispatch::new(3, "n/a", small_unsigned_label!(CAPACITY));

        let mut arena: Arena<isize, &str, small_unsigned!(CAPACITY), CAPACITY> = Arena::new();

        let n_1_idx = arena.add(n_1);
        let n_2_idx = arena.add(n_2);
        let n_3_idx = arena.add(n_3);

        assert_eq!(n_1_idx, 0);
        assert_eq!(n_2_idx, 1);
        assert_eq!(n_3_idx, 2);

        let n_2_removed = arena.remove(n_2_idx).unwrap();
        assert_eq!(n_2_removed.key(), &2);
        assert!(arena.arena[1].is_none());

        let n_4 = SmallNodeDispatch::new(4, "n/a", small_unsigned_label!(CAPACITY));
        let n_4_idx = arena.add(n_4);
        assert_eq!(n_4_idx, 1);

        let n_5 = SmallNodeDispatch::new(5, "n/a", small_unsigned_label!(CAPACITY));
        let n_5_idx = arena.add(n_5);
        assert_eq!(n_5_idx, 3);
    }

    #[test]
    fn test_index_mut() {
        let n_1 = SmallNodeDispatch::new(1, "n/a", small_unsigned_label!(CAPACITY));
        let mut arena: Arena<isize, &str, small_unsigned!(CAPACITY), CAPACITY> = Arena::new();
        let n_1_idx = arena.add(n_1);
        assert_eq!(arena[n_1_idx].val(), &"n/a");
        let n_1_mut_ref = &mut arena[n_1_idx];
        n_1_mut_ref.set_val("This is a value. There are many like it but this one is mine.");
        assert_ne!(arena[n_1_idx].val(), &"n/a");
    }

    #[test]
    fn test_index_1() {
        let n_1 = SmallNodeDispatch::new(0xD00DFEED_u64, "n/a", small_unsigned_label!(CAPACITY));
        let mut arena: Arena<u64, &str, small_unsigned!(CAPACITY), CAPACITY> = Arena::new();
        let n_1_idx = arena.add(n_1);
        let n_1_ref = &arena[n_1_idx];
        assert_eq!(n_1_ref.key(), &0xD00DFEED_u64);
    }

    #[test]
    #[should_panic]
    fn test_index_2() {
        let n_1 = SmallNodeDispatch::new(0xD00DFEED_u64, "n/a", small_unsigned_label!(CAPACITY));
        let mut arena: Arena<u64, &str, small_unsigned!(CAPACITY), CAPACITY> = Arena::new();
        arena.add(n_1);
        let _ = &arena[1]; // OOB
    }

    #[test]
    fn test_capacity() {
        let arena = Arena::<i8, u128, small_unsigned!(CAPACITY), CAPACITY>::new();
        assert_eq!(arena.capacity(), CAPACITY);

        let arena = Arena::<i32, &str, small_unsigned!(1337), 1337>::new();
        assert_eq!(arena.capacity(), 1337);
    }

    #[test]
    fn test_sort() {
        let mut arena = Arena::<usize, &str, small_unsigned!(CAPACITY), CAPACITY>::new();

        // Simple 3-node tree:
        //
        //     2
        //     |
        // ---------
        // |       |
        // 1       3
        //
        let n_1 = SmallNodeDispatch::new(1, "n/a", small_unsigned_label!(CAPACITY));
        let mut n_2 = SmallNodeDispatch::new(2, "n/a", small_unsigned_label!(CAPACITY) );
        let n_3 = SmallNodeDispatch::new(3, "n/a", small_unsigned_label!(CAPACITY));

        n_2.set_left_idx(Some(2));
        n_2.set_right_idx(Some(0));

        arena.add(n_3);
        arena.add(n_2);
        arena.add(n_1);

        // Unsorted (insertion/"physical" order)
        assert_eq!(arena.arena[0].as_ref().unwrap().key(), &3);
        assert_eq!(arena.arena[1].as_ref().unwrap().key(), &2);
        assert_eq!(arena.arena[2].as_ref().unwrap().key(), &1);

        // Would be supplied for the above tree
        let sort_metadata = smallvec! {
            NodeGetHelper::new(Some(2), Some(1), false),
            NodeGetHelper::new(Some(1), None, false),
            NodeGetHelper::new(Some(0), Some(1), false),
        };

        arena.sort(1, sort_metadata);

        // Sorted ("logical" order)
        assert_eq!(arena.arena[0].as_ref().unwrap().key(), &1);
        assert_eq!(arena.arena[1].as_ref().unwrap().key(), &2);
        assert_eq!(arena.arena[2].as_ref().unwrap().key(), &3);
    }

    #[test]
    fn test_node_packing() {
        const SMALL_CAPACITY: usize = 100;
        const LARGE_CAPACITY: usize = 1_000;

        let small_arena = Arena::<u64, u64, small_unsigned!(SMALL_CAPACITY), SMALL_CAPACITY>::new();
        let large_arena = Arena::<u64, u64, small_unsigned!(LARGE_CAPACITY), LARGE_CAPACITY>::new();

        let small_arena_size = size_of_val(&small_arena);
        let large_arena_size = size_of_val(&large_arena);

        println!("\nArena sizes:");
        println!("\tSmall: {} bytes", small_arena_size);
        println!("\tBig: {} bytes", large_arena_size);

        assert!(small_arena_size < large_arena_size);

        let small_node_size = small_arena.node_size();
        let large_node_size = large_arena.node_size();

        println!("\nNode sizes:");
        println!("\tSmall: {} bytes", small_node_size);
        println!("\tBig: {} bytes", large_node_size);

        assert!(small_node_size < large_node_size);
    }
}

*/