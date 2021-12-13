use core::cmp::Ordering;

use crate::set::SgSet;
use crate::tree::{IntoIter as TreeIntoIter, Iter as TreeIter};

use smallvec::SmallVec;

// General Iterators ---------------------------------------------------------------------------------------------------

/// An iterator over the items of a [`SgSet`][crate::set::SgSet].
///
/// This `struct` is created by the [`iter`][crate::set::SgSet::iter] method on [`SgSet`][crate::set::SgSet].
/// See its documentation for more.
pub struct Iter<'a, T: Ord + Default, const N: usize> {
    ref_iter: TreeIter<'a, T, (), N>,
}

impl<'a, T: Ord + Default, const N: usize> Iter<'a, T, N> {
    /// Construct reference iterator.
    pub(crate) fn new(set: &'a SgSet<T, N>) -> Self {
        Iter {
            ref_iter: TreeIter::new(&set.bst),
        }
    }
}

impl<'a, T: Ord + Default, const N: usize> Iterator for Iter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ref_iter.next().map(|(k, _)| k)
    }
}

/// An owning iterator over the items of a [`SgSet`][crate::set::SgSet].
///
/// This `struct` is created by the [`into_iter`][crate::set::SgSet::into_iter] method on [`SgSet`][crate::set::SgSet]
/// (provided by the IntoIterator trait). See its documentation for more.
pub struct IntoIter<T: Ord + Default, const N: usize> {
    cons_iter: TreeIntoIter<T, (), N>,
}

impl<T: Ord + Default, const N: usize> IntoIter<T, N> {
    /// Construct owning iterator.
    pub(crate) fn new(set: SgSet<T, N>) -> Self {
        IntoIter {
            cons_iter: TreeIntoIter::new(set.bst),
        }
    }
}

impl<T: Ord + Default, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cons_iter.next().map(|(k, _)| k)
    }
}

// Difference Iterator -------------------------------------------------------------------------------------------------

// TODO: these need more trait implementations for full compatibility
// TODO: make this a lazy iterator like `std::collections::btree_set::Difference`

/// An iterator producing elements in the difference of [`SgSet`][crate::set::SgSet]s.
///
/// This `struct` is created by the [`difference`][crate::set::SgSet::difference] method
/// on [`SgSet`][crate::set::SgSet]. See its documentation for more.
pub struct Difference<'a, T, const N: usize> {
    pub(crate) inner: smallvec::IntoIter<[&'a T; N]>,
}

impl<'a, T: Ord + Default, const N: usize> Difference<'a, T, N> {
    /// Construct `Difference` iterator.
    pub(crate) fn new(this: &'a SgSet<T, N>, other: &SgSet<T, N>) -> Self {
        let mut diff = SmallVec::<[&'a T; N]>::default();

        for val in this {
            if !other.contains(val) {
                diff.push(val);
            }
        }

        Difference {
            inner: diff.into_iter(),
        }
    }
}

impl<'a, T: Ord + Default, const N: usize> Iterator for Difference<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next()
    }
}

// Symmetric Difference Iterator ---------------------------------------------------------------------------------------

// TODO: these need more trait implementations for full compatibility
// TODO: make this a lazy iterator like `std::collections::btree_set::Difference`

/// An iterator producing elements in the symmetric difference of [`SgSet`][crate::set::SgSet]s.
///
/// This `struct` is created by the [`symmetric_difference`][crate::set::SgSet::symmetric_difference]
/// method on [`SgSet`][crate::set::SgSet]. See its documentation for more.
pub struct SymmetricDifference<'a, T, const N: usize> {
    pub(crate) inner: smallvec::IntoIter<[&'a T; N]>,
}

impl<'a, T: Ord + Default, const N: usize> SymmetricDifference<'a, T, N> {
    /// Construct `SymmetricDifference` iterator.
    pub(crate) fn new(this: &'a SgSet<T, N>, other: &'a SgSet<T, N>) -> Self {
        let mut sym_diff = SmallVec::<[&'a T; N]>::default();

        for val in this {
            if !other.contains(val) {
                sym_diff.push(val);
            }
        }

        for val in other {
            if !this.contains(val) {
                sym_diff.push(val);
            }
        }

        sym_diff.sort_unstable();

        SymmetricDifference {
            inner: sym_diff.into_iter(),
        }
    }
}

impl<'a, T: Ord + Default, const N: usize> Iterator for SymmetricDifference<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next()
    }
}

// Union Iterator ------------------------------------------------------------------------------------------------------

// TODO: these need more trait implementations for full compatibility
// TODO: make this a lazy iterator like `std::collections::btree_set::Union`

/// An iterator producing elements in the union of [`SgSet`][crate::set::SgSet]s.
///
/// This `struct` is created by the [`union`][crate::set::SgSet::difference] method on [`SgSet`][crate::set::SgSet].
/// See its documentation for more.
pub struct Union<'a, T, const N: usize> {
    pub(crate) inner: smallvec::IntoIter<[&'a T; N]>,
}

impl<'a, T: Ord + Default, const N: usize> Union<'a, T, N> {
    /// Construct `Union` iterator.
    pub(crate) fn new(this: &'a SgSet<T, N>, other: &'a SgSet<T, N>) -> Self {
        let mut union = SmallVec::<[&'a T; N]>::default();

        for val in this {
            union.push(val);
        }

        for val in other {
            if !union.contains(&val) {
                union.push(val);
            }
        }

        union.sort_unstable();

        Union {
            inner: union.into_iter(),
        }
    }
}

impl<'a, T: Ord + Default, const N: usize> Iterator for Union<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next()
    }
}

// Intersection Iterator -----------------------------------------------------------------------------------------------

// TODO: these need more trait implementations for full compatibility
// TODO: make this a lazy iterator like `std::collections::btree_set::Intersection`

/// An iterator producing elements in the intersection of [`SgSet`][crate::set::SgSet]s.
///
/// This `struct` is created by the [`intersection`][crate::set::SgSet::difference] method on [`SgSet`][crate::set::SgSet].
/// See its documentation for more.
pub struct Intersection<'a, T, const N: usize> {
    pub(crate) inner: smallvec::IntoIter<[&'a T; N]>,
}

impl<'a, T: Ord + Default, const N: usize> Intersection<'a, T, N> {
    /// Construct `Intersection` iterator.
    pub(crate) fn new(this: &'a SgSet<T, N>, other: &SgSet<T, N>) -> Self {
        let mut self_iter = this.into_iter();
        let mut other_iter = other.into_iter();
        let mut opt_self_val = self_iter.next();
        let mut opt_other_val = other_iter.next();
        let mut intersection = SmallVec::<[&'a T; N]>::default();

        // O(n), linear time
        while let (Some(self_val), Some(other_val)) = (opt_self_val, opt_other_val) {
            match self_val.cmp(other_val) {
                Ordering::Less => {
                    opt_self_val = self_iter.next();
                }
                Ordering::Equal => {
                    intersection.push(self_val);
                    opt_self_val = self_iter.next();
                    opt_other_val = other_iter.next();
                }
                Ordering::Greater => {
                    opt_other_val = other_iter.next();
                }
            }
        }

        Intersection {
            inner: intersection.into_iter(),
        }
    }
}

impl<'a, T: Ord + Default, const N: usize> Iterator for Intersection<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next()
    }
}
