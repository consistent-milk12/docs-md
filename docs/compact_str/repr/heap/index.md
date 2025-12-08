*[compact_str](../../index.md) / [repr](../index.md) / [heap](index.md)*

---

# Module `heap`

## Modules

- [`heap_capacity`](heap_capacity/index.md) - 
- [`inline_capacity`](inline_capacity/index.md) - 

## Structs

### `HeapBuffer`

```rust
struct HeapBuffer {
    ptr: ptr::NonNull<u8>,
    len: usize,
    cap: super::capacity::Capacity,
}
```

#### Implementations

- `fn new(text: &str) -> Result<Self, ReserveError>` — [`ReserveError`](../../index.md)

- `fn with_capacity(capacity: usize) -> Result<Self, ReserveError>` — [`ReserveError`](../../index.md)

- `fn with_additional(text: &str, additional: usize) -> Result<Self, ReserveError>` — [`ReserveError`](../../index.md)

- `fn capacity(self: &Self) -> usize`

- `fn realloc(self: &mut Self, new_capacity: usize) -> Result<usize, ()>`

- `unsafe fn set_len(self: &mut Self, len: usize)`

- `fn dealloc(self: &mut Self)`

#### Trait Implementations

##### `impl Clone for HeapBuffer`

- `fn clone(self: &Self) -> Self`

##### `impl Drop for HeapBuffer`

- `fn drop(self: &mut Self)`

## Functions

### `amortized_growth`

```rust
fn amortized_growth(cur_len: usize, additional: usize) -> usize
```

[`HeapBuffer`](#heapbuffer) grows at an amortized rates of 1.5x

Note: this is different than [`std::string::String`](../../../clap_builder/index.md), which grows at a rate of 2x. It's debated
which is better, for now we'll stick with a rate of 1.5x

### `allocate_ptr`

```rust
fn allocate_ptr(capacity: usize) -> Result<(super::capacity::Capacity, ptr::NonNull<u8>), crate::ReserveError>
```

Allocates a buffer on the heap that we can use to store a string, optionally stores the capacity
of said buffer on the heap.

Returns a [`Capacity`](../capacity/index.md) that either indicates the capacity is stored on the heap, or is stored
in the `Capacity` itself.

### `deallocate_ptr`

```rust
fn deallocate_ptr(ptr: ptr::NonNull<u8>, cap: super::capacity::Capacity)
```

Deallocates a buffer on the heap, handling when the capacity is also stored on the heap

### `do_alloc`

```rust
unsafe fn do_alloc(layout: core::alloc::Layout) -> Result<ptr::NonNull<u8>, crate::ReserveError>
```

SAFETY: `layout` must not be zero sized

## Type Aliases

### `StrBuffer`

```rust
type StrBuffer = [u8; 0];
```

## Constants

### `MIN_HEAP_SIZE`

```rust
const MIN_HEAP_SIZE: usize = 32usize;
```

The minimum size we'll allocate on the heap is one usize larger than our max inline size

### `UNKNOWN`

```rust
const UNKNOWN: usize = 0usize;
```

