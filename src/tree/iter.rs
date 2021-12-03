use smallvec::SmallVec;

use super::tree::SGTree;
use super::node_dispatch::{SmallNode, SmallNodeDispatch};

// Immutable Reference iterator ----------------------------------------------------------------------------------------

/// Uses iterative in-order tree traversal algorithm.
/// Maintains a small stack of arena indexes (won't contain all indexes simultaneously for a balanced tree).
pub struct Iter<'a, K: Default, V: Default, const N: usize> {
    bst: &'a SGTree<K, V, N>,
    idx_stack: SmallVec<[usize; N]>,
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iter<'a, K, V, N> {
    pub fn new(bst: &'a SGTree<K, V, N>) -> Self {
        let mut ordered_iter = Iter {
            bst,
            idx_stack: SmallVec::<[usize; N]>::new(),
        };

        if let Some(root_idx) = ordered_iter.bst.root_idx {
            let mut curr_idx = root_idx;
            loop {
                let node = &ordered_iter.bst.arena[curr_idx];
                match node.left_idx() {
                    Some(lt_idx) => {
                        ordered_iter.idx_stack.push(curr_idx);
                        curr_idx = lt_idx;
                    }
                    None => {
                        ordered_iter.idx_stack.push(curr_idx);
                        break;
                    }
                }
            }
        }

        ordered_iter
    }
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iterator for Iter<'a, K, V, N> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.idx_stack.pop() {
            Some(pop_idx) => {
                let node = &self.bst.arena[pop_idx];
                if let Some(gt_idx) = node.right_idx() {
                    let mut curr_idx = gt_idx;
                    loop {
                        let node = &self.bst.arena[curr_idx];
                        match node.left_idx() {
                            Some(lt_idx) => {
                                self.idx_stack.push(curr_idx);
                                curr_idx = lt_idx;
                            }
                            None => {
                                self.idx_stack.push(curr_idx);
                                break;
                            }
                        }
                    }
                }

                let node = &self.bst.arena[pop_idx];
                Some((&node.key(), &node.val()))
            }
            None => None,
        }
    }
}

// Mutable Reference iterator ------------------------------------------------------------------------------------------

pub struct IterMut<'a, K: Default, V: Default, const N: usize> {
    arena_iter_mut: core::slice::IterMut<'a, Option<SmallNodeDispatch<K, V>>>,
}

impl<'a, K: Ord + Default, V: Default, const N: usize> IterMut<'a, K, V, N> {
    pub fn new(bst: &'a mut SGTree<K, V, N>) -> Self {
        bst.sort_arena();
        IterMut {
            arena_iter_mut: bst.arena.iter_mut(),
        }
    }
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iterator for IterMut<'a, K, V, N> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.arena_iter_mut.next() {
            Some(Some(node)) => Some((node.key(), node.val_mut())), // Change `mut` method to return `(&K, &mut V)`?
            _ => None,
        }
    }
}

// Consuming iterator --------------------------------------------------------------------------------------------------

/// Cheats a little by using internal flattening logic to sort, instead of re-implementing proper traversal.
/// Maintains a shrinking list of arena indexes, initialized with all of them.
pub struct IntoIter<K: Default, V: Default, const N: usize> {
    bst: SGTree<K, V, N>,
    sorted_idxs: SmallVec<[usize; N]>,
}

impl<K: Ord + Default, V: Default, const N: usize> IntoIter<K, V, N> {
    /// Constructor
    pub fn new(bst: SGTree<K, V, N>) -> Self {
        let mut ordered_iter = IntoIter {
            bst,
            sorted_idxs: SmallVec::<[usize; N]>::new(),
        };

        if let Some(root_idx) = ordered_iter.bst.root_idx {
            ordered_iter.sorted_idxs = ordered_iter.bst.flatten_subtree_to_sorted_idxs(root_idx);
            ordered_iter.sorted_idxs.reverse();
        }

        ordered_iter
    }
}

impl<K: Ord + Default, V: Default, const N: usize> Iterator for IntoIter<K, V, N> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.sorted_idxs.pop() {
            Some(idx) => match self.bst.priv_remove_by_idx(idx) {
                Some((key, val)) => Some((key, val)),
                None => {
                    debug_assert!(false, "Use of invalid index in consuming iterator!");
                    None
                }
            },
            None => None,
        }
    }
}
