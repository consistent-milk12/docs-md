*[miniz_oxide](../index.md) / [inflate](index.md)*

---

# Module `inflate`

This module contains functionality for decompression.

## Contents

- [Modules](#modules)
  - [`core`](#core)
  - [`output_buffer`](#output_buffer)
  - [`stream`](#stream)
- [Enums](#enums)
  - [`TINFLStatus`](#tinflstatus)
- [Functions](#functions)
  - [`decompress_slice_iter_to_slice`](#decompress_slice_iter_to_slice)
- [Constants](#constants)
  - [`TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS`](#tinfl_status_failed_cannot_make_progress)
  - [`TINFL_STATUS_BAD_PARAM`](#tinfl_status_bad_param)
  - [`TINFL_STATUS_ADLER32_MISMATCH`](#tinfl_status_adler32_mismatch)
  - [`TINFL_STATUS_FAILED`](#tinfl_status_failed)
  - [`TINFL_STATUS_DONE`](#tinfl_status_done)
  - [`TINFL_STATUS_NEEDS_MORE_INPUT`](#tinfl_status_needs_more_input)
  - [`TINFL_STATUS_HAS_MORE_OUTPUT`](#tinfl_status_has_more_output)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`core`](#core) | mod | Streaming decompression functionality. |
| [`output_buffer`](#output_buffer) | mod |  |
| [`stream`](#stream) | mod | Extra streaming decompression functionality. |
| [`TINFLStatus`](#tinflstatus) | enum | Return status codes. |
| [`decompress_slice_iter_to_slice`](#decompress_slice_iter_to_slice) | fn | Decompress one or more source slices from an iterator into the output slice. |
| [`TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS`](#tinfl_status_failed_cannot_make_progress) | const |  |
| [`TINFL_STATUS_BAD_PARAM`](#tinfl_status_bad_param) | const |  |
| [`TINFL_STATUS_ADLER32_MISMATCH`](#tinfl_status_adler32_mismatch) | const |  |
| [`TINFL_STATUS_FAILED`](#tinfl_status_failed) | const |  |
| [`TINFL_STATUS_DONE`](#tinfl_status_done) | const |  |
| [`TINFL_STATUS_NEEDS_MORE_INPUT`](#tinfl_status_needs_more_input) | const |  |
| [`TINFL_STATUS_HAS_MORE_OUTPUT`](#tinfl_status_has_more_output) | const |  |

## Modules

- [`core`](core/index.md) — Streaming decompression functionality.
- [`output_buffer`](output_buffer/index.md)
- [`stream`](stream/index.md) — Extra streaming decompression functionality.

## Enums

### `TINFLStatus`

```rust
enum TINFLStatus {
    FailedCannotMakeProgress,
    BadParam,
    Adler32Mismatch,
    Failed,
    Done,
    NeedsMoreInput,
    HasMoreOutput,
}
```

Return status codes.

#### Variants

- **`FailedCannotMakeProgress`**

  More input data was expected, but the caller indicated that there was no more data, so the
  input stream is likely truncated.
  
  This can't happen if you have provided the
  `TINFL_FLAG_HAS_MORE_INPUT` flag to the
  decompression.  By setting that flag, you indicate more input exists but is not provided,
  and so reaching the end of the input data without finding the end of the compressed stream
  would instead return a `NeedsMoreInput` status.

- **`BadParam`**

  The output buffer is an invalid size; consider the `flags` parameter.

- **`Adler32Mismatch`**

  The decompression went fine, but the adler32 checksum did not match the one
  provided in the header.

- **`Failed`**

  Failed to decompress due to invalid data.

- **`Done`**

  Finished decompression without issues.
  
  This indicates the end of the compressed stream has been reached.

- **`NeedsMoreInput`**

  The decompressor needs more input data to continue decompressing.
  
  This occurs when there's no more consumable input, but the end of the stream hasn't been
  reached, and you have supplied the
  `TINFL_FLAG_HAS_MORE_INPUT` flag to the
  decompressor.  Had you not supplied that flag (which would mean you were asserting that you
  believed all the data was available) you would have gotten a
  `FailedCannotMakeProcess` instead.

- **`HasMoreOutput`**

  There is still pending data that didn't fit in the output buffer.

#### Implementations

- <span id="tinflstatus-from-i32"></span>`fn from_i32(value: i32) -> Option<TINFLStatus>` — [`TINFLStatus`](#tinflstatus)

#### Trait Implementations

##### `impl Clone for TINFLStatus`

- <span id="tinflstatus-clone"></span>`fn clone(&self) -> TINFLStatus` — [`TINFLStatus`](#tinflstatus)

##### `impl Copy for TINFLStatus`

##### `impl Debug for TINFLStatus`

- <span id="tinflstatus-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TINFLStatus`

##### `impl Hash for TINFLStatus`

- <span id="tinflstatus-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for TINFLStatus`

- <span id="tinflstatus-eq"></span>`fn eq(&self, other: &TINFLStatus) -> bool` — [`TINFLStatus`](#tinflstatus)

##### `impl StructuralPartialEq for TINFLStatus`

## Functions

### `decompress_slice_iter_to_slice`

```rust
fn decompress_slice_iter_to_slice<'out, 'inp>(out: &'out mut [u8], it: impl Iterator<Item = &'inp [u8]>, zlib_header: bool, ignore_adler32: bool) -> Result<usize, TINFLStatus>
```

Decompress one or more source slices from an iterator into the output slice.

* On success, returns the number of bytes that were written.
* On failure, returns the failure status code.

This will fail if the output buffer is not large enough, but in that case
the output buffer will still contain the partial decompression.

* `out` the output buffer.
* `it` the iterator of input slices.
* `zlib_header` if the first slice out of the iterator is expected to have a
  Zlib header. Otherwise the slices are assumed to be the deflate data only.
* `ignore_adler32` if the adler32 checksum should be calculated or not.

## Constants

### `TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS`

```rust
const TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS: i32 = -4i32;
```

### `TINFL_STATUS_BAD_PARAM`

```rust
const TINFL_STATUS_BAD_PARAM: i32 = -3i32;
```

### `TINFL_STATUS_ADLER32_MISMATCH`

```rust
const TINFL_STATUS_ADLER32_MISMATCH: i32 = -2i32;
```

### `TINFL_STATUS_FAILED`

```rust
const TINFL_STATUS_FAILED: i32 = -1i32;
```

### `TINFL_STATUS_DONE`

```rust
const TINFL_STATUS_DONE: i32 = 0i32;
```

### `TINFL_STATUS_NEEDS_MORE_INPUT`

```rust
const TINFL_STATUS_NEEDS_MORE_INPUT: i32 = 1i32;
```

### `TINFL_STATUS_HAS_MORE_OUTPUT`

```rust
const TINFL_STATUS_HAS_MORE_OUTPUT: i32 = 2i32;
```

