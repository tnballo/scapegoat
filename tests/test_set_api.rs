use scapegoat::SGSet;
use std::iter::FromIterator;

#[test]
fn test_basic_set_functionality() {
    let mut sgs = SGSet::new();

    assert!(sgs.is_empty());

    #[cfg(not(feature = "high_assurance"))]
    {
        sgs.insert(1);
        sgs.insert(2);
        sgs.insert(3);
        sgs.insert(4);
        sgs.insert(5);
    }

    #[allow(unused_must_use)]
    #[cfg(feature = "high_assurance")]
    {
        sgs.checked_insert(1);
        sgs.checked_insert(2);
        sgs.checked_insert(3);
        sgs.checked_insert(4);
        sgs.checked_insert(5);
    }

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

    #[cfg(not(feature = "high_assurance"))]
    {
        sgs.insert(0);
        sgs.insert(3);
        sgs.insert(10);
    }

    #[allow(unused_must_use)]
    #[cfg(feature = "high_assurance")]
    {
        sgs.checked_insert(0);
        sgs.checked_insert(3);
        sgs.checked_insert(10);
    }

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
    let sgs = SGSet::from_iter(keys.into_iter());

    assert!(sgs.len() == 3);
    assert_eq!(sgs.into_iter().collect::<Vec<usize>>(), vec![1, 10, 100]);
}

#[test]
fn test_set_iter() {
    let keys = vec![1, 2, 3];
    let sgs = SGSet::from_iter(keys.into_iter());
    let mut sgs_iter = sgs.iter();

    assert_eq!(sgs_iter.next(), Some(&1));
    assert_eq!(sgs_iter.next(), Some(&2));
    assert_eq!(sgs_iter.next(), Some(&3));
    assert_eq!(sgs_iter.next(), None);
}

#[test]
fn test_set_append() {
    let mut a = SGSet::new();

    #[cfg(not(feature = "high_assurance"))]
    {
        a.insert(1);
        a.insert(2);
        a.insert(3);
    }

    #[allow(unused_must_use)]
    #[cfg(feature = "high_assurance")]
    {
        a.checked_insert(1);
        a.checked_insert(2);
        a.checked_insert(3);
    }

    let mut b = SGSet::new();

    #[cfg(not(feature = "high_assurance"))]
    {
        b.insert(4);
        b.insert(5);
        b.insert(6);
    }

    #[allow(unused_must_use)]
    #[cfg(feature = "high_assurance")]
    {
        b.checked_insert(4);
        b.checked_insert(5);
        b.checked_insert(6);
    }

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
    let mut a = SGSet::new();

    #[cfg(not(feature = "high_assurance"))]
    {
        a.insert(2);
        a.insert(4);
        a.insert(6);
        a.insert(8);
        a.insert(10);
    }

    #[allow(unused_must_use)]
    #[cfg(feature = "high_assurance")]
    {
        a.checked_insert(2);
        a.checked_insert(4);
        a.checked_insert(6);
        a.checked_insert(8);
        a.checked_insert(10);
    }

    let mut b = SGSet::new();

    #[cfg(not(feature = "high_assurance"))]
    {
        b.insert(1);
        b.insert(2);
        b.insert(3);
        b.insert(4);
        b.insert(10);
    }

    #[allow(unused_must_use)]
    #[cfg(feature = "high_assurance")]
    {
        b.checked_insert(1);
        b.checked_insert(2);
        b.checked_insert(3);
        b.checked_insert(4);
        b.checked_insert(10);
    }

    let intersection: Vec<_> = a.intersection(&b).cloned().collect();
    assert_eq!(intersection, [2, 4, 10]);

    let c: SGSet<usize> = SGSet::new();
    assert!(c.is_empty());

    let intersection: Vec<_> = c.intersection(&b).cloned().collect();
    assert_eq!(intersection, []);
}

#[test]
fn test_set_difference() {
    let a = SGSet::from_iter(&[1, 3, 9, 7]);
    let b = SGSet::from_iter(&[2, 8, 9, 1]);
    assert_eq!(
        a.difference(&b).copied().collect::<Vec<&usize>>(),
        vec![&3, &7]
    );
}

#[test]
fn test_set_symmetric_difference() {
    let a = SGSet::from_iter(&[1, 2, 3, 4, 5]);
    let b = SGSet::from_iter(&[4, 5, 6, 7, 8]);
    assert_eq!(
        a.symmetric_difference(&b).copied().collect::<Vec<&usize>>(),
        vec![&1, &2, &3, &6, &7, &8]
    );
}

#[test]
fn test_set_union() {
    let a = SGSet::from_iter(&[1, 3, 9, 7]);
    let b = SGSet::from_iter(&[2, 8]);
    assert_eq!(
        a.union(&b).copied().collect::<Vec<&usize>>(),
        vec![&1, &2, &3, &7, &8, &9]
    );
}

#[test]
fn test_set_is_superset() {
    let a = SGSet::from_iter(&[1, 3, 5]);
    let b = SGSet::from_iter(&[5, 1]);
    let c = SGSet::from_iter(&[1, 3, 4, 5]);
    assert!(a.is_superset(&b));
    assert!(!b.is_superset(&a));
    assert!(!a.is_superset(&c));
}

#[test]
fn test_set_is_subset() {
    let a = SGSet::from_iter(&[2, 4, 6]);
    let b = SGSet::from_iter(&[1, 2, 3, 4, 5, 6, 7]);
    let c = SGSet::from_iter(&[1, 2, 3, 4, 5]);
    assert!(a.is_subset(&b));
    assert!(!b.is_subset(&a));
    assert!(!a.is_subset(&c));
}

#[test]
fn test_set_is_disjoint() {
    let a = SGSet::from_iter(&[1, 2, 3]);
    let b = SGSet::from_iter(&[4, 5, 6]);
    let c = SGSet::from_iter(&[3, 4, 5]);
    assert!(a.is_disjoint(&b));
    assert!(!a.is_disjoint(&c));
}
