use std::iter::FromIterator;
use std::collections::BTreeMap;

use scapegoat::SGMap;

use rand::Rng;

#[test]
fn test_debug() {
    let sgm = SGMap::from([(3, 4), (1, 2), (5, 6)]);
    let btm = BTreeMap::from([(3, 4), (1, 2), (5, 6)]);
    assert!(sgm.iter().eq(btm.iter()));

    let sgt_str = format!("{:#?}", sgm);
    let btm_str = format!("{:#?}", btm);
    assert_eq!(sgt_str, btm_str);

    println!("DEBUG:\n{}", sgt_str);
}

#[test]
fn test_basic_map_functionality() {
    let mut sgm = SGMap::new();

    assert!(sgm.is_empty());

    #[cfg(not(feature = "high_assurance"))]
    {
        sgm.insert(1, "1");
        sgm.insert(2, "2");
        sgm.insert(3, "3");
        sgm.insert(4, "4");
        sgm.insert(5, "5");
    }

    #[allow(unused_must_use)]
    #[cfg(feature = "high_assurance")]
    {
        sgm.insert(1, "1");
        sgm.insert(2, "2");
        sgm.insert(3, "3");
        sgm.insert(4, "4");
        sgm.insert(5, "5");
    }

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

    #[cfg(not(feature = "high_assurance"))]
    {
        sgm.insert(0, "0");
        sgm.insert(3, "3");
        sgm.insert(10, "10");
    }

    #[allow(unused_must_use)]
    #[cfg(feature = "high_assurance")]
    {
        sgm.insert(0, "0");
        sgm.insert(3, "3");
        sgm.insert(10, "10");
    }

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

#[cfg(feature = "high_assurance")]
#[should_panic(expected = "Stack-storage capacity exceeded!")]
#[test]
fn test_map_from_iter_panic() {
    let sgm_temp: SGMap<isize, isize> = SGMap::new();
    let max_capacity = sgm_temp.capacity();
    let _ = SGMap::from_iter((0..(max_capacity + 1)).map(|val| (val, val)));
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
fn test_map_iter_mut() {
    let key_val_tuples = vec![
        ("h", 8),
        ("d", 4),
        ("b", 2),
        ("e", 5),
        ("f", 6),
        ("a", 1),
        ("g", 7),
        ("c", 3),
    ];

    let mut sgm = SGMap::from_iter(key_val_tuples.into_iter());
    assert_eq!(sgm.len(), 8);
    assert_eq!(sgm.first_key_value(), Some((&"a", &1)));
    assert_eq!(sgm.last_key_value(), Some((&"h", &8)));

    for (key, val) in sgm.iter_mut() {
        if (key != &"a") && (key != &"f") {
            *val += 10;
        }
    }

    assert_eq!(sgm.len(), 8);
    assert_eq!(sgm.first_key_value(), Some((&"a", &1)));
    assert_eq!(sgm.last_key_value(), Some((&"h", &18)));

    assert_eq!(
        sgm.into_iter().collect::<Vec<(&str, usize)>>(),
        vec![
            ("a", 1),
            ("b", 12),
            ("c", 13),
            ("d", 14),
            ("e", 15),
            ("f", 6),
            ("g", 17),
            ("h", 18),
        ],
    );
}

#[test]
fn test_map_iter_mut_rand() {
    let mut sgm = SGMap::<isize, isize>::new();
    let mut rng = rand::thread_rng();

    for _ in 0..500 {
        #[cfg(not(feature = "high_assurance"))]
        sgm.insert(rng.gen(), 0);

        #[allow(unused_must_use)]
        #[cfg(feature = "high_assurance")]
        {
            sgm.insert(rng.gen(), 0);
        }
    }

    let min_key = *sgm.first_key().unwrap();
    let max_key = *sgm.last_key().unwrap();

    let mut last_key_opt = None;
    for (key, val) in sgm.iter_mut() {
        *val += 25;
        if let Some(last_key) = last_key_opt {
            assert!(key >= last_key);
        }
        last_key_opt = Some(key);
    }

    assert_eq!(min_key, *sgm.first_key().unwrap());
    assert_eq!(max_key, *sgm.last_key().unwrap());

    let result_vec = sgm.into_iter().collect::<Vec<(isize, isize)>>();
    assert!(result_vec.as_slice().windows(2).all(|w| w[0].0 <= w[1].0));
    assert!(result_vec.iter().all(|(_, v)| *v == 25));
}

#[test]
fn test_map_append() {
    let mut a = SGMap::new();

    #[cfg(not(feature = "high_assurance"))]
    {
        a.insert(1, "1");
        a.insert(2, "2");
        a.insert(3, "3");
    }

    #[allow(unused_must_use)]
    #[cfg(feature = "high_assurance")]
    {
        a.insert(1, "1");
        a.insert(2, "2");
        a.insert(3, "3");
    }

    let mut b = SGMap::new();

    #[cfg(not(feature = "high_assurance"))]
    {
        b.insert(4, "4");
        b.insert(5, "5");
        b.insert(6, "6");
        a.append(&mut b);
    }

    #[allow(unused_must_use)]
    #[cfg(feature = "high_assurance")]
    {
        b.insert(4, "4");
        b.insert(5, "5");
        b.insert(6, "6");
        a.append(&mut b);
    }

    assert!(b.is_empty());
    assert_eq!(a.len(), 6);

    assert_eq!(
        a.into_iter().collect::<Vec<(usize, &str)>>(),
        vec![(1, "1"), (2, "2"), (3, "3"), (4, "4"), (5, "5"), (6, "6")]
    );
}
