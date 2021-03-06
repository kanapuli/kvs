extern crate serde;

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
//use serde_json::Deserializer;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Range;

const COMPACTION_THRESHOLD: u64 = 1024;
//KvStore stores the key and values in the log files.
//The log files are created with monotonically increasing number extension
//For Faster query, an index is stored in RAM as a BTreeMap
pub struct KvStore {
    //log file reader
    reader: BufReaderWithPos<File>,
    //file writer
    writer: BufWriterWithPos<File>,
    //log file path
    path: PathBuf,
    current_gen: u64,
    index: BTreeMap<String, CommandPos>,
    //uncompacted represents the number of bytes representing
    //"stale" commands that could be deleted during compaction
    uncompacted: u64,
}

impl KvStore {
    fn new_log_file(
        path: &Path,
        gen: u64,
        readers: &mut HashMap<u64, BufReaderWithPos<File>>,
    ) -> Result<BufWriterWithPos<File>> {
        let path = log_path(path, gen);
        let writer = BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&path)?,
        )?;
        readers.insert(gen, BufReaderWithPos::new(File::open(&path)?)?);
        Ok(writer)
    }
}

//log_path gives a name for a new logfile
fn log_path(dir: &Path, gen: u64) -> PathBuf {
    dir.join(format!("{}.log", gen))
}
//Enum to represent command
#[derive(Deserialize, Serialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Self {
        Command::Set { key, value }
    }

    fn rm(key: String) -> Self {
        Command::Remove { key }
    }
}

//CommandPos represents the length and position of json-serialized
//command in the log
struct CommandPos {
    gen: u64,
    pos: u64,
    len: u64,
}

impl From<(u64, Range<u64>)> for CommandPos {
    fn from((gen, range): (u64, Range<u64>)) -> Self {
        CommandPos {
            gen,
            pos: range.start,
            len: range.end - range.start,
        }
    }
}
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

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
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
