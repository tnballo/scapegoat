use crate::entry::{OccupiedEntry, VacantEntry};
use crate::map::SgMap;
use crate::tree::{IntoIter as TreeIntoIter, Iter as TreeIter, IterMut as TreeIterMut};

// General Iterators ---------------------------------------------------------------------------------------------------

/// An iterator over the entries of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`iter`][crate::map::SgMap::iter] method on [`SgMap`][crate::map::SgMap].
/// documentation for more.
///
pub struct Iter<'a, T: Ord + Default, V: Default, const N: usize> {
    ref_iter: TreeIter<'a, T, V, N>,
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iter<'a, K, V, N> {
    /// Construct reference iterator.
    pub(crate) fn new(map: &'a SgMap<K, V, N>) -> Self {
        Iter {
            ref_iter: TreeIter::new(&map.bst),
        }
    }
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iterator for Iter<'a, K, V, N> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.ref_iter.next()
    }
}

impl<'a, K: Ord + Default, V: Default, const N: usize> ExactSizeIterator for Iter<'a, K, V, N> {
    fn len(&self) -> usize {
        self.ref_iter.len()
    }
}

/// An owning iterator over the entries of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`into_iter`][crate::map::SgMap::into_iter] method on [`SgMap`][crate::map::SgMap].
/// documentation for more.
pub struct IntoIter<K: Ord + Default, V: Default, const N: usize> {
    cons_iter: TreeIntoIter<K, V, N>,
}

impl<K: Ord + Default, V: Default, const N: usize> IntoIter<K, V, N> {
    /// Construct owning iterator.
    pub(crate) fn new(map: SgMap<K, V, N>) -> Self {
        IntoIter {
            cons_iter: TreeIntoIter::new(map.bst),
        }
    }
}

impl<K: Ord + Default, V: Default, const N: usize> Iterator for IntoIter<K, V, N> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.cons_iter.next()
    }
}

impl<K: Ord + Default, V: Default, const N: usize> ExactSizeIterator for IntoIter<K, V, N> {
    fn len(&self) -> usize {
        self.cons_iter.len()
    }
}

/// An mutable iterator over the entries of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`iter_mut`][crate::map::SgMap::iter_mut] method on [`SgMap`][crate::map::SgMap].
/// documentation for more.
pub struct IterMut<'a, K: Ord + Default, V: Default, const N: usize> {
    mut_iter: TreeIterMut<'a, K, V, N>,
}

impl<'a, K: Ord + Default, V: Default, const N: usize> IterMut<'a, K, V, N> {
    /// Construct owning iterator.
    pub(crate) fn new(map: &'a mut SgMap<K, V, N>) -> Self {
        IterMut {
            mut_iter: TreeIterMut::new(&mut map.bst),
        }
    }
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iterator for IterMut<'a, K, V, N> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.mut_iter.next()
    }
}

impl<'a, K: Ord + Default, V: Default, const N: usize> ExactSizeIterator for IterMut<'a, K, V, N> {
    fn len(&self) -> usize {
        self.mut_iter.len()
    }
}

// Key Iterators -------------------------------------------------------------------------------------------------------

// TODO: these need more trait implementations for full compatibility

/// An iterator over the keys of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`keys`][crate::map::SgMap::keys] method on [`SgMap`][crate::map::SgMap].
/// See its documentation for more.
pub struct Keys<'a, K: Ord + Default, V: Default, const N: usize> {
    pub(crate) inner: Iter<'a, K, V, N>,
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iterator for Keys<'a, K, V, N> {
    type Item = &'a K;

    fn next(&mut self) -> Option<&'a K> {
        self.inner.next().map(|(k, _)| k)
    }
}

impl<'a, K: Ord + Default, V: Default, const N: usize> ExactSizeIterator for Keys<'a, K, V, N> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// An owning iterator over the keys of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`into_keys`][crate::map::SgMap::into_keys] method on [`SgMap`][crate::map::SgMap].
/// See its documentation for more.
pub struct IntoKeys<K: Ord + Default, V: Default, const N: usize> {
    pub(crate) inner: IntoIter<K, V, N>,
}

