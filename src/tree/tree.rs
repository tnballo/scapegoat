use core::borrow::Borrow;
use core::cmp::Ordering;
use core::iter::FromIterator;
use core::mem;
use core::ops::Index;

use super::arena::NodeArena;
use super::iter::{ConsumingIter, Iter, IterMut};
use super::node::{Node, NodeGetHelper, NodeRebuildHelper};
use super::types::{
    Idx, IdxVec, RebuildMetaVec, SortMetaVec, SortNodeRefIdxPairVec, SortNodeRefVec,
};

#[cfg(feature = "high_assurance")]
use super::error::SGErr;

use crate::{ALPHA_DENOM, ALPHA_NUM};

#[allow(unused_imports)]
use micromath::F32Ext;
use smallnum::SmallUnsigned;
use smallvec::smallvec;

/// A memory-efficient, self-balancing binary search tree.
#[allow(clippy::upper_case_acronyms)] // TODO: Removal == breaking change, e.g. v2.0
pub struct SGTree<K: Ord, V> {
    pub(crate) arena: NodeArena<K, V>,
    pub(crate) root_idx: Option<Idx>,
    max_idx: Idx,
    min_idx: Idx,
    curr_size: Idx,
    max_size: Idx,
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
    /// Attempting to insert pairs beyond capacity will panic, unless the `high_assurance` feature is enabled.
    ///
    /// If using `std`: fast capacity, e.g. number of tree pairs stored on the stack.
    /// Pairs inserted beyond capacity will be stored on the heap.
    pub fn capacity(&self) -> usize {
        self.arena.capacity()
    }

