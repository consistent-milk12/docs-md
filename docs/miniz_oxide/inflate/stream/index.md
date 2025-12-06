*[miniz_oxide](../../index.md) / [inflate](../index.md) / [stream](index.md)*

---

# Module `stream`

Extra streaming decompression functionality.

As of now this is mainly intended for use to build a higher-level wrapper.

## Structs

### `MinReset`

```rust
struct MinReset;
```

Resets state, without performing expensive ops (e.g. zeroing buffer)

Note that not zeroing buffer can lead to security issues when dealing with untrusted input.

#### Trait Implementations

##### `impl ResetPolicy for MinReset`

- `fn reset(self: &Self, state: &mut InflateState)` — [`InflateState`](#inflatestate)

### `ZeroReset`

```rust
struct ZeroReset;
```

Resets state and zero memory, continuing to use the same data format.

#### Trait Implementations

##### `impl ResetPolicy for ZeroReset`

- `fn reset(self: &Self, state: &mut InflateState)` — [`InflateState`](#inflatestate)

### `FullReset`

```rust
struct FullReset(crate::DataFormat);
```

Full reset of the state, including zeroing memory.

Requires to provide new data format.

#### Trait Implementations

##### `impl ResetPolicy for FullReset`

- `fn reset(self: &Self, state: &mut InflateState)` — [`InflateState`](#inflatestate)

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

- `fn new(data_format: DataFormat) -> InflateState` — [`DataFormat`](../../index.md), [`InflateState`](#inflatestate)

- `fn decompressor(self: &mut Self) -> &mut DecompressorOxide` — [`DecompressorOxide`](../core/index.md)

- `const fn last_status(self: &Self) -> TINFLStatus` — [`TINFLStatus`](../index.md)

- `fn reset(self: &mut Self, data_format: DataFormat)` — [`DataFormat`](../../index.md)

- `fn reset_as<T: ResetPolicy>(self: &mut Self, policy: T)`

#### Trait Implementations

##### `impl Clone for InflateState`

- `fn clone(self: &Self) -> InflateState` — [`InflateState`](#inflatestate)

##### `impl Default for InflateState`

- `fn default() -> Self`

## Traits

### `ResetPolicy`

```rust
trait ResetPolicy { ... }
```

Tag that determines reset policy of [InflateState](#inflatestate)

#### Required Methods

- `fn reset(self: &Self, state: &mut InflateState)`

  Performs reset

## Functions

### `inflate`

```rust
fn inflate(state: &mut InflateState, input: &[u8], output: &mut [u8], flush: crate::MZFlush) -> crate::StreamResult
```

Try to decompress from `input` to `output` with the given [`InflateState`](#inflatestate)

# `flush`

Generally, the various [`MZFlush`](../../index.md) flags have meaning only on the compression side.  They can be
supplied here, but the only one that has any semantic meaning is `MZFlush::Finish`, which is a
signal that the stream is expected to finish, and failing to do so is an error.  It isn't
necessary to specify it when the stream ends; you'll still get returned a
`MZStatus::StreamEnd` anyway.  Other values either have no effect or cause errors.  It's
likely that you'll almost always just want to use `MZFlush::None`.

# Errors

Returns `MZError::Buf` if the size of the `output` slice is empty or no progress was made due
to lack of expected input data, or if called with `MZFlush::Finish` and input wasn't all
consumed.

Returns `MZError::Data` if this or a a previous call failed with an error return from
[`TINFLStatus`](../index.md); probably indicates corrupted data.

Returns `MZError::Stream` when called with `MZFlush::Full` (meaningless on
decompression), or when called without `MZFlush::Finish` after an earlier call with
`MZFlush::Finish` has been made.

