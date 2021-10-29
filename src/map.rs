use core::borrow::Borrow;
use core::iter::FromIterator;
use core::ops::Index;
use core::fmt::{self, Debug};

use crate::tree::{ConsumingIter, Iter, IterMut, SGTree};

#[cfg(feature = "high_assurance")]
use crate::tree::SGErr;

/// Ordered map.
/// A wrapper interface for `SGTree`.
/// API examples and descriptions are all adapted or directly copied from the standard library's [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html).
#[allow(clippy::upper_case_acronyms)] // TODO: Removal == breaking change, e.g. v2.0
#[derive(Clone)]
pub struct SGMap<K: Ord, V> {
    bst: SGTree<K, V>,
}

impl<K: Ord, V> SGMap<K, V> {
    /// Makes a new, empty `SGMap`.
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
    /// Attempting to insert pairs beyond capacity will panic, unless the `high_assurance` feature is enabled.
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
    #[cfg(not(feature = "high_assurance"))]
    pub fn append(&mut self, other: &mut SGMap<K, V>) {
        self.bst.append(&mut other.bst);
    }

    /// Attempts to move all elements from `other` into `self`, leaving `other` empty.
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
    #[cfg(feature = "high_assurance")]
    pub fn append(&mut self, other: &mut SGMap<K, V>) -> Result<(), SGErr> {
        self.bst.append(&mut other.bst)
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
    #[cfg(not(feature = "high_assurance"))]
    pub fn insert(&mut self, key: K, val: V) -> Option<V>
    where
        K: Ord,
    {
        self.bst.insert(key, val)
    }

    /// Insert a key-value pair into the map.
    /// Returns `Err` if map's stack capacity is full, else the `Ok` contains:
    /// * `None` if the map did not have this key present.
    /// * The old value if the tree did have this key present (both the value and key are updated,
    /// this accommodates types that can be `==` without being identical).
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::{SGMap, SGErr};
    ///
    /// let mut map = SGMap::new();
    /// assert_eq!(map.insert(37, "a"), Ok(None));
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Ok(Some("b")));
    /// assert_eq!(map[&37], "c");
    ///
    /// let mut key = 38;
    /// while map.len() < map.capacity() {
    ///     map.insert(key, "filler");
    ///     key += 1;
    /// }
    ///
    /// assert_eq!(map.first_key(), Some(&37));
    /// assert_eq!(map.last_key(), Some(&(37 + (map.capacity() - 1))));
    /// assert_eq!(map.len(), map.capacity());
    ///
    /// assert_eq!(map.insert(key, "out of bounds"), Err(SGErr::StackCapacityExceeded));
    /// ```
    #[cfg(feature = "high_assurance")]
    pub fn insert(&mut self, key: K, val: V) -> Result<Option<V>, SGErr>
    where
        K: Ord,
    {
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

    /// Removes a key from the map, returning the stored key and value if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
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
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.bst.remove_entry(key)
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` such that `f(&k, &mut v)` returns `false`.
    /// The elements are visited in ascending key order.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut map: SGMap<i32, i32> = (0..8).map(|x| (x, x*10)).collect();
    /// // Keep only the elements with even-numbered keys.
    /// map.retain(|&k, _| k % 2 == 0);
    /// assert!(map.into_iter().eq(vec![(0, 0), (2, 20), (4, 40), (6, 60)]));
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        K: Ord,
        F: FnMut(&K, &mut V) -> bool,
    {
        self.bst.retain(|k, v| f(k, v));
    }

    /// Splits the collection into two at the given key. Returns everything after the given key,
    /// including the key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let mut a = SGMap::new();
    /// a.insert(1, "a");
    /// a.insert(2, "b");
    /// a.insert(3, "c");
    /// a.insert(17, "d");
    /// a.insert(41, "e");
    ///
    /// let b = a.split_off(&3);
    ///
    /// assert_eq!(a.len(), 2);
    /// assert_eq!(b.len(), 3);
    ///
    /// assert_eq!(a[&1], "a");
    /// assert_eq!(a[&2], "b");
    ///
    /// assert_eq!(b[&3], "c");
    /// assert_eq!(b[&17], "d");
    /// assert_eq!(b[&41], "e");
    /// ```
    pub fn split_off<Q>(&mut self, key: &Q) -> SGMap<K, V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        SGMap {
            bst: self.bst.split_off(key),
        }
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
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
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.bst.remove(key)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
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
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.bst.get_key_value(key)
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
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
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.bst.get(key)
    }

    // Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
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
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
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

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
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
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
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
    pub fn first_key_value(&self) -> Option<(&K, &V)>
    where
        K: Ord,
    {
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
    pub fn first_key(&self) -> Option<&K>
    where
        K: Ord,
    {
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
    pub fn pop_first(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
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
    pub fn last_key_value(&self) -> Option<(&K, &V)>
    where
        K: Ord,
    {
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
    pub fn last_key(&self) -> Option<&K>
    where
        K: Ord,
    {
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
    pub fn pop_last(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
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

// Default constructor.
impl<K: Ord, V> Default for SGMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// Debug
impl<K: Ord + Debug, V: Debug> Debug for SGMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

// Construct from array.
impl<K: Ord, V, const N: usize> From<[(K, V); N]> for SGMap<K, V> {
    /// ```
    /// use scapegoat::SGMap;
    ///
    /// let map1 = SGMap::from([(1, 2), (3, 4)]);
    /// let map2: SGMap<_, _> = [(1, 2), (3, 4)].into();
    /// assert_eq!(map1, map2);
    /// ```
    fn from(arr: [(K, V); N]) -> Self {
        core::array::IntoIter::new(arr).collect()
    }
}

// Indexing
impl<K: Ord, V> Index<&K> for SGMap<K, V> {
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        &self.bst[key]
    }
}

// Construct from iterator.
impl<K: Ord, V> FromIterator<(K, V)> for SGMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut sgm = SGMap::new();
        sgm.bst = SGTree::from_iter(iter);
        sgm
    }
}

// Extension from iterator.
impl<K: Ord, V> Extend<(K, V)> for SGMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.bst.extend(iter);
    }
}

// Extension from reference iterator.
impl<'a, K: Ord + Copy, V: Copy> Extend<(&'a K, &'a V)> for SGMap<K, V> {
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    }
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
