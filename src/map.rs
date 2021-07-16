use core::iter::FromIterator;
use core::ops::Index;

use crate::tree::{ConsumingIter, Iter, IterMut, SGTree};

/// Ordered map.
/// A wrapper interface for `SGTree`.
/// API examples and descriptions are all adapted or directly copied from the standard library's [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html).
#[allow(clippy::upper_case_acronyms)] // Removal == breaking change, e.g. v2.0
pub struct SGMap<K: Ord, V> {
    bst: SGTree<K, V>,
}

impl<K: Ord, V> SGMap<K, V> {
    /// Constructor.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    ///
    /// map.insert(1, "a");
    /// ```
    pub fn new() -> Self {
        SGMap { bst: SGTree::new() }
    }

    /// `#![no_std]`: total capacity, e.g. maximum number of map pairs.
    /// Attempting to insert pairs beyond capacity will panic.
    ///
    /// If using `std`: fast capacity, e.g. number of map pairs stored on the stack.
    /// Pairs inserted beyond capacity will be stored on the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::<usize, &str>::new();
    ///
    /// assert!(map.capacity() > 0)
    /// ```
    pub fn capacity(&self) -> usize {
        self.bst.capacity()
    }

    /// Moves all elements from `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut a = SGMap::new();
    /// a.insert(1, "a");
    /// a.insert(2, "b");
    /// a.insert(3, "c");
    ///
    /// let mut b = SGMap::new();
    /// b.insert(3, "d");
    /// b.insert(4, "e");
    /// b.insert(5, "f");
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.len(), 5);
    /// assert_eq!(b.len(), 0);
    ///
    /// assert_eq!(a[&1], "a");
    /// assert_eq!(a[&2], "b");
    /// assert_eq!(a[&3], "d");
    /// assert_eq!(a[&4], "e");
    /// assert_eq!(a[&5], "f");
    /// ```
    pub fn append(&mut self, other: &mut SGMap<K, V>) {
        self.bst.append(&mut other.bst);
    }

    /// Insert a key-value pair into the map.
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, the old value is returned,
    /// and the key is updated. This accommodates types that can be `==` without being identical.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// assert_eq!(map.insert(37, "a"), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Some("b"));
    /// assert_eq!(map[&37], "c");
    /// ```
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.bst.insert(key, val)
    }

    /// Gets an iterator over the entries of the map, sorted by key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert(3, "c");
    /// map.insert(2, "b");
    /// map.insert(1, "a");
    ///
    /// for (key, value) in map.iter() {
    ///     println!("{}: {}", key, value);
    /// }
    ///
    /// let (first_key, first_value) = map.iter().next().unwrap();
    /// assert_eq!((*first_key, *first_value), (1, "a"));
    /// ```
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(&self.bst)
    }

    /// Gets a mutable iterator over the entries of the map, sorted by key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// // Add 10 to the value if the key isn't "a"
    /// for (key, value) in map.iter_mut() {
    ///     if key != &"a" {
    ///         *value += 10;
    ///     }
    /// }
    ///
    /// let (second_key, second_value) = map.iter().skip(1).next().unwrap();
    /// assert_eq!((*second_key, *second_value), ("b", 12));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(&mut self.bst)
    }

    /// Removes a key from the map, returning the stored key and value if the key was previously in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.remove_entry(&1), Some((1, "a")));
    /// assert_eq!(map.remove_entry(&1), None);
    /// ```
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        self.bst.remove_entry(key)
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.bst.remove(key)
    }

    /// Returns the key-value pair corresponding to the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.bst.get_key_value(key)
    }

    /// Returns a reference to the value corresponding to the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        self.bst.get(key)
    }

    /// Get mutable reference corresponding to key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert(1, "a");
    /// if let Some(x) = map.get_mut(&1) {
    ///     *x = "b";
    /// }
    /// assert_eq!(map[&1], "b");
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.bst.get_mut(key)
    }

    /// Clears the map, removing all elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut a = SGMap::new();
    /// a.insert(1, "a");
    /// a.clear();
    /// assert!(a.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.bst.clear()
    }

    /// Returns `true` if the map contains a value for the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.bst.contains_key(key)
    }

    /// Returns `true` if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut a = SGMap::new();
    /// assert!(a.is_empty());
    /// a.insert(1, "a");
    /// assert!(!a.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bst.is_empty()
    }

    /// Returns a reference to the first key-value pair in the map.
    /// The key in this pair is the minimum key in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// assert_eq!(map.first_key_value(), None);
    /// map.insert(1, "b");
    /// map.insert(2, "a");
    /// assert_eq!(map.first_key_value(), Some((&1, &"b")));
    /// ```
    pub fn first_key_value(&self) -> Option<(&K, &V)> {
        self.bst.first_key_value()
    }

    /// Returns a reference to the first/minium key in the map, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// assert_eq!(map.first_key_value(), None);
    /// map.insert(1, "b");
    /// map.insert(2, "a");
    /// assert_eq!(map.first_key(), Some(&1));
    /// ```
    pub fn first_key(&self) -> Option<&K> {
        self.bst.first_key()
    }

    /// Removes and returns the first element in the map.
    /// The key of this element is the minimum key that was in the map.
    ///
    /// # Examples
    ///
    /// Draining elements in ascending order, while keeping a usable map each iteration.
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert(1, "a");
    /// map.insert(2, "b");
    /// while let Some((key, _val)) = map.pop_first() {
    ///     assert!((&map).into_iter().all(|(k, _v)| *k > key));
    /// }
    /// assert!(map.is_empty());
    /// ```
    pub fn pop_first(&mut self) -> Option<(K, V)> {
        self.bst.pop_first()
    }

    /// Returns a reference to the last key-value pair in the map.
    /// The key in this pair is the maximum key in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert(1, "b");
    /// map.insert(2, "a");
    /// assert_eq!(map.last_key_value(), Some((&2, &"a")));
    /// ```
    pub fn last_key_value(&self) -> Option<(&K, &V)> {
        self.bst.last_key_value()
    }

    /// Returns a reference to the last/maximum key in the map, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert(1, "b");
    /// map.insert(2, "a");
    /// assert_eq!(map.last_key(), Some(&2));
    /// ```
    pub fn last_key(&self) -> Option<&K> {
        self.bst.last_key()
    }

    /// Removes and returns the last element in the map.
    /// The key of this element is the maximum key that was in the map.
    ///
    /// # Examples
    ///
    /// Draining elements in descending order, while keeping a usable map each iteration.
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map = SGMap::new();
    /// map.insert(1, "a");
    /// map.insert(2, "b");
    /// while let Some((key, _val)) = map.pop_last() {
    ///     assert!((&map).into_iter().all(|(k, _v)| *k < key));
    /// }
    /// assert!(map.is_empty());
    /// ```
    pub fn pop_last(&mut self) -> Option<(K, V)> {
        self.bst.pop_last()
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut a = SGMap::new();
    /// assert_eq!(a.len(), 0);
    /// a.insert(1, "a");
    /// assert_eq!(a.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.bst.len()
    }
}

