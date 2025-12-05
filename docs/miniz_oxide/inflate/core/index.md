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
    state: core::State,
    num_bits: u32,
    z_header0: u32,
    z_header1: u32,
    z_adler32: u32,
    finish: u8,
    block_type: u8,
    check_adler32: u32,
    dist: u32,
    counter: u32,
    num_extra: u8,
    table_sizes: [u16; 3],
    bit_buf: u64,
    tables: [HuffmanTable; 3],
    code_size_literal: [u8; 288],
    code_size_dist: [u8; 32],
    code_size_huffman: [u8; 19],
    raw_header: [u8; 4],
    len_codes: [u8; 512],
}
```

Main decompression struct.


#### Fields

- **`state`**: `core::State`

  Current state of the decompressor.

- **`num_bits`**: `u32`

  Number of bits in the bit buffer.

- **`z_header0`**: `u32`

  Zlib CMF

- **`z_header1`**: `u32`

  Zlib FLG

- **`z_adler32`**: `u32`

  Adler32 checksum from the zlib header.

- **`finish`**: `u8`

  1 if the current block is the last block, 0 otherwise.

- **`block_type`**: `u8`

  The type of the current block.
  or if in a dynamic block, which huffman table we are currently

- **`check_adler32`**: `u32`

  1 if the adler32 value should be checked.

- **`dist`**: `u32`

  Last match distance.

- **`counter`**: `u32`

  Variable used for match length, symbols, and a number of other things.

- **`num_extra`**: `u8`

  Number of extra bits for the last length or distance code.

- **`table_sizes`**: `[u16; 3]`

  Number of entries in each huffman table.

- **`bit_buf`**: `u64`

  Buffer of input data.

- **`tables`**: `[HuffmanTable; 3]`

  Huffman tables.

- **`raw_header`**: `[u8; 4]`

  Raw block header.

- **`len_codes`**: `[u8; 512]`

  Huffman length codes.

#### Implementations

- `fn new() -> DecompressorOxide` — [`DecompressorOxide`](../../../inflate/core/index.md)

- `fn init(self: &mut Self)`

- `fn adler32(self: &Self) -> Option<u32>`

- `fn adler32_header(self: &Self) -> Option<u32>`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> DecompressorOxide` — [`DecompressorOxide`](../../../inflate/core/index.md)

##### `impl Default`

- `fn default() -> Self`

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

