# Crate `miniz_oxide`

A pure rust replacement for the [miniz](https://github.com/richgel999/miniz)
DEFLATE/zlib encoder/decoder.
Used a rust back-end for the
[flate2](https://github.com/alexcrichton/flate2-rs) crate.


## Contents

- [Modules](#modules)
  - [`inflate`](#inflate)
  - [`shared`](#shared)
- [Structs](#structs)
  - [`StreamResult`](#streamresult)
- [Enums](#enums)
  - [`MZFlush`](#mzflush)
  - [`MZStatus`](#mzstatus)
  - [`MZError`](#mzerror)
  - [`DataFormat`](#dataformat)
- [Type Aliases](#type-aliases)
  - [`MZResult`](#mzresult)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`inflate`](#inflate) | mod | This module contains functionality for decompression. |
| [`shared`](#shared) | mod |  |
| [`StreamResult`](#streamresult) | struct | A structure containing the result of a call to the inflate or deflate streaming functions. |
| [`MZFlush`](#mzflush) | enum | A list of flush types. |
| [`MZStatus`](#mzstatus) | enum | A list of miniz successful status codes. |
| [`MZError`](#mzerror) | enum | A list of miniz failed status codes. |
| [`DataFormat`](#dataformat) | enum | How compressed data is wrapped. |
| [`MZResult`](#mzresult) | type | `Result` alias for all miniz status codes both successful and failed. |

## Modules

- [`inflate`](inflate/index.md) - This module contains functionality for decompression.
- [`shared`](shared/index.md) - 

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

- <span id="streamresult-error"></span>`const fn error(error: MZError) -> StreamResult` — [`MZError`](#mzerror), [`StreamResult`](#streamresult)

#### Trait Implementations

##### `impl Clone for StreamResult`

- <span id="streamresult-clone"></span>`fn clone(&self) -> StreamResult` — [`StreamResult`](#streamresult)

##### `impl Copy for StreamResult`

##### `impl Debug for StreamResult`

- <span id="streamresult-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StreamResult`

##### `impl Hash for StreamResult`

- <span id="streamresult-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for StreamResult`

- <span id="streamresult-eq"></span>`fn eq(&self, other: &StreamResult) -> bool` — [`StreamResult`](#streamresult)

##### `impl StructuralPartialEq for StreamResult`

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

- <span id="mzflush-new"></span>`fn new(flush: i32) -> Result<Self, MZError>` — [`MZError`](#mzerror)

#### Trait Implementations

##### `impl Clone for MZFlush`

- <span id="mzflush-clone"></span>`fn clone(&self) -> MZFlush` — [`MZFlush`](#mzflush)

##### `impl Copy for MZFlush`

##### `impl Debug for MZFlush`

- <span id="mzflush-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MZFlush`

##### `impl Hash for MZFlush`

- <span id="mzflush-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for MZFlush`

- <span id="mzflush-eq"></span>`fn eq(&self, other: &MZFlush) -> bool` — [`MZFlush`](#mzflush)

##### `impl StructuralPartialEq for MZFlush`

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
`deflate::stream::deflate()` or [`inflate::stream::inflate()`](inflate/stream/index.md).

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

##### `impl Clone for MZStatus`

- <span id="mzstatus-clone"></span>`fn clone(&self) -> MZStatus` — [`MZStatus`](#mzstatus)

##### `impl Copy for MZStatus`

##### `impl Debug for MZStatus`

- <span id="mzstatus-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MZStatus`

##### `impl Hash for MZStatus`

- <span id="mzstatus-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for MZStatus`

- <span id="mzstatus-eq"></span>`fn eq(&self, other: &MZStatus) -> bool` — [`MZStatus`](#mzstatus)

##### `impl StructuralPartialEq for MZStatus`

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

These are emitted as the `Err` side of a [`MZResult`](#mzresult) in the [`StreamResult`](#streamresult) returned from
`deflate::stream::deflate()` or [`inflate::stream::inflate()`](inflate/stream/index.md).

#### Variants

- **`ErrNo`**

  Unused

- **`Stream`**

  General stream error.
  
  See [`inflate::stream::inflate()`](inflate/stream/index.md) docs for details of how it can occur there.
  
  See `deflate::stream::deflate()` docs for how it can in principle occur there, though it's
  believed impossible in practice.

- **`Data`**

  Error in inflation; see [`inflate::stream::inflate()`](inflate/stream/index.md) for details.
  
  Not returned from `deflate::stream::deflate()`.

- **`Mem`**

  Unused

- **`Buf`**

  Buffer-related error.
  
  See the docs of `deflate::stream::deflate()` or [`inflate::stream::inflate()`](inflate/stream/index.md) for details
  of when it would trigger in the one you're using.

- **`Version`**

  Unused

- **`Param`**

  Bad parameters.
  
  This can be returned from `deflate::stream::deflate()` in the case of bad parameters.  See
  `TDEFLStatus::BadParam`.

#### Trait Implementations

##### `impl Clone for MZError`

- <span id="mzerror-clone"></span>`fn clone(&self) -> MZError` — [`MZError`](#mzerror)

##### `impl Copy for MZError`

##### `impl Debug for MZError`

- <span id="mzerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MZError`

##### `impl Hash for MZError`

- <span id="mzerror-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for MZError`

- <span id="mzerror-eq"></span>`fn eq(&self, other: &MZError) -> bool` — [`MZError`](#mzerror)

##### `impl StructuralPartialEq for MZError`

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

- <span id="dataformat-from-window-bits"></span>`fn from_window_bits(window_bits: i32) -> DataFormat` — [`DataFormat`](#dataformat)

- <span id="dataformat-to-window-bits"></span>`fn to_window_bits(self) -> i32`

#### Trait Implementations

##### `impl Clone for DataFormat`

- <span id="dataformat-clone"></span>`fn clone(&self) -> DataFormat` — [`DataFormat`](#dataformat)

##### `impl Copy for DataFormat`

##### `impl Debug for DataFormat`

- <span id="dataformat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DataFormat`

##### `impl Hash for DataFormat`

- <span id="dataformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for DataFormat`

- <span id="dataformat-eq"></span>`fn eq(&self, other: &DataFormat) -> bool` — [`DataFormat`](#dataformat)

##### `impl StructuralPartialEq for DataFormat`

## Type Aliases

### `MZResult`

```rust
type MZResult = Result<MZStatus, MZError>;
```

`Result` alias for all miniz status codes both successful and failed.

