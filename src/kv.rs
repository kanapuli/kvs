use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader,BufWriter, Read, Seek, SeekFrom, Write};
use crate::{KvsError, Result};

//KvStore stores the key and values in memory
pub struct KvStore {
  //directory for the log to store
  path: PathBuf,
  readers: HashMap<u64, BufReaderWithPos<File>>,
  //writer of the current log
  writer: BufWriterWithPos<File>, 

}

struct BufReaderWithPos<R: Read + Seek> {
  reader: BufReader<R>,
  pos: u64,
}

struct BufWriterWithPos<W: Write + Seek>{
  writer: BufWriter<W>,
  pos: u64,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
  fn new(mut inner: W) -> Result<Self> {
    let pos = inner.seek(SeekFrom::Current(0))?;
    Ok(BufWriterWithPos{
      writer: BufWriter::new(inner),
      pos,
    })
    
  }

}

//impl KvStore {
//    #[warn(dead_code)]
//    // creates a new instance of KvStore
//    pub fn new() -> Self {
//        KvStore {
//            map: HashMap::new(),
//        }
//    }
//
//    //set creates a new Key Value pair
//    pub fn set(&mut self, key: String, value: String) {
//        self.map.insert(key, value);
//    }
//
//    //get gets the value for a key
//    pub fn get(&self, key: String) -> Option<String> {
//        let data = self.map.get(&key);
//        match data {
//            Some(value) => Some(value.to_string()),
//            None => None,
//        }
//    }
//
//    //remove remove a key and a value from the in-memory store
//    pub fn remove(&mut self, key: String) {
//        self.map.remove(&key);
//    }
//
//    //    pub fn open(&self, _path: &Path) -> Result<KvStore> {
//    //        unimplemented!();
//    //    }
//}
