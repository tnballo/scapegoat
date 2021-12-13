#[cfg(not(feature = "high_assurance"))]
use scapegoat::SgMap;

#[cfg(feature = "high_assurance")]
use scapegoat::{SgError, SgMap};

// Identity permutation fill: (0, 0), (1, 1), (2, 2), ... , (n, n)
fn id_perm_fill<K, V>(sgm: &mut SgMap<K, V>)
where
    K: From<usize> + Ord,
    V: From<usize>,
{
    sgm.clear();
    for i in 0..sgm.capacity() {
        #[cfg(not(feature = "high_assurance"))]
        assert!(sgm.insert(K::from(i), V::from(i)).is_none());

        #[cfg(feature = "high_assurance")]
        assert!(sgm.insert(K::from(i), V::from(i)).is_ok());
    }
    assert_eq!(sgm.len(), sgm.capacity());
}

fn main() {
    let mut sgm: SgMap<usize, usize> = SgMap::new();
    id_perm_fill(&mut sgm);

    #[cfg(not(feature = "high_assurance"))]
    {
        // Would be panic if !#[no_std]
        assert_eq!(sgm.insert(usize::MAX, usize::MAX), None);
    }
    #[cfg(feature = "high_assurance")]
    {
        assert_eq!(
            sgm.insert(usize::MAX, usize::MAX),
            Err(SgError::StackCapacityExceeded)
        );
    }
}
