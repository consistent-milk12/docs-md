*[miniz_oxide](../index.md) / [inflate](index.md)*

---

# Module `inflate`

This module contains functionality for decompression.

## Modules

- [`core`](core/index.md) - Streaming decompression functionality.
- [`stream`](stream/index.md) - Extra streaming decompression functionality.

## Structs

### `DecompressError`

```rust
struct DecompressError {
    pub status: TINFLStatus,
    pub output: crate::alloc::vec::Vec<u8>,
}
```

Struct return when decompress_to_vec functions fail.

#### Fields

- **`status`**: `TINFLStatus`

  Decompressor status on failure. See [TINFLStatus] for details.

- **`output`**: `crate::alloc::vec::Vec<u8>`

  The currently decompressed data if any.

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

##### `impl Display`

- `fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn from_i32(value: i32) -> Option<TINFLStatus>`

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

- `fn clone(self: &Self) -> TINFLStatus`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &TINFLStatus) -> bool`

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

## Functions

### `decompress_to_vec`

```rust
fn decompress_to_vec(input: &[u8]) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```

Decompress the deflate-encoded data in `input` to a vector.

NOTE: This function will not bound the output, so if the output is large enough it can result in an out of memory error.
It is therefore suggested to not use this for anything other than test programs, use the functions with a specified limit, or
ideally streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`](../../clap_builder/error/index.md) containing the [`Vec`](#vec) of decompressed data on success, and a [struct][DecompressError] containing the status and so far decompressed data if any on failure.

### `decompress_to_vec_zlib`

```rust
fn decompress_to_vec_zlib(input: &[u8]) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```

Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector.

NOTE: This function will not bound the output, so if the output is large enough it can result in an out of memory error.
It is therefore suggested to not use this for anything other than test programs, use the functions with a specified limit, or
ideally streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`](../../clap_builder/error/index.md) containing the [`Vec`](#vec) of decompressed data on success, and a [struct][DecompressError] containing the status and so far decompressed data if any on failure.

### `decompress_to_vec_with_limit`

```rust
fn decompress_to_vec_with_limit(input: &[u8], max_size: usize) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```

Decompress the deflate-encoded data in `input` to a vector.

The vector is grown to at most `max_size` bytes; if the data does not fit in that size,
the error [struct][DecompressError] will contain the status `TINFLStatus::HasMoreOutput` and the data that was decompressed on failure.

As this function tries to decompress everything in one go, it's not ideal for general use outside of tests or where the output size is expected to be small.
It is suggested to use streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`](../../clap_builder/error/index.md) containing the [`Vec`](#vec) of decompressed data on success, and a [struct][DecompressError] on failure.

### `decompress_to_vec_zlib_with_limit`

```rust
fn decompress_to_vec_zlib_with_limit(input: &[u8], max_size: usize) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```

Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector.
The vector is grown to at most `max_size` bytes; if the data does not fit in that size,
the error [struct][DecompressError] will contain the status `TINFLStatus::HasMoreOutput` and the data that was decompressed on failure.

As this function tries to decompress everything in one go, it's not ideal for general use outside of tests or where the output size is expected to be small.
It is suggested to use streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`](../../clap_builder/error/index.md) containing the [`Vec`](#vec) of decompressed data on success, and a [struct][DecompressError] on failure.

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

