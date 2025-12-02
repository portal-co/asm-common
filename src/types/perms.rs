//! Permission types and input stream abstractions.
//!
//! This module provides types for tracking fine-grained permissions on code
//! and data, along with types for representing and streaming inputs with
//! associated permission bits.

use super::*;

/// A single permission type.
///
/// Represents one of the four permission types that can be associated with
/// each byte of code or data.
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::perms::Perm;
///
/// let read = Perm::Read;
/// let write = Perm::Write;
/// let exec = Perm::Exec;
/// let no_jump = Perm::NoJump;
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Perm {
    /// Read permission
    Read,
    /// Write permission
    Write,
    /// Execute permission
    Exec,
    /// NOT a valid jump target
    NoJump,
}
/// A set of all four permission values.
///
/// This struct holds values of type `T` for each of the four permission types.
/// It's used to associate permission-related data with code or memory.
///
/// # Type Parameters
///
/// - `T`: The type of value associated with each permission (e.g., `bool`, `BitSlice`)
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::perms::Perms;
///
/// let perms = Perms {
///     r: true,   // Readable
///     w: false,  // Not writable
///     x: true,   // Executable
///     nj: false, // Can be a jump target
/// };
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Perms<T> {
    /// Read permission value
    pub r: T,
    /// Write permission value
    pub w: T,
    /// Execute permission value
    pub x: T,
    /// No-jump (not a jump target) permission value
    pub nj: T,
}
impl<T> Perms<T> {
    /// Transforms each permission value using the provided function.
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Perms<U> {
        let Perms { r, w, x, nj } = self;
        Perms {
            r: f(r),
            w: f(w),
            x: f(x),
            nj: f(nj),
        }
    }

    /// Transforms each permission value using a fallible function.
    ///
    /// Returns an error if any transformation fails.
    pub fn try_map<U, E>(self, mut f: impl FnMut(T) -> Result<U, E>) -> Result<Perms<U>, E> {
        let Perms { r, w, x, nj } = self;
        Ok(Perms {
            r: f(r)?,
            w: f(w)?,
            x: f(x)?,
            nj: f(nj)?,
        })
    }

