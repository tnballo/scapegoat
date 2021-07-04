use core::cmp::Ordering;
use core::iter::FromIterator;
use core::mem;
use core::ops::Index;

use smallvec::{SmallVec, smallvec};
use libm::{floor, log2, log10};

mod arena;
pub use arena::NodeArena;

mod node;
pub use node::{Node, NodeGetHelper, NodeRebuildHelper};

#[cfg(test)]
mod test;

mod iter;
pub use iter::{InOrderIterator, RefInOrderIterator};

use crate::MAX_ELEMS;

type IdxVec = SmallVec<[usize; MAX_ELEMS]>;

/// A memory-efficient, self-balancing binary search tree.
#[allow(clippy::upper_case_acronyms)] // Removal == breaking change, e.g. v2.0
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

    /// Constructor.
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

    /// `#![no_std]`: total capacity, e.g. maximum number of tree pairs.
    /// Attempting to insert pairs beyond capacity will panic.
    ///
    /// If using `std`: fast capacity, e.g. number of tree pairs stored on the stack.
    /// Pairs inserted beyond capacity will be stored on the heap.
    pub fn capacity(&self) -> usize {
        self.arena.capacity()
    }

    /// Moves all elements from `other` into `self`, leaving `other` empty.
    pub fn append(&mut self, other: &mut SGTree<K, V>) {
        // Nothing to append!
        if other.is_empty() {
            return;
        }

        // Nothing to append to!
        if self.is_empty() {
            mem::swap(self, other);
            return;
        }

        // Rip elements directly out of other's arena and clear it
        for arena_idx in 0..other.arena.len() {
            if let Some(node) = other.arena.remove(arena_idx) {
                self.insert(node.key, node.val);
            }
        }
        other.clear();
    }

    /// Insert a key-value pair into the tree.
    /// If the tree did not have this key present, `None` is returned.
    /// If the tree did have this key present, the value is updated, the old value is returned,
    /// and the key is updated. This accommodates types that can be `==` without being identical.
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        let mut path = IdxVec::new();
        let new_node = Node::new(key, val);

        // Optional rebalance
        let opt_val = self.priv_insert(&mut path, new_node);
        if path.len() > log_3_2(self.max_size) {
            if let Some(scapegoat_idx) = self.find_scapegoat(&path) {
                self.rebuild(scapegoat_idx);
            }
        }

        opt_val
    }

    /// Removes a key from the tree, returning the stored key and value if the key was previously in the tree.
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
            }
            None => None,
        }
    }

    /// Removes a key from the tree, returning the value at the key if the key was previously in the tree.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.remove_entry(key) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    /// Returns the key-value pair corresponding to the given key.
    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        let ngh = self.priv_get(key);
        match ngh.node_idx {
            Some(idx) => {
                let node = self.arena.hard_get(idx);
                Some((&node.key, &node.val))
            }
            None => None,
        }
    }

    /// Returns a reference to the value corresponding to the given key.
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
            }
            None => None,
        }
    }

    /// Clears the tree, removing all elements.
    pub fn clear(&mut self) {
        let rebal_cnt = self.rebal_cnt;
        *self = SGTree::new();
        self.rebal_cnt = rebal_cnt;
    }

    /// Returns `true` if the tree contains a value for the given key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Returns `true` if the tree contains no elements.
    pub fn is_empty(&self) -> bool {
        self.root_idx.is_none()
    }

    /// Returns a reference to the first key-value pair in the tree.
    /// The key in this pair is the minimum key in the tree.
    pub fn first_key_value(&self) -> Option<(&K, &V)> {
        match self.arena.get(self.min_idx) {
            Some(node) => Some((&node.key, &node.val)),
            None => None,
        }
    }

    /// Returns a reference to the first/minium key in the tree, if any.
    pub fn first_key(&self) -> Option<&K> {
        match self.first_key_value() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }

    /// Removes and returns the first element in the tree.
    /// The key of this element is the minimum key that was in the tree.
    pub fn pop_first(&mut self) -> Option<(K, V)> {
        match self.priv_remove_by_idx(self.min_idx) {
            Some(node) => Some((node.key, node.val)),
            None => None,
        }
    }

    /// Returns a reference to the last key-value pair in the tree.
    /// The key in this pair is the maximum key in the tree.
    pub fn last_key_value(&self) -> Option<(&K, &V)> {
        match self.arena.get(self.max_idx) {
            Some(node) => Some((&node.key, &node.val)),
            None => None,
        }
    }

    /// Returns a reference to the last/maximum key in the tree, if any.
    pub fn last_key(&self) -> Option<&K> {
        match self.last_key_value() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }

    /// Removes and returns the last element in the tree.
    /// The key of this element is the maximum key that was in the tree.
    pub fn pop_last(&mut self) -> Option<(K, V)> {
        match self.priv_remove_by_idx(self.max_idx) {
            Some(node) => Some((node.key, node.val)),
            None => None,
        }
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        self.curr_size
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
                        Ordering::Less => match node.left_idx {
                            Some(lt_idx) => {
                                opt_parent_idx = Some(curr_idx);
                                curr_idx = lt_idx;
                                is_right_child = false;
                            }
                            None => return NodeGetHelper::new(None, None, false),
                        },
                        Ordering::Equal => {
                            return NodeGetHelper::new(
                                Some(curr_idx),
                                opt_parent_idx,
                                is_right_child,
                            );
                        }
                        Ordering::Greater => match node.right_idx {
                            Some(gt_idx) => {
                                opt_parent_idx = Some(curr_idx);
                                curr_idx = gt_idx;
                                is_right_child = true;
                            }
                            None => return NodeGetHelper::new(None, None, false),
                        },
                    }
                }
            }
            None => NodeGetHelper::new(None, None, false),
        }
    }

    // Sorted insert of node into the tree.
    // Maintains a traversal path to avoid nodes needing to maintain a parent index.
    // If a node with the same key existed, overwrites both that nodes key and value with the new one's and returns the old value.
    fn priv_insert(&mut self, path: &mut IdxVec, new_node: Node<K, V>) -> Option<V> {
        match self.root_idx {
            // Sorted insert
            Some(idx) => {
                // Iterative traversal
                let mut curr_idx = idx;
                let mut opt_val = None;
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

                                    ngh = NodeGetHelper::new(
                                        Some(new_node_idx),
                                        Some(curr_idx),
                                        false,
                                    );
                                    break;
                                }
                            }
                        }
                        Ordering::Equal => {
                            curr_node.key = new_node.key; // Necessary b/c Eq may not consider all struct members
                            opt_val = Some(mem::replace(&mut curr_node.val, new_node.val)); // Overwrite value
                            ngh = NodeGetHelper::new(None, None, false);
                            break;
                        }
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

                                    ngh = NodeGetHelper::new(
                                        Some(new_node_idx),
                                        Some(curr_idx),
                                        true,
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }

                // Link to parent
                if let Some(parent_idx) = ngh.parent_idx {
                    self.curr_size += 1;
                    self.max_size += 1;

                    let parent_node = self.arena.hard_get_mut(parent_idx);
                    if ngh.is_right_child {
                        parent_node.right_idx = ngh.node_idx;
                    } else {
                        parent_node.left_idx = ngh.node_idx;
                    }
                }

                // Return old value if overwritten
                opt_val
            }

            // Empty tree
            None => {
                self.curr_size += 1;
                self.max_size += 1;

                let root_idx = self.arena.add(new_node);
                self.root_idx = Some(root_idx);
                self.max_idx = root_idx;
                self.min_idx = root_idx;

                None
            }
        }
    }

    // Remove a node by key.
    fn priv_remove_by_key(&mut self, key: &K) -> Option<Node<K, V>> {
        let ngh = self.priv_get(key);
        self.priv_remove(ngh)
    }

    // Remove a node by index.
    // A wrapper for by-key removal, traversal is still required to determine node parent.
    fn priv_remove_by_idx(&mut self, idx: usize) -> Option<Node<K, V>> {
        match self.arena.get(idx) {
            Some(node) => {
                let ngh = self.priv_get(&node.key);
                debug_assert!(
                    ngh.node_idx.unwrap() == idx,
                    "By-key retrieval index doesn't match arena storage index!"
                );
                self.priv_remove(ngh)
            }
            None => None,
        }
    }

    // Remove a node from the tree, re-linking remaining nodes as necessary.
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
                                    min_parent_idx = min_idx;
                                    min_idx = lt_idx;
                                }
                                // Min node found, unlink it
                                None => match min_node.right_idx {
                                    Some(_) => {
                                        let unlink_new_child = min_node.right_idx;
                                        if min_parent_idx == node_idx {
                                            node_to_remove_right_idx = unlink_new_child;
                                        } else {
                                            let min_parent_node =
                                                self.arena.hard_get_mut(min_parent_idx);
                                            min_parent_node.left_idx = unlink_new_child;
                                        }
                                        break;
                                    }
                                    None => {
                                        if min_parent_idx == node_idx {
                                            node_to_remove_right_idx = None;
                                        } else {
                                            let min_parent_node =
                                                self.arena.hard_get_mut(min_parent_idx);
                                            min_parent_node.left_idx = None;
                                        }
                                        break;
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
                    }
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
            }
            None => None,
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
            }
            None => self.min_idx = 0,
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
            }
            None => self.max_idx = 0,
        }
    }

    // Traverse upward, using path information, to find first unbalanced parent
    fn find_scapegoat(&self, path: &[usize]) -> Option<usize> {
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

        Some(path[parent_path_idx])
    }

    // Iterative subtree size computation
    fn get_subtree_size(&self, idx: usize) -> usize {
        let mut subtree_worklist: SmallVec<[&Node<K, V>; MAX_ELEMS]> = smallvec![self.arena.hard_get(idx)];
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
        self.rebalance_subtree_from_sorted_idxs(idx, &sorted_sub);
        self.rebal_cnt += 1;
    }

    // Flatten subtree into array of node indexs sorted by node key
    fn flatten_subtree_to_sorted_idxs(&self, idx: usize) -> IdxVec {
        let mut subtree_node_idx_pairs: SmallVec<[(&Node<K, V>, usize); MAX_ELEMS]> = smallvec![(self.arena.hard_get(idx), idx)];
        let mut subtree_worklist: SmallVec<[&Node<K, V>; MAX_ELEMS]> = smallvec![self.arena.hard_get(idx)];

        while let Some(node) = subtree_worklist.pop() {
            if let Some(left_idx) = node.left_idx {
                let left_child_node = self.arena.hard_get(left_idx);
                subtree_node_idx_pairs.push((left_child_node, left_idx));
                subtree_worklist.push(left_child_node);
            }

            if let Some(right_idx) = node.right_idx {
                let right_child_node = self.arena.hard_get(right_idx);
                subtree_node_idx_pairs.push((right_child_node, right_idx));
                subtree_worklist.push(right_child_node);
            }
        }

        // Sort by Node key
        // Faster than sort_by() but may not preserve order of equal elements - OK b/c tree won't have equal nodes
        subtree_node_idx_pairs.sort_unstable_by(|a, b| a.0.key.cmp(&b.0.key));

        subtree_node_idx_pairs.iter().map(|(_, idx)| *idx).collect()
    }

    // Height re-balance of subtree (e.g. depth of the two subtrees of every node never differs by more than one).
    // Adapted from public interview question: https://afteracademy.com/blog/sorted-array-to-balanced-bst
    fn rebalance_subtree_from_sorted_idxs(
        &mut self,
        old_subtree_root_idx: usize,
        sorted_arena_idxs: &[usize],
    ) {
        if sorted_arena_idxs.len() <= 1 {
            return;
        }

        debug_assert!(
            self.root_idx.is_some(),
            "Internal invariant failed: rebalance of multi-node tree without root!"
        );

        let sorted_last_idx = sorted_arena_idxs.len() - 1;
        let subtree_root_sorted_idx = sorted_last_idx / 2;
        let subtree_root_arena_idx = sorted_arena_idxs[subtree_root_sorted_idx];
        let mut subtree_worklist = SmallVec::<[(usize, NodeRebuildHelper); MAX_ELEMS]>::new();

        // Init worklist with middle node (balanced subtree root)
        subtree_worklist.push((
            subtree_root_sorted_idx,
            NodeRebuildHelper::new(0, sorted_last_idx),
        ));

        // Update tree root or subtree parent
        if let Some(root_idx) = self.root_idx {
            if sorted_arena_idxs.contains(&root_idx) {
                self.root_idx = Some(subtree_root_arena_idx);
            } else {
                let old_subtree_root = self.arena.hard_get(old_subtree_root_idx);
                let ngh = self.priv_get(&old_subtree_root.key);
                debug_assert!(
                    ngh.parent_idx.is_some(),
                    "Internal invariant failed: rebalance of non-root parent-less node!"
                );
                if let Some(parent_idx) = ngh.parent_idx {
                    let parent_node = self.arena.hard_get_mut(parent_idx);
                    if ngh.is_right_child {
                        parent_node.right_idx = Some(subtree_root_arena_idx);
                    } else {
                        parent_node.left_idx = Some(subtree_root_arena_idx);
                    }
                }
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

        debug_assert!(
            self.get_subtree_size(subtree_root_arena_idx) == sorted_arena_idxs.len(),
            "Internal invariant failed: rebalance dropped node count!"
        );
    }
}

// Convenience Traits --------------------------------------------------------------------------------------------------

// Default constructor
impl<K: Ord, V> Default for SGTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// Indexing
impl<K: Ord, V> Index<&K> for SGTree<K, V> {
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        self.get(key).expect("No value found for key")
    }
}

// Iterators -----------------------------------------------------------------------------------------------------------

// Construction iterator
impl<K: Ord, V> FromIterator<(K, V)> for SGTree<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut sgt = SGTree::new();

        for (k, v) in iter {
            sgt.insert(k, v);
        }

        sgt
    }
}

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

// Helpers -------------------------------------------------------------------------------------------------------------

// TODO: move inside to make static private function!
// TODO: this needs verification - seems to rebalance a little too aggressively
fn log_3_2(val: usize) -> usize {
    let rebal_denum: f64 = log2(3.0_f64); // TODO: how to eval at compile time? const doesn't work
    let rebal_num = log10(val as f64);
    floor(rebal_num / rebal_denum) as usize
}
