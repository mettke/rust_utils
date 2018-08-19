//! A TrieMap with owned nodes.
//!
//! Allows generic types for key and data elements.
//! Data elements may be fetched using either their
//! full key or a given prefix.
//!
//! The Key must implement `Ord` to allow quick
//! lookup.

use std::collections::BTreeMap;

/// A TrieMap with owned nodes.
///
/// Allows generic types for key and data elements.
/// Data elements may be fetched using either their
/// full key or a given prefix.
///
/// The Key must implement `Ord` to allow quick
/// lookup.
#[derive(Debug, Clone)]
pub struct BTrieMap<K: Ord + Clone, V> {
    children: BTreeMap<K, Box<BTrieMap<K, V>>>,
    value: Option<V>,
}

impl<'a, K: 'a + Ord + Clone, V> Default for BTrieMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// private methods
impl<'a, K: 'a + Ord + Clone, V> BTrieMap<K, V> {
    fn get_node<I: Iterator<Item = &'a K>>(&self, mut iter: I) -> Option<&Self> {
        if let Some(key) = iter.next() {
            if let Some(node) = self.children.get(&key) {
                return node.get_node(iter);
            } else {
                return None;
            }
        }
        Some(self)
    }

    fn get_or_create_node<I: Iterator<Item = &'a K>>(&mut self, mut iter: I) -> &mut Self {
        if let Some(key) = iter.next() {
            if !self.children.contains_key(&key) {
                self.children.insert(key.clone(), Box::new(Self::new()));
            }
            let node = self.children.get_mut(&key).unwrap();
            return node.get_or_create_node(iter);
        }
        self
    }

    fn get_values(&'a self, vector: &mut Vec<&'a V>) {
        if let Some(value) = self.value.as_ref() {
            vector.push(value);
        }
        for node in self.children.values() {
            node.get_values(vector);
        }
    }
}

impl<'a, K: 'a + Ord + Clone, V> BTrieMap<K, V> {
    /// Creates an empty `BTrieMap`
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::BTrieMap;
    ///
    /// let trie: BTrieMap<u8, bool> = BTrieMap::new();
    /// ```
    pub fn new() -> Self {
        BTrieMap {
            children: BTreeMap::new(),
            value: None,
        }
    }

    /// Inserts a given value into the `BTrieMap`. An existing Value with the given
    /// key will be overridden
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::BTrieMap;
    ///
    /// let mut trie: BTrieMap<u8, bool> = BTrieMap::new();
    ///
    /// trie.insert("Test".as_bytes(), true);
    /// ```
    pub fn insert<I: IntoIterator<Item = &'a K>>(&mut self, key: I, value: V) {
        let node = self.get_or_create_node(key.into_iter());
        node.value = Some(value);
    }

    /// Returns `true` if the `BTrieMap` contains an element equal to the
    /// given value
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::BTrieMap;
    ///
    /// let mut trie: BTrieMap<u8, bool> = BTrieMap::new();
    ///
    /// trie.insert("Test".as_bytes(), true);
    /// trie.insert("Test2".as_bytes(), true);
    /// trie.insert("Test3".as_bytes(), true);
    ///
    /// assert_eq!(trie.contains("Test".as_bytes()), true);
    /// assert_eq!(trie.contains("Test4".as_bytes()), false);
    /// ```
    pub fn contains<I: IntoIterator<Item = &'a K>>(&self, key: I) -> bool {
        self.get_node(key.into_iter())
            .map(|node| node.value.is_some())
            .unwrap_or(false)
    }

    /// Returns the value available in the `BTrieMap` under the given key
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::BTrieMap;
    ///
    /// let mut trie: BTrieMap<u8, bool> = BTrieMap::new();
    ///
    /// trie.insert("Test".as_bytes(), true);
    /// trie.insert("Test2".as_bytes(), true);
    /// trie.insert("Test3".as_bytes(), false);
    ///
    /// assert_eq!(Some(&true), trie.get("Test".as_bytes()));
    /// ```
    pub fn get<I: IntoIterator<Item = &'a K>>(&self, key: I) -> Option<&V> {
        self.get_node(key.into_iter())
            .and_then(|node| node.value.as_ref())
    }

    /// Returns all values available in the `BTrieMap` under a given key prefix
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_utils::BTrieMap;
    ///
    /// let mut trie: BTrieMap<u8, bool> = BTrieMap::new();
    ///
    /// trie.insert("Test".as_bytes(), true);
    /// trie.insert("Test2".as_bytes(), true);
    /// trie.insert("Test3".as_bytes(), false);
    ///
    /// assert_eq!(vec![&true, &true, &false], trie.get_with_prefix("Test".as_bytes()));
    /// ```
    pub fn get_with_prefix<I: IntoIterator<Item = &'a K>>(&self, prefix: I) -> Vec<&V> {
        let mut vec = Vec::new();
        if let Some(node) = self.get_node(prefix.into_iter()) {
            node.get_values(&mut vec);
        }
        vec
    }
}

// Ensure that `BTrieMap` and its read-only iterators are covariant in their type parameters
#[allow(dead_code)]
fn assert_covariance() {
    fn a<'a>(x: BTrieMap<&'static str, &'static str>) -> BTrieMap<&'a str, &'a str> {
        x
    }
}

#[cfg(test)]
mod tests {
    use BTrieMap;

    #[test]
    fn test_insert_and_contains() {
        let mut trie = BTrieMap::new();
        trie.insert("dog".as_bytes(), true);
        assert!(trie.contains("dog".as_bytes()));
        assert!(!trie.contains("dog ".as_bytes()));
    }

    #[test]
    fn test_insert_and_get() {
        let mut trie = BTrieMap::new();
        trie.insert("dog".as_bytes(), true);
        assert_eq!(Some(&true), trie.get("dog".as_bytes()));
        assert_eq!(None, trie.get("dog ".as_bytes()));
    }

    #[test]
    fn test_get_with_prefix() {
        let mut trie = BTrieMap::new();
        trie.insert("dog".as_bytes(), true);
        trie.insert("deer".as_bytes(), true);
        trie.insert("deal".as_bytes(), false);
        assert_eq!(vec![&false, &true], trie.get_with_prefix("de".as_bytes()));
    }
}
