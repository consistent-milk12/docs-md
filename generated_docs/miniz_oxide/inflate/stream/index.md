*[miniz_oxide](../../index.md) / [inflate](../index.md) / [stream](index.md)*

---

# Module `stream`

Extra streaming decompression functionality.

As of now this is mainly intended for use to build a higher-level wrapper.

## Contents

- [Structs](#structs)
  - [`MinReset`](#minreset)
  - [`ZeroReset`](#zeroreset)
  - [`FullReset`](#fullreset)
  - [`InflateState`](#inflatestate)
- [Traits](#traits)
  - [`ResetPolicy`](#resetpolicy)
- [Functions](#functions)
  - [`inflate`](#inflate)
  - [`inflate_loop`](#inflate-loop)
  - [`push_dict_out`](#push-dict-out)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MinReset`](#minreset) | struct | Resets state, without performing expensive ops (e.g. zeroing buffer) |
| [`ZeroReset`](#zeroreset) | struct | Resets state and zero memory, continuing to use the same data format. |
| [`FullReset`](#fullreset) | struct | Full reset of the state, including zeroing memory. |
| [`InflateState`](#inflatestate) | struct | A struct that compbines a decompressor with extra data for streaming decompression. |
| [`ResetPolicy`](#resetpolicy) | trait | Tag that determines reset policy of [InflateState](struct.InflateState.html) |
| [`inflate`](#inflate) | fn | Try to decompress from `input` to `output` with the given [`InflateState`] |
| [`inflate_loop`](#inflate-loop) | fn |  |
| [`push_dict_out`](#push-dict-out) | fn |  |

## Structs

### `MinReset`

```rust
struct MinReset;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:21`](../../../../.source_1765633015/miniz_oxide-0.8.9/src/inflate/stream.rs#L21)*

Resets state, without performing expensive ops (e.g. zeroing buffer)

Note that not zeroing buffer can lead to security issues when dealing with untrusted input.

#### Trait Implementations

##### `impl Any for MinReset`

- <span id="minreset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MinReset`

- <span id="minreset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MinReset`

- <span id="minreset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MinReset`

- <span id="minreset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MinReset`

- <span id="minreset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ResetPolicy for MinReset`

- <span id="minreset-resetpolicy-reset"></span>`fn reset(&self, state: &mut InflateState)` — [`InflateState`](#inflatestate)

##### `impl<U> TryFrom for MinReset`

- <span id="minreset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="minreset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MinReset`

- <span id="minreset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="minreset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ZeroReset`

```rust
struct ZeroReset;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:35`](../../../../.source_1765633015/miniz_oxide-0.8.9/src/inflate/stream.rs#L35)*

Resets state and zero memory, continuing to use the same data format.

#### Trait Implementations

##### `impl Any for ZeroReset`

- <span id="zeroreset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ZeroReset`

- <span id="zeroreset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ZeroReset`

- <span id="zeroreset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ZeroReset`

- <span id="zeroreset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ZeroReset`

- <span id="zeroreset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ResetPolicy for ZeroReset`

- <span id="zeroreset-resetpolicy-reset"></span>`fn reset(&self, state: &mut InflateState)` — [`InflateState`](#inflatestate)

##### `impl<U> TryFrom for ZeroReset`

- <span id="zeroreset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="zeroreset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ZeroReset`

- <span id="zeroreset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="zeroreset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FullReset`

```rust
struct FullReset(crate::DataFormat);
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:48`](../../../../.source_1765633015/miniz_oxide-0.8.9/src/inflate/stream.rs#L48)*

Full reset of the state, including zeroing memory.

Requires to provide new data format.

#### Trait Implementations

##### `impl Any for FullReset`

- <span id="fullreset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FullReset`

- <span id="fullreset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FullReset`

- <span id="fullreset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for FullReset`

- <span id="fullreset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FullReset`

- <span id="fullreset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ResetPolicy for FullReset`

- <span id="fullreset-resetpolicy-reset"></span>`fn reset(&self, state: &mut InflateState)` — [`InflateState`](#inflatestate)

##### `impl<U> TryFrom for FullReset`

- <span id="fullreset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fullreset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FullReset`

- <span id="fullreset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fullreset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `InflateState`

```rust
struct InflateState {
    decomp: crate::inflate::core::DecompressorOxide,
    dict: [u8; 32768],
    dict_ofs: usize,
    dict_avail: usize,
    first_call: bool,
    has_flushed: bool,
    data_format: crate::DataFormat,
    last_status: crate::inflate::TINFLStatus,
}
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:61-83`](../../../../.source_1765633015/miniz_oxide-0.8.9/src/inflate/stream.rs#L61-L83)*

A struct that compbines a decompressor with extra data for streaming decompression.


#### Fields

- **`decomp`**: `crate::inflate::core::DecompressorOxide`

  Inner decompressor struct

- **`dict`**: `[u8; 32768]`

  Buffer of input bytes for matches.
  TODO: Could probably do this a bit cleaner with some
  Cursor-like class.
  We may also look into whether we need to keep a buffer here, or just one in the
  decompressor struct.

- **`dict_ofs`**: `usize`

  Where in the buffer are we currently at?

- **`dict_avail`**: `usize`

  How many bytes of data to be flushed is there currently in the buffer?

- **`data_format`**: `crate::DataFormat`

  Whether the input data is wrapped in a zlib header and checksum.
  TODO: This should be stored in the decompressor.

#### Implementations

- <span id="inflatestate-new"></span>`fn new(data_format: DataFormat) -> InflateState` — [`DataFormat`](../../index.md#dataformat), [`InflateState`](#inflatestate)

  Create a new state.

  

  Note that this struct is quite large due to internal buffers, and as such storing it on

  the stack is not recommended.

  

  # Parameters

  `data_format`: Determines whether the compressed data is assumed to wrapped with zlib

  metadata.

- <span id="inflatestate-decompressor"></span>`fn decompressor(&mut self) -> &mut DecompressorOxide` — [`DecompressorOxide`](../core/index.md#decompressoroxide)

  Access the innner decompressor.

- <span id="inflatestate-last-status"></span>`const fn last_status(&self) -> TINFLStatus` — [`TINFLStatus`](../index.md#tinflstatus)

  Return the status of the last call to `inflate` with this `InflateState`.

- <span id="inflatestate-reset"></span>`fn reset(&mut self, data_format: DataFormat)` — [`DataFormat`](../../index.md#dataformat)

  Reset the decompressor without re-allocating memory, using the given

  data format.

- <span id="inflatestate-reset-as"></span>`fn reset_as<T: ResetPolicy>(&mut self, policy: T)`

  Resets the state according to specified policy.

#### Trait Implementations

##### `impl Any for InflateState`

- <span id="inflatestate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InflateState`

- <span id="inflatestate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InflateState`

- <span id="inflatestate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for InflateState`

- <span id="inflatestate-clone"></span>`fn clone(&self) -> InflateState` — [`InflateState`](#inflatestate)

##### `impl CloneToUninit for InflateState`

- <span id="inflatestate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Default for InflateState`

- <span id="inflatestate-default"></span>`fn default() -> Self`

##### `impl<T> From for InflateState`

- <span id="inflatestate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InflateState`

- <span id="inflatestate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for InflateState`

- <span id="inflatestate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inflatestate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InflateState`

- <span id="inflatestate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inflatestate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ResetPolicy`

```rust
trait ResetPolicy { ... }
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:13-16`](../../../../.source_1765633015/miniz_oxide-0.8.9/src/inflate/stream.rs#L13-L16)*

Tag that determines reset policy of [InflateState](#inflatestate)

#### Required Methods

- `fn reset(&self, state: &mut InflateState)`

  Performs reset

#### Implementors

- [`FullReset`](#fullreset)
- [`MinReset`](#minreset)
- [`ZeroReset`](#zeroreset)

## Functions

### `inflate`

```rust
fn inflate(state: &mut InflateState, input: &[u8], output: &mut [u8], flush: crate::MZFlush) -> crate::StreamResult
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:186-295`](../../../../.source_1765633015/miniz_oxide-0.8.9/src/inflate/stream.rs#L186-L295)*

Try to decompress from `input` to `output` with the given [`InflateState`](#inflatestate)

# `flush`

Generally, the various [`MZFlush`](../../index.md) flags have meaning only on the compression side.  They can be
supplied here, but the only one that has any semantic meaning is [`MZFlush::Finish`](../../index.md), which is a
signal that the stream is expected to finish, and failing to do so is an error.  It isn't
necessary to specify it when the stream ends; you'll still get returned a
[`MZStatus::StreamEnd`](../../index.md) anyway.  Other values either have no effect or cause errors.  It's
likely that you'll almost always just want to use [`MZFlush::None`](../../index.md).

# Errors

Returns [`MZError::Buf`](../../index.md) if the size of the `output` slice is empty or no progress was made due
to lack of expected input data, or if called with [`MZFlush::Finish`](../../index.md) and input wasn't all
consumed.

Returns [`MZError::Data`](../../index.md) if this or a a previous call failed with an error return from
[`TINFLStatus`](../index.md); probably indicates corrupted data.

Returns [`MZError::Stream`](../../index.md) when called with [`MZFlush::Full`](../../index.md) (meaningless on
decompression), or when called without [`MZFlush::Finish`](../../index.md) after an earlier call with
[`MZFlush::Finish`](../../index.md) has been made.

### `inflate_loop`

```rust
fn inflate_loop(state: &mut InflateState, next_in: &mut &[u8], next_out: &mut &mut [u8], total_in: &mut usize, total_out: &mut usize, decomp_flags: u32, flush: crate::MZFlush) -> crate::MZResult
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:297-370`](../../../../.source_1765633015/miniz_oxide-0.8.9/src/inflate/stream.rs#L297-L370)*

### `push_dict_out`

```rust
fn push_dict_out(state: &mut InflateState, next_out: &mut &mut [u8]) -> usize
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:372-379`](../../../../.source_1765633015/miniz_oxide-0.8.9/src/inflate/stream.rs#L372-L379)*

