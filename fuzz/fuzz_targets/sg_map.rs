#![no_main]
#![feature(map_first_last)]

use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::fmt::Debug;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use scapegoat::SGMap;

#[derive(Arbitrary, Debug)]
enum MapMethod<K: Ord + Debug, V: Debug> {
    // APIs ------------------------------------------------------------------------------------------------------------
    Append { other: Vec<(K, V)> },
    // capacity() returns a constant. Omitted, irrelevant coverage.
    Clear,
    ContainsKey { key: K },
    FirstKey,
    FirstKeyValue,
    Get { key: K },
    GetKeyValue { key: K },
    GetMut { key: K },
    Insert { key: K, val: V },
    IsEmpty,
    LastKey,
    LastKeyValue,
    Len,
    New,
    PopFirst,
    PopLast,
    Remove { key: K },
    RemoveEntry { key: K },
    // Trait Equivalence -----------------------------------------------------------------------------------------------
    // TODO: IntoIterator
    // TODO: Eq
    // TODO: Ord
    // TODO: PartialEq
    // TODO: PartialOrd
}

fn checked_get_len<K: Ord, V>(sg_map: &SGMap<K, V>, bt_map: &BTreeMap<K, V>) -> usize {
    let len = sg_map.len();
    assert_eq!(
        len,
        bt_map.len()
    );

    len
}

fn assert_len_unchanged<K: Ord, V>(sg_map: &SGMap<K, V>, bt_map: &BTreeMap<K, V>, old_len: usize) {
    assert_eq!(
        checked_get_len(&sg_map, &bt_map),
        old_len
    );
}

// Differential fuzzing harness
fuzz_target!(|methods: Vec<MapMethod<usize, usize>>| {
    let mut sg_map = SGMap::new();      // Data structure under test
    let mut bt_map = BTreeMap::new();   // Reference data structure

    for m in methods {
        match m {
            MapMethod::Append { other } => {
                let mut sg_other = SGMap::from_iter(other.clone());
                let mut bt_other = BTreeMap::from_iter(other);
                let len_old = checked_get_len(&sg_map, &bt_map);

                sg_map.append(&mut sg_other);
                bt_map.append(&mut bt_other);

                assert!(sg_other.is_empty());
                assert!(bt_other.is_empty());

                assert!(checked_get_len(&sg_map, &bt_map) >= len_old);
            },
            MapMethod::Clear => {
                sg_map.clear();
                bt_map.clear();

                assert!(sg_map.is_empty());
                assert!(bt_map.is_empty());

                assert_eq!(sg_map.len(), 0);
                assert_eq!(bt_map.len(), 0);
            },
            MapMethod::ContainsKey { key } => {
                assert_eq!(
                    sg_map.contains_key(&key),
                    bt_map.contains_key(&key)
                );
            },
            MapMethod::FirstKey => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                match bt_map.first_entry() {
                    Some(occupied_entry) => {
                        assert_eq!(
                            sg_map.first_key(),
                            Some(occupied_entry.key()),
                        );
                    },
                    None => {
                        assert_eq!(
                            sg_map.first_key(),
                            None
                        );
                    }
                };

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            },
            MapMethod::FirstKeyValue => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(
                    sg_map.first_key_value(),
                    bt_map.first_key_value()
                );

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            },
            MapMethod::Get { key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(
                    sg_map.get(&key),
                    bt_map.get(&key)
                );

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            },
            MapMethod::GetKeyValue { key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(
                    sg_map.get_key_value(&key),
                    bt_map.get_key_value(&key)
                );

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            },
            MapMethod::GetMut { key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(
                    sg_map.get_mut(&key),
                    bt_map.get_mut(&key)
                );

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            },
            MapMethod::Insert { key, val } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(
                    sg_map.insert(key, val),
                    bt_map.insert(key, val)
                );

                assert!(checked_get_len(&sg_map, &bt_map) >= len_old);
            },
            MapMethod::IsEmpty => {
                assert_eq!(
                    sg_map.is_empty(),
                    bt_map.is_empty(),
                );
            },
            MapMethod::LastKey => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                match bt_map.last_entry() {
                    Some(occupied_entry) => {
                        assert_eq!(
                            sg_map.last_key(),
                            Some(occupied_entry.key()),
                        );
                    },
                    None => {
                        assert_eq!(
                            sg_map.last_key(),
                            None
                        );
                    }
                };

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            },
            MapMethod::LastKeyValue => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(
                    sg_map.last_key_value(),
                    bt_map.last_key_value()
                );

                assert_len_unchanged(&sg_map, &bt_map, len_old);
            },
            MapMethod::Len => {
                assert_eq!(
                    sg_map.len(),
                    bt_map.len()
                );
            },
            MapMethod::New => {
                sg_map = SGMap::new();
                bt_map = BTreeMap::new();
            },
            MapMethod::PopFirst => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(
                    sg_map.pop_first(),
                    bt_map.pop_first()
                );

                assert!(checked_get_len(&sg_map, &bt_map) <= len_old);
            },
            MapMethod::PopLast => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(
                    sg_map.pop_last(),
                    bt_map.pop_last()
                );

                assert!(checked_get_len(&sg_map, &bt_map) <= len_old);
            },
            MapMethod::Remove { key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(
                    sg_map.remove(&key),
                    bt_map.remove(&key)
                );

                assert!(checked_get_len(&sg_map, &bt_map) <= len_old);
            },
            MapMethod::RemoveEntry { key } => {
                let len_old = checked_get_len(&sg_map, &bt_map);

                assert_eq!(
                    sg_map.remove_entry(&key),
                    bt_map.remove_entry(&key)
                );

                assert!(checked_get_len(&sg_map, &bt_map) <= len_old);
            }
        }
    }
});