// Convenience Traits --------------------------------------------------------------------------------------------------

// Default constructor
impl<K: Ord, V> Default for SGMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// Indexing
impl<K: Ord, V> Index<&K> for SGMap<K, V> {
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        &self.bst[key]
    }
}

// Construction iterator
impl<K: Ord, V> FromIterator<(K, V)> for SGMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut sgm = SGMap::new();
        sgm.bst = SGTree::from_iter(iter);
        sgm
    }
}

// Extension from iterator
impl<K: Ord, V> Extend<(K, V)> for SGMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |(k, v)| {
            self.insert(k, v);
        });
    }

    /*
    // TODO: currently unstable: https://github.com/rust-lang/rust/issues/72631
    fn extend_one(&mut self, (k, v): (K, V)) {
        self.insert(k, v);
    }
    */
}

// Extension from reference iterator
impl<'a, K: Ord + Copy, V: Copy> Extend<(&'a K, &'a V)> for SGMap<K, V> {
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    }

    /*
    // TODO: currently unstable: https://github.com/rust-lang/rust/issues/72631
    fn extend_one(&mut self, (&k, &v): (&'a K, &'a V)) {
        self.insert(k, v);
    }
    */
}

/*
TODO: investigate
impl<K: Ord + PartialEq, V: PartialEq> PartialEq for SGMap<K, V> {
    fn eq(&self, other: &SGMap<K, V>) -> bool {
        (self.len() == other.len()) && (self.iter().zip(other).all(|(a, b)| a == b))
    }
}
*/

/*
TODO: investigate
impl<K: PartialOrd, V: PartialOrd> PartialOrd for SGMap<K, V> {
    fn partial_cmp(&self, other: &SGMap<K, V>) -> Option<core::cmp::Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}
*/

// Iterators -----------------------------------------------------------------------------------------------------------

// Reference iterator
impl<'a, K: Ord, V> IntoIterator for &'a SGMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Consuming iterator
impl<K: Ord, V> IntoIterator for SGMap<K, V> {
    type Item = (K, V);
    type IntoIter = ConsumingIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        ConsumingIter::new(self.bst)
    }
}
