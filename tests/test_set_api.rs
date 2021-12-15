use std::collections::BTreeSet;
use std::iter::FromIterator;

use scapegoat::{SgError, SgSet};

const DEFAULT_CAPACITY: usize = 10;

// Normal APIs ---------------------------------------------------------------------------------------------------------

#[test]
fn test_debug() {
    let sgs = SgSet::from([3, 4, 1, 2, 5, 6]);
    let bts = BTreeSet::from([3, 4, 1, 2, 5, 6]);
    assert!(sgs.iter().eq(bts.iter()));

    let sgt_str = format!("{:#?}", sgs);
    let bts_str = format!("{:#?}", bts);
    assert_eq!(sgt_str, bts_str);

    println!("DEBUG:\n{}", sgt_str);
}

#[test]
fn test_clone() {
    let sgs_1 = SgSet::from([3, 4, 1, 2, 5, 6]);
    let sgs_2 = sgs_1.clone();
    assert_eq!(sgs_1, sgs_2);
}

#[test]
fn test_basic_set_functionality() {
    let mut sgs = SgSet::<_, 10>::new();

    assert!(sgs.is_empty());

    sgs.insert(1);
    sgs.insert(2);
    sgs.insert(3);
    sgs.insert(4);
    sgs.insert(5);

    assert!(!sgs.is_empty());
    assert_eq!(sgs.len(), 5);

    for k in 1..=5 {
        assert!(sgs.contains(&k));
    }

    sgs.remove(&3);

    assert_eq!(
        (&sgs).into_iter().collect::<Vec<&usize>>(),
        vec![&1, &2, &4, &5]
    );

    let val = sgs.pop_first().unwrap();
    assert_eq!(val, 1);

    assert_eq!(
        (&sgs).into_iter().collect::<Vec<&usize>>(),
        vec![&2, &4, &5]
    );

    let val = sgs.pop_last().unwrap();
    assert_eq!(val, 5);

    assert_eq!((&sgs).into_iter().collect::<Vec<&usize>>(), vec![&2, &4]);

    assert_eq!(sgs.len(), 2);

    sgs.insert(0);
    sgs.insert(3);
    sgs.insert(10);

    assert_eq!(sgs.len(), 5);

    assert_eq!(
        (&sgs).into_iter().collect::<Vec<&usize>>(),
        vec![&0, &2, &3, &4, &10]
    );

    sgs.clear();
    assert_eq!(sgs.len(), 0);
    assert!(sgs.is_empty());

    let empty_vec: Vec<usize> = Vec::new();

    assert_eq!(sgs.into_iter().collect::<Vec<usize>>(), empty_vec);
}

#[test]
fn test_set_from_iter() {
    let keys = vec![1, 10, 100];
    let sgs = SgSet::<_, 3>::from_iter(keys.into_iter());

    assert!(sgs.len() == 3);
    assert_eq!(sgs.into_iter().collect::<Vec<usize>>(), vec![1, 10, 100]);
}

/*
TODO: re-enable for tinyvec

#[should_panic(expected = "Stack-storage capacity exceeded!")]
#[test]
fn test_set_from_iter_panic() {
    let _: SgSet<usize, DEFAULT_CAPACITY> = SgSet::from_iter(0..(DEFAULT_CAPACITY + 1));
}
*/

#[test]
fn test_set_iter() {
    let keys = vec![1, 2, 3];
    let sgs = SgSet::<_, 3>::from_iter(keys.into_iter());
    let mut sgs_iter = sgs.iter();

    assert_eq!(sgs_iter.next(), Some(&1));
    assert_eq!(sgs_iter.next(), Some(&2));
    assert_eq!(sgs_iter.next(), Some(&3));
    assert_eq!(sgs_iter.next(), None);
}

