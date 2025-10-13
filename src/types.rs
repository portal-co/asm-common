use core::{iter::once, ops::Index};

use bitvec::{order::Lsb0, slice::BitSlice};
use either::Either;
use embedded_io::ErrorType;
use itertools::Itertools;

pub mod ops;
pub mod perms;
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
