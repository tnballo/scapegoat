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
