*[object](../../index.md) / [read](../index.md) / [read_cache](index.md)*

---

# Module `read_cache`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReadCache`](#readcache) | struct | An implementation of [`ReadRef`] for data in a stream that implements `Read + Seek`. |
| [`ReadCacheInternal`](#readcacheinternal) | struct |  |
| [`ReadCacheRange`](#readcacherange) | struct | An implementation of [`ReadRef`] for a range of data in a stream that implements `Read + Seek`. |
| [`ReadCacheOps`](#readcacheops) | trait | Operations required to implement [`ReadCache`]. |

## Structs

### `ReadCache<R: ReadCacheOps>`

```rust
struct ReadCache<R: ReadCacheOps> {
    cache: core::cell::RefCell<ReadCacheInternal<R>>,
}
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:31-33`](../../../../.source_1765210505/object-0.37.3/src/read/read_cache.rs#L31-L33)*

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

- <span id="readcache-new"></span>`fn new(read: R) -> Self`

- <span id="readcache-range"></span>`fn range(&self, offset: u64, size: u64) -> ReadCacheRange<'_, R>` — [`ReadCacheRange`](../index.md)

- <span id="readcache-clear"></span>`fn clear(&mut self)`

- <span id="readcache-into-inner"></span>`fn into_inner(self) -> R`

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadCacheOps> Debug for ReadCache<R>`

- <span id="readcache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, R: ReadCacheOps> ReadRef for &'a ReadCache<R>`

- <span id="a-readcache-len"></span>`fn len(self) -> Result<u64, ()>`

- <span id="a-readcache-read-bytes-at"></span>`fn read_bytes_at(self, offset: u64, size: u64) -> Result<&'a [u8], ()>`

- <span id="a-readcache-read-bytes-at-until"></span>`fn read_bytes_at_until(self, range: Range<u64>, delimiter: u8) -> Result<&'a [u8], ()>`

### `ReadCacheInternal<R: ReadCacheOps>`

```rust
struct ReadCacheInternal<R: ReadCacheOps> {
    read: R,
    bufs: alloc::collections::btree_map::BTreeMap<(u64, u64), alloc::boxed::Box<[u8]>>,
    strings: alloc::collections::btree_map::BTreeMap<(u64, u8), alloc::boxed::Box<[u8]>>,
    len: Option<u64>,
}
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:36-41`](../../../../.source_1765210505/object-0.37.3/src/read/read_cache.rs#L36-L41)*

#### Implementations

- <span id="readcacheinternal-range-in-bounds"></span>`fn range_in_bounds(&mut self, range: &Range<u64>) -> Result<(), ()>`

- <span id="readcacheinternal-len"></span>`fn len(&mut self) -> Result<u64, ()>`

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadCacheOps> Debug for ReadCacheInternal<R>`

- <span id="readcacheinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReadCacheRange<'a, R: ReadCacheOps>`

```rust
struct ReadCacheRange<'a, R: ReadCacheOps> {
    r: &'a ReadCache<R>,
    offset: u64,
    size: u64,
}
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:172-176`](../../../../.source_1765210505/object-0.37.3/src/read/read_cache.rs#L172-L176)*

An implementation of [`ReadRef`](../index.md) for a range of data in a stream that
implements `Read + Seek`.

Shares an underlying [`ReadCache`](../index.md) with a lifetime of `'a`.

#### Trait Implementations

##### `impl<'a, R: ReadCacheOps> Clone for ReadCacheRange<'a, R>`

- <span id="readcacherange-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a, R: ReadCacheOps> Copy for ReadCacheRange<'a, R>`

##### `impl<'a, R: fmt::Debug + ReadCacheOps> Debug for ReadCacheRange<'a, R>`

- <span id="readcacherange-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, R: ReadCacheOps> ReadRef for ReadCacheRange<'a, R>`

- <span id="readcacherange-len"></span>`fn len(self) -> Result<u64, ()>`

- <span id="readcacherange-read-bytes-at"></span>`fn read_bytes_at(self, offset: u64, size: u64) -> Result<&'a [u8], ()>`

- <span id="readcacherange-read-bytes-at-until"></span>`fn read_bytes_at_until(self, range: Range<u64>, delimiter: u8) -> Result<&'a [u8], ()>`

## Traits

### `ReadCacheOps`

```rust
trait ReadCacheOps { ... }
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:222-242`](../../../../.source_1765210505/object-0.37.3/src/read/read_cache.rs#L222-L242)*

Operations required to implement [`ReadCache`](../index.md).

This is a subset of the `Read` and `Seek` traits.
A blanket implementation is provided for all types that implement
`Read + Seek`.

#### Required Methods

- `fn len(&mut self) -> Result<u64, ()>`

  Return the length of the stream.

- `fn seek(&mut self, pos: u64) -> Result<u64, ()>`

  Seek to the given position in the stream.

- `fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()>`

  Read up to `buf.len()` bytes into `buf`.

- `fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ()>`

  Read exactly `buf.len()` bytes into `buf`.

