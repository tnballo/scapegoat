use std::fmt;
use std::collections::BTreeSet;
use std::iter::FromIterator;

use super::{SGTree, Node};

use ruut;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

// TODO: convert Vec usage to BTreeSet as appropriate

// Test Helpers --------------------------------------------------------------------------------------------------------

// Build a small tree for testing
pub fn get_test_tree_and_keys() -> (SGTree<usize, &'static str>, Vec<usize>) {
    let keys = vec![2, 1, 6, 5, 15, 4, 12, 16, 3, 9, 13, 17, 7, 11, 14, 18, 10];
    let mut sgt = SGTree::new();

    assert!(sgt.is_empty());

    for k in &keys {
        sgt.insert(*k, "n/a");
        assert_logical_invariants(&sgt);
    }

    assert!(!sgt.is_empty());
    assert!(sgt.rebal_cnt() < keys.len());

    for k in &keys {
        assert!(sgt.contains_key(k));
    }

    (sgt, keys)
}

// Pretty print tree
pub fn pretty_print<K: Ord + fmt::Debug, V>(sgt: &SGTree<K, V>) {
    let sgt_lisp = stg_to_lisp_str(&sgt);
    println!("{}", ruut::prettify(sgt_lisp, ruut::InputFormat::LispLike, "unused".to_string(), "unused".to_string(), None).unwrap());
}

// Convert tree to a Lisp-like string.
fn stg_to_lisp_str<K: Ord + fmt::Debug, V>(sgt: &SGTree<K, V>) -> String {
    match sgt.root_idx {
        Some(idx) => sgt_to_lisp_str_helper(sgt, idx),
        None => String::from("()")
    }
}

// Helper function to convert tree to a Lisp-like string.
fn sgt_to_lisp_str_helper<K: Ord + fmt::Debug, V>(sgt: &SGTree<K, V>, idx: usize) -> String {
    let node = sgt.arena.hard_get(idx);
    match (node.left_idx, node.right_idx) {
        // No children
        (None, None) => format!("{:?}", node.key),
        // Left child only
        (Some(left_idx), None) => format!("{:?} ({})", node.key, sgt_to_lisp_str_helper(sgt, left_idx)),
        // Right child only
        (None, Some(right_idx)) => format!("{:?} ({})", node.key, sgt_to_lisp_str_helper(sgt, right_idx)),
        // Two children
        (Some(left_idx), Some(right_idx)) => format!(
            "{:?} ({}, {})",
            node.key,
            sgt_to_lisp_str_helper(sgt, left_idx),
            sgt_to_lisp_str_helper(sgt, right_idx)
        )
    }
}

// Verify three logical invariants for the tree:
// 1. A right child node's key is always greater than it's parent's key
// 2. A left child node's key is always less than it's parent's key
// 3. Every node has at most 1 parent
fn assert_logical_invariants<K: Ord, V>(sgt: &SGTree<K, V>) {
    if let Some(root_idx) = sgt.root_idx {
        let mut child_idxs = vec![root_idx]; // Count as "child" to make sure there's no other ref to this index
        let mut subtree_worklist = vec![sgt.arena.hard_get(root_idx)];

        while let Some(node) = subtree_worklist.pop() {
            if let Some(left_idx) = node.left_idx {
                let left_child_node = sgt.arena.hard_get(left_idx);
                assert!(left_child_node.key < node.key, "Internal invariant failed: left child >= parent!");
                child_idxs.push(left_idx);
                subtree_worklist.push(left_child_node);
            }

            if let Some(right_idx) = node.right_idx {
                let right_child_node = sgt.arena.hard_get(right_idx);
                assert!(right_child_node.key > node.key, "Internal invariant failed: right child <= parent!");
                child_idxs.push(right_idx);
                subtree_worklist.push(right_child_node);
            }
        }

        let mut dedup_child_idxs = child_idxs.clone();
        dedup_child_idxs.sort();
        dedup_child_idxs.dedup();
        assert!(dedup_child_idxs.len() == child_idxs.len(), "Internal invariant failed: node with multiple parents present!");
    }
}

// Inserts random keys, and randomly removes one of them
fn logical_fuzz(iter_cnt: usize, check_invars: bool) {

    let mut sgt = SGTree::new();
    let mut shadow_keys = BTreeSet::new();
    let mut rng = SmallRng::from_entropy();
    let mut removal_cnt = 0;

    for _ in 0..iter_cnt {

        // Rand value insert
        let rand_key: usize = rng.gen();
        sgt.insert(rand_key, "n/a");
        shadow_keys.insert(rand_key);

        // Randomly scheduled removal
        // Even though it's the key we just inserted, the tree likely rebalanced so the key could be anywhere
        if (rand_key % 10) == 0 {
            assert!(shadow_keys.remove(&rand_key));
            assert!(sgt.contains_key(&rand_key));
            sgt.remove(&rand_key);
            removal_cnt += 1;
        }

        // Verify internal state
        if check_invars {
            assert_logical_invariants(&sgt);
        }
    }

    let rebal_cnt = sgt.rebal_cnt();
    assert_eq!(sgt.into_iter().map(|(k, _)| k).collect::<BTreeSet<usize>>(), shadow_keys);
    println!("Fuzz summary: {} iterations, {} removals, {} rebalances.", iter_cnt, removal_cnt, rebal_cnt);
}

