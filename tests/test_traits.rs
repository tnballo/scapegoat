use scapegoat::{SgMap, SgSet};

fn is_auto_trait_friendly<T: Sized + Send + Sync + Unpin>() {}
fn is_default<T: Default>() {}

#[test]
fn test_auto_traits_map() {
    is_auto_trait_friendly::<SgMap<usize, usize, 10>>();
}

#[test]
fn test_auto_traits_set() {
    is_auto_trait_friendly::<SgSet<usize, 10>>();
}

#[test]
fn test_default_map() {
    is_default::<SgMap<usize, usize, 10>>();
}

#[test]
fn test_default_set() {
    is_default::<SgSet<usize, 10>>();
}
