use core::borrow::Borrow;
use core::cmp::Ordering;
use core::iter::FromIterator;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use crate::tree::{
    ConsumingIter as TreeConsumingIter, ElemRefIter, ElemRefVec, Iter as TreeIter, SGTree,
};

#[cfg(feature = "high_assurance")]
use crate::tree::SGErr;

/// Ordered set.
/// API examples and descriptions are all adapted or directly copied from the standard library's [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html).
#[allow(clippy::upper_case_acronyms)] // TODO: Removal == breaking change, e.g. v2.0
pub struct SGSet<T: Ord> {
    bst: SGTree<T, ()>,
}

impl<T: Ord> SGSet<T> {
    /// Constructor.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut set: SGSet<i32> = SGSet::new();
    /// ```
    pub fn new() -> Self {
        SGSet { bst: SGTree::new() }
    }

    /// `#![no_std]`: total capacity, e.g. maximum number of set elements.
    /// Attempting to insert elements beyond capacity will panic, unless the `high_assurance` feature is enabled.
    ///
    /// If using `std`: fast capacity, e.g. number of set elements stored on the stack.
    /// Elements inserted beyond capacity will be stored on the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut set: SGSet<i32> = SGSet::new();
    ///
    /// assert!(set.capacity() > 0)
    /// ```
    pub fn capacity(&self) -> usize {
        self.bst.capacity()
    }

    /// Moves all elements from `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut a = SGSet::new();
    /// a.insert(1);
    /// a.insert(2);
    /// a.insert(3);
    ///
    /// let mut b = SGSet::new();
    /// b.insert(3);
    /// b.insert(4);
    /// b.insert(5);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.len(), 5);
    /// assert_eq!(b.len(), 0);
    ///
    /// assert!(a.contains(&1));
    /// assert!(a.contains(&2));
    /// assert!(a.contains(&3));
    /// assert!(a.contains(&4));
    /// assert!(a.contains(&5));
    /// ```
    #[cfg(not(feature = "high_assurance"))]
    pub fn append(&mut self, other: &mut SGSet<T>)
    where
        T: Ord,
    {
        self.bst.append(&mut other.bst);
    }

    /// Attempts to move all elements from `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut a = SGSet::new();
    /// a.insert(1);
    /// a.insert(2);
    /// a.insert(3);
    ///
    /// let mut b = SGSet::new();
    /// b.insert(3);
    /// b.insert(4);
    /// b.insert(5);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.len(), 5);
    /// assert_eq!(b.len(), 0);
    ///
    /// assert!(a.contains(&1));
    /// assert!(a.contains(&2));
    /// assert!(a.contains(&3));
    /// assert!(a.contains(&4));
    /// assert!(a.contains(&5));
    /// ```
    #[cfg(feature = "high_assurance")]
    pub fn append(&mut self, other: &mut SGSet<T>) -> Result<(), SGErr> {
        self.bst.append(&mut other.bst)
    }

    /// Adds a value to the set.
    /// If the set did not have this value present, `true` is returned.
    /// If the set did have this value present, `false` is returned, and the entry is overwritten.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut set = SGSet::new();
    ///
    /// assert_eq!(set.insert(2), true);
    /// assert_eq!(set.insert(2), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    #[cfg(not(feature = "high_assurance"))]
    pub fn insert(&mut self, value: T) -> bool
    where
        T: Ord,
    {
        self.bst.insert(value, ()).is_none()
    }

    /// Adds a value to the set.
    /// Returns `Err` if sets's stack capacity is full, else the `Ok` contains:
    /// * `true` if the set did not have this value present.
    /// * `false` if the set did have this value present (and that old entry is overwritten).
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::{SGSet, SGErr};
    ///
    /// let mut set = SGSet::new();
    ///
    /// assert_eq!(set.insert(2), Ok(true));
    /// assert_eq!(set.insert(2), Ok(false));
    /// assert_eq!(set.len(), 1);
    ///
    /// let mut elem = 3;
    /// while set.len() < set.capacity() {
    ///     set.insert(elem);
    ///     elem += 1;
    /// }
    ///
    /// assert_eq!(set.insert(elem), Err(SGErr::StackCapacityExceeded));
    /// ```
    #[cfg(feature = "high_assurance")]
    pub fn insert(&mut self, value: T) -> Result<bool, SGErr>
    where
        T: Ord,
    {
        match self.bst.insert(value, ()) {
            Ok(opt_val) => Ok(opt_val.is_none()),
            Err(_) => Err(SGErr::StackCapacityExceeded),
        }
    }