    /// Returns permission values as immutable references.
    pub fn as_ref<'a>(&'a self) -> Perms<&'a T> {
        match self {
            Perms { r, w, x, nj } => Perms { r, w, x, nj },
        }
    }

    /// Returns permission values as mutable references.
    pub fn as_mut<'a>(&'a mut self) -> Perms<&'a mut T> {
        match self {
            Perms { r, w, x, nj } => Perms { r, w, x, nj },
        }
    }
}
#[cfg(feature = "enum-map")]
const _: () = {
    use core::mem::{replace, MaybeUninit};
    impl<T, U: Into<T>> From<Perms<U>> for enum_map::EnumMap<Perm, T> {
        fn from(value: Perms<U>) -> Self {
            let value: Perms<T> = Perms {
                r: value.r.into(),
                w: value.w.into(),
                x: value.x.into(),
                nj: value.nj.into(),
            };
            let mut value: Perms<MaybeUninit<T>> = Perms {
                r: MaybeUninit::new(value.r),
                w: MaybeUninit::new(value.w),
                x: MaybeUninit::new(value.x),
                nj: MaybeUninit::new(value.nj),
            };
            return unsafe {
                enum_map::enum_map! {
                    Perm::Read => replace(&mut value.r,MaybeUninit::uninit()).assume_init(),
                    Perm::Write => replace(&mut value.w,MaybeUninit::uninit()).assume_init(),
                    Perm::Exec => replace(&mut value.x,MaybeUninit::uninit()).assume_init(),
                    Perm::NoJump => replace(&mut value.nj,MaybeUninit::uninit()).assume_init()
                }
            };
        }
    }
    impl<T, U: Into<T>> From<enum_map::EnumMap<Perm, U>> for Perms<T> {
        fn from(value: enum_map::EnumMap<Perm, U>) -> Self {
            let mut value: enum_map::EnumMap<Perm, MaybeUninit<T>> =
                value.map(|_, a| MaybeUninit::new(a.into()));
            return unsafe {
                Self {
                    r: replace(&mut value[Perm::Read], MaybeUninit::uninit()).assume_init(),
                    w: replace(&mut value[Perm::Write], MaybeUninit::uninit()).assume_init(),
                    x: replace(&mut value[Perm::Exec], MaybeUninit::uninit()).assume_init(),
                    nj: replace(&mut value[Perm::NoJump], MaybeUninit::uninit()).assume_init(),
                }
            };
        }
    }
};
/// A borrowed reference to input code with associated permission bits.
///
/// Represents a slice of code bytes along with corresponding permission bit slices.
/// All slices must have the same length, which is enforced at construction time.
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::perms::{InputRef, Perms};
/// use bitvec::prelude::*;
///
/// let code = &[0x90, 0x90][..]; // NOP instructions
/// let bits = bits![0, 1];
/// let perms = Perms {
///     r: bits,
///     w: bits,
///     x: bits,
///     nj: bits,
/// };
///
/// let input_ref = InputRef::new(code, perms).unwrap();
/// assert_eq!(input_ref.len(), 2);
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct InputRef<'a> {
    /// The code bytes
    pub code: &'a [u8],
    /// Read permission bits (one per byte)
    pub r: &'a BitSlice,
    /// Write permission bits (one per byte)
    pub w: &'a BitSlice,
    /// Execute permission bits (one per byte)
    pub x: &'a BitSlice,
    /// No-jump permission bits (one per byte)
    pub nj: &'a BitSlice,
    /// Zero-sized field that attests all slices have the same length
    attest_same_size: (),
}
/// A trait for types that can receive input code with permissions.
///
/// Similar to `std::io::Write`, but for code with permission bits.
/// Implementors can accept chunks of code and their associated permissions.
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::perms::{InputStream, InputRef};
/// use embedded_io::ErrorType;
/// use no_error_type::NoError;
///
/// struct MyStream;
///
/// impl ErrorType for MyStream {
///     type Error = NoError;
/// }
///
/// impl InputStream for MyStream {
///     fn write(&mut self, i: InputRef<'_>) -> Result<usize, Self::Error> {
///         // Process the input...
///         Ok(i.len())
///     }
/// }
/// ```
pub trait InputStream: ErrorType {
    /// Writes some input code and permissions to the stream.
    ///
    /// Returns the number of bytes written on success.
    fn write(&mut self, i: InputRef<'_>) -> Result<usize, Self::Error>;

    /// Writes all input code and permissions to the stream.
    ///
    /// Repeatedly calls `write` until all input is consumed.
    fn write_all(&mut self, mut i: InputRef<'_>) -> Result<(), Self::Error> {
        while i.code.len() > 0 {
            let x = self.write(i.nest())?;
            i = i.subref(x..);
        }
        Ok(())
    }
}
impl<'a, T: InputStream> InputStream for &'a mut T {
    fn write(&mut self, i: InputRef<'_>) -> Result<usize, Self::Error> {
        (&mut **self).write(i)
    }
    fn write_all(&mut self, mut i: InputRef<'_>) -> Result<(), Self::Error> {
        (&mut **self).write_all(i)
    }
}
impl<'a> Index<Perm> for InputRef<'a> {
    type Output = &'a BitSlice;
    fn index(&self, index: Perm) -> &Self::Output {
        match index {
            Perm::Read => &self.r,
            Perm::Write => &self.w,
            Perm::Exec => &self.x,
            Perm::NoJump => &self.nj,
        }
    }
}
impl<'a> InputRef<'a> {
    /// Returns an iterator over code bytes and their permission values.
    ///
    /// Each item is a tuple of `(byte, Perms<bool>)`.
    pub fn iter(&self) -> impl Iterator<Item = (u8, Perms<bool>)> + use<'a> {
        return self.code.iter().cloned().zip(
            self.r
                .iter()
                .by_vals()
                .zip(self.w.iter().by_vals())
                .zip(self.x.iter().by_vals())
                .zip(self.nj.iter().by_vals())
                .map(|x| Perms {
                    r: x.0 .0 .0,
                    w: x.0 .0 .1,
                    x: x.0 .1,
                    nj: x.1,
                }),
        );
    }

    /// Returns the length of the input in bytes.
    pub fn len(&self) -> usize {
        return self.code.len();
    }

    /// Creates a sub-reference by indexing into the input.
    ///
    /// All slices (code and permissions) are indexed with the same range.
    pub fn subref<T: Clone>(self, r: T) -> Self
    where
        [u8]: Index<T, Output = [u8]>,
        BitSlice: Index<T, Output = BitSlice>,
    {
        // let r = start..end;
        Self {
            code: &self.code[r.clone()],
            r: &self.r[r.clone()],
            w: &self.w[r.clone()],
            x: &self.x[r.clone()],
            nj: &self.nj[r],
            attest_same_size: (),
        }
    }

