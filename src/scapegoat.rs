use std::cmp::Ordering;

use crate::node::{Node, NodeGetHelper, NodeRebuildHelper};
use crate::arena::NodeArena;

pub mod iter;
use iter::{InOrderIterator, RefInOrderIterator};

#[cfg(test)]
mod test;

// TODO: review use of "if let Some(x)" => replace with match when only 2 possible options
// TODO: for pattern matching, check if "return" keyword is necessary (yes for the loops we need to break, possible no otherwise)
// TODO: verify current size and max size tracking against the paper

pub struct SGTree<K: Ord, V> {
    arena: NodeArena<K, V>,
    root_idx: Option<usize>,
    max_idx: usize,
    min_idx: usize,
    curr_size: usize,
    max_size: usize,
    rebal_cnt: usize,
}

impl<K: Ord, V> SGTree<K, V> {

    // Public API ------------------------------------------------------------------------------------------------------

    /// TODO: docs
    pub fn new() -> Self {
        SGTree {
            arena: NodeArena::new(),
            root_idx: None,
            max_idx: 0,
            min_idx: 0,
            curr_size: 0,
            max_size: 0,
            rebal_cnt: 0,
        }
    }

    /// TODO: docs
    pub fn insert(&mut self, key: K, val: V) {

        let mut path = Vec::new(); // For backwards traversal without parent pointer on node (strict scapegoat)
        let new_node = Node::new(key, val);

        // Optional rebalance
        self.priv_insert(&mut path, new_node);
        if path.len() > log_3_2(self.max_size) {
            if let Some(scapegoat_idx) = self.find_scapegoat(&path) {
                self.rebuild(scapegoat_idx);
            }
        }
    }

