#![no_std]
#[cfg(feature = "alloc")]
extern crate alloc;
// pub mod target;
pub mod types;
#[cfg(feature = "ratchet")]
pub mod ratchet;
pub use embedded_io::{Error as IOError, ErrorKind, ErrorType};
