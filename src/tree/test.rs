use core::fmt::Debug;
use core::iter::FromIterator;
use std::collections::{BTreeMap, BTreeSet, HashSet};

use super::SGTree;

#[cfg(not(feature = "alt_impl"))]
use super::SGErr;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use smallvec::{smallvec, SmallVec};

use crate::MAX_ELEMS;

// Test Helpers --------------------------------------------------------------------------------------------------------

// Build a small tree for testing.
pub fn get_test_tree_and_keys() -> (SGTree<usize, &'static str>, Vec<usize>) {
    let keys = vec![2, 1, 6, 5, 15, 4, 12, 16, 3, 9, 13, 17, 7, 11, 14, 18, 10];
    let mut sgt = SGTree::new();

    assert!(sgt.is_empty());

    for k in &keys {
        #[cfg(not(feature = "high_assurance"))]
        sgt.insert(*k, "n/a");

        #[cfg(feature = "high_assurance")]
        assert!(sgt.insert(*k, "n/a").is_ok());

        assert_logical_invariants(&sgt);
    }

    assert!(!sgt.is_empty());
    assert!(sgt.rebal_cnt() < keys.len());

    for k in &keys {
        assert!(sgt.contains_key(k));
    }

    (sgt, keys)
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
            if let Some(left_idx) = node.left_idx() {
                let left_child_node = sgt.arena.hard_get(left_idx);
                assert!(
                    left_child_node.key < node.key,
                    "Internal invariant failed: left child >= parent!"
                );
                child_idxs.push(left_idx);
                subtree_worklist.push(left_child_node);
            }

            if let Some(right_idx) = node.right_idx() {
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

    for _ in 0..iter_cnt {
        let rand_key: usize;
        if check_invars {
            rand_key = slow_rng.gen();
        } else {
            rand_key = fast_rng.gen();
        }

        // Rand value insert
        shadow_keys.insert(rand_key);

        #[cfg(not(feature = "high_assurance"))]
        sgt.insert(rand_key, "n/a");

        #[cfg(feature = "high_assurance")]
        assert!(sgt.insert(rand_key, "n/a").is_ok());

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

            // Verify internal state post-remove
            if check_invars {
                assert_logical_invariants(&sgt);
            }
        }
    }

    let final_keys = sgt.into_iter().map(|(k, _)| k).collect::<BTreeSet<usize>>();

    if final_keys != shadow_keys {
        let diff_this: Vec<usize> = final_keys.difference(&shadow_keys).cloned().collect();
        let diff_other: Vec<usize> = shadow_keys.difference(&final_keys).cloned().collect();
        println!("Keys in SGTree and NOT in reference BTree: {:?}", diff_this);
        println!(
            "Keys in reference BTree and NOT in SGTree: {:?}",
            diff_other
        );
        panic!("Keys do not match shadow set!");
    }
}

// Identity permutation fill: (0, 0), (1, 1), (2, 2), ... , (n, n)
// This does a bunch of dynamic checks for testing purposes.
#[allow(dead_code)]
fn id_perm_fill<K, V>(sgt: &mut SGTree<K, V>)
where
    K: From<usize> + Eq + Debug + Ord,
    V: From<usize> + Eq + Debug,
{
    sgt.clear();
    for i in 0..sgt.capacity() {
        #[cfg(not(feature = "high_assurance"))]
        assert!(sgt.insert(K::from(i), V::from(i)).is_none());

        #[cfg(feature = "high_assurance")]
        assert!(sgt.insert(K::from(i), V::from(i)).is_ok());
    }

    assert_eq!(sgt.len(), sgt.capacity());
    assert_eq!(sgt.first_key_value(), Some((&K::from(0), &V::from(0))));
    assert_eq!(
        sgt.last_key_value(),
        Some((&K::from(sgt.capacity() - 1), &V::from(sgt.capacity() - 1)))
    );
}

