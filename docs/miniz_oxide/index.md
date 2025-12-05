# Crate `miniz_oxide`

A pure rust replacement for the [miniz](https://github.com/richgel999/miniz)
DEFLATE/zlib encoder/decoder.
Used a rust back-end for the
[flate2](https://github.com/alexcrichton/flate2-rs) crate.


## Modules

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

- `const fn error(error: MZError) -> StreamResult` — [`MZError`](../index.md), [`StreamResult`](../index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> StreamResult` — [`StreamResult`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StreamResult) -> bool` — [`StreamResult`](../index.md)

##### `impl StructuralPartialEq`

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

- `fn new(flush: i32) -> Result<Self, MZError>` — [`MZError`](../index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> MZFlush` — [`MZFlush`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &MZFlush) -> bool` — [`MZFlush`](../index.md)

##### `impl StructuralPartialEq`

### `MZStatus`

```rust
enum MZStatus {
    Ok,
    StreamEnd,
    NeedDict,
}
```

A list of miniz successful status codes.

These are emitted as the [`Ok`](#ok) side of a [`MZResult`](#mzresult) in the [`StreamResult`](#streamresult) returned from
`deflate::stream::deflate()` or `inflate::stream::inflate()`.

#### Variants

- **`Ok`**

  Operation succeeded.
  
  Some data was decompressed or compressed; see the byte counters in the [`StreamResult`](#streamresult) for
  details.

- **`StreamEnd`**

  Operation succeeded and end of deflate stream was found.
  
  X-ref `TINFLStatus::Done` or
  `TDEFLStatus::Done` for `inflate` or `deflate`
  respectively.

- **`NeedDict`**

  Unused

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> MZStatus` — [`MZStatus`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &MZStatus) -> bool` — [`MZStatus`](../index.md)

##### `impl StructuralPartialEq`

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

These are emitted as the [`Err`](#err) side of a [`MZResult`](#mzresult) in the [`StreamResult`](#streamresult) returned from
`deflate::stream::deflate()` or `inflate::stream::inflate()`.

#### Variants

- **`ErrNo`**

  Unused

- **`Stream`**

  General stream error.
  
  See `inflate::stream::inflate()` docs for details of how it can occur there.
  
  See `deflate::stream::deflate()` docs for how it can in principle occur there, though it's
  believed impossible in practice.

- **`Data`**

  Error in inflation; see `inflate::stream::inflate()` for details.
  
  Not returned from `deflate::stream::deflate()`.

- **`Mem`**

  Unused

- **`Buf`**

  Buffer-related error.
  
  See the docs of `deflate::stream::deflate()` or `inflate::stream::inflate()` for details
  of when it would trigger in the one you're using.

- **`Version`**

  Unused

- **`Param`**

  Bad parameters.
  
  This can be returned from `deflate::stream::deflate()` in the case of bad parameters.  See
  `TDEFLStatus::BadParam`.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> MZError` — [`MZError`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &MZError) -> bool` — [`MZError`](../index.md)

##### `impl StructuralPartialEq`

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

- `fn from_window_bits(window_bits: i32) -> DataFormat` — [`DataFormat`](../index.md)

- `fn to_window_bits(self: Self) -> i32`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> DataFormat` — [`DataFormat`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &DataFormat) -> bool` — [`DataFormat`](../index.md)

##### `impl StructuralPartialEq`

## Type Aliases

### `MZResult`

```rust
type MZResult = Result<MZStatus, MZError>;
```

`Result` alias for all miniz status codes both successful and failed.

