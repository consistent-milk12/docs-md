*[hashbrown](../index.md) / [raw](index.md)*

---

# Module `raw`

## Modules

- [`alloc`](alloc/index.md) - 

## Structs

### `ProbeSeq`

```rust
struct ProbeSeq {
    pos: usize,
    stride: usize,
}
```

Probe sequence based on triangular numbers, which is guaranteed (since our
table size is a power of two) to visit every group of elements exactly once.

A triangular probe has us jump by 1 more group every time. So first we
jump by 1 group (meaning we just continue our linear scan), then 2 groups
(skipping over 1 group), then 3 groups (skipping over 2 groups), and so on.

Proof that the probe will visit every group in the table:
<https://fgiesen.wordpress.com/2015/02/22/triangular-numbers-mod-2n/>

#### Implementations

- `fn move_next(self: &mut Self, bucket_mask: usize)`

#### Trait Implementations

##### `impl Clone for ProbeSeq`

- `fn clone(self: &Self) -> ProbeSeq` — [`ProbeSeq`](#probeseq)

### `TableLayout`

```rust
struct TableLayout {
    size: usize,
    ctrl_align: usize,
}
```

Helper which allows the max calculation for `ctrl_align` to be statically computed for each `T`
while keeping the rest of `calculate_layout_for` independent of `T`

#### Implementations

- `const fn new<T>() -> Self`

- `fn calculate_layout_for(self: Self, buckets: usize) -> Option<(Layout, usize)>`

#### Trait Implementations

##### `impl Clone for TableLayout`

- `fn clone(self: &Self) -> TableLayout` — [`TableLayout`](#tablelayout)

##### `impl Copy for TableLayout`

### `Bucket<T>`

```rust
struct Bucket<T> {
    ptr: core::ptr::NonNull<T>,
}
```

A reference to a hash table bucket containing a `T`.

This is usually just a pointer to the element itself. However if the element
is a ZST, then we instead track the index of the element in the table so
that `erase` works properly.

#### Implementations

- `unsafe fn from_base_index(base: NonNull<T>, index: usize) -> Self`

- `unsafe fn to_base_index(self: &Self, base: NonNull<T>) -> usize`

- `fn as_ptr(self: &Self) -> *mut T`

- `fn as_non_null(self: &Self) -> NonNull<T>`

- `unsafe fn next_n(self: &Self, offset: usize) -> Self`

- `unsafe fn drop(self: &Self)`

- `unsafe fn read(self: &Self) -> T`

- `unsafe fn write(self: &Self, val: T)`

- `unsafe fn as_ref<'a>(self: &Self) -> &'a T`

- `unsafe fn as_mut<'a>(self: &Self) -> &'a mut T`

#### Trait Implementations

##### `impl<T> Clone for Bucket<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T> Send for Bucket<T>`

### `RawTable<T, A: Allocator>`

```rust
struct RawTable<T, A: Allocator> {
    table: RawTableInner,
    alloc: A,
    marker: core::marker::PhantomData<T>,
}
```

A raw hash table with an unsafe API.

#### Implementations

- `const fn new() -> Self`

- `fn with_capacity(capacity: usize) -> Self`

#### Trait Implementations

##### `impl<T: Clone, A: Allocator + Clone> Clone for RawTable<T, A>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl<T, A: Allocator + Default> Default for RawTable<T, A>`

- `fn default() -> Self`

##### `impl<T, A: Allocator> Drop for RawTable<T, A>`

- `fn drop(self: &mut Self)`

##### `impl<T, A: Allocator> IntoIterator for RawTable<T, A>`

- `type Item = T`

- `type IntoIter = RawIntoIter<T, A>`

- `fn into_iter(self: Self) -> RawIntoIter<T, A>` — [`RawIntoIter`](#rawintoiter)

##### `impl<T: Clone, A: Allocator + Clone> RawTableClone for RawTable<T, A>`

- `unsafe fn clone_from_spec(self: &mut Self, source: &Self)`

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

Non-generic part of `RawTable` which allows functions to be instantiated only once regardless
of how many different key-value types are used.

#### Implementations

- `unsafe fn new_uninitialized<A>(alloc: &A, table_layout: TableLayout, buckets: usize, fallibility: Fallibility) -> Result<Self, TryReserveError>` — [`TableLayout`](#tablelayout), [`Fallibility`](#fallibility), [`TryReserveError`](../index.md)

- `fn fallible_with_capacity<A>(alloc: &A, table_layout: TableLayout, capacity: usize, fallibility: Fallibility) -> Result<Self, TryReserveError>` — [`TableLayout`](#tablelayout), [`Fallibility`](#fallibility), [`TryReserveError`](../index.md)

- `fn with_capacity<A>(alloc: &A, table_layout: TableLayout, capacity: usize) -> Self` — [`TableLayout`](#tablelayout)

- `unsafe fn fix_insert_index(self: &Self, index: usize) -> usize`

- `fn find_insert_index_in_group(self: &Self, group: &Group, probe_seq: &ProbeSeq) -> Option<usize>` — [`Group`](../control/group/sse2/index.md), [`ProbeSeq`](#probeseq)

- `unsafe fn find_or_find_insert_index_inner(self: &Self, hash: u64, eq: &mut dyn FnMut(usize) -> bool) -> Result<usize, usize>`

- `unsafe fn prepare_insert_index(self: &mut Self, hash: u64) -> (usize, Tag)` — [`Tag`](../control/tag/index.md)

- `unsafe fn find_insert_index(self: &Self, hash: u64) -> usize`

- `unsafe fn find_inner(self: &Self, hash: u64, eq: &mut dyn FnMut(usize) -> bool) -> Option<usize>`

- `unsafe fn prepare_rehash_in_place(self: &mut Self)`

- `unsafe fn iter<T>(self: &Self) -> RawIter<T>` — [`RawIter`](#rawiter)

- `unsafe fn drop_elements<T>(self: &mut Self)`

- `unsafe fn drop_inner_table<T, A: Allocator>(self: &mut Self, alloc: &A, table_layout: TableLayout)` — [`TableLayout`](#tablelayout)

- `unsafe fn bucket<T>(self: &Self, index: usize) -> Bucket<T>` — [`Bucket`](#bucket)

- `unsafe fn bucket_ptr(self: &Self, index: usize, size_of: usize) -> *mut u8`

- `fn data_end<T>(self: &Self) -> NonNull<T>`

- `fn probe_seq(self: &Self, hash: u64) -> ProbeSeq` — [`ProbeSeq`](#probeseq)

- `unsafe fn record_item_insert_at(self: &mut Self, index: usize, old_ctrl: Tag, new_ctrl: Tag)` — [`Tag`](../control/tag/index.md)

- `fn is_in_same_group(self: &Self, i: usize, new_i: usize, hash: u64) -> bool`

- `unsafe fn set_ctrl_hash(self: &mut Self, index: usize, hash: u64)`

- `unsafe fn replace_ctrl_hash(self: &mut Self, index: usize, hash: u64) -> Tag` — [`Tag`](../control/tag/index.md)

- `unsafe fn set_ctrl(self: &mut Self, index: usize, ctrl: Tag)` — [`Tag`](../control/tag/index.md)

- `unsafe fn ctrl(self: &Self, index: usize) -> *mut Tag` — [`Tag`](../control/tag/index.md)

- `fn ctrl_slice(self: &mut Self) -> &mut [Tag]` — [`Tag`](../control/tag/index.md)

- `fn buckets(self: &Self) -> usize`

- `unsafe fn is_bucket_full(self: &Self, index: usize) -> bool`

- `fn num_ctrl_bytes(self: &Self) -> usize`

- `fn is_empty_singleton(self: &Self) -> bool`

- `fn prepare_resize<'a, A>(self: &Self, alloc: &'a A, table_layout: TableLayout, capacity: usize, fallibility: Fallibility) -> Result<crate::scopeguard::ScopeGuard<Self, impl FnMut(&mut Self) + 'a>, TryReserveError>` — [`TableLayout`](#tablelayout), [`Fallibility`](#fallibility), [`ScopeGuard`](../scopeguard/index.md), [`TryReserveError`](../index.md)

- `unsafe fn reserve_rehash_inner<A>(self: &mut Self, alloc: &A, additional: usize, hasher: &dyn Fn(&mut Self, usize) -> u64, fallibility: Fallibility, layout: TableLayout, drop: Option<fn(*mut u8)>) -> Result<(), TryReserveError>` — [`Fallibility`](#fallibility), [`TableLayout`](#tablelayout), [`TryReserveError`](../index.md)

- `unsafe fn full_buckets_indices(self: &Self) -> FullBucketsIndices` — [`FullBucketsIndices`](#fullbucketsindices)

- `unsafe fn resize_inner<A>(self: &mut Self, alloc: &A, capacity: usize, hasher: &dyn Fn(&mut Self, usize) -> u64, fallibility: Fallibility, layout: TableLayout) -> Result<(), TryReserveError>` — [`Fallibility`](#fallibility), [`TableLayout`](#tablelayout), [`TryReserveError`](../index.md)

- `unsafe fn rehash_in_place(self: &mut Self, hasher: &dyn Fn(&mut Self, usize) -> u64, size_of: usize, drop: Option<fn(*mut u8)>)`

- `unsafe fn free_buckets<A>(self: &mut Self, alloc: &A, table_layout: TableLayout)` — [`TableLayout`](#tablelayout)

- `unsafe fn allocation_info(self: &Self, table_layout: TableLayout) -> (NonNull<u8>, Layout)` — [`TableLayout`](#tablelayout)

- `unsafe fn allocation_size_or_zero(self: &Self, table_layout: TableLayout) -> usize` — [`TableLayout`](#tablelayout)

- `fn clear_no_drop(self: &mut Self)`

- `unsafe fn erase(self: &mut Self, index: usize)`

### `RawIterRange<T>`

```rust
struct RawIterRange<T> {
    current_group: self::bitmask::BitMaskIter,
    data: Bucket<T>,
    next_ctrl: *const u8,
    end: *const u8,
}
```

Iterator over a sub-range of a table. Unlike `RawIter` this iterator does
not track an item count.

#### Implementations

- `unsafe fn new(ctrl: *const u8, data: Bucket<T>, len: usize) -> Self` — [`Bucket`](#bucket)

- `unsafe fn next_impl<const DO_CHECK_PTR_RANGE: bool>(self: &mut Self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

- `unsafe fn fold_impl<F, B>(self: Self, n: usize, acc: B, f: F) -> B`

#### Trait Implementations

##### `impl<T> Clone for RawIterRange<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T> FusedIterator for RawIterRange<T>`

##### `impl<I> IntoIterator for RawIterRange<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T> Iterator for RawIterRange<T>`

- `type Item = Bucket<T>`

- `fn next(self: &mut Self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> Send for RawIterRange<T>`

##### `impl<T> Sync for RawIterRange<T>`

### `RawIter<T>`

```rust
struct RawIter<T> {
    iter: RawIterRange<T>,
    items: usize,
}
```

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

- `unsafe fn drop_elements(self: &mut Self)`

#### Trait Implementations

##### `impl<T> Clone for RawIter<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T> Default for RawIter<T>`

- `fn default() -> Self`

##### `impl<T> ExactSizeIterator for RawIter<T>`

##### `impl<T> FusedIterator for RawIter<T>`

##### `impl<I> IntoIterator for RawIter<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T> Iterator for RawIter<T>`

- `type Item = Bucket<T>`

- `fn next(self: &mut Self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `FullBucketsIndices`

```rust
struct FullBucketsIndices {
    current_group: self::bitmask::BitMaskIter,
    group_first_index: usize,
    ctrl: core::ptr::NonNull<u8>,
    items: usize,
}
```

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

- `unsafe fn next_impl(self: &mut Self) -> Option<usize>`

#### Trait Implementations

##### `impl Clone for FullBucketsIndices`

- `fn clone(self: &Self) -> FullBucketsIndices` — [`FullBucketsIndices`](#fullbucketsindices)

##### `impl Default for FullBucketsIndices`

- `fn default() -> Self`

##### `impl ExactSizeIterator for FullBucketsIndices`

##### `impl FusedIterator for FullBucketsIndices`

##### `impl<I> IntoIterator for FullBucketsIndices`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for FullBucketsIndices`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `RawIntoIter<T, A: Allocator>`

```rust
struct RawIntoIter<T, A: Allocator> {
    iter: RawIter<T>,
    allocation: Option<(core::ptr::NonNull<u8>, crate::alloc::alloc::Layout, A)>,
    marker: core::marker::PhantomData<T>,
}
```

Iterator which consumes a table and returns elements.

#### Implementations

- `fn iter(self: &Self) -> RawIter<T>` — [`RawIter`](#rawiter)

#### Trait Implementations

##### `impl<T, A: Allocator> Default for RawIntoIter<T, A>`

- `fn default() -> Self`

##### `impl<T, A: Allocator> Drop for RawIntoIter<T, A>`

- `fn drop(self: &mut Self)`

##### `impl<T, A: Allocator> ExactSizeIterator for RawIntoIter<T, A>`

##### `impl<T, A: Allocator> FusedIterator for RawIntoIter<T, A>`

##### `impl<I> IntoIterator for RawIntoIter<T, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T, A: Allocator> Iterator for RawIntoIter<T, A>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

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

Iterator which consumes elements without freeing the table storage.

#### Implementations

- `fn iter(self: &Self) -> RawIter<T>` — [`RawIter`](#rawiter)

#### Trait Implementations

##### `impl<T, A: Allocator> Drop for RawDrain<'_, T, A>`

- `fn drop(self: &mut Self)`

##### `impl<T, A: Allocator> ExactSizeIterator for RawDrain<'_, T, A>`

##### `impl<T, A: Allocator> FusedIterator for RawDrain<'_, T, A>`

##### `impl<I> IntoIterator for RawDrain<'a, T, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T, A: Allocator> Iterator for RawDrain<'_, T, A>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T, A> Send for RawDrain<'_, T, A>`

##### `impl<T, A> Sync for RawDrain<'_, T, A>`

### `RawIterHash<T>`

```rust
struct RawIterHash<T> {
    inner: RawIterHashIndices,
    _marker: core::marker::PhantomData<T>,
}
```

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

- `unsafe fn new<A: Allocator>(table: &RawTable<T, A>, hash: u64) -> Self` — [`RawTable`](#rawtable)

#### Trait Implementations

##### `impl<T> Clone for RawIterHash<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T> Default for RawIterHash<T>`

- `fn default() -> Self`

##### `impl<I> IntoIterator for RawIterHash<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T> Iterator for RawIterHash<T>`

- `type Item = Bucket<T>`

- `fn next(self: &mut Self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

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

#### Implementations

- `unsafe fn new(table: &RawTableInner, hash: u64) -> Self` — [`RawTableInner`](#rawtableinner)

#### Trait Implementations

##### `impl Clone for RawIterHashIndices`

- `fn clone(self: &Self) -> RawIterHashIndices` — [`RawIterHashIndices`](#rawiterhashindices)

##### `impl Default for RawIterHashIndices`

- `fn default() -> Self`

##### `impl<I> IntoIterator for RawIterHashIndices`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for RawIterHashIndices`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `RawExtractIf<'a, T, A: Allocator>`

```rust
struct RawExtractIf<'a, T, A: Allocator> {
    pub iter: RawIter<T>,
    pub table: &'a mut RawTable<T, A>,
}
```

#### Implementations

- `fn next<F>(self: &mut Self, f: F) -> Option<T>`

## Enums

### `Fallibility`

```rust
enum Fallibility {
    Fallible,
    Infallible,
}
```

Whether memory allocation errors should return an error or abort.

#### Implementations

- `fn capacity_overflow(self: Self) -> TryReserveError` — [`TryReserveError`](../index.md)

- `fn alloc_err(self: Self, layout: Layout) -> TryReserveError` — [`TryReserveError`](../index.md)

#### Trait Implementations

##### `impl Clone for Fallibility`

- `fn clone(self: &Self) -> Fallibility` — [`Fallibility`](#fallibility)

##### `impl Copy for Fallibility`

## Traits

### `SizedTypeProperties`

```rust
trait SizedTypeProperties: Sized { ... }
```

#### Required Methods

- `const IS_ZERO_SIZED: bool`

- `const NEEDS_DROP: bool`

### `RawTableClone`

```rust
trait RawTableClone { ... }
```

Specialization of `clone_from` for `Copy` types

#### Required Methods

- `fn clone_from_spec(self: &mut Self, source: &Self)`

## Functions

### `offset_from`

```rust
unsafe fn offset_from<T>(to: *const T, from: *const T) -> usize
```

### `h1`

```rust
fn h1(hash: u64) -> usize
```

Primary hash function, used to select the initial bucket to probe from.

### `capacity_to_buckets`

```rust
fn capacity_to_buckets(cap: usize, table_layout: TableLayout) -> Option<usize>
```

Returns the number of buckets needed to hold the given number of items,
taking the maximum load factor into account.

Returns `None` if an overflow occurs.

This ensures that `buckets * table_layout.size >= table_layout.ctrl_align`.

### `ensure_bucket_bytes_at_least_ctrl_align`

```rust
fn ensure_bucket_bytes_at_least_ctrl_align(table_layout: TableLayout, buckets: usize)
```

### `bucket_mask_to_capacity`

```rust
fn bucket_mask_to_capacity(bucket_mask: usize) -> usize
```

Returns the maximum effective capacity for the given bucket mask, taking
the maximum load factor into account.

### `prev_pow2`

```rust
fn prev_pow2(z: usize) -> usize
```

Find the previous power of 2. If it's already a power of 2, it's unchanged.
Passing zero is undefined behavior.

### `maximum_buckets_in`

```rust
fn maximum_buckets_in(allocation_size: usize, table_layout: TableLayout, group_width: usize) -> usize
```

Finds the largest number of buckets that can fit in `allocation_size`
provided the given TableLayout.

This relies on some invariants of `capacity_to_buckets`, so only feed in
an `allocation_size` calculated from `capacity_to_buckets`.

