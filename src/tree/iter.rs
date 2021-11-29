use smallnum::SmallUnsigned;
use smallvec::SmallVec;

use super::tree::SGTree;

// Immutable Reference iterator ----------------------------------------------------------------------------------------

/// Uses iterative in-order tree traversal algorithm.
/// Maintains a small stack of arena indexes (won't contain all indexes simultaneously for a balanced tree).
pub struct Iter<'a, K: Ord, V, I, const C: usize> {
    bst: &'a SGTree<K, V>,
    idx_stack: SmallVec<[I; C]>,
}

impl<'a, K: Ord, V, I: SmallUnsigned, const C: usize> Iter<'a, K, V, I, C> {
    pub fn new(bst: &'a SGTree<K, V>) -> Self {
        let mut ordered_iter = Iter {
            bst,
            idx_stack: SmallVec::<[I; C]>::new(),
        };

        if let Some(root_idx) = ordered_iter.bst.root_idx {
            let mut curr_idx = root_idx;
            loop {
                let node = ordered_iter.bst.arena.hard_get(curr_idx);
                match node.left_idx() {
                    Some(lt_idx) => {
                        ordered_iter.idx_stack.push(I::checked_from(curr_idx));
                        curr_idx = lt_idx;
                    }
                    None => {
                        ordered_iter.idx_stack.push(I::checked_from(curr_idx));
                        break;
                    }
                }
            }
        }

        ordered_iter
    }
}

impl<'a, K: Ord, V, I: SmallUnsigned, const C: usize> Iterator for Iter<'a, K, V, I, C> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.idx_stack.pop() {
            Some(pop_idx) => {
                let node = self.bst.arena.hard_get(pop_idx.usize());
                if let Some(gt_idx) = node.right_idx() {
                    let mut curr_idx = gt_idx;
                    loop {
                        let node = self.bst.arena.hard_get(curr_idx);
                        match node.left_idx() {
                            Some(lt_idx) => {
                                self.idx_stack.push(I::checked_from(curr_idx));
                                curr_idx = lt_idx;
                            }
                            None => {
                                self.idx_stack.push(I::checked_from(curr_idx));
                                break;
                            }
                        }
                    }
                }

                let node = self.bst.arena.hard_get(pop_idx.usize());
                Some((&node.key, &node.val))
            }
            None => None,
        }
    }
}

// Mutable Reference iterator ----------------------------------------------------------------------------------------

pub struct IterMut<'a, K: Ord, V> {
    arena_iter_mut: core::slice::IterMut<'a, Option<(K, V)>>,
}

impl<'a, K: Ord, V> IterMut<'a, K, V> {
    pub fn new(bst: &'a mut SGTree<K, V>) -> Self {
        bst.sort_arena();
        IterMut {
            arena_iter_mut: bst.arena.iter_mut(),
            // TODO: figure this out
            // arena_iter_mut: bst.arena.iter_mut().map(|n| (n.key, n.val))
        }
    }
}

impl<'a, K: Ord, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.arena_iter_mut.next() {
            Some(Some((key, val))) => Some((key, val)),
            _ => None,
        }
    }
}

// Consuming iterator --------------------------------------------------------------------------------------------------

/// Cheats a little by using internal flattening logic to sort, instead of re-implementing proper traversal.
/// Maintains a shrinking list of arena indexes, initialized with all of them.
pub struct IntoIter<K: Ord, V, I, const C: usize> {
    bst: SGTree<K, V>,
    sorted_idxs: SmallVec<[I; C]>,
}

impl<K: Ord, V, I, const C: usize> IntoIter<K, V, I, C> {
    /// Constructor
    pub fn new(bst: SGTree<K, V>) -> Self {
        let mut ordered_iter = IntoIter {
            bst,
            sorted_idxs: SmallVec::<[I; C]>::new(),
        };

        if let Some(root_idx) = ordered_iter.bst.root_idx {
            ordered_iter.sorted_idxs = ordered_iter.bst.flatten_subtree_to_sorted_idxs(root_idx);
            ordered_iter.sorted_idxs.reverse();
        }

        ordered_iter
    }
}

impl<K: Ord, V, I: SmallUnsigned, const C: usize> Iterator for IntoIter<K, V, I, C> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.sorted_idxs.pop() {
            Some(idx) => match self.bst.priv_remove_by_idx(idx.usize()) {
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
