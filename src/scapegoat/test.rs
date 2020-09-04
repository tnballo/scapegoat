use std::collections::BTreeSet;
use std::fmt;
use std::iter::FromIterator;

use super::SGTree;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

// Test Helpers --------------------------------------------------------------------------------------------------------

// Build a small tree for testing.
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

// Pretty print tree.
pub fn pretty_print<K: Ord + fmt::Debug, V>(sgt: &SGTree<K, V>) {
    let sgt_lisp = sgt_to_lisp_str(&sgt);
    if sgt_lisp == "()" {
        println!("(empty tree)");
    } else {
        println!(
            "{}",
            ruut::prettify(
                sgt_lisp,
                ruut::InputFormat::LispLike,
                "unused".to_string(),
                "unused".to_string(),
                None
            )
            .unwrap()
        );
    }
}

// Convert tree to a Lisp-like string.
fn sgt_to_lisp_str<K: Ord + fmt::Debug, V>(sgt: &SGTree<K, V>) -> String {
    match sgt.root_idx {
        Some(idx) => sgt_to_lisp_str_helper(sgt, idx),
        None => String::from("()"),
    }
}

// Helper function to convert tree to a Lisp-like string.
fn sgt_to_lisp_str_helper<K: Ord + fmt::Debug, V>(sgt: &SGTree<K, V>, idx: usize) -> String {
    let node = sgt.arena.hard_get(idx);
    match (node.left_idx, node.right_idx) {
        // No children
        (None, None) => format!("{:?} [{}]", node.key, idx),
        // Left child only
        (Some(left_idx), None) => format!(
            "{:?} [{}] ({})",
            node.key,
            idx,
            sgt_to_lisp_str_helper(sgt, left_idx)
        ),
        // Right child only
        (None, Some(right_idx)) => format!(
            "{:?} [{}] ({})",
            node.key,
            idx,
            sgt_to_lisp_str_helper(sgt, right_idx)
        ),
        // Two children
        (Some(left_idx), Some(right_idx)) => format!(
            "{:?} [{}] ({}, {})",
            node.key,
            idx,
            sgt_to_lisp_str_helper(sgt, left_idx),
            sgt_to_lisp_str_helper(sgt, right_idx)
        ),
    }
}

// Verify three logical invariants for the tree:
// 1. A right child node's key is always greater than it's parent's key.
// 2. A left child node's key is always less than it's parent's key.
// 3. Every node has at most 1 parent.
fn assert_logical_invariants<K: Ord, V>(sgt: &SGTree<K, V>) {
    if let Some(root_idx) = sgt.root_idx {
        let mut child_idxs = vec![root_idx]; // Count as "child" to make sure there's no other ref to this index
        let mut subtree_worklist = vec![sgt.arena.hard_get(root_idx)];

        while let Some(node) = subtree_worklist.pop() {
            if let Some(left_idx) = node.left_idx {
                let left_child_node = sgt.arena.hard_get(left_idx);
                assert!(
                    left_child_node.key < node.key,
                    "Internal invariant failed: left child >= parent!"
                );
                child_idxs.push(left_idx);
                subtree_worklist.push(left_child_node);
            }

            if let Some(right_idx) = node.right_idx {
                let right_child_node = sgt.arena.hard_get(right_idx);
                assert!(
                    right_child_node.key > node.key,
                    "Internal invariant failed: right child <= parent!"
                );
                child_idxs.push(right_idx);
                subtree_worklist.push(right_child_node);
            }
        }

        let mut dedup_child_idxs = child_idxs.clone();
        dedup_child_idxs.sort_unstable();
        dedup_child_idxs.dedup();
        assert!(
            dedup_child_idxs.len() == child_idxs.len(),
            "Internal invariant failed: node with multiple parents present!"
        );
    }
}

// Inserts random keys, and randomly removes 20%.
fn logical_fuzz(iter_cnt: usize, check_invars: bool) {
    let mut sgt = SGTree::new();
    let mut shadow_keys = BTreeSet::new();
    let mut fast_rng = SmallRng::from_entropy();
    let mut slow_rng = rand::thread_rng();
    let mut removal_cnt = 0;

    for _ in 0..iter_cnt {
        let rand_key: usize;
        if check_invars {
            rand_key = slow_rng.gen();
        } else {
            rand_key = fast_rng.gen();
        }

        // Rand value insert
        sgt.insert(rand_key, "n/a");
        shadow_keys.insert(rand_key);

        // Verify internal state post-insert
        if check_invars {
            assert_logical_invariants(&sgt);
        }

        // Randomly scheduled removal
        // Even though it's the key we just inserted, the tree likely rebalanced so the key could be anywhere
        if (rand_key % 5) == 0 {
            assert!(shadow_keys.remove(&rand_key));
            assert!(sgt.contains_key(&rand_key));
            sgt.remove(&rand_key);
            removal_cnt += 1;

            // Verify internal state post-remove
            if check_invars {
                assert_logical_invariants(&sgt);
            }
        }
    }

    let rebal_cnt = sgt.rebal_cnt();
    let final_keys = sgt.into_iter().map(|(k, _)| k).collect::<BTreeSet<usize>>();

    if final_keys != shadow_keys {
        let diff_this: Vec<usize> = final_keys.difference(&shadow_keys).cloned().collect();
        let diff_other: Vec<usize> = shadow_keys.difference(&final_keys).cloned().collect();
        println!("Keys in SGTree and NOT in reference BTree: {:?}", diff_this);
        println!(
            "Keys in reference BTree and NOT in SGTree: {:?}",
            diff_other
        );
        assert!(false, "Keys do not match shadow set!");
    }

    println!(
        "Fuzz summary: {} iterations, {} removals, {} rebalances.",
        iter_cnt, removal_cnt, rebal_cnt
    );
}