    /// TODO: docs
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        match self.priv_remove_by_key(key) {
            Some(node) => {
                if self.max_size > (2 * self.curr_size) {
                    if let Some(root_idx) = self.root_idx {
                        self.rebuild(root_idx);
                        self.max_size = self.curr_size;
                    }
                }
                Some((node.key, node.val))
            },
            None => None,
        }
    }

    /// TODO: docs
    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.remove_entry(key) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    /// Get both key and value references corresponding to key.
    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        let ngh = self.priv_get(key);
        match ngh.node_idx {
            Some(idx) => {
                let node = self.arena.hard_get(idx);
                Some((&node.key, &node.val))
            },
            None => None
        }
    }

    /// Get value reference corresponding to key.
    pub fn get(&self, key: &K) -> Option<&V> {
        match self.get_key_value(key) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    /// Get mutable reference corresponding to key.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let ngh = self.priv_get(key);
        match ngh.node_idx {
            Some(idx) => {
                let node = self.arena.hard_get_mut(idx);
                Some(&mut node.val)
            },
            None => None
        }
    }

    /// TODO: docs
    pub fn clear(&mut self) {
        let rebal_cnt = self.rebal_cnt;
        *self = SGTree::new();
        self.rebal_cnt = rebal_cnt;
    }

    /// TODO: docs
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// TODO: docs
    pub fn is_empty(&self) -> bool {
        self.root_idx.is_none()
    }

    /// TODO: docs
    pub fn min_key_value(&self) -> Option<(&K, &V)> {
        match self.arena.get(self.min_idx) {
            Some(node) => Some((&node.key, &node.val)),
            None => None,
        }
    }

    /// TODO: docs
    pub fn min_key(&self) -> Option<&K> {
        match self.min_key_value() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }

    /// TODO: docs
    pub fn remove_min(&mut self) -> Option<(K, V)> {
        match self.priv_remove_by_idx(self.min_idx) {
            Some(node) => Some((node.key, node.val)),
            None => None,
        }
    }

    /// TODO: docs
    pub fn max_key_value(&self) -> Option<(&K, &V)> {
        match self.arena.get(self.max_idx) {
            Some(node) => Some((&node.key, &node.val)),
            None => None,
        }
    }

    /// TODO: docs
    pub fn max_key(&self) -> Option<&K> {
        match self.max_key_value() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }

    /// TODO: docs
    pub fn remove_max(&mut self) -> Option<(K, V)> {
        match self.priv_remove_by_idx(self.max_idx) {
            Some(node) => Some((node.key, node.val)),
            None => None,
        }
    }

    /// Get the number of times this tree rebalanced itself (for testing and/or performance engineering)
    pub fn rebal_cnt(&self) -> usize {
        self.rebal_cnt
    }

    // Private API -----------------------------------------------------------------------------------------------------

    // Iterative search. If key found, returns node idx, parent idx, and a bool indicating if node is right child
    fn priv_get(&self, key: &K) -> NodeGetHelper {
        match self.root_idx {
            Some(root_idx) => {
                let mut opt_parent_idx = None;
                let mut curr_idx = root_idx;
                let mut is_right_child = false;
                loop {
                    let node = self.arena.hard_get(curr_idx);
                    match key.cmp(&node.key) {
                        Ordering::Less => {
                            match node.left_idx {
                                Some(lt_idx) => {
                                    opt_parent_idx = Some(curr_idx);
                                    curr_idx = lt_idx;
                                    is_right_child = false;
                                },
                                None => return NodeGetHelper::new(None, None, false)
                            }
                        },
                        Ordering::Equal => {
                            return NodeGetHelper::new(Some(curr_idx), opt_parent_idx, is_right_child);
                        },
                        Ordering::Greater => {
                            match node.right_idx {
                                Some(gt_idx) => {
                                    opt_parent_idx = Some(curr_idx);
                                    curr_idx = gt_idx;
                                    is_right_child = true;
                                },
                                None => return NodeGetHelper::new(None, None, false)
                            }
                        },
                    }
                }
            },
            None => NodeGetHelper::new(None, None, false)
        }
    }

    // TODO: desc
    fn priv_insert(&mut self, path: &mut Vec<usize>, new_node: Node<K, V>) {
        self.curr_size += 1;
        self.max_size += 1;

        match self.root_idx {

            // Sorted insert
            Some(idx) => {

                // Iterative traversal
                let mut curr_idx = idx;
                let ngh;
                loop {

                    let mut curr_node = self.arena.hard_get_mut(curr_idx);
                    path.push(curr_idx);

                    match &new_node.key.cmp(&curr_node.key) {
                        Ordering::Less => {
                            match curr_node.left_idx {
                                Some(left_idx) => curr_idx = left_idx,
                                None => {
                                    // New min check
                                    let mut new_min_found = false;
                                    let min_node = self.arena.hard_get(self.min_idx);
                                    if new_node.key < min_node.key {
                                        new_min_found = true;
                                    }

                                    // Left insert
                                    let new_node_idx = self.arena.add(new_node);

                                    // New min update
                                    if new_min_found {
                                        self.min_idx = new_node_idx;
                                    }

                                    ngh = NodeGetHelper::new(Some(new_node_idx), Some(curr_idx), false);
                                    break;
                                }
                            }
                        },
                        Ordering::Equal => {
                            curr_node.key = new_node.key; // Necessary because Ord != Eq
                            curr_node.val = new_node.val; // Overwrite value
                            ngh = NodeGetHelper::new(None, None, false);
                            break;
                        },
                        Ordering::Greater => {
                            match curr_node.right_idx {
                                Some(right_idx) => curr_idx = right_idx,
                                None => {
                                    // New max check
                                    let mut new_max_found = false;
                                    let max_node = self.arena.hard_get(self.max_idx);
                                    if new_node.key > max_node.key {
                                        new_max_found = true;
                                    }

                                    // Right insert
                                    let new_node_idx = self.arena.add(new_node);

                                    // New max update
                                    if new_max_found {
                                        self.max_idx = new_node_idx;
                                    }

                                    ngh = NodeGetHelper::new(Some(new_node_idx), Some(curr_idx), true);
                                    break;
                                }
                            }
                        }
                    }
                }

                // Link to parent
                if let Some(parent_idx) = ngh.parent_idx {
                    let parent_node = self.arena.hard_get_mut(parent_idx);
                    if ngh.is_right_child {
                        parent_node.right_idx = ngh.node_idx;
                    } else {
                        parent_node.left_idx = ngh.node_idx;
                    }
                }
            },

            // Empty tree
            None => {
                let root_idx = self.arena.add(new_node);
                self.root_idx = Some(root_idx);
                self.max_idx = root_idx;
                self.min_idx = root_idx;
            }
        }
    }

    // TODO: desc
    fn priv_remove_by_key(&mut self, key: &K) -> Option<Node<K, V>> {
        let ngh = self.priv_get(key);
        self.priv_remove(ngh)
    }

    // TODO: desc
    fn priv_remove_by_idx(&mut self, idx: usize) -> Option<Node<K, V>> {
        match self.arena.get(idx) {
            Some(node) => {
                let ngh = self.priv_get(&node.key);
                debug_assert!(ngh.node_idx.unwrap() == idx, "By-key retrieval index doesn't match arena storage index!");
                self.priv_remove(ngh)
            },
            None => None
        }
    }

    // TODO: desc
    fn priv_remove(&mut self, ngh: NodeGetHelper) -> Option<Node<K, V>> {
        match ngh.node_idx {
            Some(node_idx) => {
                let node_to_remove = self.arena.hard_get(node_idx);

                // Copy out child indexes to reduce scope of above immutable borrow
                let node_to_remove_left_idx = node_to_remove.left_idx;
                let mut node_to_remove_right_idx = node_to_remove.right_idx;

                let new_child = match (node_to_remove_left_idx, node_to_remove_right_idx) {
                    // No children
                    (None, None) => None,
                    // Left child only
                    (Some(left_idx), None) => Some(left_idx),
                    // Right child only
                    (None, Some(right_idx)) => Some(right_idx),
                    // Zero-copy algorithm for removal of node with two children:
                    // 1. Iterative search for min node in right subtree
                    // 2. Unlink min node from it's parent (has either no children or a right child)
                    // 3. Re-link min node to removed node's children
                    (Some(_), Some(right_idx)) => {
                        let mut min_idx = right_idx;
                        let mut min_parent_idx = node_idx;
                        loop {
                            let min_node = self.arena.hard_get(min_idx);
                            match min_node.left_idx {
                                // Continue search for min node
                                Some(lt_idx) => {
                                    min_idx = lt_idx;
                                    min_parent_idx = min_idx;
                                },
                                // Min node found, unlink it
                                None => {
                                    match min_node.right_idx {
                                        Some(_) => {
                                            let unlink_new_child = min_node.right_idx;
                                            if min_parent_idx == node_idx {
                                                node_to_remove_right_idx = unlink_new_child;
                                            } else {
                                                let min_parent_node = self.arena.hard_get_mut(min_parent_idx);
                                                min_parent_node.left_idx = unlink_new_child;
                                            }
                                            break;
                                        },
                                        None => {
                                            if min_parent_idx == node_idx {
                                                node_to_remove_right_idx = None;
                                            } else {
                                                let min_parent_node = self.arena.hard_get_mut(min_parent_idx);
                                                min_parent_node.left_idx = None;
                                            }
                                            break;
                                        },
                                    }
                                },
                            }
                        }

                        // Re-link min node to removed node's children
                        let min_node = self.arena.hard_get_mut(min_idx);
                        min_node.right_idx = node_to_remove_right_idx;
                        min_node.left_idx = node_to_remove_left_idx;

                        // Return as new child
                        Some(min_idx)
                    }
                };

                // Update parent or root
                match ngh.parent_idx {
                    Some(parent_idx) => {
                        let parent_node = self.arena.hard_get_mut(parent_idx);
                        if ngh.is_right_child {
                            parent_node.right_idx = new_child;
                        } else {
                            parent_node.left_idx = new_child;
                        }
                    },
                    None => {
                        self.root_idx = new_child;
                    }
                }

                // Perform removal
                let removed_node = self.arena.hard_remove(node_idx);
                self.curr_size -= 1;

                // Update min/max
                if node_idx == self.min_idx {
                    self.update_min_idx();
                } else if node_idx == self.max_idx {
                    self.update_max_idx();
                }

                Some(removed_node)

            },
            None => None
        }
    }

    /// Minimum update without recursion
    fn update_min_idx(&mut self) {
        match self.root_idx {
            Some(root_idx) => {
                let mut curr_idx = root_idx;
                loop {
                    let node = self.arena.hard_get(curr_idx);
                    match node.left_idx {
                        Some(lt_idx) => curr_idx = lt_idx,
                        None => {
                            self.min_idx = curr_idx;
                            return;
                        }
                    }
                }
            },
            None => self.min_idx = 0
        }
    }

    /// Maximum update without recursion
    fn update_max_idx(&mut self) {
        match self.root_idx {
            Some(root_idx) => {
                let mut curr_idx = root_idx;
                loop {
                    let node = self.arena.hard_get(curr_idx);
                    match node.right_idx {
                        Some(gt_idx) => curr_idx = gt_idx,
                        None => {
                            self.max_idx = curr_idx;
                            return;
                        }
                    }
                }
            },
            None => self.max_idx = 0
        }
    }

    // Traverse upward, using path information, to find first unbalanced parent
    fn find_scapegoat(&self, path: &Vec<usize>) -> Option<usize> {

        if path.len() <= 1 {
            return None;
        }

        let mut node_subtree_size = 1;
        let mut parent_path_idx = path.len() - 1;
        let mut parent_subtree_size = self.get_subtree_size(path[parent_path_idx]);

		while (parent_path_idx > 0) && (3 * node_subtree_size) <= (2 * parent_subtree_size) {
            node_subtree_size = parent_subtree_size;
            parent_path_idx -= 1;
            parent_subtree_size = self.get_subtree_size(path[parent_path_idx])
        }

        return Some(path[parent_path_idx]);
    }

    // Iterative subtree size computation
    fn get_subtree_size(&self, idx: usize) -> usize {
        let mut subtree_worklist = vec![self.arena.hard_get(idx)];
        let mut subtree_size = 0;

        while let Some(node) = subtree_worklist.pop() {
            subtree_size += 1;

            if let Some(left_idx) = node.left_idx {
                subtree_worklist.push(self.arena.hard_get(left_idx));
            }

            if let Some(right_idx) = node.right_idx {
                subtree_worklist.push(self.arena.hard_get(right_idx));
            }
        }

        subtree_size
    }

    // Iterative in-place rebuild for balanced subtree
    fn rebuild(&mut self, idx: usize) {
        let sorted_sub = self.flatten_subtree_to_sorted_idxs(idx);
        self.rebalance_subtree_from_sorted_idxs(&sorted_sub);
        self.rebal_cnt += 1;
    }

    // Flatten subtree into array of node indexs sorted by node key
    fn flatten_subtree_to_sorted_idxs(&self, idx: usize) -> Vec<usize> {
        let mut subtree_node_idx_pairs = vec![(self.arena.hard_get(idx), idx)];
        let mut subtree_worklist = vec![self.arena.hard_get(idx)];

        while let Some(node) = subtree_worklist.pop() {
            if let Some(left_idx) = node.left_idx {
                let node = self.arena.hard_get(left_idx);
                subtree_node_idx_pairs.push((node, left_idx));
                subtree_worklist.push(node);
            }

            if let Some(right_idx) = node.right_idx {
                let node = self.arena.hard_get(right_idx);
                subtree_node_idx_pairs.push((node, right_idx));
                subtree_worklist.push(node);
            }
        }

        // Leverage Node's Ord trait impl to sort by key
        // Faster than sort_by() but may not preserve order of equal elements - OK b/c tree won't have equal nodes
        subtree_node_idx_pairs.sort_unstable_by(|a, b| a.0.cmp(b.0));

        subtree_node_idx_pairs.iter().map(|(_, idx)| *idx).collect()
    }

    // Height re-balance of subtree (e.g. depth of the two subtrees of every node never differs by more than one).
    // Adapted from public interview question: https://afteracademy.com/blog/sorted-array-to-balanced-bst
    fn rebalance_subtree_from_sorted_idxs(&mut self, sorted_arena_idxs: &Vec<usize>) {

        if sorted_arena_idxs.len() <= 1 {
            return;
        }

        let sorted_last_idx =  sorted_arena_idxs.len() - 1;
        let sorted_subtree_root_idx = sorted_last_idx / 2;
        let mut subtree_worklist = Vec::new();

        // Init worklist with middle node (balanced subtree root)
        subtree_worklist.push((sorted_subtree_root_idx, NodeRebuildHelper::new(0, sorted_last_idx)));

        // Update root tree root
        if let Some(root_idx) = self.root_idx {
            if sorted_arena_idxs.contains(&root_idx) {
                self.root_idx = Some(sorted_arena_idxs[sorted_subtree_root_idx]);
            }
        }

        // Iteratively re-assign all children
        while let Some((sorted_idx, parent_nrh)) = subtree_worklist.pop() {

            let parent_node = self.arena.hard_get_mut(sorted_arena_idxs[sorted_idx]);
            parent_node.left_idx = None;
            parent_node.right_idx = None;

            // Set left child
            if parent_nrh.low_idx < parent_nrh.mid_idx {
                let child_nrh = NodeRebuildHelper::new(parent_nrh.low_idx, parent_nrh.mid_idx - 1);
                parent_node.left_idx = Some(sorted_arena_idxs[child_nrh.mid_idx]);
                subtree_worklist.push((child_nrh.mid_idx, child_nrh));
            }

            // Set right child
            if parent_nrh.mid_idx < parent_nrh.high_idx {
                let child_nrh = NodeRebuildHelper::new(parent_nrh.mid_idx + 1, parent_nrh.high_idx);
                parent_node.right_idx = Some(sorted_arena_idxs[child_nrh.mid_idx]);
                subtree_worklist.push((child_nrh.mid_idx, child_nrh));
            }
        }
    }
}

// TODO: move inside to make static private function!
// TODO: description here
fn log_3_2(val: usize) -> usize {
    let REBAL_DENUM: f64 = 3.0_f64.log(2.0); // TODO: how to eval at compile time? const doesn't work
    let rebal_num = (val as f64).log10();
    (rebal_num / REBAL_DENUM).floor() as usize
}

// Iterators -----------------------------------------------------------------------------------------------------------

// Reference iterator
impl<'a, K: Ord, V> IntoIterator for &'a SGTree<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = RefInOrderIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        RefInOrderIterator::new(&self)
    }
}

// Consuming iterator
impl<K: Ord, V> IntoIterator for SGTree<K, V> {
    type Item = (K, V);
    type IntoIter = InOrderIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        InOrderIterator::new(self)
    }
}