    /// Moves all elements from `other` into `self`, leaving `other` empty.
    #[cfg(not(feature = "high_assurance"))]
    pub fn append(&mut self, other: &mut SGTree<K, V>)
    where
        K: Ord,
    {
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
            if let Some(node) = other.arena.remove(arena_idx as Idx) {
                self.insert(node.key, node.val);
            }
        }
        other.clear();
    }

    /// Attempts to move all elements from `other` into `self`, leaving `other` empty.
    #[cfg(feature = "high_assurance")]
    pub fn append(&mut self, other: &mut SGTree<K, V>) -> Result<(), SGErr> {
        // Nothing to append!
        if other.is_empty() {
            return Ok(());
        }

        // Nothing to append to!
        if self.is_empty() {
            mem::swap(self, other);
            return Ok(());
        }

        // Rip elements directly out of other's arena and clear it
        if (self.len() + other.len()) <= self.capacity() {
            for arena_idx in 0..other.arena.len() {
                if let Some(node) = other.arena.remove(arena_idx as Idx) {
                    self.insert(node.key, node.val)?;
                }
            }
            other.clear();
        } else {
            // Preemptive - we haven't mutated `self` or `other`!
            // Caller can assume unchanged state.
            return Err(SGErr::StackCapacityExceeded);
        }

        Ok(())
    }

    /// Insert a key-value pair into the tree.
    /// If the tree did not have this key present, `None` is returned.
    /// If the tree did have this key present, the value is updated, the old value is returned,
    /// and the key is updated. This accommodates types that can be `==` without being identical.
    #[cfg(not(feature = "high_assurance"))]
    pub fn insert(&mut self, key: K, val: V) -> Option<V>
    where
        K: Ord,
    {
        self.priv_balancing_insert(key, val)
    }

    /// Insert a key-value pair into the tree.
    /// Returns `Err` if tree's stack capacity is full, else the `Ok` contains:
    /// * `None` if the tree did not have this key present.
    /// * The old value if the tree did have this key present (both the value and key are updated,
    /// this accommodates types that can be `==` without being identical).
    #[cfg(feature = "high_assurance")]
    pub fn insert(&mut self, key: K, val: V) -> Result<Option<V>, SGErr> {
        match self.capacity() > self.len() {
            true => Ok(self.priv_balancing_insert(key, val)),
            false => Err(SGErr::StackCapacityExceeded),
        }
    }

    /// Gets an iterator over the entries of the tree, sorted by key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use scapegoat::SGTree;
    ///
    /// let mut tree = SGTree::new();
    /// tree.insert(3, "c");
    /// tree.insert(2, "b");
    /// tree.insert(1, "a");
    ///
    /// for (key, value) in tree.iter() {
    ///     println!("{}: {}", key, value);
    /// }
    ///
    /// let (first_key, first_value) = tree.iter().next().unwrap();
    /// assert_eq!((*first_key, *first_value), (1, "a"));
    /// ```
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(self)
    }

    /// Gets a mutable iterator over the entries of the tree, sorted by key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use scapegoat::SGTree;
    ///
    /// let mut tree = SGTree::new();
    /// tree.insert("a", 1);
    /// tree.insert("b", 2);
    /// tree.insert("c", 3);
    ///
    /// // Add 10 to the value if the key isn't "a"
    /// for (key, value) in tree.iter_mut() {
    ///     if key != &"a" {
    ///         *value += 10;
    ///     }
    /// }
    ///
    /// let (second_key, second_value) = tree.iter().skip(1).next().unwrap();
    /// assert_eq!((*second_key, *second_value), ("b", 12));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(self)
    }

    /// Removes a key from the tree, returning the stored key and value if the key was previously in the tree.
    ///
    /// The key may be any borrowed form of the map’s key type, but the ordering
    /// on the borrowed form must match the ordering on the key type.
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
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
    ///
    /// The key may be any borrowed form of the map’s key type, but the ordering
    /// on the borrowed form must match the ordering on the key type.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.remove_entry(key).map(|(_, v)| v)
    }

    /// Retains only the elements specified by the predicate.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
        K: Ord,
    {
        self.priv_drain_filter(|k, v| !f(k, v));
    }

    /// Splits the collection into two at the given key. Returns everything after the given key, including the key.
    pub fn split_off<Q>(&mut self, key: &Q) -> Self
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.priv_drain_filter(|k, _| k >= key)
    }

    /// Returns the key-value pair corresponding to the given key.
    ///
    /// The supplied key may be any borrowed form of the map’s key type,
    /// but the ordering on the borrowed form must match the ordering on the key type.
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
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
    ///
    /// The key may be any borrowed form of the map’s key type, but the ordering
    /// on the borrowed form must match the ordering on the key type.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.get_key_value(key).map(|(_, v)| v)
    }

    /// Get mutable reference corresponding to key.
    ///
    /// The key may be any borrowed form of the map’s key type,
    /// but the ordering on the borrowed form must match the ordering on the key type.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
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
    ///
    /// The key may be any borrowed form of the map’s key type, but the
    /// ordering on the borrowed form must match the ordering on the key type.
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.get(key).is_some()
    }

    /// Returns `true` if the tree contains no elements.
    pub fn is_empty(&self) -> bool {
        self.root_idx.is_none()
    }

    /// Returns a reference to the first key-value pair in the tree.
    /// The key in this pair is the minimum key in the tree.
    pub fn first_key_value(&self) -> Option<(&K, &V)>
    where
        K: Ord,
    {
        self.arena
            .get(self.min_idx)
            .map(|node| (&node.key, &node.val))
    }

    /// Returns a reference to the first/minium key in the tree, if any.
    pub fn first_key(&self) -> Option<&K>
    where
        K: Ord,
    {
        self.first_key_value().map(|(k, _)| k)
    }

    /// Removes and returns the first element in the tree.
    /// The key of this element is the minimum key that was in the tree.
    pub fn pop_first(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
        match self.priv_remove_by_idx(self.min_idx) {
            Some(node) => Some((node.key, node.val)),
            None => None,
        }
    }

    /// Returns a reference to the last key-value pair in the tree.
    /// The key in this pair is the maximum key in the tree.
    pub fn last_key_value(&self) -> Option<(&K, &V)>
    where
        K: Ord,
    {
        self.arena
            .get(self.max_idx)
            .map(|node| (&node.key, &node.val))
    }

    /// Returns a reference to the last/maximum key in the tree, if any.
    pub fn last_key(&self) -> Option<&K>
    where
        K: Ord,
    {
        self.last_key_value().map(|(k, _)| k)
    }

    /// Removes and returns the last element in the tree.
    /// The key of this element is the maximum key that was in the tree.
    pub fn pop_last(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
        match self.priv_remove_by_idx(self.max_idx) {
            Some(node) => Some((node.key, node.val)),
            None => None,
        }
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        self.curr_size as usize
    }

    /// Get the number of times this tree rebalanced itself (for testing and/or performance engineering).
    /// This count will wrap if `usize::MAX` is exceeded.
    pub fn rebal_cnt(&self) -> usize {
        self.rebal_cnt
    }

    // Crate-internal API ----------------------------------------------------------------------------------------------

    // Remove a node by index.
    // A wrapper for by-key removal, traversal is still required to determine node parent.
    pub(crate) fn priv_remove_by_idx(&mut self, idx: Idx) -> Option<Node<K, V>> {
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

    // Flatten subtree into array of node indexs sorted by node key
    pub(crate) fn flatten_subtree_to_sorted_idxs(&self, idx: Idx) -> IdxVec {
        let mut subtree_node_idx_pairs: SortNodeRefIdxPairVec<K, V> =
            smallvec![(self.arena.hard_get(idx), idx)];
        let mut subtree_worklist: SortNodeRefVec<K, V> = smallvec![self.arena.hard_get(idx)];

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

    /// Sort the internal arena such that logically contiguous nodes are in-order (by key).
    pub(crate) fn sort_arena(&mut self) {
        if let Some(root_idx) = self.root_idx {
            let mut sort_metadata = self
                .arena
                .iter()
                .filter(|n| n.is_some())
                .map(|n| n.as_ref().unwrap())
                .map(|n| self.priv_get(&n.key))
                .collect::<SortMetaVec>();

            sort_metadata.sort_by_key(|ngh| &self.arena.hard_get(ngh.node_idx.unwrap()).key);
            let sorted_root_idx = self.arena.sort(root_idx, sort_metadata);

            self.root_idx = Some(sorted_root_idx);
            self.update_max_idx();
            self.update_min_idx();
        }
    }

    // Private API -----------------------------------------------------------------------------------------------------

    // Iterative search. If key found, returns node idx, parent idx, and a bool indicating if node is right child
    fn priv_get<Q>(&self, key: &Q) -> NodeGetHelper
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        match self.root_idx {
            Some(root_idx) => {
                let mut opt_parent_idx = None;
                let mut curr_idx = root_idx;
                let mut is_right_child = false;
                loop {
                    let node = self.arena.hard_get(curr_idx);
                    match key.cmp(&node.key.borrow()) {
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

    // Sorted insert of node into the tree (outer).
    // Re-balances the tree if necessary.
    fn priv_balancing_insert(&mut self, key: K, val: V) -> Option<V> {
        let mut path = IdxVec::new();
        let new_node = Node::new(key, val);

        // Potential rebalance
        let opt_val = self.priv_insert(&mut path, new_node);
        if path.len() > Self::alpha_balance_depth(self.max_size) {
            if let Some(scapegoat_idx) = self.find_scapegoat(&path) {
                self.rebuild(scapegoat_idx);
            }
        }

        opt_val
    }

    // Sorted insert of node into the tree (inner).
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
                debug_assert_eq!(self.curr_size, 0);
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
    fn priv_remove_by_key<Q>(&mut self, key: &Q) -> Option<Node<K, V>>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        let ngh = self.priv_get(key);
        self.priv_remove(ngh)
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

    /// Temporary internal drain_filter() implementation. To be replaced/supplemented with a public implementation.
    fn priv_drain_filter<Q, F>(&mut self, mut pred: F) -> Self
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
        F: FnMut(&Q, &mut V) -> bool,
    {
        /*
        // TODO: make public version with this signature
        pub fn drain_filter<F>(&mut self, pred: F) -> DrainFilter<'_, K, V, F>
        where
            K: Ord,
            F: FnMut(&K, &mut V) -> bool,
        {
        */

        // TODO: this implementation is rather inefficient!

        let mut key_idxs = IdxVec::new();
        let mut remove_idxs = IdxVec::new();

        // Below iter_mut() will want to sort, require want consistent indexes, so do work up front
        self.sort_arena();

        // Safely treat mutable ref as immutable, init list of node's arena indexes
        for (k, _) in &(*self) {
            let ngh = self.priv_get(k.borrow());
            debug_assert!(ngh.node_idx.is_some());
            key_idxs.push(ngh.node_idx.unwrap());
        }

        // Filter arena index list to those not matching predicate
        for (i, (k, v)) in self.iter_mut().enumerate() {
            if pred(k.borrow(), v) {
                remove_idxs.push(key_idxs[i]);
            }
        }

        // Drain non-matches
        let mut drained_sgt = Self::new();
        for i in remove_idxs {
            if let Some(node) = self.priv_remove_by_idx(i) {
                #[cfg(not(feature = "high_assurance"))]
                {
                    drained_sgt.insert(node.key, node.val);
                }
                #[allow(unused_must_use)]
                #[cfg(feature = "high_assurance")]
                {
                    drained_sgt.insert(node.key, node.val);
                }
            }
        }

        drained_sgt
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

    // Traverse upward, using path information, to find first unbalanced parent.
    // Uses the algorithm proposed in the original paper (Galperin and Rivest, 1993).
    #[cfg(not(feature = "alt_impl"))]
    fn find_scapegoat(&self, path: &[Idx]) -> Option<Idx> {
        if path.len() <= 1 {
            return None;
        }

        let mut node_subtree_size = 1;
        let mut parent_path_idx = path.len() - 1;
        let mut parent_subtree_size = self.get_subtree_size(path[parent_path_idx]);

        while (parent_path_idx > 0)
            && (ALPHA_DENOM * node_subtree_size as f32) <= (ALPHA_NUM * parent_subtree_size as f32)
        {
            node_subtree_size = parent_subtree_size;
            parent_path_idx -= 1;
            parent_subtree_size = self.get_subtree_size(path[parent_path_idx]);

            debug_assert!(parent_subtree_size > node_subtree_size);
        }

        Some(path[parent_path_idx])
    }

    // Traverse upward, using path information, to find first unbalanced parent.
    // Uses an alternate algorithm proposed in Galperin's PhD thesis (1996).
    #[cfg(feature = "alt_impl")]
    fn find_scapegoat(&self, path: &[Idx]) -> Option<Idx> {
        if path.len() <= 1 {
            return None;
        }

        let mut i = 0;
        let mut node_subtree_size = 1;
        let mut parent_path_idx = path.len() - 1;
        let mut parent_subtree_size = self.get_subtree_size(path[parent_path_idx]);

        while (parent_path_idx > 0) && (i <= Self::alpha_balance_depth(node_subtree_size)) {
            node_subtree_size = parent_subtree_size;
            parent_path_idx -= 1;
            i += 1;
            parent_subtree_size = self.get_subtree_size(path[parent_path_idx]);

            debug_assert!(parent_subtree_size > node_subtree_size);
        }

        Some(path[parent_path_idx])
    }

    // Iterative subtree size computation
    fn get_subtree_size(&self, idx: Idx) -> Idx {
        let mut subtree_worklist: SortNodeRefVec<K, V> = smallvec![self.arena.hard_get(idx)];
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
    fn rebuild(&mut self, idx: Idx) {
        let sorted_sub = self.flatten_subtree_to_sorted_idxs(idx);
        self.rebalance_subtree_from_sorted_idxs(idx, &sorted_sub);
        self.rebal_cnt = self.rebal_cnt.wrapping_add(1);
    }

    // Height re-balance of subtree (e.g. depth of the two subtrees of every node never differs by more than one).
    // Adapted from public interview question: https://afteracademy.com/blog/sorted-array-to-balanced-bst
    fn rebalance_subtree_from_sorted_idxs(
        &mut self,
        old_subtree_root_idx: Idx,
        sorted_arena_idxs: &[Idx],
    ) {
        if sorted_arena_idxs.len() <= 1 {
            return;
        }

        debug_assert!(
            self.root_idx.is_some(),
            "Internal invariant failed: rebalance of multi-node tree without root!"
        );

        let sorted_last_idx = (sorted_arena_idxs.len() - 1) as Idx;
        let subtree_root_sorted_idx = sorted_last_idx / 2;
        let subtree_root_arena_idx = sorted_arena_idxs[subtree_root_sorted_idx.usize()];
        let mut subtree_worklist = RebuildMetaVec::new();

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
            let parent_node = self
                .arena
                .hard_get_mut(sorted_arena_idxs[sorted_idx.usize()]);
            parent_node.left_idx = None;
            parent_node.right_idx = None;

            // Set left child
            if parent_nrh.low_idx < parent_nrh.mid_idx {
                let child_nrh = NodeRebuildHelper::new(parent_nrh.low_idx, parent_nrh.mid_idx - 1);
                parent_node.left_idx = Some(sorted_arena_idxs[child_nrh.mid_idx.usize()]);
                subtree_worklist.push((child_nrh.mid_idx, child_nrh));
            }

            // Set right child
            if parent_nrh.mid_idx < parent_nrh.high_idx {
                let child_nrh = NodeRebuildHelper::new(parent_nrh.mid_idx + 1, parent_nrh.high_idx);
                parent_node.right_idx = Some(sorted_arena_idxs[child_nrh.mid_idx.usize()]);
                subtree_worklist.push((child_nrh.mid_idx, child_nrh));
            }
        }

        debug_assert!(
            self.get_subtree_size(subtree_root_arena_idx) == (sorted_arena_idxs.len() as Idx),
            "Internal invariant failed: rebalance dropped node count!"
        );
    }

    // Alpha weight balance computation helper.
    fn alpha_balance_depth(val: Idx) -> usize {
        // log base (1/alpha), hence (denom/num)
        (val as f32).log(ALPHA_DENOM / ALPHA_NUM).floor() as usize
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
            #[cfg(not(feature = "high_assurance"))]
            sgt.insert(k, v);

            #[cfg(feature = "high_assurance")]
            sgt.insert(k, v).expect("Stack-storage capacity exceeded!");
        }

        sgt
    }
}

// Reference iterator, mutable
impl<'a, K: Ord, V> IntoIterator for &'a mut SGTree<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// Reference iterator, immutable
impl<'a, K: Ord, V> IntoIterator for &'a SGTree<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Consuming iterator
impl<K: Ord, V> IntoIterator for SGTree<K, V> {
    type Item = (K, V);
    type IntoIter = ConsumingIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        ConsumingIter::new(self)
    }
}