    /// Gets an iterator that visits the values in the `SGSet` in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let set: SGSet<usize> = [1, 2, 3].iter().cloned().collect();
    /// let mut set_iter = set.iter();
    /// assert_eq!(set_iter.next(), Some(&1));
    /// assert_eq!(set_iter.next(), Some(&2));
    /// assert_eq!(set_iter.next(), Some(&3));
    /// assert_eq!(set_iter.next(), None);
    /// ```
    ///
    /// Values returned by the iterator are returned in ascending order:
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let set: SGSet<usize> = [3, 1, 2].iter().cloned().collect();
    /// let mut set_iter = set.iter();
    /// assert_eq!(set_iter.next(), Some(&1));
    /// assert_eq!(set_iter.next(), Some(&2));
    /// assert_eq!(set_iter.next(), Some(&3));
    /// assert_eq!(set_iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    /// Removes a value from the set. Returns whether the value was
    /// present in the set.
    ///
    /// The value may be any borrowed form of the set's value type,
    /// but the ordering on the borrowed form *must* match the
    /// ordering on the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut set = SGSet::new();
    ///
    /// set.insert(2);
    /// assert_eq!(set.remove(&2), true);
    /// assert_eq!(set.remove(&2), false);
    /// ```
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.bst.remove(value).is_some()
    }

    /// Splits the collection into two at the given value. Returns everything after the given value,
    /// including the value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut a = SGSet::new();
    /// a.insert(1);
    /// a.insert(2);
    /// a.insert(3);
    /// a.insert(17);
    /// a.insert(41);
    ///
    /// let b = a.split_off(&3);
    ///
    /// assert_eq!(a.len(), 2);
    /// assert_eq!(b.len(), 3);
    ///
    /// assert!(a.contains(&1));
    /// assert!(a.contains(&2));
    ///
    /// assert!(b.contains(&3));
    /// assert!(b.contains(&17));
    /// assert!(b.contains(&41));
    /// ```
    pub fn split_off<Q>(&mut self, value: &Q) -> SGSet<T>
    where
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        SGSet {
            bst: self.bst.split_off(value),
        }
    }

    /*
    // TODO v2.0: impl and add fuzz test
    /// Adds a value to the set, replacing the existing value, if any, that is equal to the given
    /// one. Returns the replaced value.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut set = SGSet::new();
    /// set.insert(Vec::<i32>::new());
    ///
    /// assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
    /// set.replace(Vec::with_capacity(10));
    /// assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
    /// ```
    pub fn replace(&mut self, value: T) -> Option<T>
    where
        T: Ord,
    {
        self.bst.remove_entry(&value).map(|(k, _)| k)
    }
    */

    /*
    // TODO v2.0: impl and add fuzz test
    /// Removes and returns the value in the set, if any, that is equal to the given one.
    ///
    /// The value may be any borrowed form of the set's value type,
    /// but the ordering on the borrowed form *must* match the
    /// ordering on the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut set: SGSet<_> = [1, 2, 3].iter().cloned().collect();
    /// assert_eq!(set.take(&2), Some(2));
    /// assert_eq!(set.take(&2), None);
    /// ```
    pub fn take<Q: ?Sized>(&mut self, value: &Q) -> Option<T>
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.bst.remove_entry(value).map(|(k, v)| k)
    }
    */

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` such that `f(&e)` returns `false`.
    /// The elements are visited in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let xs = [1, 2, 3, 4, 5, 6];
    /// let mut set: SGSet<i32> = xs.iter().cloned().collect();
    /// // Keep only the even numbers.
    /// set.retain(|&k| k % 2 == 0);
    /// assert!(set.iter().eq([2, 4, 6].iter()));
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        T: Ord,
        F: FnMut(&T) -> bool,
    {
        self.bst.retain(|k, _| f(k));
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// The value may be any borrowed form of the set's value type,
    /// but the ordering on the borrowed form *must* match the
    /// ordering on the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let set: SGSet<_> = [1, 2, 3].iter().cloned().collect();
    /// assert_eq!(set.get(&2), Some(&2));
    /// assert_eq!(set.get(&4), None);
    /// ```
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.bst.get_key_value(value).map(|(k, _)| k)
    }

    /// Clears the set, removing all values.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut v = SGSet::new();
    /// v.insert(1);
    /// v.clear();
    /// assert!(v.is_empty());;
    /// ```
    pub fn clear(&mut self) {
        self.bst.clear()
    }

    /// Returns `true` if the set contains a value.
    ///
    /// The value may be any borrowed form of the set's value type,
    /// but the ordering on the borrowed form *must* match the
    /// ordering on the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let set: SGSet<_> = [1, 2, 3].iter().cloned().collect();
    /// assert_eq!(set.contains(&1), true);
    /// assert_eq!(set.contains(&4), false);
    /// ```
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.bst.contains_key(value)
    }

    /// Returns a reference to the first/minium value in the set, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut map = SGSet::new();
    /// assert_eq!(map.first(), None);
    /// map.insert(1);
    /// assert_eq!(map.first(), Some(&1));
    /// map.insert(2);
    /// assert_eq!(map.first(), Some(&1));
    /// ```
    pub fn first(&self) -> Option<&T>
    where
        T: Ord,
    {
        self.bst.first_key()
    }

    /// Removes the first value from the set and returns it, if any.
    /// The first value is the minimum value that was in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut set = SGSet::new();
    ///
    /// set.insert(1);
    /// while let Some(n) = set.pop_first() {
    ///     assert_eq!(n, 1);
    /// }
    /// assert!(set.is_empty());
    /// ```
    pub fn pop_first(&mut self) -> Option<T>
    where
        T: Ord,
    {
        self.bst.pop_first().map(|(k, _)| k)
    }

    /// Returns the last/maximum value in the set, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut map = SGSet::new();
    /// assert_eq!(map.first(), None);
    /// map.insert(1);
    /// assert_eq!(map.last(), Some(&1));
    /// map.insert(2);
    /// assert_eq!(map.last(), Some(&2));
    /// ```
    pub fn last(&self) -> Option<&T>
    where
        T: Ord,
    {
        self.bst.last_key()
    }

    /// Removes the last value from the set and returns it, if any.
    /// The last value is the maximum value that was in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut set = SGSet::new();
    ///
    /// set.insert(1);
    /// while let Some(n) = set.pop_last() {
    ///     assert_eq!(n, 1);
    /// }
    /// assert!(set.is_empty());
    /// ```
    pub fn pop_last(&mut self) -> Option<T>
    where
        T: Ord,
    {
        self.bst.pop_last().map(|(k, _)| k)
    }

    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut v = SGSet::new();
    /// assert_eq!(v.len(), 0);
    /// v.insert(1);
    /// assert_eq!(v.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.bst.len()
    }

    /// Returns an iterator over values representing set difference, e.g., values in `self` but not in `other`, in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut a = SGSet::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let mut b = SGSet::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let diff: Vec<_> = a.difference(&b).cloned().collect();
    /// assert_eq!(diff, [1]);
    /// ```
    pub fn difference(&self, other: &SGSet<T>) -> ElemRefIter<T>
    where
        T: Ord,
    {
        let mut diff = ElemRefVec::new();
        for val in self {
            if !other.contains(val) {
                diff.push(val);
            }
        }
        diff.into_iter()
    }

    /// Returns an iterator over values representing symmetric set difference, e.g., values in `self` or `other` but not both, in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut a = SGSet::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let mut b = SGSet::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let sym_diff: Vec<_> = a.symmetric_difference(&b).cloned().collect();
    /// assert_eq!(sym_diff, [1, 3]);
    /// ```
    pub fn symmetric_difference<'a>(&'a self, other: &'a SGSet<T>) -> ElemRefIter<T>
    where
        T: Ord,
    {
        let mut sym_diff = ElemRefVec::new();
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

    /// Returns an iterator over values representing set intersection, e.g., values in both `self` and `other`, in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut a = SGSet::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let mut b = SGSet::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let intersection: Vec<_> = a.intersection(&b).cloned().collect();
    /// assert_eq!(intersection, [2]);
    /// ```
    pub fn intersection(&self, other: &SGSet<T>) -> ElemRefIter<T>
    where
        T: Ord,
    {
        let mut self_iter = self.into_iter();
        let mut other_iter = other.into_iter();
        let mut opt_self_val = self_iter.next();
        let mut opt_other_val = other_iter.next();
        let mut intersect = ElemRefVec::new();

        // Linear time
        while let (Some(self_val), Some(other_val)) = (opt_self_val, opt_other_val) {
            match self_val.cmp(other_val) {
                Ordering::Less => {
                    opt_self_val = self_iter.next();
                }
                Ordering::Equal => {
                    intersect.push(self_val);
                    opt_self_val = self_iter.next();
                    opt_other_val = other_iter.next();
                }
                Ordering::Greater => {
                    opt_other_val = other_iter.next();
                }
            }
        }

        intersect.into_iter()
    }

    /// Returns an iterator over values representing set union, e.g., values in `self` or `other`, in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut a = SGSet::new();
    /// a.insert(1);
    ///
    /// let mut b = SGSet::new();
    /// b.insert(2);
    ///
    /// let union: Vec<_> = a.union(&b).cloned().collect();
    /// assert_eq!(union, [1, 2]);
    /// ```
    pub fn union<'a>(&'a self, other: &'a SGSet<T>) -> ElemRefIter<T>
    where
        T: Ord,
    {
        let mut union = ElemRefVec::new();

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
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let mut v = SGSet::new();
    /// assert!(v.is_empty());
    /// v.insert(1);
    /// assert!(!v.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bst.is_empty()
    }

    /// Returns `true` if `self` has no elements in common with other (empty intersection).
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    /// let a: SGSet<_> = [1, 2, 3].iter().cloned().collect();
    /// let mut b = SGSet::new();
    ///
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(4);
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(1);
    /// assert_eq!(a.is_disjoint(&b), false);
    /// ```
    pub fn is_disjoint(&self, other: &SGSet<T>) -> bool
    where
        T: Ord,
    {
        self.intersection(other).count() == 0
    }

    /// Returns `true` if `self` is a subset of `other`, e.g., `other` contains at least all the values in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let sup: SGSet<_> = [1, 2, 3].iter().cloned().collect();
    /// let mut set = SGSet::new();
    ///
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(2);
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(4);
    /// assert_eq!(set.is_subset(&sup), false);
    /// ```
    pub fn is_subset(&self, other: &SGSet<T>) -> bool
    where
        T: Ord,
    {
        self.intersection(other).count() == self.len()
    }

    /// Returns `true` if `self` is a superset of `other`, e.g., `self` contains at least all the values in `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let sub: SGSet<_> = [1, 2].iter().cloned().collect();
    /// let mut set = SGSet::new();
    ///
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(0);
    /// set.insert(1);
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(2);
    /// assert_eq!(set.is_superset(&sub), true);
    /// ```
    pub fn is_superset(&self, other: &SGSet<T>) -> bool
    where
        T: Ord,
    {
        other.is_subset(self)
    }
}

