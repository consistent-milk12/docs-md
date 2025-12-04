*[miniz_oxide](../../../index.md) / [deflate](../../index.md) / [core](../index.md) / [deflate_flags](index.md)*

---

# Module `deflate_flags`

## Constants

### `TDEFL_WRITE_ZLIB_HEADER`

```rust
const TDEFL_WRITE_ZLIB_HEADER: u32 = 4_096u32;
```

Whether to use a zlib wrapper.

### `TDEFL_COMPUTE_ADLER32`

```rust
const TDEFL_COMPUTE_ADLER32: u32 = 8_192u32;
```

Should we compute the adler32 checksum.

### `TDEFL_GREEDY_PARSING_FLAG`

```rust
const TDEFL_GREEDY_PARSING_FLAG: u32 = 16_384u32;
```

Should we use greedy parsing (as opposed to lazy parsing where look ahead one or more
bytes to check for better matches.)

### `TDEFL_NONDETERMINISTIC_PARSING_FLAG`

```rust
const TDEFL_NONDETERMINISTIC_PARSING_FLAG: u32 = 32_768u32;
```

Used in miniz to skip zero-initializing hash and dict. We don't do this here, so
this flag is ignored.

### `TDEFL_RLE_MATCHES`

```rust
const TDEFL_RLE_MATCHES: u32 = 65_536u32;
```

Only look for matches with a distance of 0.

### `TDEFL_FILTER_MATCHES`

```rust
const TDEFL_FILTER_MATCHES: u32 = 131_072u32;
```

Only use matches that are at least 6 bytes long.

### `TDEFL_FORCE_ALL_STATIC_BLOCKS`

```rust
const TDEFL_FORCE_ALL_STATIC_BLOCKS: u32 = 262_144u32;
```

Force the compressor to only output static blocks. (Blocks using the default huffman codes
specified in the deflate specification.)

### `TDEFL_FORCE_ALL_RAW_BLOCKS`

```rust
const TDEFL_FORCE_ALL_RAW_BLOCKS: u32 = 524_288u32;
```

Force the compressor to only output raw/uncompressed blocks.

