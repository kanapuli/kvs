use std::collections::HashMap;

//KvStore stores the key and values in memory
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    #[warn(dead_code)]
    // creates a new instance of KvStore
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }

    //set creates a new Key Value pair
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    //get gets the value for a key
    pub fn get(&self, key: String) -> Option<String> {
        let data = self.map.get(&key);
        match data {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }

    //remove remove a key and a value from the in-memory store
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }

    //    pub fn open(&self, _path: &Path) -> Result<KvStore> {
    //        unimplemented!();
    //    }
}
