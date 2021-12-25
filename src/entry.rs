use crate::tree::{Idx, SmallNode};
use crate::SgMap;

/// A view into an occupied entry in a `SgMap`.
/// It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, K: Ord + Default, V: Default, const N: usize> {
    pub(super) node_idx: usize,
    pub(super) table: &'a mut SgMap<K, V, N>,
}

/// A view into a vacant entry in a `SgMap`.
/// It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, K: Ord + Default, V: Default, const N: usize> {
    pub(super) key: K,
    pub(super) table: &'a mut SgMap<K, V, N>,
}

impl<'a, K: Ord + Default, V: Default, const N: usize> VacantEntry<'a, K, V, N> {
    /// Gets a reference to the key that would be used when inserting a value
    /// through the VacantEntry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Take ownership of the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    /// use scapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    ///
    /// if let Entry::Vacant(v) = map.entry("poneyland") {
    ///     v.into_key();
    /// }
    /// ```
    pub fn into_key(self) -> K {
        self.key
    }

    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns a mutable reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    /// use scapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, u32, 2>::new();
    ///
    /// if let Entry::Vacant(o) = map.entry("poneyland") {
    ///     o.insert(37);
    /// }
    /// assert_eq!(map["poneyland"], 37);
    /// ```
    pub fn insert(self, value: V) -> &'a mut V {
        let (_, new_node_idx) = self.table.bst.priv_balancing_insert::<Idx>(self.key, value);

        self.table.bst.arena[new_node_idx].get_mut().1
    }
}

impl<'a, K: Ord + Default, V: Default, const N: usize> OccupiedEntry<'a, K, V, N> {
    /// Gets a reference to the key in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    pub fn key(&self) -> &K {
        self.table.bst.arena[self.node_idx].key()
    }

    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    /// use scapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     assert_eq!(o.get(), &12);
    /// }
    /// ```
    pub fn get(&self) -> &V {
        self.table.bst.arena[self.node_idx].val()
    }

    /// Gets a mutable reference to the value in the entry.
    ///
    /// If you need a reference to the `OccupiedEntry` that may outlive the
    /// destruction of the `Entry` value, see [`into_mut`].
    ///
    /// [`into_mut`]: OccupiedEntry::into_mut
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    /// use scapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     *o.get_mut() += 10;
    ///     assert_eq!(*o.get(), 22);
    ///
    ///     // We can use the same Entry multiple times.
    ///     *o.get_mut() += 2;
    /// }
    /// assert_eq!(map["poneyland"], 24);
    /// ```
    pub fn get_mut(&mut self) -> &mut V {
        self.table.bst.arena[self.node_idx].get_mut().1
    }

    /// Converts the entry into a mutable reference to its value.
    ///
    /// If you need multiple references to the `OccupiedEntry`, see [`get_mut`].
    ///
    /// [`get_mut`]: OccupiedEntry::get_mut
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    /// use scapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     *o.into_mut() += 10;
    /// }
    /// assert_eq!(map["poneyland"], 22);
    /// ```
    pub fn into_mut(self) -> &'a mut V {
        self.table.bst.arena[self.node_idx].get_mut().1
    }

    /// Sets the value of the entry with the `OccupiedEntry`'s key,
    /// and returns the entry's old value.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    /// use scapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     assert_eq!(o.insert(15), 12);
    /// }
    /// assert_eq!(map["poneyland"], 15);
    /// ```
    pub fn insert(&mut self, value: V) -> V {
        core::mem::replace(self.get_mut(), value)
    }

    /// Take ownership of the key and value from the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    /// use scapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     // We delete the entry from the map.
    ///     o.remove_entry();
    /// }
    ///
    /// // If now try to get the value, it will panic:
    /// // println!("{}", map["poneyland"]);
    /// ```
    pub fn remove_entry(self) -> (K, V) {
        self.table
            .bst
            .priv_remove_by_idx(self.node_idx)
            .expect("Must be occupied")
    }

    /// Takes the value of the entry out of the map, and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    /// use scapegoat::map_types::Entry;
    ///
    /// let mut map = SgMap::<&str, usize, 2>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     assert_eq!(o.remove(), 12);
    /// }
    /// // If we try to get "poneyland"'s value, it'll panic:
    /// // println!("{}", map["poneyland"]);
    /// ```
    pub fn remove(self) -> V {
        self.remove_entry().1
    }
}
