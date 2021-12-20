use crate::SgMap;

pub struct OccupiedEntry<'a, K: Ord + Default, V: Default, const N: usize> {
    table: &'a mut SgMap<K, V, N>,
}

pub struct VacantEntry<'a, K: Ord + Default, V: Default, const N: usize> {
    key: K,
    table: &'a mut SgMap<K, V, N>,
}
