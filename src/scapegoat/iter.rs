use crate::scapegoat::SGTree;

// TODO: add pre-order and post-order iterators

// Reference iterator --------------------------------------------------------------------------------------------------

/// Uses iterative in-order tree traversal algorithm.
/// Maintains a small stack of arena indexes (won't contain all indexes simultaneously for a balanced tree).
/// This iterator is more memory efficient than the consuming variant, but slower.
pub struct RefInOrderIterator<'a, K: Ord, V> {
    tree: &'a SGTree<K, V>,
    idx_stack: Vec<usize>,
}

impl<'a, K: Ord, V> RefInOrderIterator<'a, K, V> {

    pub fn new(tree: &'a SGTree<K, V>) -> Self {
        let mut ordered_iter = RefInOrderIterator {
            tree,
            idx_stack: Vec::new(),
        };

        if let Some(root_idx) = ordered_iter.tree.root_idx {
            let mut curr_idx = root_idx;
            loop {
                let node = ordered_iter.tree.arena.hard_get(curr_idx);
                match node.left_idx {
                    Some(lt_idx) => {
                        ordered_iter.idx_stack.push(curr_idx);
                        curr_idx = lt_idx;
                    },
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

impl<'a, K: Ord, V> Iterator for RefInOrderIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        match self.idx_stack.pop() {
            Some(pop_idx) => {
                let node = self.tree.arena.hard_get(pop_idx);
                if let Some(gt_idx) = node.right_idx {
                    let mut curr_idx = gt_idx;
                    loop {
                        let node = self.tree.arena.hard_get(curr_idx);
                        match node.left_idx {
                            Some(lt_idx) => {
                                self.idx_stack.push(curr_idx);
                                curr_idx = lt_idx;
                            },
                            None => {
                                self.idx_stack.push(curr_idx);
                                break;
                            }
                        }
                    }
                }

                let node = self.tree.arena.hard_get(pop_idx);
                return Some((&node.key, &node.val))
            },
            None => None
        }
    }
}

// Consuming iterator --------------------------------------------------------------------------------------------------

/// Cheats a little by using internal flattening logic to sort, instead of re-implementing proper traversal.
/// Maintains a shrinking list of arena indexes, initialized with all of them.
/// This iterator is less memory efficient than the reference variant, but faster.
pub struct InOrderIterator<K: Ord, V> {
    tree: SGTree<K, V>,
    sorted_idxs: Vec<usize>,
}

impl<K: Ord, V> InOrderIterator<K, V> {

    pub fn new(tree: SGTree<K, V>) -> Self {
        let mut ordered_iter = InOrderIterator {
            tree,
            sorted_idxs: Vec::new()
        };

        if let Some(root_idx) = ordered_iter.tree.root_idx {
            ordered_iter.sorted_idxs = ordered_iter.tree.flatten_subtree_to_sorted_idxs(root_idx);
            ordered_iter.sorted_idxs.reverse();
        }

        ordered_iter
    }
}

impl<K: Ord, V> Iterator for InOrderIterator<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        match self.sorted_idxs.pop() {
            Some(idx) => {
                match self.tree.priv_remove_by_idx(idx) {
                    Some (node) => Some((node.key, node.val)),
                    None => {
                        debug_assert!(false, "Use of invalid index in consuming iterator!");
                        None
                    }
                }
            },
            None => None
        }
    }
}