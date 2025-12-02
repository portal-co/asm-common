//! Arithmetic operations, signedness, endianness, and comparison types.
//!
//! This module provides fundamental types for representing arithmetic and logical
//! operations at the assembly level, along with related concepts like signedness
//! and endianness.

use super::*;

/// An arithmetic or logical operation.
///
/// Represents various arithmetic, logical, and bitwise operations that can be
/// performed on values in assembly code.
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::ops::{Arith, Sign};
///
/// let add = Arith::Add;
/// let signed_div = Arith::Div(Sign::Signed);
/// let logical_and = Arith::And;
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash, Debug)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Arith {
    /// Addition operation
    Add,
    /// Subtraction operation
    Sub,
    /// Multiplication operation
    ///
    /// Results in a [`Bitness`] doubled, or with an incremented
    /// [`Bitness::log2`]
    Mul,
    /// Division operation with specified signedness
    Div(Sign),
    /// Remainder (modulo) operation with specified signedness
    Rem(Sign),
    /// Bitwise AND operation
    And,
    /// Bitwise OR operation
    Or,
    /// Bitwise XOR operation
    Xor,
    /// Shift left operation
    Shl,
    /// Shift right operation with specified signedness (logical vs arithmetic)
    Shr(Sign),
    /// Rotate left operation with specified signedness
    Rotl(Sign),
    /// Rotate right operation with specified signedness
    Rotr(Sign),
}
/// The signedness of a numeric value or operation.
///
/// Determines whether a value should be interpreted as signed or unsigned,
/// which affects operations like division, comparison, and right shifts.
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::ops::{Sign, Arith};
///
/// let signed_div = Arith::Div(Sign::Signed);
/// let unsigned_div = Arith::Div(Sign::Unsigned);
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash, Debug)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sign {
    /// Unsigned (non-negative) interpretation
    Unsigned,
    /// Signed (two's complement) interpretation
    Signed,
}
/// Byte order (endianness) for multi-byte values.
///
/// Specifies the order in which bytes are arranged in memory for values
/// larger than one byte.
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::ops::Endian;
///
/// let little = Endian::Little; // x86, ARM
/// let big = Endian::Big;       // Network byte order
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash, Debug)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Endian {
    /// Little-endian: least significant byte first
    Little,
    /// Big-endian: most significant byte first
    Big,
}
/// A method of extending values to larger bit widths.
///
/// Specifies how a value should be extended when converting to a larger
/// bit width (e.g., 32-bit to 64-bit).
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::ops::Ext;
///
/// let sign_ext = Ext::Sign; // Preserve sign bit
/// let zero_ext = Ext::Zero; // Fill with zeros
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash, Debug)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Ext {
    /// Sign extension: replicate the sign bit
    Sign,
    /// Zero extension: fill with zeros
    Zero,
}
/// Comparison operations.
///
/// Represents various comparison operations that can be performed between values.
/// Ordered comparisons (less than, greater than, etc.) require specifying signedness.
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::ops::{Cmp, Sign};
///
/// let equal = Cmp::Eq;
/// let signed_less = Cmp::Lt(Sign::Signed);
/// let unsigned_greater = Cmp::Gt(Sign::Unsigned);
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash, Debug)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Cmp {
    /// Less than or equal to (≤)
    Le(Sign),
    /// Less than (<)
    Lt(Sign),
    /// Equal to (==)
    Eq,
    /// Greater than (>)
    Gt(Sign),
    /// Greater than or equal to (≥)
    Ge(Sign),
    /// Not equal to (≠)
    Ne,
}
