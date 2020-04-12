extern crate failure;

use failure::Fail;
use std::io;

#[derive(Fail, Debug)]
pub enum KvsError {
    ///IOError
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    ///Serialization and Deserialization Error
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    ///Removing Non-Existent Key Error
    #[fail(display = "Key not found")]
    KeyNotFound,

    ///Unexpected Command Error
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}

impl From<io: Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}
