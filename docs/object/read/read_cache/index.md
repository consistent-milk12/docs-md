*[object](../../index.md) / [read](../index.md) / [read_cache](index.md)*

---

# Module `read_cache`

## Structs

### `ReadCache<R: ReadCacheOps>`

```rust
struct ReadCache<R: ReadCacheOps> {
    cache: core::cell::RefCell<ReadCacheInternal<R>>,
}
```

An implementation of [`ReadRef`](../index.md) for data in a stream that implements
`Read + Seek`.

Contains a cache of read-only blocks of data, allowing references to
them to be returned. Entries in the cache are never removed.
Entries are keyed on the offset and size of the read.
Currently overlapping reads are considered separate reads.

This is primarily intended for environments where memory mapped files
are not available or not suitable, such as WebAssembly.

Note that malformed files can cause the cache to grow much larger than
the file size.

#### Implementations

- `fn new(read: R) -> Self`

- `fn range(self: &Self, offset: u64, size: u64) -> ReadCacheRange<'_, R>` — [`ReadCacheRange`](../index.md)

- `fn clear(self: &mut Self)`

- `fn into_inner(self: Self) -> R`

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + ReadCacheOps> Debug for ReadCache<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ReadCacheInternal<R: ReadCacheOps>`

```rust
struct ReadCacheInternal<R: ReadCacheOps> {
    read: R,
    bufs: alloc::collections::btree_map::BTreeMap<(u64, u64), alloc::boxed::Box<[u8]>>,
    strings: alloc::collections::btree_map::BTreeMap<(u64, u8), alloc::boxed::Box<[u8]>>,
    len: Option<u64>,
}
```

#### Implementations

- `fn range_in_bounds(self: &mut Self, range: &Range<u64>) -> Result<(), ()>`

- `fn len(self: &mut Self) -> Result<u64, ()>`

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + ReadCacheOps> Debug for ReadCacheInternal<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ReadCacheRange<'a, R: ReadCacheOps>`

```rust
struct ReadCacheRange<'a, R: ReadCacheOps> {
    r: &'a ReadCache<R>,
    offset: u64,
    size: u64,
}
```

An implementation of [`ReadRef`](../index.md) for a range of data in a stream that
implements `Read + Seek`.

Shares an underlying [`ReadCache`](../index.md) with a lifetime of `'a`.

#### Trait Implementations

##### `impl<'a, R: ReadCacheOps> Clone for ReadCacheRange<'a, R>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a, R: ReadCacheOps> Copy for ReadCacheRange<'a, R>`

##### `impl<'a, R: $crate::fmt::Debug + ReadCacheOps> Debug for ReadCacheRange<'a, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, R: ReadCacheOps> ReadRef for ReadCacheRange<'a, R>`

- `fn len(self: Self) -> Result<u64, ()>`

- `fn read_bytes_at(self: Self, offset: u64, size: u64) -> Result<&'a [u8], ()>`

- `fn read_bytes_at_until(self: Self, range: Range<u64>, delimiter: u8) -> Result<&'a [u8], ()>`

## Traits

### `ReadCacheOps`

```rust
trait ReadCacheOps { ... }
```

Operations required to implement [`ReadCache`](../index.md).

This is a subset of the `Read` and `Seek` traits.
A blanket implementation is provided for all types that implement
`Read + Seek`.

#### Required Methods

- `fn len(self: &mut Self) -> Result<u64, ()>`

  Return the length of the stream.

- `fn seek(self: &mut Self, pos: u64) -> Result<u64, ()>`

  Seek to the given position in the stream.

- `fn read(self: &mut Self, buf: &mut [u8]) -> Result<usize, ()>`

  Read up to `buf.len()` bytes into `buf`.

- `fn read_exact(self: &mut Self, buf: &mut [u8]) -> Result<(), ()>`

  Read exactly `buf.len()` bytes into `buf`.