// Tests ---------------------------------------------------------------------------------------------------------------

#[test]
fn test_tree_packing() {
    // Assumes `SG_MAX_STACK_ELEMS == 1024` (default)
    if MAX_ELEMS == 1024 {
        // No features
        #[cfg(target_pointer_width = "64")]
        #[cfg(not(feature = "high_assurance"))]
        #[cfg(not(feature = "low_mem_insert"))]
        #[cfg(not(feature = "fast_rebalance"))]
        {
            assert_eq!(core::mem::size_of::<SGTree<u32, u32>>(), 49_248);
        }

        // All features
        #[cfg(target_pointer_width = "64")]
        #[cfg(feature = "high_assurance")]
        #[cfg(feature = "low_mem_insert")]
        #[cfg(feature = "fast_rebalance")]
        {
            assert_eq!(core::mem::size_of::<SGTree<u32, u32>>(), 20_528);
        }

        // low_mem_insert only
        #[cfg(target_pointer_width = "64")]
        #[cfg(not(feature = "high_assurance"))]
        #[cfg(feature = "low_mem_insert")]
        #[cfg(not(feature = "fast_rebalance"))]
        {
            assert_eq!(core::mem::size_of::<SGTree<u32, u32>>(), 41_040);
        }

        // high_assurance only
        #[cfg(target_pointer_width = "64")]
        #[cfg(feature = "high_assurance")]
        #[cfg(not(feature = "low_mem_insert"))]
        #[cfg(not(feature = "fast_rebalance"))]
        {
            assert_eq!(core::mem::size_of::<SGTree<u32, u32>>(), 18_496);
        }

        // fast_rebalance only
        #[cfg(target_pointer_width = "64")]
        #[cfg(not(feature = "high_assurance"))]
        #[cfg(not(feature = "low_mem_insert"))]
        #[cfg(feature = "fast_rebalance")]
        {
            assert_eq!(core::mem::size_of::<SGTree<u32, u32>>(), 57_440);
        }

        // Optimize for size
        #[cfg(target_pointer_width = "64")]
        #[cfg(feature = "high_assurance")]
        #[cfg(feature = "low_mem_insert")]
        #[cfg(not(feature = "fast_rebalance"))]
        {
            assert_eq!(core::mem::size_of::<SGTree<u32, u32>>(), 16_432);
        }
    }
}