// Convenience Traits --------------------------------------------------------------------------------------------------

// Default constructor
impl<T: Ord> Default for SGSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Construction iterator
impl<T: Ord> FromIterator<T> for SGSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut sgs = SGSet::new();

        for v in iter {
            #[cfg(not(feature = "high_assurance"))]
            sgs.insert(v);

            #[cfg(feature = "high_assurance")]
            sgs.insert(v).expect("Stack-storage capacity exceeded!");
        }

        sgs
    }
}

// Extension from iterator
impl<T: Ord> Extend<T> for SGSet<T> {
    fn extend<TreeIter: IntoIterator<Item = T>>(&mut self, iter: TreeIter) {
        iter.into_iter().for_each(move |elem| {
            #[cfg(not(feature = "high_assurance"))]
            self.insert(elem);

            #[cfg(feature = "high_assurance")]
            self.insert(elem).expect("Stack-storage capacity exceeded!");
        });
    }

    /*
    TODO: currently unstable: https://github.com/rust-lang/rust/issues/72631
    fn extend_one(&mut self, elem: T) {
        self.insert(elem);
    }
    */
}

// Extension from reference iterator
impl<'a, T: 'a + Ord + Copy> Extend<&'a T> for SGSet<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }

    /*
    TODO: currently unstable: https://github.com/rust-lang/rust/issues/72631
    fn extend_one(&mut self, &elem: &'a T) {
        self.insert(elem);
    }
    */
}