// Tests ---------------------------------------------------------------------------------------------------------------

#[test]
fn test_node_ord() {
    let n_1 = Node::new(0, 5);
    let mut n_2 = Node::new(0, 5);
    let n_3 = Node::new(1, 5);

    n_2.left_idx = Some(7);

    assert!(n_1 == n_2);
    assert!(n_3 > n_2);
    assert!(n_1 < n_3);
}

#[test]
fn test_ref_iter() {
    let (sgt, keys) = get_test_tree_and_keys();
    let mut ref_iter_keys = Vec::<usize>::new();

    println!("\nReference iterator output:\n");
    for (k, _) in &sgt {
        ref_iter_keys.push(*k);
        println!("ITER: {}", k);
    }

    let k_1 = BTreeSet::from_iter(keys.iter().cloned());
    let k_2 = BTreeSet::from_iter(ref_iter_keys.iter().cloned());
    assert_eq!(k_1, k_2);
    assert!(ref_iter_keys.windows(2).all(|w| w[0] < w[1]));
}

#[test]
fn test_iter() {
    let (sgt, keys) = get_test_tree_and_keys();
    let mut iter_keys = Vec::<usize>::new();

    println!("\nConsuming iterator output:\n");
    for (k, _) in sgt {
        iter_keys.push(k);
        println!("ITER: {}", k);
    }

    let k_1 = BTreeSet::from_iter(keys.iter().cloned());
    let k_2 = BTreeSet::from_iter(iter_keys.iter().cloned());
    assert_eq!(k_1, k_2);
    assert!(iter_keys.windows(2).all(|w| w[0] < w[1]));
}

#[test]
fn test_two_child_removal_case_1() {
    let keys = vec![2, 1, 3];
    let mut sgt = SGTree::new();
    let to_remove = 2;

    for k in &keys {
        sgt.insert(*k, "n/a");
    }

    println!("\nBefore two child removal case 1:\n");
    pretty_print(&sgt);

    sgt.remove(&to_remove);
    assert_logical_invariants(&sgt);

    println!("\nAfter two child removal case 1 (removed {}):\n", to_remove);
    pretty_print(&sgt);

    assert_eq!(sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(), vec![1, 3]);
}

#[test]
fn test_two_child_removal_case_2() {
    let keys = vec![2, 1, 4, 3];
    let mut sgt = SGTree::new();
    let to_remove = 2;

    for k in &keys {
        sgt.insert(*k, "n/a");
    }

    println!("\nBefore two child removal case 2:\n");
    pretty_print(&sgt);

    sgt.remove(&to_remove);
    assert_logical_invariants(&sgt);

    println!("\nAfter two child removal case 2 (removed {}):\n", to_remove);
    pretty_print(&sgt);

    assert_eq!(sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(), vec![1, 3, 4]);
}

#[test]
fn test_two_child_removal_case_3() {
    let keys = vec![2, 1, 5, 4, 3, 6];
    let mut sgt = SGTree::new();
    let to_remove = 3;

    for k in &keys {
        sgt.insert(*k, "n/a");
    }

    println!("\nBefore two child removal case 3:\n");
    pretty_print(&sgt);

    sgt.remove(&to_remove);
    assert_logical_invariants(&sgt);

    println!("\nAfter two child removal case 3 (removed {}):\n", to_remove);
    pretty_print(&sgt);

    assert_eq!(sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(), vec![1, 2, 4, 5, 6]);
}

#[test]
fn test_rand_remove() {

    let (mut sgt, mut keys) = get_test_tree_and_keys();
    let mut rng = SmallRng::from_entropy();

    println!("\nAfter {} insertions, {} rebalance(s):\n", keys.len(), sgt.rebal_cnt());
    pretty_print(&sgt);

    // Remove half of keys at random
    let mut keys_to_remove = Vec::new();
    for _ in 0..=(keys.len() / 2) {
        keys_to_remove.push(keys.remove(rng.gen_range(0, keys.len())));
    }
    for k in &keys_to_remove {
        println!("Removing {}", k);
        assert!(sgt.contains_key(k));
        let (removed_key, _) = sgt.remove_entry(k).unwrap();
        assert_eq!(*k, removed_key);
        assert_logical_invariants(&sgt);
    }

    println!("\nAfter {} insertions, {} rebalance(s):\n", keys_to_remove.len(), sgt.rebal_cnt());
    pretty_print(&sgt);
}

#[test]
fn test_logical_fuzz_fast() {
    logical_fuzz(5000, false);
}

#[test]
fn test_logical_fuzz_slow() {
    logical_fuzz(2500, true);
}