impl<K: Ord + Default, V: Default, const N: usize> Iterator for IntoKeys<K, V, N> {
    type Item = K;

    fn next(&mut self) -> Option<K> {
        self.inner.next().map(|(k, _)| k)
    }
}

impl<K: Ord + Default, V: Default, const N: usize> ExactSizeIterator for IntoKeys<K, V, N> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

// Value Iterators -----------------------------------------------------------------------------------------------------

// TODO: these need more trait implementations for full compatibility

/// An iterator over the values of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`values`][crate::map::SgMap::values] method on [`SgMap`][crate::map::SgMap].
/// See its documentation for more.
pub struct Values<'a, K: Ord + Default, V: Default, const N: usize> {
    pub(crate) inner: Iter<'a, K, V, N>,
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iterator for Values<'a, K, V, N> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|(_, v)| v)
    }
}

impl<'a, K: Ord + Default, V: Default, const N: usize> ExactSizeIterator for Values<'a, K, V, N> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// An owning iterator over the values of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`into_values`][crate::map::SgMap::into_values] method on [`SgMap`][crate::map::SgMap].
/// See its documentation for more.
pub struct IntoValues<K: Ord + Default, V: Default, const N: usize> {
    pub(crate) inner: IntoIter<K, V, N>,
}

impl<K: Ord + Default, V: Default, const N: usize> Iterator for IntoValues<K, V, N> {
    type Item = V;

    fn next(&mut self) -> Option<V> {
        self.inner.next().map(|(_, v)| v)
    }
}

impl<K: Ord + Default, V: Default, const N: usize> ExactSizeIterator for IntoValues<K, V, N> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// A mutable iterator over the values of a [`SgMap`][crate::map::SgMap].
///
/// This `struct` is created by the [`values_mut`][crate::map::SgMap::values_mut] method on [`SgMap`][crate::map::SgMap].
/// See its documentation for more.
pub struct ValuesMut<'a, K: Ord + Default, V: Default, const N: usize> {
    pub(crate) inner: IterMut<'a, K, V, N>,
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iterator for ValuesMut<'a, K, V, N> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<&'a mut V> {
        self.inner.next().map(|(_, v)| v)
    }
}

impl<'a, K: Ord + Default, V: Default, const N: usize> ExactSizeIterator
    for ValuesMut<'a, K, V, N>
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

// Entry API -----------------------------------------------------------------------------------------------------

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`SgMap`].
pub enum Entry<'a, K: Ord + Default, V: Default, const N: usize> {
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V, N>),
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V, N>),
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Entry<'a, K, V, N> {
    /// Ensures a value is in the entry by inserting the default if empty, and returns a mutable
    /// reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 10>::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    /// ```
    pub fn or_insert(self, default: V) -> &'a mut V {
        todo!()
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty, and returns a mutable
    /// reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 10>::new();
    /// let x = 42;
    /// map.entry("poneyland").or_insert_with(|| x);
    ///
    /// assert_eq!(map["poneyland"], 42);
    /// ```
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        todo!()
    }

    /// Ensures a value is in the entry by inserting, if empty, the result of the default function.
    /// This method allows for generating key-derived values for insertion by providing the default
    /// function a reference to the key that was moved during the `.entry(key)` method call.
    ///
    /// The reference to the moved key is provided so that cloning or copying the key is
    /// unnecessary, unlike with `.or_insert_with(|| ... )`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 10>::new();
    ///
    /// map.entry("poneyland").or_insert_with_key(|key| key.chars().count());
    ///
    /// assert_eq!(map["poneyland"], 9);
    /// ```
    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
        todo!()
    }

    /// Returns a reference to this entry's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 10>::new();
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    pub fn key(&self) -> &K {
        todo!()
    }

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, usize, 10>::new();
    ///
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 42);
    ///
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 43);
    /// ```
    pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self {
        todo!()
    }

    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SgMap;
    ///
    /// let mut map = SgMap::<&str, Option<usize>, 10>::new();
    /// map.entry("poneyland").or_default();
    ///
    /// assert_eq!(map["poneyland"], None);
    /// ```
    pub fn or_default(self) -> &'a mut V {
        todo!()
    }
}
