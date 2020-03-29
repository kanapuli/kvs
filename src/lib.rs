//KvStore stores the key and values in memory
pub struct KvStore {}

impl KvStore {
    #[warn(dead_code)]
    // creates a new instance of KvStore
    pub fn new() -> Self {
        unimplemented!();
    }

    //set creates a new Key Value pair
    pub fn set(&mut self, _key: String, _value: String) {
        unimplemented!();
    }

    //get gets the value for a key
    pub fn get(&self, _key: String) -> Option<String> {
        unimplemented!();
    }

    //remove remove a key and a value from the in-memory store
    pub fn remove(&mut self, _key: String) {
        unimplemented!();
    }
}