#[test]
fn test_ref_iter() {
    let (sgt, keys) = get_test_tree_and_keys();
    let mut ref_iter_keys = Vec::<usize>::new();

    for (k, _) in &sgt {
        ref_iter_keys.push(*k);
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

    for (k, _) in sgt {
        iter_keys.push(k);
    }

    let k_1 = BTreeSet::from_iter(keys.iter().cloned());
    let k_2 = BTreeSet::from_iter(iter_keys.iter().cloned());
    assert_eq!(k_1, k_2);
    assert!(iter_keys.windows(2).all(|w| w[0] < w[1]));
}

#[test]
fn test_from_iter() {
    let mut key_val_tuples = Vec::new();
    key_val_tuples.push((1, "1"));
    key_val_tuples.push((2, "2"));
    key_val_tuples.push((3, "3"));

    let sgt = SGTree::from_iter(key_val_tuples.into_iter());

    assert!(sgt.len() == 3);
    assert_eq!(
        sgt.into_iter().collect::<Vec<(usize, &str)>>(),
        vec![(1, "1"), (2, "2"), (3, "3")]
    );
}

#[cfg(feature = "high_assurance")]
#[should_panic(expected = "Stack-storage capacity exceeded!")]
#[test]
fn test_from_iter_panic() {
    let sgt_temp: SGTree<isize, isize> = SGTree::new();
    let max_capacity = sgt_temp.capacity();
    let _ = SGTree::from_iter((0..(max_capacity + 1)).map(|val| (val, val)));
}

#[test]
fn test_append() {
    let mut a = SGTree::new();

    #[cfg(not(feature = "high_assurance"))]
    {
        a.insert(1, "1");
        a.insert(2, "2");
        a.insert(3, "3");
    }

    #[cfg(feature = "high_assurance")]
    {
        assert!(a.insert(1, "1").is_ok());
        assert!(a.insert(2, "2").is_ok());
        assert!(a.insert(3, "3").is_ok());
    }

    let mut b = SGTree::new();

    #[cfg(not(feature = "high_assurance"))]
    {
        b.insert(4, "4");
        b.insert(5, "5");
        b.insert(6, "6");
        a.append(&mut b);
    }

    #[cfg(feature = "high_assurance")]
    {
        assert!(b.insert(4, "4").is_ok());
        assert!(b.insert(5, "5").is_ok());
        assert!(b.insert(6, "6").is_ok());
        assert!(a.append(&mut b).is_ok());
    }

    assert!(b.is_empty());
    assert_eq!(a.len(), 6);

    assert_eq!(
        a.into_iter().collect::<Vec<(usize, &str)>>(),
        vec![(1, "1"), (2, "2"), (3, "3"), (4, "4"), (5, "5"), (6, "6")]
    );
}

#[test]
fn test_two_child_removal_case_1() {
    let keys = vec![2, 1, 3];
    let mut sgt = SGTree::new();
    let to_remove = 2;

    for k in &keys {
        #[cfg(not(feature = "high_assurance"))]
        sgt.insert(*k, "n/a");

        #[cfg(feature = "high_assurance")]
        assert!(sgt.insert(*k, "n/a").is_ok());
    }

    sgt.remove(&to_remove);
    assert_logical_invariants(&sgt);

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
        #[cfg(not(feature = "high_assurance"))]
        sgt.insert(*k, "n/a");

        #[cfg(feature = "high_assurance")]
        assert!(sgt.insert(*k, "n/a").is_ok());
    }

    sgt.remove(&to_remove);
    assert_logical_invariants(&sgt);

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
        #[cfg(not(feature = "high_assurance"))]
        sgt.insert(*k, "n/a");

        #[cfg(feature = "high_assurance")]
        assert!(sgt.insert(*k, "n/a").is_ok());
    }

    sgt.remove(&to_remove);
    assert_logical_invariants(&sgt);

    assert_eq!(
        sgt.into_iter().map(|(k, _)| k).collect::<Vec<usize>>(),
        vec![1, 2, 4, 5, 6]
    );
}

