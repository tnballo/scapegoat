use scapegoat::{SGMap, SGSet};

fn is_auto_trait_friendly<T: Sized + Send + Sync + Unpin>() {}

#[test]
fn test_auto_traits_map() {
    is_auto_trait_friendly::<SGMap<usize, usize, 10>>();
}

#[test]
fn test_auto_traits_set() {
    is_auto_trait_friendly::<SGSet<usize, 10>>();
}