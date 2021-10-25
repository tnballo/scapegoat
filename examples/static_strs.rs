use scapegoat::SGMap;
use smallvec::{smallvec, SmallVec};

const FIXED_BUF_LEN: usize = 5;

// !#[no_std] demo mutable manipulation of SGMap<isize, &str>
fn main() {
    let mut example = SGMap::new();
    let mut stack_str = "your friend the";

    // Insert "dynamically" (as if heap)
    #[cfg(not(feature = "high_assurance"))]
    {
        example.insert(3, "the");
        example.insert(2, "don't blame");
        example.insert(1, "Please");
        example.insert(4, "borrow checker");
    }
    #[allow(unused_must_use)]
    #[cfg(feature = "high_assurance")]
    {
        example.insert(3, "the");
        example.insert(2, "don't blame");
        example.insert(1, "Please");
        example.insert(4, "borrow checker");
    }

    // Ordered reference iterator
    assert!(example
        .iter()
        .map(|(_, v)| *v)
        .collect::<SmallVec<[&str; FIXED_BUF_LEN]>>()
        .iter()
        .eq(["Please", "don't blame", "the", "borrow checker"].iter()));

    // Container indexing
    assert_eq!(example[&3], "the");

    // Fast (no search) head removal
    let please_tuple = example.pop_first().unwrap();
    assert_eq!(please_tuple, (1, "Please"));

    // By-predicate removal (iterates all entries)
    example.retain(|_, v| !v.contains("a"));

    // Extension
    let iterable: SmallVec<[(isize, &str); 3]> =
        smallvec![(1337, "safety!"), (0, "Leverage"), (100, "for")];
    example.extend(iterable.into_iter());

    // Value mutation
    if let Some(three_val) = example.get_mut(&3) {
        *three_val = &mut stack_str;
    }

    // New message :)
    assert!(example
        .iter()
        .map(|(_, v)| *v)
        .collect::<SmallVec<[&str; FIXED_BUF_LEN]>>()
        .iter()
        .eq([
            "Leverage",
            "your friend the",
            "borrow checker",
            "for",
            "safety!"
        ]
        .iter()));
}