#[test]
fn test_set_append() {
    let mut a = SgSet::new();

    a.insert(1);
    a.insert(2);
    a.insert(3);

    let mut b = SgSet::<_, 10>::new();

    b.insert(4);
    b.insert(5);
    b.insert(6);
    a.append(&mut b);

    assert!(b.is_empty());
    assert_eq!(a.len(), 6);

    assert_eq!(
        a.into_iter().collect::<Vec<usize>>(),
        vec![1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn test_set_intersection() {
    let mut a = SgSet::new();

    a.insert(2);
    a.insert(4);
    a.insert(6);
    a.insert(8);
    a.insert(10);

    let mut b = SgSet::new();

    b.insert(1);
    b.insert(2);
    b.insert(3);
    b.insert(4);
    b.insert(10);

    let intersection: Vec<_> = a.intersection(&b).cloned().collect();
    assert_eq!(intersection, [2, 4, 10]);

    let c: SgSet<usize, 10> = SgSet::new();
    assert!(c.is_empty());

    let intersection: Vec<_> = c.intersection(&b).cloned().collect();
    assert_eq!(intersection, []);
}

#[test]
fn test_set_difference() {
    let a = SgSet::from_iter([1, 3, 9, 7]);
    let b = SgSet::<_, 4>::from_iter([2, 8, 9, 1]);
    assert_eq!(
        a.difference(&b).copied().collect::<Vec<usize>>(),
        vec![3, 7]
    );
}

#[test]
fn test_set_symmetric_difference() {
    let a = SgSet::from_iter([1, 2, 3, 4, 5]);
    let b = SgSet::<_, 5>::from_iter([4, 5, 6, 7, 8]);
    assert_eq!(
        a.symmetric_difference(&b).copied().collect::<Vec<usize>>(),
        vec![1, 2, 3, 6, 7, 8]
    );
}

#[test]
fn test_set_union() {
    let a: SgSet<_, DEFAULT_CAPACITY> = SgSet::from_iter([1, 3, 9, 7]);
    let b = SgSet::<_, DEFAULT_CAPACITY>::from_iter([2, 8]);
    assert_eq!(
        a.union(&b).copied().collect::<Vec<usize>>(),
        vec![1, 2, 3, 7, 8, 9]
    );
}

#[test]
fn test_set_is_superset() {
    let a = SgSet::from_iter([1, 3, 5]);
    let b = SgSet::from_iter([5, 1]);
    let c = SgSet::<_, 4>::from_iter([1, 3, 4, 5]);
    assert!(a.is_superset(&b));
    assert!(!b.is_superset(&a));
    assert!(!a.is_superset(&c));
}

#[test]
fn test_set_is_subset() {
    let a = SgSet::from_iter([2, 4, 6]);
    let b = SgSet::<_, DEFAULT_CAPACITY>::from_iter([1, 2, 3, 4, 5, 6, 7]);
    let c = SgSet::<_, DEFAULT_CAPACITY>::from_iter([1, 2, 3, 4, 5]);
    assert!(a.is_subset(&b));
    assert!(!b.is_subset(&a));
    assert!(!a.is_subset(&c));
}

#[test]
fn test_set_is_disjoint() {
    let a = SgSet::from_iter([1, 2, 3]);
    let b = SgSet::from_iter([4, 5, 6]);
    let c = SgSet::<_, 3>::from_iter([3, 4, 5]);
    assert!(a.is_disjoint(&b));
    assert!(!a.is_disjoint(&c));
}

// Fallible APIs -------------------------------------------------------------------------------------------------------

#[test]
fn test_map_insert_fallible() {
    let mut a = SgSet::<_, 3>::new();

    assert!(a.try_insert(1).is_ok());
    assert!(a.try_insert(2).is_ok());

    assert_eq!(a.try_insert(3), Ok(true));
    assert_eq!(a.try_insert(1), Ok(false));
    assert_eq!(a.try_insert(4), Err(SgError::StackCapacityExceeded));
}

#[test]
fn test_map_append_fallible() {
    let mut a = SgSet::<_, 6>::new();

    assert!(a.try_insert(1).is_ok());
    assert!(a.try_insert(2).is_ok());
    assert!(a.try_insert(3).is_ok());

    let mut b = SgSet::<_, 6>::new();

    assert!(b.try_insert(4).is_ok());
    assert!(b.try_insert(5).is_ok());
    assert!(b.try_insert(6).is_ok());
    assert!(a.try_append(&mut b).is_ok());

    assert!(b.is_empty());
    assert_eq!(b.try_insert(7), Ok(true));

    assert_eq!(a.len(), 6);
    assert_eq!(a.len(), a.capacity());
    assert_eq!(a.try_insert(7), Err(SgError::StackCapacityExceeded));

    assert_eq!(a.pop_last(), Some(6));

    b.clear();
    assert!(b.try_insert(4).is_ok());
    assert!(b.try_insert(5).is_ok());
    assert!(b.try_insert(6).is_ok());

    println!(
        "a_len: {} of {}, b_len: {}, common_len: {}",
        a.len(),
        a.capacity(),
        b.len(),
        a.iter().filter(|k| b.contains(&k)).count()
    );

    assert!(a.try_append(&mut b).is_ok());

    assert_eq!(
        a.into_iter().collect::<Vec<usize>>(),
        vec![1, 2, 3, 4, 5, 6]
    );
}

/*

CRITICAL TODO: re-enable post tinyvec

#[should_panic]
#[test]
fn test_map_insert_panic() {

    let mut a = SgSet::<_, _, 3>::new();

    assert!(a.try_insert(1).is_ok());
    assert!(a.try_insert(2).is_ok());
    assert!(a.try_insert(3).is_ok());
    assert_eq!(a.try_insert(4), Err(SgError::StackCapacityExceeded));

    a.insert(4); // panic
}
*/
