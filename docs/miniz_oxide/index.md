# Crate `miniz_oxide`

A pure rust replacement for the [miniz](https://github.com/richgel999/miniz)
DEFLATE/zlib encoder/decoder.
Used a rust back-end for the
[flate2](https://github.com/alexcrichton/flate2-rs) crate.

# Usage
## Simple compression/decompression:
``` rust

use miniz_oxide::inflate::decompress_to_vec;
use miniz_oxide::deflate::compress_to_vec;

fn roundtrip(data: &[u8](#u8)) {
    let compressed = compress_to_vec(data, 6);
    let decompressed = decompress_to_vec(compressed.as_slice()).expect("Failed to decompress!");
#   let _ = decompressed;
}

# roundtrip(b"Test_data test data lalalal blabla");

## Modules

- [`deflate`](deflate/index.md) - This module contains functionality for compression.
- [`inflate`](inflate/index.md) - This module contains functionality for decompression.

## Structs

### `StreamResult`

```rust
struct StreamResult {
    pub bytes_consumed: usize,
    pub bytes_written: usize,
    pub status: MZResult,
}
```

A structure containing the result of a call to the inflate or deflate streaming functions.

#### Fields

- **`bytes_consumed`**: `usize`

  The number of bytes consumed from the input slice.

- **`bytes_written`**: `usize`

  The number of bytes written to the output slice.

- **`status`**: `MZResult`

  The return status of the call.

#### Implementations

- `const fn error(error: MZError) -> StreamResult`

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

- `fn clone(self: &Self) -> StreamResult`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StreamResult) -> bool`

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

## Enums

### `MZFlush`

```rust
enum MZFlush {
    None,
    Partial,
    Sync,
    Full,
    Finish,
    Block,
}
```

A list of flush types.

See <http://www.bolet.org/~pornin/deflate-flush.html> for more in-depth info.

#### Variants

- **`None`**

  Don't force any flushing.
  Used when more input data is expected.

- **`Partial`**

  Zlib partial flush.
  Currently treated as [`Sync`](#sync).

- **`Sync`**

  Finish compressing the currently buffered data, and output an empty raw block.
  Has no use in decompression.

- **`Full`**

  Same as [`Sync`](#sync), but resets the compression dictionary so that further compressed
  data does not depend on data compressed before the flush.
  
  Has no use in decompression, and is an error to supply in that case.

- **`Finish`**

  Attempt to flush the remaining data and end the stream.

- **`Block`**

  Not implemented.

#### Implementations

- `fn new(flush: i32) -> Result<Self, MZError>`
  Create an MZFlush value from an integer value.

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

- `fn clone(self: &Self) -> MZFlush`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &MZFlush) -> bool`

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

### `MZStatus`

```rust
enum MZStatus {
    Ok,
    StreamEnd,
    NeedDict,
}
```

A list of miniz successful status codes.

These are emitted as the [`Ok`](#ok) side of a [`MZResult`](miniz_oxide/index.md) in the [`StreamResult`](miniz_oxide/index.md) returned from
[`deflate::stream::deflate()`](#deflate) or [`inflate::stream::inflate()`](#inflate).

#### Variants

- **`Ok`**

  Operation succeeded.
  
  Some data was decompressed or compressed; see the byte counters in the [`StreamResult`](miniz_oxide/index.md) for
  details.

- **`StreamEnd`**

  Operation succeeded and end of deflate stream was found.
  
  X-ref [`TINFLStatus::Done`](#done) or
  [`TDEFLStatus::Done`](#done) for `inflate` or `deflate`
  respectively.

- **`NeedDict`**

  Unused

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

- `fn clone(self: &Self) -> MZStatus`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &MZStatus) -> bool`

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

### `MZError`

```rust
enum MZError {
    ErrNo,
    Stream,
    Data,
    Mem,
    Buf,
    Version,
    Param,
}
```

A list of miniz failed status codes.

These are emitted as the [`Err`](#err) side of a [`MZResult`](miniz_oxide/index.md) in the [`StreamResult`](miniz_oxide/index.md) returned from
[`deflate::stream::deflate()`](#deflate) or [`inflate::stream::inflate()`](#inflate).

#### Variants

- **`ErrNo`**

  Unused

- **`Stream`**

  General stream error.
  
  See [`inflate::stream::inflate()`](#inflate) docs for details of how it can occur there.
  
  See [`deflate::stream::deflate()`](#deflate) docs for how it can in principle occur there, though it's
  believed impossible in practice.

- **`Data`**

  Error in inflation; see [`inflate::stream::inflate()`](#inflate) for details.
  
  Not returned from [`deflate::stream::deflate()`](#deflate).

- **`Mem`**

  Unused

- **`Buf`**

  Buffer-related error.
  
  See the docs of [`deflate::stream::deflate()`](#deflate) or [`inflate::stream::inflate()`](#inflate) for details
  of when it would trigger in the one you're using.

- **`Version`**

  Unused

- **`Param`**

  Bad parameters.
  
  This can be returned from [`deflate::stream::deflate()`](#deflate) in the case of bad parameters.  See
  [`TDEFLStatus::BadParam`](#badparam).

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

- `fn clone(self: &Self) -> MZError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &MZError) -> bool`

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

### `DataFormat`

```rust
enum DataFormat {
    Zlib,
    ZLibIgnoreChecksum,
    Raw,
}
```

How compressed data is wrapped.

#### Variants

- **`Zlib`**

  Wrapped using the [zlib](http://www.zlib.org/rfc-zlib.html) format.

- **`ZLibIgnoreChecksum`**

  Zlib wrapped but ignore and don't compute the adler32 checksum.
  Currently only used for inflate, behaves the same as Zlib for compression.

- **`Raw`**

  Raw DEFLATE.

#### Implementations

- `fn from_window_bits(window_bits: i32) -> DataFormat`

- `fn to_window_bits(self: Self) -> i32`

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

- `fn clone(self: &Self) -> DataFormat`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &DataFormat) -> bool`

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

## Type Aliases

### `MZResult`

```rust
type MZResult = Result<MZStatus, MZError>;
```

`Result` alias for all miniz status codes both successful and failed.

