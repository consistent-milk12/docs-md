*[hashbrown](../index.md) / [raw](index.md)*

---

# Module `raw`

## Contents

- [Modules](#modules)
  - [`alloc`](#alloc)
- [Structs](#structs)
  - [`ProbeSeq`](#probeseq)
  - [`TableLayout`](#tablelayout)
  - [`Bucket`](#bucket)
  - [`RawTable`](#rawtable)
  - [`RawTableInner`](#rawtableinner)
  - [`RawIterRange`](#rawiterrange)
  - [`RawIter`](#rawiter)
  - [`FullBucketsIndices`](#fullbucketsindices)
  - [`RawIntoIter`](#rawintoiter)
  - [`RawDrain`](#rawdrain)
  - [`RawIterHash`](#rawiterhash)
  - [`RawIterHashIndices`](#rawiterhashindices)
  - [`RawExtractIf`](#rawextractif)
- [Enums](#enums)
  - [`Fallibility`](#fallibility)
- [Traits](#traits)
  - [`SizedTypeProperties`](#sizedtypeproperties)
  - [`RawTableClone`](#rawtableclone)
- [Functions](#functions)
  - [`offset_from`](#offset-from)
  - [`h1`](#h1)
  - [`capacity_to_buckets`](#capacity-to-buckets)
  - [`ensure_bucket_bytes_at_least_ctrl_align`](#ensure-bucket-bytes-at-least-ctrl-align)
  - [`bucket_mask_to_capacity`](#bucket-mask-to-capacity)
  - [`prev_pow2`](#prev-pow2)
  - [`maximum_buckets_in`](#maximum-buckets-in)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`alloc`](#alloc) | mod |  |
| [`ProbeSeq`](#probeseq) | struct | Probe sequence based on triangular numbers, which is guaranteed (since our table size is a power of two) to visit every group of elements exactly once. |
| [`TableLayout`](#tablelayout) | struct | Helper which allows the max calculation for `ctrl_align` to be statically computed for each `T` while keeping the rest of `calculate_layout_for` independent of `T` |
| [`Bucket`](#bucket) | struct | A reference to a hash table bucket containing a `T`. |
| [`RawTable`](#rawtable) | struct | A raw hash table with an unsafe API. |
| [`RawTableInner`](#rawtableinner) | struct | Non-generic part of `RawTable` which allows functions to be instantiated only once regardless of how many different key-value types are used. |
| [`RawIterRange`](#rawiterrange) | struct | Iterator over a sub-range of a table. |
| [`RawIter`](#rawiter) | struct | Iterator which returns a raw pointer to every full bucket in the table. |
| [`FullBucketsIndices`](#fullbucketsindices) | struct | Iterator which returns an index of every full bucket in the table. |
| [`RawIntoIter`](#rawintoiter) | struct | Iterator which consumes a table and returns elements. |
| [`RawDrain`](#rawdrain) | struct | Iterator which consumes elements without freeing the table storage. |
| [`RawIterHash`](#rawiterhash) | struct | Iterator over occupied buckets that could match a given hash. |
| [`RawIterHashIndices`](#rawiterhashindices) | struct |  |
| [`RawExtractIf`](#rawextractif) | struct |  |
| [`Fallibility`](#fallibility) | enum | Whether memory allocation errors should return an error or abort. |
| [`SizedTypeProperties`](#sizedtypeproperties) | trait |  |
| [`RawTableClone`](#rawtableclone) | trait | Specialization of `clone_from` for `Copy` types |
| [`offset_from`](#offset-from) | fn |  |
| [`h1`](#h1) | fn | Primary hash function, used to select the initial bucket to probe from. |
| [`capacity_to_buckets`](#capacity-to-buckets) | fn | Returns the number of buckets needed to hold the given number of items, taking the maximum load factor into account. |
| [`ensure_bucket_bytes_at_least_ctrl_align`](#ensure-bucket-bytes-at-least-ctrl-align) | fn |  |
| [`bucket_mask_to_capacity`](#bucket-mask-to-capacity) | fn | Returns the maximum effective capacity for the given bucket mask, taking the maximum load factor into account. |
| [`prev_pow2`](#prev-pow2) | fn | Find the previous power of 2. |
| [`maximum_buckets_in`](#maximum-buckets-in) | fn | Finds the largest number of buckets that can fit in `allocation_size` provided the given TableLayout. |

## Modules

- [`alloc`](alloc/index.md)

## Structs

### `ProbeSeq`

```rust
struct ProbeSeq {
    pos: usize,
    stride: usize,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:76-79`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L76-L79)*

Probe sequence based on triangular numbers, which is guaranteed (since our
table size is a power of two) to visit every group of elements exactly once.

A triangular probe has us jump by 1 more group every time. So first we
jump by 1 group (meaning we just continue our linear scan), then 2 groups
(skipping over 1 group), then 3 groups (skipping over 2 groups), and so on.

Proof that the probe will visit every group in the table:
<https://fgiesen.wordpress.com/2015/02/22/triangular-numbers-mod-2n/>

#### Implementations

- <span id="probeseq-move-next"></span>`fn move_next(&mut self, bucket_mask: usize)`

#### Trait Implementations

##### `impl Clone for ProbeSeq`

- <span id="probeseq-clone"></span>`fn clone(&self) -> ProbeSeq` — [`ProbeSeq`](#probeseq)

### `TableLayout`

```rust
struct TableLayout {
    size: usize,
    ctrl_align: usize,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:198-201`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L198-L201)*

Helper which allows the max calculation for `ctrl_align` to be statically computed for each `T`
while keeping the rest of `calculate_layout_for` independent of `T`

#### Implementations

- <span id="tablelayout-new"></span>`const fn new<T>() -> Self`

- <span id="tablelayout-calculate-layout-for"></span>`fn calculate_layout_for(self, buckets: usize) -> Option<(Layout, usize)>`

#### Trait Implementations

##### `impl Clone for TableLayout`

- <span id="tablelayout-clone"></span>`fn clone(&self) -> TableLayout` — [`TableLayout`](#tablelayout)

##### `impl Copy for TableLayout`

### `Bucket<T>`

```rust
struct Bucket<T> {
    ptr: core::ptr::NonNull<T>,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:245-251`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L245-L251)*

A reference to a hash table bucket containing a `T`.

This is usually just a pointer to the element itself. However if the element
is a ZST, then we instead track the index of the element in the table so
that `erase` works properly.

#### Implementations

- <span id="bucket-from-base-index"></span>`unsafe fn from_base_index(base: NonNull<T>, index: usize) -> Self`

- <span id="bucket-to-base-index"></span>`unsafe fn to_base_index(&self, base: NonNull<T>) -> usize`

- <span id="bucket-as-ptr"></span>`fn as_ptr(&self) -> *mut T`

- <span id="bucket-as-non-null"></span>`fn as_non_null(&self) -> NonNull<T>`

- <span id="bucket-next-n"></span>`unsafe fn next_n(&self, offset: usize) -> Self`

- <span id="bucket-drop"></span>`unsafe fn drop(&self)`

- <span id="bucket-read"></span>`unsafe fn read(&self) -> T`

- <span id="bucket-write"></span>`unsafe fn write(&self, val: T)`

- <span id="bucket-as-ref"></span>`unsafe fn as_ref<'a>(&self) -> &'a T`

- <span id="bucket-as-mut"></span>`unsafe fn as_mut<'a>(&self) -> &'a mut T`

#### Trait Implementations

##### `impl<T> Clone for Bucket<T>`

- <span id="bucket-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Send for Bucket<T>`

### `RawTable<T, A: Allocator>`

```rust
struct RawTable<T, A: Allocator> {
    table: RawTableInner,
    alloc: A,
    marker: core::marker::PhantomData<T>,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:600-605`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L600-L605)*

A raw hash table with an unsafe API.

#### Implementations

- <span id="rawtable-new"></span>`const fn new() -> Self`

- <span id="rawtable-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

#### Trait Implementations

##### `impl<T: Clone, A: Allocator + Clone> Clone for RawTable<T, A>`

- <span id="rawtable-clone"></span>`fn clone(&self) -> Self`

- <span id="rawtable-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<T, A: Allocator + Default> Default for RawTable<T, A>`

- <span id="rawtable-default"></span>`fn default() -> Self`

##### `impl<T, A: Allocator> Drop for RawTable<T, A>`

- <span id="rawtable-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> IntoIterator for RawTable<T, A>`

- <span id="rawtable-intoiterator-type-item"></span>`type Item = T`

- <span id="rawtable-intoiterator-type-intoiter"></span>`type IntoIter = RawIntoIter<T, A>`

- <span id="rawtable-into-iter"></span>`fn into_iter(self) -> RawIntoIter<T, A>` — [`RawIntoIter`](#rawintoiter)

##### `impl<T: Clone, A: Allocator + Clone> RawTableClone for RawTable<T, A>`

- <span id="rawtable-clone-from-spec"></span>`unsafe fn clone_from_spec(&mut self, source: &Self)`

##### `impl<T, A> Send for RawTable<T, A>`

##### `impl<T, A> Sync for RawTable<T, A>`

### `RawTableInner`

```rust
struct RawTableInner {
    bucket_mask: usize,
    ctrl: core::ptr::NonNull<u8>,
    growth_left: usize,
    items: usize,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:609-623`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L609-L623)*

Non-generic part of `RawTable` which allows functions to be instantiated only once regardless
of how many different key-value types are used.

#### Implementations

- <span id="rawtableinner-const-new"></span>`const NEW: Self`

- <span id="rawtableinner-new"></span>`const fn new() -> Self`

### `RawIterRange<T>`

```rust
struct RawIterRange<T> {
    current_group: self::bitmask::BitMaskIter,
    data: Bucket<T>,
    next_ctrl: *const u8,
    end: *const u8,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:3540-3554`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L3540-L3554)*

Iterator over a sub-range of a table. Unlike `RawIter` this iterator does
not track an item count.

#### Implementations

- <span id="rawiterrange-new"></span>`unsafe fn new(ctrl: *const u8, data: Bucket<T>, len: usize) -> Self` — [`Bucket`](#bucket)

- <span id="rawiterrange-next-impl"></span>`unsafe fn next_impl<const DO_CHECK_PTR_RANGE: bool>(&mut self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

- <span id="rawiterrange-fold-impl"></span>`unsafe fn fold_impl<F, B>(self, n: usize, acc: B, f: F) -> B`

#### Trait Implementations

##### `impl<T> Clone for RawIterRange<T>`

- <span id="rawiterrange-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> FusedIterator for RawIterRange<T>`

##### `impl IntoIterator for RawIterRange<T>`

- <span id="rawiterrange-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawiterrange-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawiterrange-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for RawIterRange<T>`

- <span id="rawiterrange-iterator-type-item"></span>`type Item = Bucket<T>`

- <span id="rawiterrange-next"></span>`fn next(&mut self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

- <span id="rawiterrange-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Send for RawIterRange<T>`

##### `impl<T> Sync for RawIterRange<T>`

### `RawIter<T>`

```rust
struct RawIter<T> {
    iter: RawIterRange<T>,
    items: usize,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:3812-3815`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L3812-L3815)*

Iterator which returns a raw pointer to every full bucket in the table.

For maximum flexibility this iterator is not bound by a lifetime, but you
must observe several rules when using it:
- You must not free the hash table while iterating (including via growing/shrinking).
- It is fine to erase a bucket that has been yielded by the iterator.
- Erasing a bucket that has not yet been yielded by the iterator may still
  result in the iterator yielding that bucket (unless `reflect_remove` is called).
- It is unspecified whether an element inserted after the iterator was
  created will be yielded by that iterator (unless `reflect_insert` is called).
- The order in which the iterator yields bucket is unspecified and may
  change in the future.

#### Implementations

- <span id="rawiter-drop-elements"></span>`unsafe fn drop_elements(&mut self)`

#### Trait Implementations

##### `impl<T> Clone for RawIter<T>`

- <span id="rawiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Default for RawIter<T>`

- <span id="rawiter-default"></span>`fn default() -> Self`

##### `impl<T> ExactSizeIterator for RawIter<T>`

##### `impl<T> FusedIterator for RawIter<T>`

##### `impl IntoIterator for RawIter<T>`

- <span id="rawiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for RawIter<T>`

- <span id="rawiter-iterator-type-item"></span>`type Item = Bucket<T>`

- <span id="rawiter-next"></span>`fn next(&mut self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

- <span id="rawiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="rawiter-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `FullBucketsIndices`

```rust
struct FullBucketsIndices {
    current_group: self::bitmask::BitMaskIter,
    group_first_index: usize,
    ctrl: core::ptr::NonNull<u8>,
    items: usize,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:3897-3912`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L3897-L3912)*

Iterator which returns an index of every full bucket in the table.

For maximum flexibility this iterator is not bound by a lifetime, but you
must observe several rules when using it:
- You must not free the hash table while iterating (including via growing/shrinking).
- It is fine to erase a bucket that has been yielded by the iterator.
- Erasing a bucket that has not yet been yielded by the iterator may still
  result in the iterator yielding index of that bucket.
- It is unspecified whether an element inserted after the iterator was
  created will be yielded by that iterator.
- The order in which the iterator yields indices of the buckets is unspecified
  and may change in the future.

#### Implementations

- <span id="fullbucketsindices-next-impl"></span>`unsafe fn next_impl(&mut self) -> Option<usize>`

#### Trait Implementations

##### `impl Clone for FullBucketsIndices`

- <span id="fullbucketsindices-clone"></span>`fn clone(&self) -> FullBucketsIndices` — [`FullBucketsIndices`](#fullbucketsindices)

##### `impl Default for FullBucketsIndices`

- <span id="fullbucketsindices-default"></span>`fn default() -> Self`

##### `impl ExactSizeIterator for FullBucketsIndices`

##### `impl FusedIterator for FullBucketsIndices`

##### `impl IntoIterator for FullBucketsIndices`

- <span id="fullbucketsindices-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="fullbucketsindices-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="fullbucketsindices-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FullBucketsIndices`

- <span id="fullbucketsindices-iterator-type-item"></span>`type Item = usize`

- <span id="fullbucketsindices-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="fullbucketsindices-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `RawIntoIter<T, A: Allocator>`

```rust
struct RawIntoIter<T, A: Allocator> {
    iter: RawIter<T>,
    allocation: Option<(core::ptr::NonNull<u8>, crate::alloc::alloc::Layout, A)>,
    marker: core::marker::PhantomData<T>,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:4013-4017`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L4013-L4017)*

Iterator which consumes a table and returns elements.

#### Implementations

- <span id="rawintoiter-iter"></span>`fn iter(&self) -> RawIter<T>` — [`RawIter`](#rawiter)

#### Trait Implementations

##### `impl<T, A: Allocator> Default for RawIntoIter<T, A>`

- <span id="rawintoiter-default"></span>`fn default() -> Self`

##### `impl<T, A: Allocator> Drop for RawIntoIter<T, A>`

- <span id="rawintoiter-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for RawIntoIter<T, A>`

##### `impl<T, A: Allocator> FusedIterator for RawIntoIter<T, A>`

##### `impl IntoIterator for RawIntoIter<T, A>`

- <span id="rawintoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawintoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawintoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for RawIntoIter<T, A>`

- <span id="rawintoiter-iterator-type-item"></span>`type Item = T`

- <span id="rawintoiter-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="rawintoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T, A> Send for RawIntoIter<T, A>`

##### `impl<T, A> Sync for RawIntoIter<T, A>`

### `RawDrain<'a, T, A: Allocator>`

```rust
struct RawDrain<'a, T, A: Allocator> {
    iter: RawIter<T>,
    table: RawTableInner,
    orig_table: core::ptr::NonNull<RawTableInner>,
    marker: core::marker::PhantomData<&'a RawTable<T, A>>,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:4097-4109`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L4097-L4109)*

Iterator which consumes elements without freeing the table storage.

#### Implementations

- <span id="rawdrain-iter"></span>`fn iter(&self) -> RawIter<T>` — [`RawIter`](#rawiter)

#### Trait Implementations

##### `impl<T, A: Allocator> Drop for RawDrain<'_, T, A>`

- <span id="rawdrain-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for RawDrain<'_, T, A>`

##### `impl<T, A: Allocator> FusedIterator for RawDrain<'_, T, A>`

##### `impl IntoIterator for RawDrain<'a, T, A>`

- <span id="rawdrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawdrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawdrain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for RawDrain<'_, T, A>`

- <span id="rawdrain-iterator-type-item"></span>`type Item = T`

- <span id="rawdrain-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="rawdrain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T, A> Send for RawDrain<'_, T, A>`

##### `impl<T, A> Sync for RawDrain<'_, T, A>`

### `RawIterHash<T>`

```rust
struct RawIterHash<T> {
    inner: RawIterHashIndices,
    _marker: core::marker::PhantomData<T>,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:4186-4189`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L4186-L4189)*

Iterator over occupied buckets that could match a given hash.

`RawTable` only stores 7 bits of the hash value, so this iterator may return
items that have a hash value different than the one provided. You should
always validate the returned values before using them.

For maximum flexibility this iterator is not bound by a lifetime, but you
must observe several rules when using it:
- You must not free the hash table while iterating (including via growing/shrinking).
- It is fine to erase a bucket that has been yielded by the iterator.
- Erasing a bucket that has not yet been yielded by the iterator may still
  result in the iterator yielding that bucket.
- It is unspecified whether an element inserted after the iterator was
  created will be yielded by that iterator.
- The order in which the iterator yields buckets is unspecified and may
  change in the future.

#### Implementations

- <span id="rawiterhash-new"></span>`unsafe fn new<A: Allocator>(table: &RawTable<T, A>, hash: u64) -> Self` — [`RawTable`](#rawtable)

#### Trait Implementations

##### `impl<T> Clone for RawIterHash<T>`

- <span id="rawiterhash-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Default for RawIterHash<T>`

- <span id="rawiterhash-default"></span>`fn default() -> Self`

##### `impl IntoIterator for RawIterHash<T>`

- <span id="rawiterhash-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawiterhash-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawiterhash-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for RawIterHash<T>`

- <span id="rawiterhash-iterator-type-item"></span>`type Item = Bucket<T>`

- <span id="rawiterhash-next"></span>`fn next(&mut self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

### `RawIterHashIndices`

```rust
struct RawIterHashIndices {
    bucket_mask: usize,
    ctrl: core::ptr::NonNull<u8>,
    tag_hash: self::tag::Tag,
    probe_seq: ProbeSeq,
    group: self::imp::Group,
    bitmask: self::bitmask::BitMaskIter,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:4192-4209`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L4192-L4209)*

#### Implementations

- <span id="rawiterhashindices-new"></span>`unsafe fn new(table: &RawTableInner, hash: u64) -> Self` — [`RawTableInner`](#rawtableinner)

#### Trait Implementations

##### `impl Clone for RawIterHashIndices`

- <span id="rawiterhashindices-clone"></span>`fn clone(&self) -> RawIterHashIndices` — [`RawIterHashIndices`](#rawiterhashindices)

##### `impl Default for RawIterHashIndices`

- <span id="rawiterhashindices-default"></span>`fn default() -> Self`

##### `impl IntoIterator for RawIterHashIndices`

- <span id="rawiterhashindices-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawiterhashindices-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawiterhashindices-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RawIterHashIndices`

- <span id="rawiterhashindices-iterator-type-item"></span>`type Item = usize`

- <span id="rawiterhashindices-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `RawExtractIf<'a, T, A: Allocator>`

```rust
struct RawExtractIf<'a, T, A: Allocator> {
    pub iter: RawIter<T>,
    pub table: &'a mut RawTable<T, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:4315-4318`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L4315-L4318)*

#### Implementations

- <span id="rawextractif-next"></span>`fn next<F>(&mut self, f: F) -> Option<T>`

## Enums

### `Fallibility`

```rust
enum Fallibility {
    Fallible,
    Infallible,
}
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:26-29`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L26-L29)*

Whether memory allocation errors should return an error or abort.

#### Implementations

- <span id="fallibility-capacity-overflow"></span>`fn capacity_overflow(self) -> TryReserveError` — [`TryReserveError`](../index.md#tryreserveerror)

- <span id="fallibility-alloc-err"></span>`fn alloc_err(self, layout: Layout) -> TryReserveError` — [`TryReserveError`](../index.md#tryreserveerror)

#### Trait Implementations

##### `impl Clone for Fallibility`

- <span id="fallibility-clone"></span>`fn clone(&self) -> Fallibility` — [`Fallibility`](#fallibility)

##### `impl Copy for Fallibility`

## Traits

### `SizedTypeProperties`

```rust
trait SizedTypeProperties: Sized { ... }
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:51-54`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L51-L54)*

#### Associated Constants

- `const IS_ZERO_SIZED: bool`

- `const NEEDS_DROP: bool`

#### Implementors

- `T`

### `RawTableClone`

```rust
trait RawTableClone { ... }
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:3411-3413`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L3411-L3413)*

Specialization of `clone_from` for `Copy` types

#### Required Methods

- `fn clone_from_spec(&mut self, source: &Self)`

#### Implementors

- [`RawTable`](#rawtable)

## Functions

### `offset_from`

```rust
unsafe fn offset_from<T>(to: *const T, from: *const T) -> usize
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:20-22`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L20-L22)*

### `h1`

```rust
fn h1(hash: u64) -> usize
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:61-64`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L61-L64)*

Primary hash function, used to select the initial bucket to probe from.

### `capacity_to_buckets`

```rust
fn capacity_to_buckets(cap: usize, table_layout: TableLayout) -> Option<usize>
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:105-166`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L105-L166)*

Returns the number of buckets needed to hold the given number of items,
taking the maximum load factor into account.

Returns `None` if an overflow occurs.

This ensures that `buckets * table_layout.size >= table_layout.ctrl_align`.

### `ensure_bucket_bytes_at_least_ctrl_align`

```rust
fn ensure_bucket_bytes_at_least_ctrl_align(table_layout: TableLayout, buckets: usize)
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:174-179`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L174-L179)*

### `bucket_mask_to_capacity`

```rust
fn bucket_mask_to_capacity(bucket_mask: usize) -> usize
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:184-193`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L184-L193)*

Returns the maximum effective capacity for the given bucket mask, taking
the maximum load factor into account.

### `prev_pow2`

```rust
fn prev_pow2(z: usize) -> usize
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:1545-1548`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L1545-L1548)*

Find the previous power of 2. If it's already a power of 2, it's unchanged.
Passing zero is undefined behavior.

### `maximum_buckets_in`

```rust
fn maximum_buckets_in(allocation_size: usize, table_layout: TableLayout, group_width: usize) -> usize
```

*Defined in [`hashbrown-0.16.1/src/raw/mod.rs:1555-1580`](../../../.source_1765210505/hashbrown-0.16.1/src/raw/mod.rs#L1555-L1580)*

Finds the largest number of buckets that can fit in `allocation_size`
provided the given TableLayout.

This relies on some invariants of `capacity_to_buckets`, so only feed in
an `allocation_size` calculated from `capacity_to_buckets`.

