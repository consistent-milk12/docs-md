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

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:21`](../../../../.source_1765210505/miniz_oxide-0.8.9/src/inflate/stream.rs#L21)*

Resets state, without performing expensive ops (e.g. zeroing buffer)

Note that not zeroing buffer can lead to security issues when dealing with untrusted input.

#### Trait Implementations

##### `impl ResetPolicy for MinReset`

- <span id="minreset-reset"></span>`fn reset(&self, state: &mut InflateState)` — [`InflateState`](#inflatestate)

### `ZeroReset`

```rust
struct ZeroReset;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:35`](../../../../.source_1765210505/miniz_oxide-0.8.9/src/inflate/stream.rs#L35)*

Resets state and zero memory, continuing to use the same data format.

#### Trait Implementations

##### `impl ResetPolicy for ZeroReset`

- <span id="zeroreset-reset"></span>`fn reset(&self, state: &mut InflateState)` — [`InflateState`](#inflatestate)

### `FullReset`

```rust
struct FullReset(crate::DataFormat);
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:48`](../../../../.source_1765210505/miniz_oxide-0.8.9/src/inflate/stream.rs#L48)*

Full reset of the state, including zeroing memory.

Requires to provide new data format.

#### Trait Implementations

##### `impl ResetPolicy for FullReset`

- <span id="fullreset-reset"></span>`fn reset(&self, state: &mut InflateState)` — [`InflateState`](#inflatestate)

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

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:61-83`](../../../../.source_1765210505/miniz_oxide-0.8.9/src/inflate/stream.rs#L61-L83)*

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

- <span id="inflatestate-decompressor"></span>`fn decompressor(&mut self) -> &mut DecompressorOxide` — [`DecompressorOxide`](../core/index.md#decompressoroxide)

- <span id="inflatestate-last-status"></span>`const fn last_status(&self) -> TINFLStatus` — [`TINFLStatus`](../index.md#tinflstatus)

- <span id="inflatestate-reset"></span>`fn reset(&mut self, data_format: DataFormat)` — [`DataFormat`](../../index.md#dataformat)

- <span id="inflatestate-reset-as"></span>`fn reset_as<T: ResetPolicy>(&mut self, policy: T)`

#### Trait Implementations

##### `impl Clone for InflateState`

- <span id="inflatestate-clone"></span>`fn clone(&self) -> InflateState` — [`InflateState`](#inflatestate)

##### `impl Default for InflateState`

- <span id="inflatestate-default"></span>`fn default() -> Self`

## Traits

### `ResetPolicy`

```rust
trait ResetPolicy { ... }
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:13-16`](../../../../.source_1765210505/miniz_oxide-0.8.9/src/inflate/stream.rs#L13-L16)*

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

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:186-295`](../../../../.source_1765210505/miniz_oxide-0.8.9/src/inflate/stream.rs#L186-L295)*

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

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:297-370`](../../../../.source_1765210505/miniz_oxide-0.8.9/src/inflate/stream.rs#L297-L370)*

### `push_dict_out`

```rust
fn push_dict_out(state: &mut InflateState, next_out: &mut &mut [u8]) -> usize
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/stream.rs:372-379`](../../../../.source_1765210505/miniz_oxide-0.8.9/src/inflate/stream.rs#L372-L379)*

