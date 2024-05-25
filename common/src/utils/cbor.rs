//! Utils for serialization using the CBOR format

use std::{
    cell::RefCell,
    io::{self, Read},
};

use serde::de::DeserializeOwned;

/// A wrapper around `ciborium::from_reader_with_buffer` with a 64 KiB thread local buffer
pub fn from_reader<T: DeserializeOwned, R: Read>(
    reader: R,
) -> Result<T, ciborium::de::Error<io::Error>> {
    const BUFFER_SIZE: usize = 64 * 1024;

    thread_local! {
        static SCRATCH_BUFFER: RefCell<Box<[u8]>> = RefCell::new(vec![0; BUFFER_SIZE].into_boxed_slice());
    }

    SCRATCH_BUFFER
        .with_borrow_mut(|scratch_buffer| ciborium::from_reader_with_buffer(reader, scratch_buffer))
}
