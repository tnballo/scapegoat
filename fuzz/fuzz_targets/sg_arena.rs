#![no_main]

use std::collections::BTreeSet;
use std::fmt;

use libfuzzer_sys::{arbitrary::Arbitrary, fuzz_target};

use scapegoat::Arena;
use smallnum::small_unsigned;

const CAPACITY: usize = 2048;
type Idx = small_unsigned!(CAPACITY);

// Note: hard_remove() calls remove()
// We only need to test the former to get coverage, since the harness upholds valid index invariants (same as library)
#[derive(Arbitrary, Debug)]
enum ArenaMethod<K: Ord + fmt::Debug, V: fmt::Debug> {
    // new_idx_vec() always returns an array.
    New,
    // capacity() returns a constant. Omitted, irrelevant coverage.
    Add { key: K, val: V },
    Iter,
    IterMut,
    HardRemove { idx: usize },
    Len,
    IsOccupied { idx: usize },
    // sort() exercised through SgMap fuzz target (input invariants are complex, tree structure related)
    // node_size() returns a constant. Omitted, irrelevant coverage.
}

fuzz_target!(|methods: Vec<ArenaMethod<usize, usize>>| {
    let mut arena = Arena::<usize, usize, Idx, CAPACITY>::new(); // Arena under test
    let mut idx_set = BTreeSet::new(); // Currently used arena indexes

    for m in methods {
        match m {
            ArenaMethod::New => {
                arena = Arena::new();
                idx_set.clear();
            }
            ArenaMethod::Add { key, val } => {
                if idx_set.len() < CAPACITY {
                    let idx = arena.add(key, val);
                    idx_set.insert(idx);
                }
            }
            ArenaMethod::Iter => {
                let _ = arena.iter();
            }
            ArenaMethod::IterMut => {
                let _ = arena.iter_mut();
            }
            ArenaMethod::HardRemove { idx } => match idx_set.remove(&idx) {
                false => continue,
                true => {
                    let _ = arena.hard_remove(idx);
                }
            },
            ArenaMethod::Len => {
                let _ = arena.len();
            }
            ArenaMethod::IsOccupied { idx } => match idx_set.contains(&idx) {
                false => continue,
                true => assert!(arena.is_occupied(idx)),
            },
        }
    }
});
