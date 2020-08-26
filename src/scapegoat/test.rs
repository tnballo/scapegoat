use std::fmt;
use std::collections::BTreeSet;
use std::iter::FromIterator;

use super::{SGTree, Node};

use ruut;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

// Test Helpers --------------------------------------------------------------------------------------------------------

// Build a small tree for testing
pub fn get_test_tree_and_keys() -> (SGTree<usize, &'static str>, Vec<usize>) {
    let keys = vec![2, 1, 6, 5, 15, 4, 12, 16, 3, 9, 13, 17, 7, 11, 14, 18, 10];
    let mut sgt = SGTree::new();

    assert!(sgt.is_empty());

    for k in &keys {
        sgt.insert(*k, "n/a");
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
    let keys = vec![1, 2, 3];
    let mut sgt = SGTree::new();
    let to_remove = 3;

    for k in &keys {
        sgt.insert(*k, "n/a");
    }

    println!("\nBefore two child removal case 1:\n");
    pretty_print(&sgt);

    sgt.remove(&to_remove);

    println!("\nAfter two child removal case 1 (removed {}):\n", to_remove);
    pretty_print(&sgt);

    assert_eq!(sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(), vec![1, 2]);
}

#[test]
fn test_two_child_removal_case_2() {
    let keys = vec![2, 1, 4, 3];
    let mut sgt = SGTree::new();
    let to_remove = 3;

    for k in &keys {
        sgt.insert(*k, "n/a");
    }

    println!("\nBefore two child removal case 2:\n");
    pretty_print(&sgt);

    sgt.remove(&to_remove);

    println!("\nAfter two child removal case 2 (removed {}):\n", to_remove);
    pretty_print(&sgt);

    assert_eq!(sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(), vec![1, 2, 4]);
}

#[test]
fn test_two_child_removal_case_3() {
    let keys = vec![2, 1, 5, 4, 3];
    let mut sgt = SGTree::new();
    let to_remove = 3;

    for k in &keys {
        sgt.insert(*k, "n/a");
    }

    println!("\nBefore two child removal case 3:\n");
    pretty_print(&sgt);

    sgt.remove(&to_remove);

    println!("\nAfter two child removal case 3 (removed {}):\n", to_remove);
    pretty_print(&sgt);

    assert_eq!(sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(), vec![1, 2, 4, 5]);
}

#[test]
fn test_two_child_removal_case_4() {
    let keys = vec![2, 1, 4, 5];
    let mut sgt = SGTree::new();
    let to_remove = 5;

    for k in &keys {
        sgt.insert(*k, "n/a");
    }

    println!("\nBefore two child removal case 4:\n");
    pretty_print(&sgt);

    sgt.remove(&to_remove);

    println!("\nAfter two child removal case 4 (removed {}):\n", to_remove);
    pretty_print(&sgt);

    assert_eq!(sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(), vec![1, 2, 4]);
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
    }

    println!("\nAfter {} insertions, {} rebalance(s):\n", keys_to_remove.len(), sgt.rebal_cnt());
    pretty_print(&sgt);
}