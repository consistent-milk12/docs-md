# Crate `adler2`

Adler-32 checksum implementation.

This implementation features:

- Permissively licensed (0BSD) clean-room implementation.
- Zero dependencies.
- Zero `unsafe`.
- Decent performance (3-4 GB/s).
- `#![no_std]` support (with `default-features = false`).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`algo`](#algo) | mod |  |
| [`Adler32`](#adler32) | struct | Adler-32 checksum calculator. |
| [`adler32_slice`](#adler32_slice) | fn | Calculates the Adler-32 checksum of a byte slice. |

## Modules

- [`algo`](algo/index.md)

## Structs

### `Adler32`

```rust
struct Adler32 {
    a: u16,
    b: u16,
}
```

Adler-32 checksum calculator.

An instance of this type is equivalent to an Adler-32 checksum: It can be created in the default
state via `new` (or the provided `Default` impl), or from a precalculated checksum via
`from_checksum`, and the currently stored checksum can be fetched via `checksum`.

This type also implements `Hasher`, which makes it easy to calculate Adler-32 checksums of any
type that implements or derives `Hash`. This also allows using Adler-32 in a `HashMap`, although
that is not recommended (while every checksum is a hash function, they are not necessarily a
good one).

# Examples

Basic, piecewise checksum calculation:

```rust
use adler2::Adler32;

let mut adler = Adler32::new();

adler.write_slice(&[0, 1, 2]);
adler.write_slice(&[3, 4, 5]);

assert_eq!(adler.checksum(), 0x00290010);
```

Using `Hash` to process structures:

```rust
use std::hash::Hash;
use adler2::Adler32;

#[derive(Hash)]
struct Data {
    byte: u8,
    word: u16,
    big: u64,
}

let mut adler = Adler32::new();

let data = Data { byte: 0x1F, word: 0xABCD, big: !0 };
data.hash(&mut adler);

// hash value depends on architecture endianness
if cfg!(target_endian = "little") {
    assert_eq!(adler.checksum(), 0x33410990);
}
if cfg!(target_endian = "big") {
    assert_eq!(adler.checksum(), 0x331F0990);
}

```




#### Implementations

- <span id="adler32-new"></span>`fn new() -> Self`

- <span id="adler32-from-checksum"></span>`const fn from_checksum(sum: u32) -> Self`

- <span id="adler32-checksum"></span>`fn checksum(&self) -> u32`

- <span id="adler32-write-slice"></span>`fn write_slice(&mut self, bytes: &[u8])`

#### Trait Implementations

##### `impl Clone for Adler32`

- <span id="adler32-clone"></span>`fn clone(&self) -> Adler32` — [`Adler32`](#adler32)

##### `impl Copy for Adler32`

##### `impl Debug for Adler32`

- <span id="adler32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Adler32`

- <span id="adler32-default"></span>`fn default() -> Self`

##### `impl Hasher for Adler32`

- <span id="adler32-finish"></span>`fn finish(&self) -> u64`

- <span id="adler32-write"></span>`fn write(&mut self, bytes: &[u8])`

## Functions

### `adler32_slice`

```rust
fn adler32_slice(data: &[u8]) -> u32
```

Calculates the Adler-32 checksum of a byte slice.

This is a convenience function around the [`Adler32`](#adler32) type.


