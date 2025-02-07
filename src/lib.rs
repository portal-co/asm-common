#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

// pub mod target;
pub mod types;

pub use embedded_io::{Error as IOError, ErrorKind, ErrorType};