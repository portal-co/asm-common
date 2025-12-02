//! Core type definitions for assembly operations.
//!
//! This module contains the fundamental types used throughout the library
//! for representing assembly-level operations and data structures.
//!
//! ## Submodules
//!
//! - [`ops`]: Arithmetic operations, signedness, endianness, and comparisons
//! - [`perms`]: Permission types and input stream abstractions
//! - [`reg`]: Register abstractions
//! - [`mem`]: Memory sizing types
//! - [`value`]: Bit-width aware value types and constants
//!
//! ## Note on Deprecations
//!
//! Some type reexports at the module root level are deprecated since version 0.1.1
//! and will be removed in the next minor release. Use the submodules directly instead.

use bitvec::{order::Lsb0, slice::BitSlice};
use core::{iter::once, ops::Index};
use either::Either;
use embedded_io::ErrorType;
use itertools::Itertools;
pub mod ops;
pub mod perms;
pub mod reg;
pub mod mem;
pub mod value;
#[deprecated(
    note = "These reexports will be removed in the next minor release",
    since = "0.1.1"
)]
pub use ops::*;
#[deprecated(
    note = "These reexports will be removed in the next minor release",
    since = "0.1.1"
)]
pub use perms::*;
#[deprecated(
    note = "These reexports will be removed in the next minor release",
    since = "0.1.1"
)]
pub use value::*;
