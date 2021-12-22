use crate::SgMap;

pub struct OccupiedEntry<'a, K: Ord + Default, V: Default, const N: usize> {
    pub(super) node_idx: usize,
    pub(super) table: &'a mut SgMap<K, V, N>,
}

pub struct VacantEntry<'a, K: Ord + Default, V: Default, const N: usize> {
    pub(super) key: K,
    pub(super) table: &'a mut SgMap<K, V, N>,
}
