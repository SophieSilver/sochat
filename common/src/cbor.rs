//! Serialization and deserialization using the CBOR format
//!
//! A more convenient wrapper around [`ciborium`]

use std::{
    cell::RefCell,
    io::{self, Read, Write},
};

/// Error when serializing or deserializing CBOR
#[derive(Debug, Error)]
#[error(transparent)]
pub enum CborError {
    /// Error while serializing into CBOR
    Serialize(#[from] CborSerializeError),
    /// Error while deserializing from CBOR
    Deserialize(#[from] CborDeserializeError),
}

impl From<ciborium::ser::Error<io::Error>> for CborError {
    fn from(value: ciborium::ser::Error<io::Error>) -> Self {
        CborSerializeError::from(value).into()
    }
}

impl From<ciborium::de::Error<io::Error>> for CborError {
    fn from(value: ciborium::de::Error<io::Error>) -> Self {
        CborDeserializeError::from(value).into()
    }
}

/// Error while serializing into CBOR
#[derive(Debug, Error)]
#[error(transparent)]
pub struct CborSerializeError(#[from] ciborium::ser::Error<io::Error>);

/// Error while deserializing into CBOR
#[derive(Debug, Error)]
#[error(transparent)]
pub struct CborDeserializeError(#[from] ciborium::de::Error<io::Error>);

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

/// Deserialize the value from a given reader
pub fn from_reader<T: DeserializeOwned, R: Read>(reader: R) -> Result<T, CborError> {
    const BUFFER_SIZE: usize = 128 * 1024;

    thread_local! {
        static SCRATCH_BUFFER: RefCell<Box<[u8]>> = RefCell::new(vec![0; BUFFER_SIZE].into_boxed_slice());
    }

    Ok(SCRATCH_BUFFER.with_borrow_mut(|scratch_buffer| {
        ciborium::from_reader_with_buffer(reader, scratch_buffer)
    })?)
}

/// Serialize the value into a given writer
pub fn into_writer<T: Serialize + ?Sized, W: Write>(value: &T, writer: W) -> Result<(), CborError> {
    Ok(ciborium::into_writer(value, writer)?)
}
