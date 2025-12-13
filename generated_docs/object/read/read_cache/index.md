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

*Defined in [`object-0.37.3/src/read/read_cache.rs:31-33`](../../../../.source_1765521767/object-0.37.3/src/read/read_cache.rs#L31-L33)*

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

  Create an empty `ReadCache` for the given stream.

- <span id="readcache-range"></span>`fn range(&self, offset: u64, size: u64) -> ReadCacheRange<'_, R>` — [`ReadCacheRange`](../index.md#readcacherange)

  Return an implementation of `ReadRef` that restricts reads

  to the given range of the stream.

- <span id="readcache-clear"></span>`fn clear(&mut self)`

  Free buffers used by the cache.

- <span id="readcache-into-inner"></span>`fn into_inner(self) -> R`

  Unwrap this `ReadCache<R>`, returning the underlying reader.

#### Trait Implementations

##### `impl Any for ReadCache<R>`

- <span id="readcache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReadCache<R>`

- <span id="readcache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReadCache<R>`

- <span id="readcache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadCacheOps> Debug for ReadCache<R>`

- <span id="readcache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReadCache<R>`

- <span id="readcache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReadCache<R>`

- <span id="readcache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadCacheOps> ReadRef for &'a ReadCache<R>`

- <span id="a-readcache-readref-len"></span>`fn len(self) -> Result<u64, ()>`

- <span id="a-readcache-readref-read-bytes-at"></span>`fn read_bytes_at(self, offset: u64, size: u64) -> Result<&'a [u8], ()>`

- <span id="a-readcache-readref-read-bytes-at-until"></span>`fn read_bytes_at_until(self, range: Range<u64>, delimiter: u8) -> Result<&'a [u8], ()>`

##### `impl<U> TryFrom for ReadCache<R>`

- <span id="readcache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readcache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReadCache<R>`

- <span id="readcache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readcache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReadCacheInternal<R: ReadCacheOps>`

```rust
struct ReadCacheInternal<R: ReadCacheOps> {
    read: R,
    bufs: alloc::collections::btree_map::BTreeMap<(u64, u64), alloc::boxed::Box<[u8]>>,
    strings: alloc::collections::btree_map::BTreeMap<(u64, u8), alloc::boxed::Box<[u8]>>,
    len: Option<u64>,
}
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:36-41`](../../../../.source_1765521767/object-0.37.3/src/read/read_cache.rs#L36-L41)*

#### Implementations

- <span id="readcacheinternal-range-in-bounds"></span>`fn range_in_bounds(&mut self, range: &Range<u64>) -> Result<(), ()>`

  Ensures this range is contained in the len of the file

- <span id="readcacheinternal-len"></span>`fn len(&mut self) -> Result<u64, ()>`

  The length of the underlying read, memoized

#### Trait Implementations

##### `impl Any for ReadCacheInternal<R>`

- <span id="readcacheinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReadCacheInternal<R>`

- <span id="readcacheinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReadCacheInternal<R>`

- <span id="readcacheinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadCacheOps> Debug for ReadCacheInternal<R>`

- <span id="readcacheinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReadCacheInternal<R>`

- <span id="readcacheinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReadCacheInternal<R>`

- <span id="readcacheinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ReadCacheInternal<R>`

- <span id="readcacheinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readcacheinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReadCacheInternal<R>`

- <span id="readcacheinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readcacheinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReadCacheRange<'a, R: ReadCacheOps>`

```rust
struct ReadCacheRange<'a, R: ReadCacheOps> {
    r: &'a ReadCache<R>,
    offset: u64,
    size: u64,
}
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:172-176`](../../../../.source_1765521767/object-0.37.3/src/read/read_cache.rs#L172-L176)*

An implementation of [`ReadRef`](../index.md) for a range of data in a stream that
implements `Read + Seek`.

Shares an underlying [`ReadCache`](../index.md) with a lifetime of `'a`.

#### Trait Implementations

##### `impl Any for ReadCacheRange<'a, R>`

- <span id="readcacherange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReadCacheRange<'a, R>`

- <span id="readcacherange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReadCacheRange<'a, R>`

- <span id="readcacherange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: ReadCacheOps> Clone for ReadCacheRange<'a, R>`

- <span id="readcacherange-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ReadCacheRange<'a, R>`

- <span id="readcacherange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: ReadCacheOps> Copy for ReadCacheRange<'a, R>`

##### `impl<R: fmt::Debug + ReadCacheOps> Debug for ReadCacheRange<'a, R>`

- <span id="readcacherange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReadCacheRange<'a, R>`

- <span id="readcacherange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReadCacheRange<'a, R>`

- <span id="readcacherange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadCacheOps> ReadRef for ReadCacheRange<'a, R>`

- <span id="readcacherange-readref-len"></span>`fn len(self) -> Result<u64, ()>`

- <span id="readcacherange-readref-read-bytes-at"></span>`fn read_bytes_at(self, offset: u64, size: u64) -> Result<&'a [u8], ()>`

- <span id="readcacherange-readref-read-bytes-at-until"></span>`fn read_bytes_at_until(self, range: Range<u64>, delimiter: u8) -> Result<&'a [u8], ()>`

##### `impl ToOwned for ReadCacheRange<'a, R>`

- <span id="readcacherange-toowned-type-owned"></span>`type Owned = T`

- <span id="readcacherange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="readcacherange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ReadCacheRange<'a, R>`

- <span id="readcacherange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readcacherange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReadCacheRange<'a, R>`

- <span id="readcacherange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readcacherange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ReadCacheOps`

```rust
trait ReadCacheOps { ... }
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:222-242`](../../../../.source_1765521767/object-0.37.3/src/read/read_cache.rs#L222-L242)*

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

