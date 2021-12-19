use scapegoat::SgMap;
use tinyvec::{array_vec, ArrayVec};

// This const is an argument to each generic constructor below.
// So we'll use *only the bare minimum* memory for 5 elements.
// - Stack RAM usage can be precisely controlled: per map instance (constructor call-site).
// - To save executable RAM/ROM (monomorphization!), stick to a global capacity like this.
const CAPACITY: usize = 5;

// #![no_std] demo mutable manipulation of SgMap<isize, &str, 5>
fn main() {
    let mut example = SgMap::<_, _, CAPACITY>::new(); // BTreeMap::new()
    let mut stack_str = "your friend the";

    // Insert "dynamically" (as if heap)
    example.insert(3, "the");
    example.insert(2, "don't blame");
    example.insert(1, "Please");

    // Fallible insert variant
    assert!(example.try_insert(4, "borrow checker").is_ok());

    // Ordered reference iterator
    assert!(example
        .iter()
        .map(|(_, v)| *v)
        .collect::<ArrayVec<[&str; CAPACITY]>>()
        .iter()
        .eq(["Please", "don't blame", "the", "borrow checker"].iter()));

    // Container indexing
    assert_eq!(example[&3], "the");

    // Head removal
    let please_tuple = example.pop_first().unwrap();
    assert_eq!(please_tuple, (1, "Please"));

    // By-predicate removal
    example.retain(|_, v| !v.contains("a"));

    // Extension
    let iterable = array_vec![
        [(isize, &str); CAPACITY] =>
        (1337, "safety!"), (0, "Leverage"), (100, "for")
    ];
    example.extend(iterable.into_iter());

    // Value mutation
    if let Some(three_val) = example.get_mut(&3) {
        *three_val = &mut stack_str;
    }

    // New message :)
    assert!(example
        .into_values()
        .collect::<ArrayVec<[&str; CAPACITY]>>()
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
