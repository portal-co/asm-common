//! Value types with bit-width information.
//!
//! This module provides types for representing values with explicit bit-width
//! information, constants, and load/store operations.

use super::*;

/// The bit width of a value, represented logarithmically.
///
/// The actual bit width is `2^log2`. For example:
/// - `log2 = 3` → 8 bits
/// - `log2 = 5` → 32 bits
/// - `log2 = 6` → 64 bits
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::value::Bitness;
///
/// let byte = Bitness { log2: 3 };  // 2^3 = 8 bits
/// let word = Bitness { log2: 5 };  // 2^5 = 32 bits
/// let qword = Bitness { log2: 6 }; // 2^6 = 64 bits
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bitness {
    /// The logarithm base 2 of the bit width
    pub log2: u8,
}
/// A large constant value up to 512 bits.
///
/// Stores a constant value as eight 64-bit words in native endianness
/// (typically little-endian). The constant can be interpreted at various
/// bit widths using the provided methods.
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::value::{Constant, Bitness};
///
/// let constant = Constant {
///     data: [0x123456789ABCDEF0, 0, 0, 0, 0, 0, 0, 0],
/// };
///
/// let bitness = Bitness { log2: 6 }; // 64 bits
/// let bytes: Vec<u8> = constant.bytes(bitness).collect();
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Constant {
    /// The constant data stored in native endianness (usually little-endian)
    pub data: [u64; 8],
}
impl Constant {
    /// Returns an iterator over the bytes of this constant.
    ///
    /// The number of bytes returned is determined by the bitness parameter.
    /// Bytes are in native endianness order.
    ///
    /// # Examples
    ///
    /// ```
    /// use portal_pc_asm_common::types::value::{Constant, Bitness};
    ///
    /// let constant = Constant { data: [0x0102030405060708, 0, 0, 0, 0, 0, 0, 0] };
    /// let bitness = Bitness { log2: 6 }; // 64 bits = 8 bytes
    /// let bytes: Vec<u8> = constant.bytes(bitness).collect();
    /// ```
    pub fn bytes(&self, b: Bitness) -> impl Iterator<Item = u8> {
        self.data
            .into_iter()
            .flat_map(|a| a.to_ne_bytes())
            .take(1 << (b.log2 - 3))
    }

    /// Returns an iterator over the bits of this constant.
    ///
    /// The number of bits returned is determined by the bitness parameter.
    /// Bits are in LSB-first order.
    ///
    /// # Examples
    ///
    /// ```
    /// use portal_pc_asm_common::types::value::{Constant, Bitness};
    ///
    /// let constant = Constant { data: [0xFF, 0, 0, 0, 0, 0, 0, 0] };
    /// let bitness = Bitness { log2: 3 }; // 8 bits
    /// let bits: Vec<bool> = constant.bits(bitness).collect();
    /// ```
    pub fn bits(&self, b: Bitness) -> impl Iterator<Item = bool> {
        self.bytes(b)
            .flat_map(|a| bitvec::array::BitArray::<u8, Lsb0>::new(a).into_iter())
            .take(1 << (b.log2))
    }

    /// Creates a constant from an iterator of bytes.
    ///
    /// The bitness parameter determines how many bytes are consumed from the iterator.
    /// Remaining space is zero-filled.
    ///
    /// Returns `None` if the iterator doesn't provide enough bytes.
    pub fn from_bytes(b: Bitness, i: impl Iterator<Item = u8>) -> Option<Self> {
        Some(Self {
            data: array_init::from_iter(
                i.chain(once(0u8).cycle().take((512 / 8) - (1 << (b.log2 - 3))))
                    .batching(|i| array_init::from_iter(i))
                    .map(|a| u64::from_ne_bytes(a)),
            )?,
        })
    }