    /// Creates a nested reference with a shorter lifetime.
    pub fn nest<'b>(&'b self) -> InputRef<'b> {
        InputRef {
            code: &self.code,
            r: &self.r,
            w: &self.w,
            x: &self.x,
            nj: &self.nj,
            attest_same_size: (),
        }
    }

    /// Creates a new `InputRef` from code and permission bit slices.
    ///
    /// Returns `None` if the slices don't all have the same length.
    ///
    /// # Examples
    ///
    /// ```
    /// use portal_pc_asm_common::types::perms::{InputRef, Perms};
    /// use bitvec::prelude::*;
    ///
    /// let code = &[0x90][..];
    /// let bits = bits![1];
    /// let perms = Perms { r: bits, w: bits, x: bits, nj: bits };
    ///
    /// let input_ref = InputRef::new(code, perms).unwrap();
    /// ```
    pub fn new(code: &'a [u8], perms: Perms<&'a BitSlice>) -> Option<Self> {
        let r = perms.r;
        let w = perms.w;
        let x = perms.x;
        let nj = perms.nj;
        Some(Self {
            code: code,
            r: r,
            w: w,
            x: x,
            nj: nj,
            attest_same_size: if [r, w, x, nj].iter().all(|a| a.len() == code.len()) {
            } else {
                return None;
            },
        })
    }

    /// Creates a new `InputRef` using an `EnumMap` for permissions.
    ///
    /// Available only with the `enum-map` feature enabled.
    #[cfg(feature = "enum-map")]
    pub fn new_mapped(
        code: &'a [u8],
        perms: enum_map::EnumMap<Perm, &'a BitSlice>,
    ) -> Option<Self> {
        Self::new(code, perms.into())
    }
}
/// An owned input with code and permission bits.
///
/// Similar to [`InputRef`] but owns its data. Available only with the `alloc` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "alloc")]
/// # {
/// use portal_pc_asm_common::types::perms::{Input, Perms};
/// use bitvec::prelude::*;
///
/// let code = vec![0x90, 0x90]; // NOP instructions
/// let perms = Perms {
///     r: bitvec![1, 1],
///     w: bitvec![0, 0],
///     x: bitvec![1, 1],
///     nj: bitvec![0, 0],
/// };
///
/// let input = Input::new(code, perms).unwrap();
/// assert_eq!(input.len(), 2);
/// # }
/// ```
#[cfg(feature = "alloc")]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Input {
    code: alloc::vec::Vec<u8>,
    r: bitvec::vec::BitVec,
    w: bitvec::vec::BitVec,
    x: bitvec::vec::BitVec,
    nj: bitvec::vec::BitVec,
    attest_same_size: (),
}
#[cfg(feature = "alloc")]
const _: () = {
    use alloc::{borrow::ToOwned, vec::Vec};
    use bitvec::vec::BitVec;
    use no_error_type::NoError;
    impl<'a> InputRef<'a> {
        pub fn to_owned(&self) -> Input {
            Input {
                code: self.code.to_owned(),
                r: self.r.to_owned(),
                w: self.w.to_owned(),
                x: self.x.to_owned(),
                nj: self.nj.to_owned(),
                attest_same_size: (),
            }
        }
    }
    impl Input {
        pub fn as_ref<'a>(&'a self) -> InputRef<'a> {
            InputRef {
                code: &self.code,
                r: self.r.as_bitslice(),
                w: self.w.as_bitslice(),
                x: self.x.as_bitslice(),
                nj: self.nj.as_bitslice(),
                attest_same_size: (),
            }
        }
        pub fn new(code: Vec<u8>, perms: Perms<BitVec>) -> Option<Self> {
            let r = perms.r;
            let w = perms.w;
            let x = perms.x;
            let nj = perms.nj;
            let attest_same_size = if [&r, &w, &x, &nj].iter().all(|a| a.len() == code.len()) {
            } else {
                return None;
            };
            Some(Self {
                code,
                r,
                w,
                x,
                nj,
                attest_same_size,
            })
        }
        pub fn len(&self) -> usize {
            return self.code.len();
        }
        pub fn into_parts(self) -> (Vec<u8>, BitVec, BitVec, BitVec, BitVec) {
            (self.code, self.r, self.w, self.x, self.nj)
        }
        #[cfg(feature = "enum-map")]
        pub fn into_mapped_parts(self) -> (Vec<u8>, enum_map::EnumMap<Perm, BitVec>) {
            (
                self.code,
                enum_map::enum_map! {Perm::Read => self.r.clone(),Perm::Write => self.w.clone(),Perm::Exec => self.x.clone(),Perm::NoJump => self.nj.clone()},
            )
        }
        #[cfg(feature = "enum-map")]
        pub fn new_mapped(code: Vec<u8>, perms: enum_map::EnumMap<Perm, BitVec>) -> Option<Self> {
            Self::new(code, perms.into())
        }
        pub fn extend(&mut self, i: impl Iterator<Item = (u8, Perms<bool>)>) {
            for (c, Perms { r, w, x, nj }) in i {
                self.code.push(c);
                self.r.push(r);
                self.w.push(w);
                self.x.push(x);
                self.nj.push(nj);
            }
        }
        #[cfg(feature = "enum-map")]
        pub fn extend_mapped(
            &mut self,
            i: impl Iterator<Item = (u8, enum_map::EnumMap<Perm, bool>)>,
        ) {
            self.extend(i.map(|(c, p)| (c, p.into())));
        }
    }
    impl ErrorType for Input {
        type Error = NoError;
    }
    impl InputStream for Input {
        fn write(&mut self, i: InputRef<'_>) -> Result<usize, Self::Error> {
            self.code.extend_from_slice(i.code);
            for (a, b) in [
                (i.r, &mut self.r),
                (i.w, &mut self.w),
                (i.x, &mut self.x),
                (i.nj, &mut self.nj),
            ] {
                b.extend_from_bitslice(a);
            }
            Ok(i.code.len())
        }
    }
};