// Tests ---------------------------------------------------------------------------------------------------------------

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

    println!(
        "\nAfter two child removal case 1 (removed {}):\n",
        to_remove
    );
    pretty_print(&sgt);

    assert_eq!(
        sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(),
        vec![1, 3]
    );
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

    println!(
        "\nAfter two child removal case 2 (removed {}):\n",
        to_remove
    );
    pretty_print(&sgt);

    assert_eq!(
        sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(),
        vec![1, 3, 4]
    );
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

    println!(
        "\nAfter two child removal case 3 (removed {}):\n",
        to_remove
    );
    pretty_print(&sgt);

    assert_eq!(
        sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(),
        vec![1, 2, 4, 5, 6]
    );
}

#[test]
fn test_rand_remove() {
    let (mut sgt, mut keys) = get_test_tree_and_keys();
    let mut rng = SmallRng::from_entropy();

    println!(
        "\nAfter {} insertions, {} rebalance(s):\n",
        keys.len(),
        sgt.rebal_cnt()
    );
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

    println!(
        "\nAfter {} insertions, {} rebalance(s):\n",
        keys_to_remove.len(),
        sgt.rebal_cnt()
    );
    pretty_print(&sgt);
}

#[test]
fn test_clear() {
    let (mut sgt, _) = get_test_tree_and_keys();
    let empty_vec: Vec<usize> = Vec::new();
    assert!(!sgt.is_empty());
    sgt.clear();
    assert!(sgt.is_empty());
    assert_eq!(
        sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(),
        empty_vec
    );
}

#[test]
fn test_len() {
    let (mut sgt, mut keys) = get_test_tree_and_keys();
    let old_sgt_len = sgt.len();
    let old_keys_len = keys.len();
    assert_eq!(old_sgt_len, old_keys_len);

    let (min_key, _) = sgt.pop_first().unwrap();
    let (max_key, _) = sgt.pop_first().unwrap();
    assert!(min_key < max_key);
    assert_eq!(sgt.len(), old_sgt_len - 2);

    keys.pop();
    keys.pop();
    assert_eq!(keys.len(), old_keys_len - 2);

    assert_eq!(sgt.len(), keys.len());
}

#[test]
fn test_first_last() {
    let keys = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut sgt = SGTree::new();
    for k in &keys {
        sgt.insert(*k, "n/a");
        assert_logical_invariants(&sgt);
        sgt.contains_key(k);
    }

    let (min_key, _) = sgt.pop_first().unwrap();
    let (max_key, _) = sgt.pop_last().unwrap();
    assert!(min_key < max_key);
    assert_eq!(min_key, 1);
    assert_eq!(max_key, 10);

    assert_eq!(sgt.first_key_value().unwrap(), (&2, &"n/a"));
    assert_eq!(sgt.last_key_value().unwrap(), (&9, &"n/a"));

    sgt.clear();
    assert!(sgt.first_key().is_none());
    assert!(sgt.last_key().is_none());
}

#[test]
fn test_subtree_rebalance() {
    let mut sgt: SGTree<usize, &str> = SGTree::new();

    sgt.insert(237197427728999687, "n/a");
    sgt.insert(2328219650045037451, "n/a");
    sgt.insert(13658362701324851025, "n/a");

    sgt.remove(&13658362701324851025);

    sgt.insert(2239831466376212988, "n/a");
    sgt.insert(15954331640746224573, "n/a");
    sgt.insert(8202281457156668544, "n/a");
    sgt.insert(5226917524540172628, "n/a");
    sgt.insert(11823668523937575827, "n/a");
    sgt.insert(13519144312507908668, "n/a");
    sgt.insert(17799627035639903362, "n/a");
    sgt.insert(17491737414383996868, "n/a");
    sgt.insert(2247619647701733096, "n/a");
    sgt.insert(15122725631405182851, "n/a");
    sgt.insert(9837932133859010449, "n/a");
    sgt.insert(15426779056379992972, "n/a");
    sgt.insert(1963900452029117196, "n/a");
    sgt.insert(1328762018325194497, "n/a");
    sgt.insert(7471075696232724572, "n/a");
    sgt.insert(9350363297060113585, "n/a");

    sgt.remove(&9350363297060113585);

    assert!(sgt.contains_key(&11823668523937575827));
    assert!(sgt.contains_key(&13519144312507908668));
    let critical_val = 11827258012878092103;

    println!("\nBefore inserting {}:\n", critical_val);
    pretty_print(&sgt);

    sgt.insert(critical_val, "n/a");

    println!("\nAfter inserting {}:\n", critical_val);
    pretty_print(&sgt);

    assert!(sgt.contains_key(&11823668523937575827));
    assert!(sgt.contains_key(&critical_val));
    assert!(sgt.contains_key(&13519144312507908668));
}

#[test]
fn test_logical_fuzz_fast() {
    logical_fuzz(5_000, false);
}

#[test]
fn test_logical_fuzz_slow() {
    logical_fuzz(5_000, true);
}