#[test]
fn test_rand_remove() {
    let (mut sgt, mut keys) = get_test_tree_and_keys();
    let mut rng = SmallRng::from_entropy();

    // Remove half of keys at random
    let mut keys_to_remove = Vec::new();
    for _ in 0..=(keys.len() / 2) {
        keys_to_remove.push(keys.remove(rng.gen_range(0, keys.len())));
    }
    for k in &keys_to_remove {
        assert!(sgt.contains_key(k));
        let (removed_key, _) = sgt.remove_entry(k).unwrap();
        assert_eq!(*k, removed_key);
        assert_logical_invariants(&sgt);
    }
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
        #[cfg(not(feature = "high_assurance"))]
        sgt.insert(*k, "n/a");

        #[cfg(feature = "high_assurance")]
        assert!(sgt.insert(*k, "n/a").is_ok());

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

    #[cfg(not(feature = "high_assurance"))]
    {
        sgt.insert(237197427728999687, "n/a");
        sgt.insert(2328219650045037451, "n/a");
        sgt.insert(13658362701324851025, "n/a");
    }

    #[cfg(feature = "high_assurance")]
    {
        assert!(sgt.insert(237197427728999687, "n/a").is_ok());
        assert!(sgt.insert(2328219650045037451, "n/a").is_ok());
        assert!(sgt.insert(13658362701324851025, "n/a").is_ok());
    }

    sgt.remove(&13658362701324851025);

    #[cfg(not(feature = "high_assurance"))]
    {
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
    }

    #[cfg(feature = "high_assurance")]
    {
        assert!(sgt.insert(2239831466376212988, "n/a").is_ok());
        assert!(sgt.insert(15954331640746224573, "n/a").is_ok());
        assert!(sgt.insert(8202281457156668544, "n/a").is_ok());
        assert!(sgt.insert(5226917524540172628, "n/a").is_ok());
        assert!(sgt.insert(11823668523937575827, "n/a").is_ok());
        assert!(sgt.insert(13519144312507908668, "n/a").is_ok());
        assert!(sgt.insert(17799627035639903362, "n/a").is_ok());
        assert!(sgt.insert(17491737414383996868, "n/a").is_ok());
        assert!(sgt.insert(2247619647701733096, "n/a").is_ok());
        assert!(sgt.insert(15122725631405182851, "n/a").is_ok());
        assert!(sgt.insert(9837932133859010449, "n/a").is_ok());
        assert!(sgt.insert(15426779056379992972, "n/a").is_ok());
        assert!(sgt.insert(1963900452029117196, "n/a").is_ok());
        assert!(sgt.insert(1328762018325194497, "n/a").is_ok());
        assert!(sgt.insert(7471075696232724572, "n/a").is_ok());
        assert!(sgt.insert(9350363297060113585, "n/a").is_ok());
    }

    sgt.remove(&9350363297060113585);

    assert!(sgt.contains_key(&11823668523937575827));
    assert!(sgt.contains_key(&13519144312507908668));
    let critical_val = 11827258012878092103;

    #[cfg(not(feature = "high_assurance"))]
    sgt.insert(critical_val, "n/a");

    #[cfg(feature = "high_assurance")]
    assert!(sgt.insert(critical_val, "n/a").is_ok());

    assert!(sgt.contains_key(&11823668523937575827));
    assert!(sgt.contains_key(&critical_val));
    assert!(sgt.contains_key(&13519144312507908668));

    assert_eq!(sgt.rebal_cnt(), 1);
}

#[test]
fn test_logical_fuzz_fast() {
    let sgt: SGTree<usize, &str> = SGTree::new();
    logical_fuzz(sgt.capacity(), false); // Stack-only

    #[cfg(not(feature = "high_assurance"))]
    logical_fuzz(sgt.capacity() + 2_000, false); // Stack + Heap
}

#[test]
fn test_logical_fuzz_slow() {
    let sgt: SGTree<usize, &str> = SGTree::new();
    logical_fuzz(sgt.capacity(), true); // Stack-only

    #[cfg(not(feature = "high_assurance"))]
    logical_fuzz(sgt.capacity() + 2_000, true); // Stack + Heap
}

#[test]
fn test_retain() {
    let mut bt_map: BTreeMap<usize, usize> = BTreeMap::new();
    bt_map.insert(14987934384537018497, 0);
    bt_map.insert(14483576400934207487, 0);

    let mut sg_map: SGTree<usize, usize> = SGTree::new();
    #[cfg(not(feature = "high_assurance"))]
    {
        sg_map.insert(14987934384537018497, 0);
        sg_map.insert(14483576400934207487, 0);
    }
    #[cfg(feature = "high_assurance")]
    {
        assert!(sg_map.insert(14987934384537018497, 0).is_ok());
        assert!(sg_map.insert(14483576400934207487, 0).is_ok());
    }

    assert!(sg_map.iter().eq(bt_map.iter()));

    sg_map.retain(|&k, _| (k % 16766697) % 2 == 0);
    bt_map.retain(|&k, _| (k % 16766697) % 2 == 0);

    assert!(sg_map.iter().eq(bt_map.iter()));
}

