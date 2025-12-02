//! Memory operation types.
//!
//! This module provides types for representing memory operations with
//! explicit size information.

use super::*;

/// The size of a memory access operation.
///
/// Specifies the width of a memory load or store operation in bits.
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::mem::MemorySize;
///
/// let byte_access = MemorySize::_8;
/// let word_access = MemorySize::_16;
/// let dword_access = MemorySize::_32;
/// let qword_access = MemorySize::_64;
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MemorySize {
    /// 8-bit (byte) memory access
    _8,
    /// 16-bit (word) memory access
    _16,
    /// 32-bit (double word) memory access
    _32,
    /// 64-bit (quad word) memory access (default)
    #[default]
    _64,
}

/// A value tagged with a memory access size.
///
/// Wraps a value of any type with an associated [`MemorySize`], useful for
/// representing memory operations where the access size is significant.
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::mem::{MemorySize, MemorySized};
///
/// let mem_op = MemorySized {
///     value: 0x1234_5678,
///     size: MemorySize::_32,
/// };
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MemorySized<T> {
    /// The value being accessed
    pub value: T,
    /// The size of the memory access
    pub size: MemorySize,
}
