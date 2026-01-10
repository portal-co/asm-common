# portal-pc-asm-common

Common types and traits for assembly rewriting.

[![License: CC0-1.0](https://img.shields.io/badge/License-CC0%201.0-lightgrey.svg)](http://creativecommons.org/publicdomain/zero/1.0/)

## Overview

`portal-pc-asm-common` is a `no_std` Rust library that provides foundational types and traits for working with assembly-level operations, particularly for assembly rewriting and transformation tasks. The library is designed to be lightweight and portable, making it suitable for embedded systems and other constrained environments.

## Features

- **Arithmetic Operations**: Comprehensive support for arithmetic operations including add, subtract, multiply, divide, remainder, bitwise operations, and rotations
- **Permission System**: Fine-grained permission tracking for code with read, write, execute, and no-jump permissions
- **Register Abstractions**: Type-safe register representations
- **Memory Operations**: Memory sizing and addressing types
- **Value Types**: Bit-width aware value representations with constant support
- **Ratchet**: Cryptographic seed ratcheting mechanism using SHA3-256 (optional feature)
- **Serialization**: Optional serde support for all types
- **No Standard Library**: Fully `no_std` compatible for embedded and constrained environments

## Optional Features

- `enum-map`: Enables `enum_map::Enum` derives for enum types
- `exhaust`: Enables `exhaust::Exhaust` derives for exhaustive iteration
- `serde`: Enables serialization and deserialization support
- `alloc`: Enables allocating types like `Input` and `Vec` support
- `sha3`: Enables SHA3 hashing support
- `ratchet`: Enables the ratchet module (requires `sha3` feature)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
portal-pc-asm-common = "0.1.1"
```

With optional features:

```toml
[dependencies]
portal-pc-asm-common = { version = "0.1.1", features = ["serde", "alloc"] }
```

## Usage

### Basic Arithmetic Operations

```rust
use portal_pc_asm_common::types::ops::{Arith, Sign};

// Define arithmetic operations
let add_op = Arith::Add;
let signed_div = Arith::Div(Sign::Signed);
let unsigned_shr = Arith::Shr(Sign::Unsigned);
```

### Working with Permissions

```rust
use portal_pc_asm_common::types::perms::{Perm, Perms};

// Create permission sets
let perms = Perms {
    r: true,   // Read permission
    w: false,  // No write permission
    x: true,   // Execute permission
    nj: false, // Not marked as no-jump
};
```

### Register Operations

```rust
use portal_pc_asm_common::types::reg::Reg;

// Define registers
let r0 = Reg(0);
let ctx = Reg::CTX;  // Special context register (255)

// Normalize to 32 registers
let normalized = r0.r32();
```

### Memory Operations

```rust
use portal_pc_asm_common::types::mem::{MemorySize, MemorySized};

// Define memory operations with size
let mem_op = MemorySized {
    value: 0x1234,
    size: MemorySize::_64,
};
```

### Value Types with Bitness

```rust
use portal_pc_asm_common::types::value::{Bitness, Value, Constant};

// Define bit width
let bitness = Bitness { log2: 6 }; // 2^6 = 64 bits

// Create a value with offset
let value = Value {
    offset: 0x100,
    bitness,
};

// Work with constants
let constant = Constant {
    data: [0; 8], // 512-bit constant
};
```

### Using the Ratchet (with `ratchet` feature)

```rust
#[cfg(feature = "ratchet")]
use portal_pc_asm_common::ratchet::Ratchet;

#[cfg(feature = "ratchet")]
{
    // Create a ratchet from a seed
    let mut ratchet = Ratchet::from_seed([0u8; 32]);
    
    // Generate next value in sequence
    let next_value = ratchet.next();
    
    // Split data using ratchet markers
    let data = b"chunk1\x00\x00...chunk2...";
    let chunks: Vec<&[u8]> = ratchet.split(data).collect();
}
```

### Working with Input Streams (with `alloc` feature)

```rust
#[cfg(feature = "alloc")]
use portal_pc_asm_common::types::perms::{Input, InputRef, InputStream, Perms};
#[cfg(feature = "alloc")]
use bitvec::vec::BitVec;

#[cfg(feature = "alloc")]
{
    // Create an input with code and permissions
    let code = vec![0x90, 0x90]; // NOP instructions
    let perms = Perms {
        r: BitVec::repeat(true, 2),
        w: BitVec::repeat(false, 2),
        x: BitVec::repeat(true, 2),
        nj: BitVec::repeat(false, 2),
    };
    
    let input = Input::new(code, perms).unwrap();
}
```

## Module Structure

- `types`: Core type definitions
  - `ops`: Arithmetic operations, signedness, endianness, and comparisons
  - `perms`: Permission types and input stream abstractions
  - `reg`: Register abstractions
  - `mem`: Memory sizing types
  - `value`: Bit-width aware value types and constants
- `ratchet`: Cryptographic seed ratcheting (optional, requires `ratchet` feature)

## API Documentation

For detailed API documentation, run:

```bash
cargo doc --open
```

Or with all features:

```bash
cargo doc --all-features --open
```

## no_std Support

This library is `no_std` by default. For allocation support in `no_std` environments, enable the `alloc` feature:

```toml
[dependencies]
portal-pc-asm-common = { version = "0.1.1", features = ["alloc"], default-features = false }
```

## License

This project is licensed under CC0-1.0 - see the [LICENSE](LICENSE) file for details or visit [Creative Commons CC0](http://creativecommons.org/publicdomain/zero/1.0/).

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

## Version History

- **0.1.1**: Current version with hash and debug implementations
- **0.1.0**: Initial release

## Notes

Some reexports in the `types` module are deprecated since version 0.1.1 and will be removed in the next minor release. Use the submodules directly:

- Instead of `use portal_pc_asm_common::types::*`, use `use portal_pc_asm_common::types::ops::*`
- Access permission types via `use portal_pc_asm_common::types::perms::*`
- Access value types via `use portal_pc_asm_common::types::value::*`

## Goals
- [ ] Maintain consistent common types for `asm-*` crates
- [ ] Optimize bit-level operations for embedded use
- [ ] Extend trait coverage for new architectures

## Progress
- [ ] Core types implemented (Arith, Perms, Reg, Mem, Value)
- [ ] Ratchet and Serde support available via features
- [ ] Documentation and examples provided

---
*AI assisted*