// Iterators -----------------------------------------------------------------------------------------------------------

// Reference iterator
impl<'a, T: Ord> IntoIterator for &'a SGSet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Reference iterator wrapper
pub struct Iter<'a, T: Ord> {
    ref_iter: TreeIter<'a, T, ()>,
}

impl<'a, T: Ord> Iter<'a, T> {
    pub fn new(set: &'a SGSet<T>) -> Self {
        Iter {
            ref_iter: TreeIter::new(&set.bst),
        }
    }
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ref_iter.next().map(|(k, _)| k)
    }
}

// Consuming iterator
impl<T: Ord> IntoIterator for SGSet<T> {
    type Item = T;
    type IntoIter = ConsumingIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ConsumingIter::new(self)
    }
}

/// Consuming iterator wrapper
pub struct ConsumingIter<T: Ord> {
    cons_iter: TreeConsumingIter<T, ()>,
}

impl<T: Ord> ConsumingIter<T> {
    pub fn new(set: SGSet<T>) -> Self {
        ConsumingIter {
            cons_iter: TreeConsumingIter::new(set.bst),
        }
    }
}

impl<T: Ord> Iterator for ConsumingIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cons_iter.next().map(|(k, _)| k)
    }
}

// Operator Overloading ------------------------------------------------------------------------------------------------

