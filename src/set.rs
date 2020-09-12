use std::cmp::Ordering;
use std::iter::FromIterator;

use crate::scapegoat::{InOrderIterator, RefInOrderIterator, SGTree};

// Ordered set.
// A wrapper interface for `SGTree`.
/// API examples and descriptions are all adapted or directly copied from the standard library's `BTreeSet`.
pub struct SGSet<T: Ord> {
    bst: SGTree<T, ()>,
}

impl<T: Ord> SGSet<T> {
    /// Constructor.
    pub fn new() -> Self {
        SGSet { bst: SGTree::new() }
    }

    /// Moves all elements from `other` into `self`, leaving other empty.
    pub fn append(&mut self, other: &mut SGSet<T>) {
        self.bst.append(&mut other.bst);
    }

    /// Adds a value to the set.
    /// If the set did not have this value present, `true` is returned.
    pub fn insert(&mut self, value: T) -> bool {
        self.bst.insert(value, ()).is_none()
    }

    /// Removes a value from the set. Returns whether the value was present in the set.
    pub fn remove(&mut self, value: &T) -> bool {
        self.bst.remove(value).is_some()
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    pub fn get(&self, value: &T) -> Option<&T> {
        match self.bst.get_key_value(value) {
            Some((k, _)) => Some(k),
            None => None,
        }
    }

    /// Clears the set, removing all values.
    pub fn clear(&mut self) {
        self.bst.clear()
    }

    /// Returns `true` if the set contains a value.
    pub fn contains(&self, value: &T) -> bool {
        self.bst.contains_key(value)
    }

    /// Returns a reference to the first/minium value in the set, if any.
    pub fn first(&self) -> Option<&T> {
        self.bst.first_key()
    }

    /// Removes the first value from the set and returns it, if any.
    /// The first value is the minimum value that was in the set.
    pub fn pop_first(&mut self) -> Option<T> {
        match self.bst.pop_first() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }

    /// Returns the last/maximum value in the set, if any.
    pub fn last(&self) -> Option<&T> {
        self.bst.last_key()
    }

    /// Removes the last value from the set and returns it, if any.
    /// The last value is the maximum value that was in the set.
    pub fn pop_last(&mut self) -> Option<T> {
        match self.bst.pop_last() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.bst.len()
    }

    /// Returns an iterator over values representing set difference, e.g., values in `self` but not in `other`, in ascending order.
    pub fn difference(&self, other: &SGSet<T>) -> std::vec::IntoIter<&T> {
        let mut diff = Vec::new();
        for val in self {
            if !other.contains(val) {
                diff.push(val);
            }
        }
        diff.into_iter()
    }

    /// Returns an iterator over values representing symmetric set difference, e.g., values in `self` or `other` but not both, in ascending order.
    pub fn symmetric_difference<'a>(&'a self, other: &'a SGSet<T>) -> std::vec::IntoIter<&T> {
        let mut sym_diff = Vec::new();

        for val in self {
            if !other.contains(val) {
                sym_diff.push(val);
            }
        }

        for val in other {
            if !self.contains(val) {
                sym_diff.push(val);
            }
        }

        sym_diff.sort_unstable();
        sym_diff.into_iter()
    }

    /// Returns an iterator over values representing set intersection, e.g., values in `self` and also in `other`, in ascending order.
    pub fn intersection(&self, other: &SGSet<T>) -> std::vec::IntoIter<&T> {
        let mut self_iter = self.into_iter();
        let mut other_iter = other.into_iter();
        let mut opt_self_val = self_iter.next();
        let mut opt_other_val = other_iter.next();
        let mut intersect = Vec::new();

        // Linear time
        while let (Some(self_val), Some(other_val)) = (opt_self_val, opt_other_val) {
            match self_val.cmp(&other_val) {
                Ordering::Less => {
                    opt_self_val = self_iter.next();
                },
                Ordering::Equal => {
                    intersect.push(self_val);
                    opt_self_val = self_iter.next();
                    opt_other_val = other_iter.next();
                },
                Ordering::Greater => {
                    opt_other_val = other_iter.next();
                },
            }
        }

        intersect.into_iter()
    }

    /// Returns an iterator over values representing set union, e.g., values in `self` or `other`, in ascending order.
    pub fn union<'a>(&'a self, other: &'a SGSet<T>) -> std::vec::IntoIter<&T> {
        let mut union = Vec::new();

        for val in self {
            union.push(val);
        }

        for val in other {
            if !union.contains(&val) {
                union.push(val);
            }
        }

        union.sort_unstable();
        union.into_iter()
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.bst.is_empty()
    }

    /// Returns `true` if `self` has no elements in common with other (empty intersection).
    pub fn is_disjoint(&self, other: &SGSet<T>) -> bool {
        self.intersection(other).count() == 0
    }

    /// Returns `true` if `self` is a subset of `other`, e.g., `other` contains at least all the values in `self`.
    pub fn is_subset(&self, other: &SGSet<T>) -> bool {
        self.intersection(other).count() == self.len()
    }

    /// Returns `true` if `self` is a superset of `other`, e.g., `self` contains at least all the values in `other`.
    pub fn is_superset(&self, other: &SGSet<T>) -> bool {
        other.is_subset(self)
    }
}

// Default constructor
impl<T: Ord> Default for SGSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Construction iterator
impl<T: Ord> FromIterator<T> for SGSet<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut sgs = SGSet::new();

        for v in iter {
            sgs.insert(v);
        }

        sgs
    }
}

// Reference iterator
impl<'a, T: Ord> IntoIterator for &'a SGSet<T> {
    type Item = &'a T;
    type IntoIter = SetRefInOrderIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SetRefInOrderIterator::new(&self)
    }
}

// Reference iterator wrapper
pub struct SetRefInOrderIterator<'a, T: Ord> {
    ref_iter: RefInOrderIterator<'a, T, ()>,
}

impl<'a, T: Ord> SetRefInOrderIterator<'a, T> {
    pub fn new(set: &'a SGSet<T>) -> Self {
        SetRefInOrderIterator {
            ref_iter: RefInOrderIterator::new(&set.bst),
        }
    }
}

impl<'a, T: Ord> Iterator for SetRefInOrderIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ref_iter.next() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }
}

// Consuming iterator
impl<T: Ord> IntoIterator for SGSet<T> {
    type Item = T;
    type IntoIter = SetInOrderIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SetInOrderIterator::new(self)
    }
}

// Consuming iterator wrapper
pub struct SetInOrderIterator<T: Ord> {
    iter: InOrderIterator<T, ()>,
}

impl<T: Ord> SetInOrderIterator<T> {
    pub fn new(set: SGSet<T>) -> Self {
        SetInOrderIterator {
            iter: InOrderIterator::new(set.bst),
        }
    }
}

impl<T: Ord> Iterator for SetInOrderIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }
}