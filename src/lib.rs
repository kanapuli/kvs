#[deny(missing_docs)]
pub use error::{KvsError, Result};
pub use kv::KvStore;
mod error;
mod kv;