    /// Creates a constant from an iterator of bits.
    ///
    /// The bitness parameter determines how many bits are consumed from the iterator.
    /// Bits are expected in LSB-first order. Remaining space is zero-filled.
    ///
    /// Returns `None` if the iterator doesn't provide enough bits.
    pub fn from_bits(b: Bitness, i: impl Iterator<Item = bool>) -> Option<Self> {
        Self::from_bytes(
            b,
            i.chain(once(false).cycle().take(
                8 - (match (1 << (b.log2)) % 8 {
                    0 => 8,
                    a => a,
                }),
            ))
            .batching(|a| {
                Some({
                    let x: [bool; 8] = array_init::from_iter(a)?;
                    x.into_iter()
                        .rev()
                        .fold(0, |a, b| (a * 2) + (if b { 1 } else { 0u8 }))
                })
            }),
        )
    }
}
/// A value with an offset and bit width.
///
/// Represents a value that has both an offset (of generic type `G`) and
/// an associated bit width. The offset could be a register, memory location,
/// or any other value reference.
///
/// # Type Parameters
///
/// - `G`: The type of the offset (e.g., register, memory address)
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::value::{Value, Bitness};
/// use portal_pc_asm_common::types::reg::Reg;
///
/// let value = Value {
///     offset: Reg(0),
///     bitness: Bitness { log2: 6 }, // 64 bits
/// };
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Value<G> {
    /// The offset or location of the value
    pub offset: G,
    /// The bit width of the value
    pub bitness: Bitness,
}
impl<G> Value<G> {
    /// Returns a value with a mutable reference to the offset.
    pub fn as_mut<'a>(&'a mut self) -> Value<&'a mut G> {
        Value {
            offset: &mut self.offset,
            bitness: self.bitness,
        }
    }

    /// Returns a value with an immutable reference to the offset.
    pub fn as_ref<'a>(&'a self) -> Value<&'a G> {
        Value {
            offset: &self.offset,
            bitness: self.bitness,
        }
    }

    /// Maps the offset to a new type using a fallible function.
    ///
    /// The bitness is preserved while the offset is transformed.
    pub fn map<G2, E>(self, f: &mut (dyn FnMut(G) -> Result<G2, E> + '_)) -> Result<Value<G2>, E> {
        Ok(match self {
            Value { offset, bitness } => Value {
                offset: f(offset)?,
                bitness,
            },
        })
    }
}
/// A frame for load/store operations, either from a value or a constant.
///
/// Represents either a value loaded from a location (like a register or memory)
/// or a constant value. Used for representing the source or destination of
/// load and store operations.
///
/// # Type Parameters
///
/// - `G`: The type of offset used in the value variant
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::value::{LoadStoreFrame, Value, Constant, Bitness};
/// use portal_pc_asm_common::types::reg::Reg;
///
/// let from_value = LoadStoreFrame::Value {
///     bits: Bitness { log2: 6 },
///     val: Value { offset: Reg(0), bitness: Bitness { log2: 6 } },
///     bit_offset: 0,
/// };
///
/// let from_constant = LoadStoreFrame::Constant {
///     bits: Bitness { log2: 6 },
///     constant: Constant { data: [42, 0, 0, 0, 0, 0, 0, 0] },
/// };
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoadStoreFrame<G> {
    /// Load/store from a value at a specific bit offset
    Value {
        /// The bit width being accessed
        bits: Bitness,
        /// The value being accessed
        val: Value<G>,
        /// The bit offset within the value
        bit_offset: usize,
    },
    /// Load/store from a constant
    Constant {
        /// The bit width being accessed
        bits: Bitness,
        /// The constant value
        constant: Constant,
    },
}
impl<G> LoadStoreFrame<G> {
    /// Maps the offset in value variants using a fallible function.
    ///
    /// Constants are left unchanged.
    pub fn map<G2, E>(
        self,
        f: &mut (dyn FnMut(G) -> Result<G2, E> + '_),
    ) -> Result<LoadStoreFrame<G2>, E> {
        Ok(match self {
            LoadStoreFrame::Value {
                bits,
                val,
                bit_offset,
            } => LoadStoreFrame::Value {
                bits,
                val: val.map(f)?,
                bit_offset,
            },
            LoadStoreFrame::Constant { bits, constant } => {
                LoadStoreFrame::Constant { bits, constant }
            }
        })
    }

    /// Returns a frame with an immutable reference to the offset.
    pub fn as_ref<'a>(&'a self) -> LoadStoreFrame<&'a G> {
        match self {
            LoadStoreFrame::Value {
                bits,
                val,
                bit_offset,
            } => LoadStoreFrame::Value {
                bits: *bits,
                val: val.as_ref(),
                bit_offset: *bit_offset,
            },
            LoadStoreFrame::Constant { bits, constant } => LoadStoreFrame::Constant {
                bits: *bits,
                constant: *constant,
            },
        }
    }

    /// Returns a frame with a mutable reference to the offset.
    pub fn as_mut<'a>(&'a mut self) -> LoadStoreFrame<&'a mut G> {
        match self {
            LoadStoreFrame::Value {
                bits,
                val,
                bit_offset,
            } => LoadStoreFrame::Value {
                bits: *bits,
                val: val.as_mut(),
                bit_offset: *bit_offset,
            },
            LoadStoreFrame::Constant { bits, constant } => LoadStoreFrame::Constant {
                bits: *bits,
                constant: *constant,
            },
        }
    }
}
/// Marker trait for iterators representing "any" semantics.
///
/// This trait is automatically implemented for all iterator types.
pub trait Any: Iterator {}
impl<T: Iterator + ?Sized> Any for T {}

/// Marker trait for iterators representing "all" semantics.
///
/// This trait is automatically implemented for all iterator types.
pub trait All: Iterator {}
impl<T: Iterator + ?Sized> All for T {}

/// Marker trait for iterators representing assignment chains.
///
/// In an assignment chain, the first element is replaced first, continuing
/// until the last element, which is discarded.
///
/// This trait is automatically implemented for all iterator types.
pub trait AssignChain: Iterator {}
impl<T: Iterator + ?Sized> AssignChain for T {}
