//! Register type abstractions.
//!
//! This module provides types for representing CPU registers in a
//! platform-independent manner.

use super::*;

/// A register identifier.
///
/// Represents a CPU register using a single byte identifier. The register
/// numbering is abstract and can be mapped to actual hardware registers
/// as needed by the target architecture.
///
/// # Special Registers
///
/// - [`Reg::CTX`]: Context register (register 255)
///
/// # Examples
///
/// ```
/// use portal_pc_asm_common::types::reg::Reg;
///
/// let r0 = Reg(0);
/// let r1 = Reg(1);
/// let ctx = Reg::CTX;
///
/// // Normalize to 32 registers
/// let normalized = Reg(35).r32();
/// assert_eq!(normalized, Reg(3));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reg(pub u8);

impl Reg {
    /// The context register (register 255).
    ///
    /// This is a special register typically used to hold context information.
    pub const CTX: Reg = Reg(255);

    /// Swap register 0 and 31 in a 32-register set.
    ///
    /// For registers in the range 0-31, this swaps 0 with 31 and leaves
    /// all other registers unchanged. Useful for certain ABI conventions.
    ///
    /// # Examples
    ///
    /// ```
    /// use portal_pc_asm_common::types::reg::Reg;
    ///
    /// assert_eq!(Reg(0).r32_swap_0_and_31(), Reg(31));
    /// assert_eq!(Reg(31).r32_swap_0_and_31(), Reg(0));
    /// assert_eq!(Reg(5).r32_swap_0_and_31(), Reg(5));
    /// ```
    pub const fn r32_swap_0_and_31(&self) -> Self {
        match self.0 % 32 {
            0 => Self(31),
            31 => Self(0),
            v => Self(v),
        }
    }

    /// Normalize the register to a 32-register set.
    ///
    /// Returns the register number modulo 32, mapping all register
    /// identifiers into the range 0-31.
    ///
    /// # Examples
    ///
    /// ```
    /// use portal_pc_asm_common::types::reg::Reg;
    ///
    /// assert_eq!(Reg(35).r32(), Reg(3));
    /// assert_eq!(Reg(0).r32(), Reg(0));
    /// assert_eq!(Reg(31).r32(), Reg(31));
    /// ```
    pub const fn r32(&self) -> Self {
        return Self(self.0 % 32);
    }
}
