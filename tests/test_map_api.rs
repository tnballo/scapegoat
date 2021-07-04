use scapegoat::SGMap;
use std::iter::FromIterator;

#[test]
fn test_basic_map_functionality() {
    let mut sgm = SGMap::new();

    assert!(sgm.is_empty());

    sgm.insert(1, "1");
    sgm.insert(2, "2");
    sgm.insert(3, "3");
    sgm.insert(4, "4");
    sgm.insert(5, "5");

    assert!(!sgm.is_empty());
    assert_eq!(sgm.len(), 5);

    for k in 1..=5 {
        assert!(sgm.contains_key(&k));
    }

    sgm.remove(&3);

    assert_eq!(
        (&sgm)
            .into_iter()
            .map(|(k, v)| (k, *v))
            .collect::<Vec<(&usize, &str)>>(),
        vec![(&1, "1"), (&2, "2"), (&4, "4"), (&5, "5")]
    );

    let (key, val) = sgm.pop_first().unwrap();
    assert_eq!(key, 1);
    assert_eq!(val, "1");

    assert_eq!(
        (&sgm)
            .into_iter()
            .map(|(k, v)| (k, *v))
            .collect::<Vec<(&usize, &str)>>(),
        vec![(&2, "2"), (&4, "4"), (&5, "5")]
    );

    let (key, val) = sgm.pop_last().unwrap();
    assert_eq!(key, 5);
    assert_eq!(val, "5");

    assert_eq!(
        (&sgm)
            .into_iter()
            .map(|(k, v)| (k, *v))
            .collect::<Vec<(&usize, &str)>>(),
        vec![(&2, "2"), (&4, "4")]
    );

    assert_eq!(sgm.len(), 2);

    sgm.insert(0, "0");
    sgm.insert(3, "3");
    sgm.insert(10, "10");

    assert_eq!(sgm.len(), 5);

    assert_eq!(
        (&sgm)
            .into_iter()
            .map(|(k, v)| (k, *v))
            .collect::<Vec<(&usize, &str)>>(),
        vec![(&0, "0"), (&2, "2"), (&3, "3"), (&4, "4"), (&10, "10")]
    );

    sgm.clear();
    assert_eq!(sgm.len(), 0);
    assert!(sgm.is_empty());

    let empty_vec: Vec<(usize, &str)> = Vec::new();

    assert_eq!(sgm.into_iter().collect::<Vec<(usize, &str)>>(), empty_vec);
}

#[test]
fn test_map_from_iter() {
    let key_val_tuples = vec![(1, "1"), (2, "2"), (3, "3")];
    let sgm = SGMap::from_iter(key_val_tuples.into_iter());

    assert!(sgm.len() == 3);
    assert_eq!(
        sgm.into_iter().collect::<Vec<(usize, &str)>>(),
        vec![(1, "1"), (2, "2"), (3, "3")]
    );
}

#[test]
fn test_map_iter() {
    let key_val_tuples = vec![(1, "1"), (2, "2"), (3, "3")];
    let sgm = SGMap::from_iter(key_val_tuples.into_iter());
    let mut sgm_iter = sgm.iter();

    assert_eq!(sgm_iter.next(), Some((&1, &"1")));
    assert_eq!(sgm_iter.next(), Some((&2, &"2")));
    assert_eq!(sgm_iter.next(), Some((&3, &"3")));
    assert_eq!(sgm_iter.next(), None);
}

#[test]
fn test_map_append() {
    let mut a = SGMap::new();
    a.insert(1, "1");
    a.insert(2, "2");
    a.insert(3, "3");

    let mut b = SGMap::new();
    b.insert(4, "4");
    b.insert(5, "5");
    b.insert(6, "6");

    a.append(&mut b);

    assert!(b.is_empty());
    assert_eq!(a.len(), 6);

    assert_eq!(
        a.into_iter().collect::<Vec<(usize, &str)>>(),
        vec![(1, "1"), (2, "2"), (3, "3"), (4, "4"), (5, "5"), (6, "6")]
    );
}
