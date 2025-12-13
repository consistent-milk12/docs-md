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

- [`inflate`](inflate/index.md) — This module contains functionality for decompression.
- [`shared`](shared/index.md)

## Structs

### `StreamResult`

```rust
struct StreamResult {
    pub bytes_consumed: usize,
    pub bytes_written: usize,
    pub status: MZResult,
}
```

*Defined in [`miniz_oxide-0.8.9/src/lib.rs:189-196`](../../.source_1765633015/miniz_oxide-0.8.9/src/lib.rs#L189-L196)*

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

##### `impl Any for StreamResult`

- <span id="streamresult-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StreamResult`

- <span id="streamresult-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StreamResult`

- <span id="streamresult-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StreamResult`

- <span id="streamresult-clone"></span>`fn clone(&self) -> StreamResult` — [`StreamResult`](#streamresult)

##### `impl CloneToUninit for StreamResult`

- <span id="streamresult-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StreamResult`

##### `impl Debug for StreamResult`

- <span id="streamresult-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StreamResult`

##### `impl<T> From for StreamResult`

- <span id="streamresult-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for StreamResult`

- <span id="streamresult-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for StreamResult`

- <span id="streamresult-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for StreamResult`

- <span id="streamresult-partialeq-eq"></span>`fn eq(&self, other: &StreamResult) -> bool` — [`StreamResult`](#streamresult)

##### `impl StructuralPartialEq for StreamResult`

##### `impl<U> TryFrom for StreamResult`

- <span id="streamresult-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="streamresult-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StreamResult`

- <span id="streamresult-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="streamresult-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`miniz_oxide-0.8.9/src/lib.rs:47-66`](../../.source_1765633015/miniz_oxide-0.8.9/src/lib.rs#L47-L66)*

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

  Create an MZFlush value from an integer value.

  

  Returns `MZError::Param` on invalid values.

#### Trait Implementations

##### `impl Any for MZFlush`

- <span id="mzflush-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MZFlush`

- <span id="mzflush-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MZFlush`

- <span id="mzflush-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MZFlush`

- <span id="mzflush-clone"></span>`fn clone(&self) -> MZFlush` — [`MZFlush`](#mzflush)

##### `impl CloneToUninit for MZFlush`

- <span id="mzflush-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MZFlush`

##### `impl Debug for MZFlush`

- <span id="mzflush-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MZFlush`

##### `impl<T> From for MZFlush`

- <span id="mzflush-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for MZFlush`

- <span id="mzflush-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for MZFlush`

- <span id="mzflush-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MZFlush`

- <span id="mzflush-partialeq-eq"></span>`fn eq(&self, other: &MZFlush) -> bool` — [`MZFlush`](#mzflush)

##### `impl StructuralPartialEq for MZFlush`

##### `impl<U> TryFrom for MZFlush`

- <span id="mzflush-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mzflush-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MZFlush`

- <span id="mzflush-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mzflush-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MZStatus`

```rust
enum MZStatus {
    Ok,
    StreamEnd,
    NeedDict,
}
```

*Defined in [`miniz_oxide-0.8.9/src/lib.rs:90-106`](../../.source_1765633015/miniz_oxide-0.8.9/src/lib.rs#L90-L106)*

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

##### `impl Any for MZStatus`

- <span id="mzstatus-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MZStatus`

- <span id="mzstatus-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MZStatus`

- <span id="mzstatus-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MZStatus`

- <span id="mzstatus-clone"></span>`fn clone(&self) -> MZStatus` — [`MZStatus`](#mzstatus)

##### `impl CloneToUninit for MZStatus`

- <span id="mzstatus-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MZStatus`

##### `impl Debug for MZStatus`

- <span id="mzstatus-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MZStatus`

##### `impl<T> From for MZStatus`

- <span id="mzstatus-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for MZStatus`

- <span id="mzstatus-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for MZStatus`

- <span id="mzstatus-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MZStatus`

- <span id="mzstatus-partialeq-eq"></span>`fn eq(&self, other: &MZStatus) -> bool` — [`MZStatus`](#mzstatus)

##### `impl StructuralPartialEq for MZStatus`

##### `impl<U> TryFrom for MZStatus`

- <span id="mzstatus-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mzstatus-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MZStatus`

- <span id="mzstatus-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mzstatus-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`miniz_oxide-0.8.9/src/lib.rs:115-149`](../../.source_1765633015/miniz_oxide-0.8.9/src/lib.rs#L115-L149)*

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

##### `impl Any for MZError`

- <span id="mzerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MZError`

- <span id="mzerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MZError`

- <span id="mzerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MZError`

- <span id="mzerror-clone"></span>`fn clone(&self) -> MZError` — [`MZError`](#mzerror)

##### `impl CloneToUninit for MZError`

- <span id="mzerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MZError`

##### `impl Debug for MZError`

- <span id="mzerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MZError`

##### `impl<T> From for MZError`

- <span id="mzerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for MZError`

- <span id="mzerror-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for MZError`

- <span id="mzerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MZError`

- <span id="mzerror-partialeq-eq"></span>`fn eq(&self, other: &MZError) -> bool` — [`MZError`](#mzerror)

##### `impl StructuralPartialEq for MZError`

##### `impl<U> TryFrom for MZError`

- <span id="mzerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mzerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MZError`

- <span id="mzerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mzerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DataFormat`

```rust
enum DataFormat {
    Zlib,
    ZLibIgnoreChecksum,
    Raw,
}
```

*Defined in [`miniz_oxide-0.8.9/src/lib.rs:155-163`](../../.source_1765633015/miniz_oxide-0.8.9/src/lib.rs#L155-L163)*

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

##### `impl Any for DataFormat`

- <span id="dataformat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DataFormat`

- <span id="dataformat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DataFormat`

- <span id="dataformat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DataFormat`

- <span id="dataformat-clone"></span>`fn clone(&self) -> DataFormat` — [`DataFormat`](#dataformat)

##### `impl CloneToUninit for DataFormat`

- <span id="dataformat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DataFormat`

##### `impl Debug for DataFormat`

- <span id="dataformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DataFormat`

##### `impl<T> From for DataFormat`

- <span id="dataformat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for DataFormat`

- <span id="dataformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for DataFormat`

- <span id="dataformat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for DataFormat`

- <span id="dataformat-partialeq-eq"></span>`fn eq(&self, other: &DataFormat) -> bool` — [`DataFormat`](#dataformat)

##### `impl StructuralPartialEq for DataFormat`

##### `impl<U> TryFrom for DataFormat`

- <span id="dataformat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dataformat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DataFormat`

- <span id="dataformat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dataformat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `MZResult`

```rust
type MZResult = Result<MZStatus, MZError>;
```

*Defined in [`miniz_oxide-0.8.9/src/lib.rs:184`](../../.source_1765633015/miniz_oxide-0.8.9/src/lib.rs#L184)*

`Result` alias for all miniz status codes both successful and failed.

