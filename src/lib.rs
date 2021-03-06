use std::collections::HashMap;

#[deny(missing_docs)]
#[derive(Debug)]
/// Stores key-value pairs in memory.
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Creates an empty new key-value store using a hashmap data
    /// structure.
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// Sets a key with a value in the store.
    /// # Examples
    /// ```
    /// use kvs::KvStore;
    /// let mut kvs = KvStore::new();
    ///
    /// let key = String::from("foo");
    /// let value = String::from("bar");
    ///
    /// kvs.set(key, value);
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Gets a value in the store using the key.
    /// # Examples
    /// ```
    /// use kvs::KvStore;
    /// let mut kvs = KvStore::new();
    ///
    /// let key = String::from("foo");
    /// let value = String::from("bar");
    ///
    /// kvs.set(key, value);
    ///
    /// print!(kvs.get(String::from("foo")));
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        let value = self.store.get(&key);

        match value {
            None => None,
            // TODO Creating a new String from val is a hack to get a "Some" response
            // with the val param as a String type. The val parameter is a reference to a String and
            // dereferencing it (Some(*val)) doesn't work (compilation error "cannot move out of `*val`
            // which is behind a shared reference").
            Some(val) =>  Some(String::from(val)),
        }
    }

    /// Removes the value and key in the store using the key.
    /// # Examples
    /// ```
    /// use kvs::KvStore;
    /// let mut kvs = KvStore::new();
    ///
    /// let key = String::from("foo");
    /// let value = String::from("bar");
    ///
    /// kvs.set(key, value);
    ///
    /// kvs.rm(String::from("foo"));
    /// ```
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
