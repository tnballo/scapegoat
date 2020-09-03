use crate::scapegoat::{InOrderIterator, RefInOrderIterator, SGTree};

// Ordered set.
// A wrapper interface for `SGTree`.
pub struct SGSet<T: Ord> {
    bst: SGTree<T, ()>,
}

impl<T: Ord> SGSet<T> {
    /// Constructor.
    pub fn new() -> Self {
        SGSet { bst: SGTree::new() }
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

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.bst.is_empty()
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
