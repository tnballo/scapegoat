use crate::tree::{IntoIter, Iter, IterMut};

// Key Iterators -------------------------------------------------------------------------------------------------------

// TODO: these need more trait implementations for full compatibility

/// An iterator over the keys of a `SGMap`.
///
/// This `struct` is created by the [`keys`][crate::map::SGMap::keys] method on [`SGMap`][crate::map::SGMap].
/// See its documentation for more.
pub struct Keys<'a, K: Ord + Default + 'a, V: Default + 'a, const N: usize> {
    pub(crate) inner: Iter<'a, K, V, N>,
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iterator for Keys<'a, K, V, N> {
    type Item = &'a K;

    fn next(&mut self) -> Option<&'a K> {
        self.inner.next().map(|(k, _)| k)
    }
}

/// An owning iterator over the keys of a `SGMap`.
///
/// This `struct` is created by the [`into_keys`][crate::map::SGMap::into_keys] method on [`SGMap`][crate::map::SGMap].
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

// Value Iterators -----------------------------------------------------------------------------------------------------

// TODO: these need more trait implementations for full compatibility

/// An iterator over the values of a `SGMap`.
///
/// This `struct` is created by the [`values`][crate::map::SGMap::values] method on [`SGMap`][crate::map::SGMap].
/// See its documentation for more.
pub struct Values<'a, K: Ord + Default + 'a, V: Default + 'a, const N: usize> {
    pub(crate) inner: Iter<'a, K, V, N>,
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iterator for Values<'a, K, V, N> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|(_, v)| v)
    }
}

/// An owning iterator over the values of a `SGMap`.
///
/// This `struct` is created by the [`into_values`][crate::map::SGMap::into_values] method on [`SGMap`][crate::map::SGMap].
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

/// A mutable iterator over the values of a `SGMap`.
///
/// This `struct` is created by the [`values_mut`][crate::map::SGMap::values_mut] method on [`SGMap`][crate::map::SGMap].
/// See its documentation for more.
pub struct ValuesMut<'a, K: Ord + 'a, V: 'a, const N: usize> {
    pub(crate) inner: IterMut<'a, K, V, N>,
}

impl<'a, K: Ord + Default, V: Default, const N: usize> Iterator for ValuesMut<'a, K, V, N> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<&'a mut V> {
        self.inner.next().map(|(_, v)| v)
    }
}