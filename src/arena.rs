use crate::node::Node;

// MAJOR TODO: capacity shrink

/// TODO: description
pub struct NodeArena<K: Ord, V> {
    arena: Vec<Option<Node<K, V>>>,
}

impl<K: Ord, V> NodeArena<K, V> {

    // Public API ------------------------------------------------------------------------------------------------------

    /// Constructor
    pub fn new() -> Self {
        NodeArena {
            arena: Vec::new()
        }
    }

    /// Add node to area, growing if necessary, and return addition index.
    pub fn add(&mut self, node: Node<K, V>) -> usize {
        match self.arena.iter().position(|i| i.is_none()) {
            Some(free_idx) => {
                debug_assert!(self.arena[free_idx].is_none(), "Internal invariant failed: overwrite of allocated node!");
                self.arena[free_idx] = Some(node);
                free_idx
            },
            None => {
                self.arena.push(Some(node));
                self.arena.len() - 1
            }
        }
    }

    /// Remove node at given index from area.
    pub fn remove(&mut self, idx: usize) -> Option<Node<K,V>> {
        debug_assert!(idx < self.arena.len(), "API misuse: requested removal past last index!");
        if idx < self.arena.len() {

            // Move node to back, replacing with None, preserving order
            self.arena.push(None);
            let len = self.arena.len();
            self.arena.swap(idx, len - 1);

            // Retrieve node
            return match self.arena.pop() {
                Some(opt_node) => match opt_node {
                    Some(node) => Some(node),
                    None => {
                        debug_assert!(false, "Internal invariant failed: removal popped an empty node!");
                        None
                    }
                }
                None => None,
            }
        }

        None
    }

    /// Remove node at a known-good index (simpler callsite and error handling) from area.
    /// This function can panic. If the index might be invalid, use `remove` instead.
    pub fn hard_remove(&mut self, idx: usize) -> Node<K,V> {
        match self.remove(idx) {
            Some(node) => node,
            None => panic!("Internal invariant failed: attempted removal of node from invalid index."),
        }
    }

    /// Get a reference to a node
    pub fn get(&self, idx: usize) -> Option<&Node<K, V>> {
        match self.arena.get(idx) {
            Some(opt_node) => {
                match opt_node {
                    Some(node) => Some(node),
                    None => None,
                }
            }
            None => None,
        }
    }

    /// Get mutable reference to a node
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Node<K, V>> {
        match self.arena.get_mut(idx) {
            Some(opt_node) => {
                match opt_node {
                    Some(node) => Some(node),
                    None => None,
                }
            }
            None => None,
        }
    }

    /// Get reference to a node at a known-good index (simpler callsite and error handling).
    /// This function can panic. If the index might be invalid, use `get` instead.
    pub fn hard_get(&self, idx: usize) -> &Node<K, V> {
        match self.get(idx) {
            Some(node) => node,
            None => panic!("Internal invariant failed: attempted retrieval of node from invalid index."),
        }
    }

    /// Get mutable reference to a node at a known-good index (simpler callsite and error handling).
    /// This function can panic. If the index might be invalid, use `get_mut` instead.
    pub fn hard_get_mut(&mut self, idx: usize) -> &mut Node<K, V> {
        match self.get_mut(idx) {
            Some(node) => node,
            None => panic!("Internal invariant failed: attempted mutable retrieval of node from invalid index."),
        }
    }
}