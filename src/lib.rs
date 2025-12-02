//! # portal-pc-asm-common
//!
//! Common types and traits for assembly rewriting.
//!
//! This library provides foundational types and abstractions for working with
//! assembly-level operations, particularly for assembly rewriting and transformation
//! tasks. It is fully `no_std` compatible, making it suitable for embedded systems
//! and other constrained environments.
//!
//! ## Features
//!
//! - **Arithmetic Operations**: Comprehensive support for arithmetic and bitwise operations
//! - **Permission System**: Fine-grained permission tracking for code segments
//! - **Register Abstractions**: Type-safe register representations
//! - **Memory Operations**: Memory sizing and addressing types
//! - **Value Types**: Bit-width aware value representations
//! - **Ratchet**: Cryptographic seed ratcheting (optional, requires `ratchet` feature)
//!
//! ## Optional Features
//!
//! - `enum-map`: Enables `enum_map::Enum` derives
//! - `exhaust`: Enables `exhaust::Exhaust` derives for exhaustive iteration
//! - `serde`: Enables serialization/deserialization support
//! - `alloc`: Enables allocating types and `Vec` support
//! - `sha3`: Enables SHA3 hashing support
//! - `ratchet`: Enables the ratchet module (requires `sha3`)
//!
//! ## Examples
//!
//! ### Basic arithmetic operations:
//!
//! ```
//! use portal_pc_asm_common::types::ops::{Arith, Sign};
//!
//! let add = Arith::Add;
//! let signed_div = Arith::Div(Sign::Signed);
//! ```
//!
//! ### Working with permissions:
//!
//! ```
//! use portal_pc_asm_common::types::perms::Perms;
//!
//! let perms = Perms {
//!     r: true,   // Read permission
//!     w: false,  // No write permission
//!     x: true,   // Execute permission
//!     nj: false, // Not marked as no-jump
//! };
//! ```
//!
//! ### Register operations:
//!
//! ```
//! use portal_pc_asm_common::types::reg::Reg;
//!
//! let r0 = Reg(0);
//! let ctx = Reg::CTX;  // Context register
//! let normalized = r0.r32(); // Normalize to 32 registers
//! ```

#![no_std]
#[cfg(feature = "alloc")]
extern crate alloc;
// pub mod target;
pub mod types;
#[cfg(feature = "ratchet")]
pub mod ratchet;
pub use embedded_io::{Error as IOError, ErrorKind, ErrorType};
