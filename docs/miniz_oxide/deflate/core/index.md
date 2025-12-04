*[miniz_oxide](../../index.md) / [deflate](../index.md) / [core](index.md)*

---

# Module `core`

Streaming compression functionality.

## Modules

- [`deflate_flags`](deflate_flags/index.md) - 

## Structs

### `CompressorOxide`

```rust
struct CompressorOxide {
    // [REDACTED: Private Fields]
}
```

Main compression struct.

#### Implementations

- `fn new(flags: u32) -> Self`
  Create a new `CompressorOxide` with the given flags.

- `const fn adler32(self: &Self) -> u32`
  Get the adler32 checksum of the currently encoded data.

- `const fn prev_return_status(self: &Self) -> TDEFLStatus`
  Get the return status of the previous [`compress`](fn.compress.html)

- `const fn flags(self: &Self) -> i32`
  Get the raw compressor flags.

- `const fn data_format(self: &Self) -> DataFormat`
  Returns whether the compressor is wrapping the data in a zlib format or not.

- `fn reset(self: &mut Self)`
  Reset the state of the compressor, keeping the same parameters.

- `fn set_compression_level(self: &mut Self, level: CompressionLevel)`
  Set the compression level of the compressor.

- `fn set_compression_level_raw(self: &mut Self, level: u8)`
  Set the compression level of the compressor using an integer value.

- `fn set_format_and_level(self: &mut Self, data_format: DataFormat, level: u8)`
  Update the compression settings of the compressor.

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

##### `impl Default`

- `fn default() -> Self`
  Initialize the compressor with a level of 4, zlib wrapper and

### `CallbackFunc<'a>`

```rust
struct CallbackFunc<'a> {
    pub put_buf_func: &'a mut dyn FnMut(&[u8]) -> bool,
}
```

Callback function and user used in `compress_to_output`.

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

## Enums

### `CompressionStrategy`

```rust
enum CompressionStrategy {
    Default,
    Filtered,
    HuffmanOnly,
    RLE,
    Fixed,
}
```

Strategy setting for compression.

The non-default settings offer some special-case compression variants.

#### Variants

- **`Default`**

  Don't use any of the special strategies.

- **`Filtered`**

  Only use matches that are at least 5 bytes long.

- **`HuffmanOnly`**

  Don't look for matches, only huffman encode the literals.

- **`RLE`**

  Only look for matches with a distance of 1, i.e do run-length encoding only.

- **`Fixed`**

  Only use static/fixed blocks. (Blocks using the default huffman codes
  specified in the deflate specification.)

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

- `fn clone(self: &Self) -> CompressionStrategy`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CompressionStrategy) -> bool`

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

### `TDEFLFlush`

```rust
enum TDEFLFlush {
    None,
    Sync,
    Full,
    Finish,
}
```

A list of deflate flush types.

#### Variants

- **`None`**

  Normal operation.
  
  Compress as much as there is space for, and then return waiting for more input.

- **`Sync`**

  Try to flush all the current data and output an empty raw block.

- **`Full`**

  Same as `Sync`, but reset the dictionary so that the following data does not
  depend on previous data.

- **`Finish`**

  Try to flush everything and end the deflate stream.
  
  On success this will yield a `TDEFLStatus::Done` return status.

#### Implementations

- `const fn new(flush: i32) -> core::result::Result<Self, MZError>`

#### Trait Implementations

##### `impl From`

- `fn from(flush: MZFlush) -> Self`

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

- `fn clone(self: &Self) -> TDEFLFlush`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &TDEFLFlush) -> bool`

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

### `TDEFLStatus`

```rust
enum TDEFLStatus {
    BadParam,
    PutBufFailed,
    Okay,
    Done,
}
```

Return status of compression.

#### Variants

- **`BadParam`**

  Usage error.
  
  This indicates that either the [`CompressorOxide`](#compressoroxide) experienced a previous error, or the
  stream has already been `TDEFLFlush::Finish`'d.

- **`PutBufFailed`**

  Error putting data into output buffer.
  
  This usually indicates a too-small buffer.

- **`Okay`**

  Compression succeeded normally.

- **`Done`**

  Compression succeeded and the deflate stream was ended.
  
  This is the result of calling compression with `TDEFLFlush::Finish`.

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

- `fn clone(self: &Self) -> TDEFLStatus`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &TDEFLStatus) -> bool`

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

### `compress`

```rust
fn compress(d: &mut CompressorOxide, in_buf: &[u8], out_buf: &mut [u8], flush: TDEFLFlush) -> (TDEFLStatus, usize, usize)
```

Main compression function. Tries to compress as much as possible from `in_buf` and
puts compressed output into `out_buf`.

The value of `flush` determines if the compressor should attempt to flush all output
and alternatively try to finish the stream.

Use `TDEFLFlush::Finish` on the final call to signal that the stream is finishing.

Note that this function does not keep track of whether a flush marker has been output, so
if called using `TDEFLFlush::Sync`, the caller needs to ensure there is enough space in the
output buffer if they want to avoid repeated flush markers.
See #105 for details.

# Returns
Returns a tuple containing the current status of the compressor, the current position
in the input buffer and the current position in the output buffer.

### `compress_to_output`

```rust
fn compress_to_output(d: &mut CompressorOxide, in_buf: &[u8], flush: TDEFLFlush, callback_func: impl FnMut(&[u8]) -> bool) -> (TDEFLStatus, usize)
```

Main compression function. Callbacks output.

# Returns
Returns a tuple containing the current status of the compressor, the current position
in the input buffer.

The caller is responsible for ensuring the `CallbackFunc` struct will not cause undefined
behaviour.

### `create_comp_flags_from_zip_params`

```rust
fn create_comp_flags_from_zip_params(level: i32, window_bits: i32, strategy: i32) -> u32
```

Create a set of compression flags using parameters used by zlib and other compressors.
Mainly intended for use with transition from c libraries as it deals with raw integers.

# Parameters
`level` determines compression level. Clamped to maximum of 10. Negative values result in
`CompressionLevel::DefaultLevel`.
`window_bits`: Above 0, wraps the stream in a zlib wrapper, 0 or negative for a raw deflate
stream.
`strategy`: Sets the strategy if this conforms to any of the values in `CompressionStrategy`.

# Notes
This function may be removed or moved to the `miniz_oxide_c_api` in the future.

