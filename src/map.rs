use crate::scapegoat::{InOrderIterator, RefInOrderIterator, SGTree};

// Ordered map.
// A wrapper interface for `SGTree`.
pub struct SGMap<K: Ord, V> {
    bst: SGTree<K, V>,
}

impl<K: Ord, V> SGMap<K, V> {
    /// Constructor.
    pub fn new() -> Self {
        SGMap { bst: SGTree::new() }
    }

    /// Insert a key-value pair into the map.
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, the old value is returned,
    /// and the key is updated. This accommodates types that can be `==` without being identical.
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.bst.insert(key, val)
    }

    /// Removes a key from the map, returning the stored key and value if the key was previously in the map.
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        self.bst.remove_entry(key)
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.bst.remove(key)
    }

    /// Returns the key-value pair corresponding to the given key.
    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.bst.get_key_value(key)
    }

    /// Returns a reference to the value corresponding to the given key.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.bst.get(key)
    }

    /// Get mutable reference corresponding to key.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.bst.get_mut(key)
    }

    /// Clears the map, removing all elements.
    pub fn clear(&mut self) {
        self.bst.clear()
    }

    /// Returns `true` if the map contains a value for the given key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.bst.contains_key(key)
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.bst.is_empty()
    }

    /// Returns a reference to the first key-value pair in the map.
    /// The key in this pair is the minimum key in the map.
    pub fn first_key_value(&self) -> Option<(&K, &V)> {
        self.bst.first_key_value()
    }

    /// Returns a reference to the first/minium key in the map, if any.
    pub fn first_key(&self) -> Option<&K> {
        self.bst.first_key()
    }

    /// Removes and returns the first element in the map.
    /// The key of this element is the minimum key that was in the map.
    pub fn pop_first(&mut self) -> Option<(K, V)> {
        self.bst.pop_first()
    }

    /// Returns a reference to the last key-value pair in the map.
    /// The key in this pair is the maximum key in the map.
    pub fn last_key_value(&self) -> Option<(&K, &V)> {
        self.bst.last_key_value()
    }

    /// Returns a reference to the last/maximum key in the map, if any.
    pub fn last_key(&self) -> Option<&K> {
        self.bst.last_key()
    }

    /// Removes and returns the last element in the map.
    /// The key of this element is the maximum key that was in the map.
    pub fn pop_last(&mut self) -> Option<(K, V)> {
        self.bst.pop_last()
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.bst.len()
    }
}

// Reference iterator
impl<'a, K: Ord, V> IntoIterator for &'a SGMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = RefInOrderIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        RefInOrderIterator::new(&self.bst)
    }
}

// Consuming iterator
impl<K: Ord, V> IntoIterator for SGMap<K, V> {
    type Item = (K, V);
    type IntoIter = InOrderIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        InOrderIterator::new(self.bst)
    }
}
