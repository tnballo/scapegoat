use crate::tree::{Node, SGTree, IdxVec};
use super::arena::OptNode;

// TODO: add pre-order and post-order iterators

// Immutable Reference iterator ----------------------------------------------------------------------------------------

/// Uses iterative in-order tree traversal algorithm.
/// Maintains a small stack of arena indexes (won't contain all indexes simultaneously for a balanced tree).
pub struct Iter<'a, K: Ord, V> {
    bst: &'a SGTree<K, V>,
    idx_stack: IdxVec,
}

impl<'a, K: Ord, V> Iter<'a, K, V> {
    pub fn new(bst: &'a SGTree<K, V>) -> Self {
        let mut ordered_iter = Iter {
            bst,
            idx_stack: IdxVec::new(),
        };

        if let Some(root_idx) = ordered_iter.bst.root_idx {
            let mut curr_idx = root_idx;
            loop {
                let node = ordered_iter.bst.arena.hard_get(curr_idx);
                match node.left_idx {
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

impl<'a, K: Ord, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.idx_stack.pop() {
            Some(pop_idx) => {
                let node = self.bst.arena.hard_get(pop_idx);
                if let Some(gt_idx) = node.right_idx {
                    let mut curr_idx = gt_idx;
                    loop {
                        let node = self.bst.arena.hard_get(curr_idx);
                        match node.left_idx {
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

                let node = self.bst.arena.hard_get(pop_idx);
                Some((&node.key, &node.val))
            }
            None => None,
        }
    }
}

// Mutable Reference iterator ----------------------------------------------------------------------------------------

pub struct IterMut<'a, K: Ord, V> {
    //node_arena: &'a mut NodeArena<K, V>,
    arena_slice: &'a mut [OptNode<K,V>],
    sorted_idxs: IdxVec,
    //next: Option<(&'a K, &'a mut V)>,
}

impl<'a, K: Ord, V> IterMut<'a, K, V> {
    pub fn new(bst: &'a mut SGTree<K, V>) -> Self {
        let mut sorted_idxs = IdxVec::new();
        if let Some(root_idx) = bst.root_idx {
            sorted_idxs = bst.flatten_subtree_to_sorted_idxs(root_idx);
            sorted_idxs.reverse();
        };

        IterMut {
            //node_arena: &mut bst.arena,
            arena_slice: bst.arena.as_mut_slice(),
            sorted_idxs,
            //next: None,
        }
    }
}

impl<'a, K: Ord, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.sorted_idxs.pop() {
            Some(idx) => {
                // TODO: clever splitting and manipulation here
                let slice = core::mem::replace(&mut self.arena_slice, &mut []);
                //let (left, right) = self.arena_slice.split_at_mut(idx);
                let (_, right) = slice.split_at_mut(idx);
                let (mut target, _) = right.split_first_mut()?;
                //self.arena_slice = slice; // Can't merge since no copy!
                target.as_mut().map(|mut node| (&node.key, &mut node.val))

                //self.node_arena.take(idx).map(|mut node| (&node.key, &mut node.val))
                //self.next = self.node_arena.take_mut(idx).map(|mut node| (&node.key, &mut node.val));
            },
            None => None,
        }
    }
}

// Consuming iterator --------------------------------------------------------------------------------------------------

/// Cheats a little by using internal flattening logic to sort, instead of re-implementing proper traversal.
/// Maintains a shrinking list of arena indexes, initialized with all of them.
pub struct ConsumingIter<K: Ord, V> {
    bst: SGTree<K, V>,
    sorted_idxs: IdxVec,
}

impl<K: Ord, V> ConsumingIter<K, V> {
    pub fn new(bst: SGTree<K, V>) -> Self {
        let mut ordered_iter = ConsumingIter {
            bst,
            sorted_idxs: IdxVec::new(),
        };

        if let Some(root_idx) = ordered_iter.bst.root_idx {
            ordered_iter.sorted_idxs = ordered_iter.bst.flatten_subtree_to_sorted_idxs(root_idx);
            ordered_iter.sorted_idxs.reverse();
        }

        ordered_iter
    }
}

impl<K: Ord, V> Iterator for ConsumingIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.sorted_idxs.pop() {
            Some(idx) => match self.bst.priv_remove_by_idx(idx) {
                Some(node) => Some((node.key, node.val)),
                None => {
                    debug_assert!(false, "Use of invalid index in consuming iterator!");
                    None
                }
            },
            None => None,
        }
    }
}