#[test]
fn test_extend() {
    let mut sgt_1 = SGTree::new();
    let mut sgt_2 = SGTree::new();

    for i in 0..5 {
        #[cfg(not(feature = "high_assurance"))]
        sgt_1.insert(i, i);

        #[cfg(feature = "high_assurance")]
        assert!(sgt_1.insert(i, i).is_ok());
    }

    let iterable_1: SmallVec<[(&usize, &usize); 5]> =
        smallvec![(&0, &0), (&1, &1), (&2, &2), (&3, &3), (&4, &4)];

    assert!(sgt_1.iter().eq(iterable_1.into_iter()));

    for i in 5..10 {
        #[cfg(not(feature = "high_assurance"))]
        sgt_2.insert(i, i);

        #[cfg(feature = "high_assurance")]
        assert!(sgt_2.insert(i, i).is_ok());
    }

    let iterable_2: SmallVec<[(&usize, &usize); 5]> =
        smallvec![(&5, &5), (&6, &6), (&7, &7), (&8, &8), (&9, &9)];

    assert!(sgt_2.iter().eq(iterable_2.into_iter()));

    let iterable_3: SmallVec<[(&usize, &usize); 10]> = smallvec![
        (&0, &0),
        (&1, &1),
        (&2, &2),
        (&3, &3),
        (&4, &4),
        (&5, &5),
        (&6, &6),
        (&7, &7),
        (&8, &8),
        (&9, &9)
    ];

    sgt_1.extend(sgt_2.iter());
    assert_eq!(sgt_2.len(), 5);
    assert!(sgt_1.iter().eq(iterable_3.into_iter()));
}

#[test]
fn test_slice_search() {
    let bad_code: [u8; 8] = [0xB, 0xA, 0xA, 0xD, 0xC, 0x0, 0xD, 0xE];
    let bad_food: [u8; 8] = [0xB, 0xA, 0xA, 0xD, 0xF, 0x0, 0x0, 0xD];

    assert_eq!(std::mem::size_of_val(&bad_code), 8);
    assert_eq!(std::mem::size_of_val(&bad_food), 8);

    let mut sgt = SGTree::new();
    #[cfg(not(feature = "high_assurance"))]
    {
        sgt.insert(bad_code, "badcode");
        sgt.insert(bad_food, "badfood");
    }
    #[cfg(feature = "high_assurance")]
    {
        assert!(sgt.insert(bad_code, "badcode").is_ok());
        assert!(sgt.insert(bad_food, "badfood").is_ok());
    }

    let bad_vec: Vec<u8> = vec![0xB, 0xA, 0xA, 0xD];
    let bad_food_vec: Vec<u8> = vec![0xB, 0xA, 0xA, 0xD, 0xF, 0x0, 0x0, 0xD];
    let bad_dude_vec: Vec<u8> = vec![0xB, 0xA, 0xA, 0xD, 0xD, 0x0, 0x0, 0xD];

    assert_eq!(sgt.get(&bad_food_vec[..]), Some(&"badfood"));

    assert_eq!(sgt.get(&bad_vec[..]), None);

    assert_eq!(sgt.get(&bad_dude_vec[..]), None);
}

#[cfg(feature = "high_assurance")]
#[test]
fn test_high_assurance_insert() {
    let mut sgt: SGTree<usize, usize> = SGTree::new();
    id_perm_fill(&mut sgt);

    // Fallible insert
    assert_eq!(
        sgt.insert(usize::MAX, usize::MAX),
        Err(super::error::SGErr::StackCapacityExceeded)
    );
}

#[cfg(feature = "high_assurance")]
#[should_panic(expected = "Stack-storage capacity exceeded!")]
#[test]
fn test_high_assurance_extend_panic() {
    let mut sgt: SGTree<usize, usize> = SGTree::new();
    id_perm_fill(&mut sgt);

    let mut sgt_2: SGTree<usize, usize> = SGTree::new();
    for i in sgt_2.capacity()..(sgt_2.capacity() + 10) {
        assert!(sgt_2.insert(i, i).is_ok());
    }

    // Attempt to extend already full tree
    assert_eq!(sgt.len(), sgt.capacity());
    sgt.extend(sgt_2.into_iter()); // Should panic
}

