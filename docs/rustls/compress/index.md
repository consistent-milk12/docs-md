*[rustls](../index.md) / [compress](index.md)*

---

# Module `compress`

Certificate compression and decompression support

This crate supports compression and decompression everywhere
certificates are used, in accordance with [RFC8879][rfc8879](#rfc8879)
.

Note that this is only supported for TLS1.3 connections.

# Getting started

Build this crate with the `brotli` and/or `zlib` crate features.  This
adds dependencies on these crates.  They are used by default if enabled.

We especially recommend `brotli` as it has the widest deployment so far.

# Custom compression/decompression implementations

1. Implement the [`CertCompressor`](compress/index.md) and/or [`CertDecompressor`](compress/index.md) traits
2. Provide those to:
  - [`ClientConfig::cert_compressors`][cc_cc](#cc-cc)
 or [`ServerConfig::cert_compressors`][sc_cc](#sc-cc)
.
  - [`ClientConfig::cert_decompressors`][cc_cd](#cc-cd)
 or [`ServerConfig::cert_decompressors`][sc_cd](#sc-cd)
.

These are used in these circumstances:

| Peer | Client authentication | Server authentication |
| ---- | --------------------- | --------------------- |
| *Client* | [`ClientConfig::cert_compressors`][cc_cc](#cc-cc)
 | [`ClientConfig::cert_decompressors`][cc_cd](#cc-cd)
 |
| *Server* | [`ServerConfig::cert_decompressors`][sc_cd](#sc-cd)
 | [`ServerConfig::cert_compressors`][sc_cc](#sc-cc)
 |

[rfc8879](#rfc8879)
: https://datatracker.ietf.org/doc/html/rfc8879
[cc_cc](#cc-cc)
: crate::ClientConfig::cert_compressors
[sc_cc](#sc-cc)
: crate::ServerConfig::cert_compressors
[cc_cd](#cc-cd)
: crate::ClientConfig::cert_decompressors
[sc_cd](#sc-cd)
: crate::ServerConfig::cert_decompressors

## Structs

### `DecompressionFailed`

```rust
struct DecompressionFailed;
```

A content-less error for when `CertDecompressor::decompress` fails.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CompressionFailed`

```rust
struct CompressionFailed;
```

A content-less error for when `CertCompressor::compress` fails.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CompressionCacheInner`

```rust
struct CompressionCacheInner {
    // [REDACTED: Private Fields]
}
```

Innards of an enabled CompressionCache.

You cannot make one of these directly. Use `CompressionCache::new`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `CompressionLevel`

```rust
enum CompressionLevel {
    Interactive,
    Amortized,
}
```

A hint for how many resources to dedicate to a compression.

#### Variants

- **`Interactive`**

  This compression is happening interactively during a handshake.
  
  Implementations may wish to choose a conservative compression level.

- **`Amortized`**

  The compression may be amortized over many connections.
  
  Implementations may wish to choose an aggressive compression level.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> CompressionLevel`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CompressionLevel) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CompressionCache`

```rust
enum CompressionCache {
    Disabled,
    Enabled(CompressionCacheInner),
}
```

An LRU cache for compressions.

The prospect of being able to reuse a given compression for many connections
means we can afford to spend more time on that compression (by passing
`CompressionLevel::Amortized` to the compressor).

#### Variants

- **`Disabled`**

  No caching happens, and compression happens each time using
  `CompressionLevel::Interactive`.

- **`Enabled`**

  Compressions are stored in an LRU cache.

#### Implementations

- `fn new(size: usize) -> Self`
  Make a `CompressionCache` that stores up to `size` compressed

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

## Traits

### `CertDecompressor`

```rust
trait CertDecompressor: Debug + Send + Sync { ... }
```

An available certificate decompression algorithm.

#### Required Methods

- `fn decompress(self: &Self, input: &[u8], output: &mut [u8]) -> Result<(), DecompressionFailed>`

  Decompress `input`, writing the result to `output`.

- `fn algorithm(self: &Self) -> CertificateCompressionAlgorithm`

  Which algorithm this decompressor handles.

### `CertCompressor`

```rust
trait CertCompressor: Debug + Send + Sync { ... }
```

An available certificate compression algorithm.

#### Required Methods

- `fn compress(self: &Self, input: Vec<u8>, level: CompressionLevel) -> Result<Vec<u8>, CompressionFailed>`

  Compress `input`, returning the result.

- `fn algorithm(self: &Self) -> CertificateCompressionAlgorithm`

  Which algorithm this compressor handles.

## Functions

### `default_cert_decompressors`

```rust
fn default_cert_decompressors() -> &'static [&'static dyn CertDecompressor]
```

Returns the supported `CertDecompressor` implementations enabled
by crate features.

### `default_cert_compressors`

```rust
fn default_cert_compressors() -> &'static [&'static dyn CertCompressor]
```

Returns the supported `CertCompressor` implementations enabled
by crate features.