impl<T: Ord + Clone> Sub<&SGSet<T>> for &SGSet<T> {
    type Output = SGSet<T>;

    /// Returns the difference of `self` and `rhs` as a new `SGSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let a: SGSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: SGSet<_> = vec![3, 4, 5].into_iter().collect();
    ///
    /// let result = &a - &b;
    /// let result_vec: Vec<_> = result.into_iter().collect();
    /// assert_eq!(result_vec, [1, 2]);
    /// ```
    fn sub(self, rhs: &SGSet<T>) -> SGSet<T> {
        self.difference(rhs).cloned().collect()
    }
}

impl<T: Ord + Clone> BitAnd<&SGSet<T>> for &SGSet<T> {
    type Output = SGSet<T>;

    /// Returns the intersection of `self` and `rhs` as a new `SGSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let a: SGSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: SGSet<_> = vec![2, 3, 4].into_iter().collect();
    ///
    /// let result = &a & &b;
    /// let result_vec: Vec<_> = result.into_iter().collect();
    /// assert_eq!(result_vec, [2, 3]);
    /// ```
    fn bitand(self, rhs: &SGSet<T>) -> SGSet<T> {
        self.intersection(rhs).cloned().collect()
    }
}

impl<T: Ord + Clone> BitOr<&SGSet<T>> for &SGSet<T> {
    type Output = SGSet<T>;

    /// Returns the union of `self` and `rhs` as a new `SGSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let a: SGSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: SGSet<_> = vec![3, 4, 5].into_iter().collect();
    ///
    /// let result = &a | &b;
    /// let result_vec: Vec<_> = result.into_iter().collect();
    /// assert_eq!(result_vec, [1, 2, 3, 4, 5]);
    /// ```
    fn bitor(self, rhs: &SGSet<T>) -> SGSet<T> {
        self.union(rhs).cloned().collect()
    }
}

impl<T: Ord + Clone> BitXor<&SGSet<T>> for &SGSet<T> {
    type Output = SGSet<T>;

    /// Returns the symmetric difference of `self` and `rhs` as a new `SGSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scapegoat::SGSet;
    ///
    /// let a: SGSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: SGSet<_> = vec![2, 3, 4].into_iter().collect();
    ///
    /// let result = &a ^ &b;
    /// let result_vec: Vec<_> = result.into_iter().collect();
    /// assert_eq!(result_vec, [1, 4]);
    /// ```
    fn bitxor(self, rhs: &SGSet<T>) -> SGSet<T> {
        self.symmetric_difference(rhs).cloned().collect()
    }
}
