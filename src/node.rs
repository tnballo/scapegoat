use std::cmp::Ordering;

/// Binary tree node
pub struct Node<K: Ord, V> {
    pub key: K,
    pub val: V,
    pub left_idx: Option<usize>,
    pub right_idx: Option<usize>,
}

impl<K: Ord, V> Node<K, V> {

    /// Constructor
    pub fn new(key: K, val: V) -> Self {
        Node {
            key,
            val,
            left_idx: None,
            right_idx: None,
        }
    }
}

impl<K: Ord, V> Ord for Node<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K: Ord, V> PartialOrd for Node<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Eq for Node<K, V> {}
impl<K: Ord, V> PartialEq for Node<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}


/// Helper for node retrieval, eliminates the need a store parent pointer in each node
pub struct NodeGetHelper {
    pub node_idx: Option<usize>,
    pub parent_idx: Option<usize>,
    pub is_right_child: bool,
}

impl NodeGetHelper{
    /// Constructor
    pub fn new(node_idx: Option<usize>, parent_idx: Option<usize>, is_right_child: bool) -> Self {
        NodeGetHelper {
            node_idx,
            parent_idx,
            is_right_child
        }
    }
}

/// Helper for in-place iterative rebuild.
pub struct NodeRebuildHelper {
    pub low_idx: usize,
    pub high_idx: usize,
    pub mid_idx: usize,
}

impl NodeRebuildHelper {

    /// Constructor
    pub fn new(low_idx: usize, high_idx: usize) -> Self {
        debug_assert!(high_idx >= low_idx, "Node rebuild helper low/high index reversed!");
        NodeRebuildHelper {
            low_idx,
            high_idx,
            mid_idx: low_idx + ((high_idx - low_idx) / 2),
        }
    }
}