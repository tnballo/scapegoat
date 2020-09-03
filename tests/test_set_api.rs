use scapegoat::SGSet;

#[test]
fn test_basic_set_functionality() {
    let mut sgs = SGSet::new();

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
