*[gimli](../index.md) / [leb128](index.md)*

---

# Module `leb128`

Read and write DWARF's "Little Endian Base 128" (LEB128) variable length
integer encoding.

The implementation is a direct translation of the psuedocode in the DWARF 4
standard's appendix C.

Read and write signed integers:

```rust
#[cfg(all(feature = "read", feature = "write"))] {
use gimli::{EndianSlice, NativeEndian, leb128};

let mut buf = [0; 1024];

// Write to anything that implements `std::io::Write`.
{
    let mut writable = &mut buf[..];
    leb128::write::signed(&mut writable, -12345).expect("Should write number");
}

// Read from anything that implements `gimli::Reader`.
let mut readable = EndianSlice::new(&buf[..], NativeEndian);
let val = leb128::read::signed(&mut readable).expect("Should read number");
assert_eq!(val, -12345);
}
```

Or read and write unsigned integers:

```rust
#[cfg(all(feature = "read", feature = "write"))] {
use gimli::{EndianSlice, NativeEndian, leb128};

let mut buf = [0; 1024];

{
    let mut writable = &mut buf[..];
    leb128::write::unsigned(&mut writable, 98765).expect("Should write number");
}

let mut readable = EndianSlice::new(&buf[..], NativeEndian);
let val = leb128::read::unsigned(&mut readable).expect("Should read number");
assert_eq!(val, 98765);
}
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`read`](#read) | mod | A module for reading signed and unsigned integers that have been LEB128 |
| [`low_bits_of_byte`](#low_bits_of_byte) | fn |  |
| [`low_bits_of_u64`](#low_bits_of_u64) | fn |  |
| [`CONTINUATION_BIT`](#continuation_bit) | const |  |
| [`SIGN_BIT`](#sign_bit) | const |  |

## Modules

- [`read`](read/index.md) - A module for reading signed and unsigned integers that have been LEB128

## Functions

### `low_bits_of_byte`

```rust
fn low_bits_of_byte(byte: u8) -> u8
```

### `low_bits_of_u64`

```rust
fn low_bits_of_u64(val: u64) -> u8
```

## Constants

### `CONTINUATION_BIT`

```rust
const CONTINUATION_BIT: u8 = 128u8;
```

### `SIGN_BIT`

```rust
const SIGN_BIT: u8 = 64u8;
```

