use scapegoat::{SgError, SgMap};

// Identity permutation fill: (0, 0), (1, 1), (2, 2), ... , (n, n)
fn id_perm_fill<K, V, const N: usize>(sgm: &mut SgMap<K, V, N>)
where
    K: From<usize> + Ord + Default,
    V: From<usize> + Default,
{
    sgm.clear();
    for i in 0..sgm.capacity() {
        // Can insert, not yet full
        assert!(sgm.try_insert(K::from(i), V::from(i)).is_ok());
    }
    assert!(sgm.is_full());
}

fn main() {
    let mut sgm: SgMap<usize, usize, 1024> = SgMap::new();
    id_perm_fill(&mut sgm);

    // Can't insert, full!
    assert_eq!(
        sgm.try_insert(usize::MAX, usize::MAX),
        Err(SgError::StackCapacityExceeded)
    );
}
