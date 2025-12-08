*[allocator_api2](../../index.md) / [stable](../index.md) / [raw_vec](index.md)*

---

# Module `raw_vec`

## Structs

### `TryReserveError`

```rust
struct TryReserveError {
    kind: TryReserveErrorKind,
}
```

The error type for `try_reserve` methods.

#### Implementations

- `fn kind(self: &Self) -> TryReserveErrorKind` — [`TryReserveErrorKind`](../collections/index.md)

#### Trait Implementations

##### `impl Clone for TryReserveError`

- `fn clone(self: &Self) -> TryReserveError` — [`TryReserveError`](../collections/index.md)

##### `impl Debug for TryReserveError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for TryReserveError`

- `fn fmt(self: &Self, fmt: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

##### `impl Eq for TryReserveError`

##### `impl PartialEq for TryReserveError`

- `fn eq(self: &Self, other: &TryReserveError) -> bool` — [`TryReserveError`](../collections/index.md)

##### `impl StructuralPartialEq for TryReserveError`

##### `impl<T> ToString for TryReserveError`

- `fn to_string(self: &Self) -> String`

### `RawVec<T, A: Allocator>`

```rust
struct RawVec<T, A: Allocator> {
    ptr: core::ptr::NonNull<T>,
    cap: usize,
    alloc: A,
}
```

A low-level utility for more ergonomically allocating, reallocating, and deallocating
a buffer of memory on the heap without having to worry about all the corner cases
involved. This type is excellent for building your own data structures like Vec and VecDeque.
In particular:

* Produces `NonNull::dangling()` on zero-sized types.
* Produces `NonNull::dangling()` on zero-length allocations.
* Avoids freeing `NonNull::dangling()`.
* Catches all overflows in capacity computations (promotes them to "capacity overflow" panics).
* Guards against 32-bit systems allocating more than isize::MAX bytes.
* Guards against overflowing your length.
* Calls `handle_alloc_error` for fallible allocations.
* Contains a `ptr::NonNull` and thus endows the user with all related benefits.
* Uses the excess returned from the allocator to use the largest available capacity.

This type does not in anyway inspect the memory that it manages. When dropped it *will*
free its memory, but it *won't* try to drop its contents. It is up to the user of `RawVec`
to handle the actual things *stored* inside of a `RawVec`.

Note that the excess of a zero-sized types is always infinite, so `capacity()` always returns
`usize::MAX`. This means that you need to be careful when round-tripping this type with a
`Box<[T]>`, since `capacity()` won't yield the length.

#### Implementations

- `fn needs_to_grow(self: &Self, len: usize, additional: usize) -> bool`

- `fn set_ptr_and_cap(self: &mut Self, ptr: NonNull<[u8]>, cap: usize)`

- `fn grow_amortized(self: &mut Self, len: usize, additional: usize) -> Result<(), TryReserveError>` — [`TryReserveError`](../collections/index.md)

- `fn grow_exact(self: &mut Self, len: usize, additional: usize) -> Result<(), TryReserveError>` — [`TryReserveError`](../collections/index.md)

- `fn shrink(self: &mut Self, cap: usize) -> Result<(), TryReserveError>` — [`TryReserveError`](../collections/index.md)

#### Trait Implementations

##### `impl<T, A: Allocator> Drop for RawVec<T, A>`

- `fn drop(self: &mut Self)`

##### `impl<T, A> Send for RawVec<T, A>`

##### `impl<T, A> Sync for RawVec<T, A>`

## Enums

### `TryReserveErrorKind`

```rust
enum TryReserveErrorKind {
    CapacityOverflow,
    AllocError {
        layout: super::alloc::Layout,
    },
}
```

Details of the allocation that caused a `TryReserveError`

#### Variants

- **`CapacityOverflow`**

  Error due to the computed capacity exceeding the collection's maximum
  (usually `isize::MAX` bytes).

- **`AllocError`**

  The memory allocator returned an error

#### Trait Implementations

##### `impl Clone for TryReserveErrorKind`

- `fn clone(self: &Self) -> TryReserveErrorKind` — [`TryReserveErrorKind`](../collections/index.md)

##### `impl Debug for TryReserveErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for TryReserveErrorKind`

##### `impl PartialEq for TryReserveErrorKind`

- `fn eq(self: &Self, other: &TryReserveErrorKind) -> bool` — [`TryReserveErrorKind`](../collections/index.md)

##### `impl StructuralPartialEq for TryReserveErrorKind`

### `AllocInit`

```rust
enum AllocInit {
    Uninitialized,
    Zeroed,
}
```

#### Variants

- **`Uninitialized`**

  The contents of the new memory are uninitialized.

- **`Zeroed`**

  The new memory is guaranteed to be zeroed.

## Functions

### `finish_grow`

```rust
fn finish_grow<A>(new_layout: Result<super::alloc::Layout, core::alloc::LayoutError>, current_memory: Option<(core::ptr::NonNull<u8>, super::alloc::Layout)>, alloc: &mut A) -> Result<core::ptr::NonNull<[u8]>, TryReserveError>
where
    A: Allocator
```

### `handle_reserve`

```rust
fn handle_reserve(result: Result<(), TryReserveError>)
```

### `alloc_guard`

```rust
fn alloc_guard(alloc_size: usize) -> Result<(), TryReserveError>
```

### `capacity_overflow`

```rust
fn capacity_overflow() -> never
```

