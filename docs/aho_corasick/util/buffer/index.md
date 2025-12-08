*[aho_corasick](../../index.md) / [util](../index.md) / [buffer](index.md)*

---

# Module `buffer`

## Structs

### `Buffer`

```rust
struct Buffer {
    buf: alloc::vec::Vec<u8>,
    min: usize,
    end: usize,
}
```

A fairly simple roll buffer for supporting stream searches.

This buffer acts as a temporary place to store a fixed amount of data when
reading from a stream. Its central purpose is to allow "rolling" some
suffix of the data to the beginning of the buffer before refilling it with
more data from the stream. For example, let's say we are trying to match
"foobar" on a stream. When we report the match, we'd like to not only
report the correct offsets at which the match occurs, but also the matching
bytes themselves. So let's say our stream is a file with the following
contents: `test test foobar test test`. Now assume that we happen to read
the aforementioned file in two chunks: `test test foo` and `bar test test`.
Naively, it would not be possible to report a single contiguous `foobar`
match, but this roll buffer allows us to do that. Namely, after the second
read, the contents of the buffer should be `st foobar test test`, where the
search should ultimately resume immediately after `foo`. (The prefix `st `
is included because the roll buffer saves N bytes at the end of the buffer,
where N is the maximum possible length of a match.)

A lot of the logic for dealing with this is unfortunately split out between
this roll buffer and the `StreamChunkIter`.

Note also that this buffer is not actually required to just report matches.
Because a `Match` is just some offsets. But it *is* required for supporting
things like `try_stream_replace_all` because that needs some mechanism for
knowing which bytes in the stream correspond to a match and which don't. So
when a match occurs across two `read` calls, *something* needs to retain
the bytes from the previous `read` call because you don't know before the
second read call whether a match exists or not.

#### Fields

- **`buf`**: `alloc::vec::Vec<u8>`

  The raw buffer contents. This has a fixed size and never increases.

- **`min`**: `usize`

  The minimum size of the buffer, which is equivalent to the maximum
  possible length of a match. This corresponds to the amount that we
  roll

- **`end`**: `usize`

  The end of the contents of this buffer.

#### Implementations

- `fn new(min_buffer_len: usize) -> Buffer` — [`Buffer`](#buffer)

- `fn buffer(self: &Self) -> &[u8]`

- `fn min_buffer_len(self: &Self) -> usize`

- `fn free_buffer(self: &mut Self) -> &mut [u8]`

- `fn fill<R: std::io::Read>(self: &mut Self, rdr: R) -> std::io::Result<bool>`

- `fn roll(self: &mut Self)`

#### Trait Implementations

##### `impl Debug for Buffer`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Constants

### `DEFAULT_BUFFER_CAPACITY`

```rust
const DEFAULT_BUFFER_CAPACITY: usize = 65_536usize;
```

The default buffer capacity that we use for the stream buffer.

