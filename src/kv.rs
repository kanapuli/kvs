//use std::path::PathBuf;
use std::io;
//use std::fs::File;
//use std::collections::HashMap;
use crate::Result;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

//KvStore stores the key and values in memory
pub struct KvStore {}
//pub struct KvStore {
//  //directory for the log to store
//  path: PathBuf,
//  readers: HashMap<u64, BufReaderWithPos<File>>,
//  //writer of the current log
//  writer: BufWriterWithPos<File>,
//
//}

struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl<R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        //pull some bytes from source into the buffer, returning how many
        //bytes were read
        let length = self.reader.read(buf)?;
        self.pos += length as u64;
        Ok(length)
    }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        //Seek provides a cursor that can be moved within a stream
        //of bytes.
        //Seek to an offset ,in bytes, in the underlying reader
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let length = self.writer.write(buf)?;
        self.pos += length as u64;
        Ok(length)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl <W: Write + Seek> Seek for BufWriterWithPos<W> {

  fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
    self.pos = self.writer.seek(pos)?;
    Ok(self.pos)
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
