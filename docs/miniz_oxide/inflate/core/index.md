*[miniz_oxide](../../index.md) / [inflate](../index.md) / [core](index.md)*

---

# Module `core`

Streaming decompression functionality.

## Modules

- [`inflate_flags`](inflate_flags/index.md) - Flags to [`decompress()`] to control how inflation works.

## Structs

### `DecompressorOxide`

```rust
struct DecompressorOxide {
    // [REDACTED: Private Fields]
}
```

Main decompression struct.


#### Implementations

- `fn new() -> DecompressorOxide`
  Create a new tinfl_decompressor with all fields set to 0.

- `fn init(self: &mut Self)`
  Set the current state to `Start`.

- `fn adler32(self: &Self) -> Option<u32>`
  Returns the adler32 checksum of the currently decompressed data.

- `fn adler32_header(self: &Self) -> Option<u32>`
  Returns the adler32 that was read from the zlib header if it exists.

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

- `fn clone(self: &Self) -> DecompressorOxide`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl Default`

- `fn default() -> Self`
  Create a new tinfl_decompressor with all fields set to 0.

## Functions

### `decompress`

```rust
fn decompress(r: &mut DecompressorOxide, in_buf: &[u8], out: &mut [u8], out_pos: usize, flags: u32) -> (TINFLStatus, usize, usize)
```

Main decompression function. Keeps decompressing data from `in_buf` until the `in_buf` is
empty, `out` is full, the end of the deflate stream is hit, or there is an error in the
deflate stream.

# Arguments

`r` is a [`DecompressorOxide`](#decompressoroxide) struct with the state of this stream.

`in_buf` is a reference to the compressed data that is to be decompressed. The decompressor will
start at the first byte of this buffer.

`out` is a reference to the buffer that will store the decompressed data, and that
stores previously decompressed data if any.

* The offset given by `out_pos` indicates where in the output buffer slice writing should start.
* If [`TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF`](inflate_flags/index.md) is not set, the output buffer is used in a
  wrapping manner, and it's size is required to be a power of 2.
* The decompression function normally needs access to 32KiB of the previously decompressed data
  (or to the beginning of the decompressed data if less than 32KiB has been decompressed.)
    - If this data is not available, decompression may fail.
    - Some deflate compressors allow specifying a window size which limits match distances to
      less than this, or alternatively an RLE mode where matches will only refer to the previous byte
      and thus allows a smaller output buffer. The window size can be specified in the zlib
      header structure, however, the header data should not be relied on to be correct.

`flags` indicates settings and status to the decompression function.
* The [`TINFL_FLAG_HAS_MORE_INPUT`](inflate_flags/index.md) has to be specified if more compressed data is to be provided
  in a subsequent call to this function.
* See the the [`inflate_flags`](inflate_flags/index.md) module for details on other flags.

# Returns

Returns a tuple containing the status of the compressor, the number of input bytes read, and the
number of bytes output to `out`.

## Constants

### `TINFL_LZ_DICT_SIZE`

```rust
const TINFL_LZ_DICT_SIZE: usize = 32_768usize;
```

