use super::types::SortSwapVec;

// Tree Node -----------------------------------------------------------------------------------------------------------

/// Binary tree node.
pub struct Node<K: Ord, V> {
    pub key: K,
    pub val: V,
    pub left_idx: Option<usize>,
    pub right_idx: Option<usize>,
}

impl<K: Ord, V> Node<K, V> {
    /// Constructor.
    pub fn new(key: K, val: V) -> Self {
        Node {
            key,
            val,
            left_idx: None,
            right_idx: None,
        }
    }
}

// Retrieval Helper ----------------------------------------------------------------------------------------------------

/// Helper for node retrieval, usage eliminates the need a store parent pointer in each node.
pub struct NodeGetHelper {
    pub node_idx: Option<usize>,
    pub parent_idx: Option<usize>,
    pub is_right_child: bool,
}

impl NodeGetHelper {
    /// Constructor.
    pub fn new(node_idx: Option<usize>, parent_idx: Option<usize>, is_right_child: bool) -> Self {
        NodeGetHelper {
            node_idx,
            parent_idx,
            is_right_child,
        }
    }
}

// Tree Rebuild Helper -------------------------------------------------------------------------------------------------

/// Helper for in-place iterative rebuild.
pub struct NodeRebuildHelper {
    pub low_idx: usize,
    pub high_idx: usize,
    pub mid_idx: usize,
}

impl NodeRebuildHelper {
    /// Constructor.
    pub fn new(low_idx: usize, high_idx: usize) -> Self {
        debug_assert!(
            high_idx >= low_idx,
            "Node rebuild helper low/high index reversed!"
        );
        NodeRebuildHelper {
            low_idx,
            high_idx,
            mid_idx: low_idx + ((high_idx - low_idx) / 2),
        }
    }
}

// Swap History Cache --------------------------------------------------------------------------------------------------

/// TODO: documentation
/// TODO: apply this mini struct pattern elsewhere?
pub struct NodeSwapHistHelper {
    /// Map `original_idx` -> `current_idx`
    history: SortSwapVec,
}

impl NodeSwapHistHelper {

    /// TODO: docs
    pub fn new() -> NodeSwapHistHelper {
        NodeSwapHistHelper { history: SortSwapVec::new() }
    }

    /// TODO: docs
    pub fn add(&mut self, pos_1: usize, pos_2: usize) {

        debug_assert_ne!(pos_1, pos_2);

        let mut known_p1 = false;
        let mut known_p2 = false;

        // Update existing
        for (_, curr_idx) in self.history.iter_mut() {
            if *curr_idx == pos_1 {
                *curr_idx = pos_2;
                known_p1 = true;
            } else if *curr_idx == pos_2 {
                *curr_idx = pos_1;
                known_p2 = true;
            }
        }

        // Add new
        if !known_p1 {
            self.history.push((pos_1, pos_2));
        }

        // Add new
        if !known_p2 {
            self.history.push((pos_2, pos_1));
        }
    }

    // TODO: docs
    pub fn curr_idx(&self, pos: usize) -> usize {
        debug_assert!(
            self.history.iter()
                .filter(|(orig, _)| *orig == pos)
                .count() <= 1
        );

        match self.history.iter()
            .filter(|(orig, _)| *orig == pos)
            .map(|(_, curr)| *curr )
            .next() {
                Some(curr_idx) => curr_idx,
                None => pos,
        }
    }
}