#[test]
fn test_from_arr() {
    let sgt_1 = SGTree::from([(3, 4), (1, 2), (5, 6)]);
    let sgt_2: SGTree<_, _> = [(1, 2), (3, 4), (5, 6)].into();
    assert_eq!(sgt_1, sgt_2);

    let btm_1 = BTreeMap::from([(3, 4), (1, 2), (5, 6)]);
    assert!(sgt_1.iter().eq(btm_1.iter()));
}

#[test]
fn test_debug() {
    let sgt = SGTree::from([(3, 4), (1, 2), (5, 6)]);
    let btm = BTreeMap::from([(3, 4), (1, 2), (5, 6)]);
    assert!(sgt.iter().eq(btm.iter()));

    let sgt_str = format!("{:#?}", sgt);
    let btm_str = format!("{:#?}", btm);
    assert_eq!(sgt_str, btm_str);

    println!("DEBUG:\n{}", sgt_str);
}

#[test]
fn test_hash() {
    let sgt_1 = SGTree::from([(3, 4), (1, 2), (5, 6)]);
    let sgt_2: SGTree<_, _> = [(1, 2), (3, 4), (5, 6)].into();
    assert_eq!(sgt_1, sgt_2);

    let mut hash_set = HashSet::new();
    hash_set.insert(sgt_1);
    hash_set.insert(sgt_2);

    assert_eq!(hash_set.len(), 1);
}

#[test]
fn test_clone() {
    let sgt_1 = SGTree::from([(3, 4), (1, 2), (5, 6)]);
    let sgt_2 = sgt_1.clone();
    assert_eq!(sgt_1, sgt_2);
}

#[cfg(not(feature = "alt_impl"))] // This affects rebalance count and is experimental.
#[test]
fn test_set_rebal_param() {
    #[cfg(not(feature = "high_assurance"))]
    let data: Vec<(usize, usize)> = (0..10_000).map(|x| (x, x)).collect();

    #[cfg(feature = "high_assurance")]
    let data: Vec<(usize, usize)> = (0..100).map(|x| (x, x)).collect();

    let sgt_1 = SGTree::from_iter(data.clone().into_iter());

    // Lax rebalancing
    let mut sgt_2 = SGTree::new();
    assert!(sgt_2.set_rebal_param(0.9, 1.0).is_ok());
    sgt_2.extend(data.clone().into_iter());

    // Strict rebalancing
    let mut sgt_3 = SGTree::new();
    assert!(sgt_3.set_rebal_param(1.0, 2.0).is_ok());
    sgt_3.extend(data.into_iter());

    // Invalid rebalance factor
    assert_eq!(
        sgt_3.set_rebal_param(2.0, 1.0),
        Err(SGErr::RebalanceFactorOutOfRange)
    );

    // Alpha tuning OK
    assert!(sgt_3.rebal_cnt() > sgt_2.rebal_cnt());
    assert!(sgt_1.rebal_cnt() > sgt_2.rebal_cnt());
    assert!(sgt_3.rebal_cnt() > sgt_1.rebal_cnt());

    // Exact counts, useful to verify that different features being enabled don't change these numbers
    #[cfg(feature = "high_assurance")]
    {
        assert_eq!(sgt_1.rebal_cnt(), 52);
        assert_eq!(sgt_2.rebal_cnt(), 8);
        assert_eq!(sgt_3.rebal_cnt(), 93);
    }

    #[cfg(not(feature = "high_assurance"))]
    {
        assert_eq!(sgt_1.rebal_cnt(), 5_475);
        assert_eq!(sgt_2.rebal_cnt(), 1_192);
        assert_eq!(sgt_3.rebal_cnt(), 9_987);
    }
}
