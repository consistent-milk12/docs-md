*[compact_str](../../index.md) / [repr](../index.md) / [heap](index.md)*

---

# Module `heap`

## Contents

- [Modules](#modules)
  - [`heap_capacity`](#heap-capacity)
  - [`inline_capacity`](#inline-capacity)
- [Structs](#structs)
  - [`HeapBuffer`](#heapbuffer)
- [Functions](#functions)
  - [`amortized_growth`](#amortized-growth)
  - [`allocate_ptr`](#allocate-ptr)
  - [`deallocate_ptr`](#deallocate-ptr)
  - [`do_alloc`](#do-alloc)
- [Type Aliases](#type-aliases)
  - [`StrBuffer`](#strbuffer)
- [Constants](#constants)
  - [`MIN_HEAP_SIZE`](#min-heap-size)
  - [`UNKNOWN`](#unknown)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`heap_capacity`](#heap-capacity) | mod |  |
| [`inline_capacity`](#inline-capacity) | mod |  |
| [`HeapBuffer`](#heapbuffer) | struct |  |
| [`amortized_growth`](#amortized-growth) | fn | [`HeapBuffer`] grows at an amortized rates of 1.5x |
| [`allocate_ptr`](#allocate-ptr) | fn | Allocates a buffer on the heap that we can use to store a string, optionally stores the capacity of said buffer on the heap. |
| [`deallocate_ptr`](#deallocate-ptr) | fn | Deallocates a buffer on the heap, handling when the capacity is also stored on the heap |
| [`do_alloc`](#do-alloc) | fn | SAFETY: `layout` must not be zero sized |
| [`StrBuffer`](#strbuffer) | type |  |
| [`MIN_HEAP_SIZE`](#min-heap-size) | const | The minimum size we'll allocate on the heap is one usize larger than our max inline size |
| [`UNKNOWN`](#unknown) | const |  |

## Modules

- [`heap_capacity`](heap_capacity/index.md)
- [`inline_capacity`](inline_capacity/index.md)

## Structs

### `HeapBuffer`

```rust
struct HeapBuffer {
    ptr: ptr::NonNull<u8>,
    len: usize,
    cap: super::capacity::Capacity,
}
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:26-30`](../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L26-L30)*

#### Implementations

- <span id="heapbuffer-new"></span>`fn new(text: &str) -> Result<Self, ReserveError>` — [`ReserveError`](../../index.md#reserveerror)

- <span id="heapbuffer-with-capacity"></span>`fn with_capacity(capacity: usize) -> Result<Self, ReserveError>` — [`ReserveError`](../../index.md#reserveerror)

- <span id="heapbuffer-with-additional"></span>`fn with_additional(text: &str, additional: usize) -> Result<Self, ReserveError>` — [`ReserveError`](../../index.md#reserveerror)

- <span id="heapbuffer-capacity"></span>`fn capacity(&self) -> usize`

- <span id="heapbuffer-realloc"></span>`fn realloc(&mut self, new_capacity: usize) -> Result<usize, ()>`

- <span id="heapbuffer-set-len"></span>`unsafe fn set_len(&mut self, len: usize)`

- <span id="heapbuffer-dealloc"></span>`fn dealloc(&mut self)`

#### Trait Implementations

##### `impl Clone for HeapBuffer`

- <span id="heapbuffer-clone"></span>`fn clone(&self) -> Self`

##### `impl Drop for HeapBuffer`

- <span id="heapbuffer-drop"></span>`fn drop(&mut self)`

## Functions

### `amortized_growth`

```rust
fn amortized_growth(cur_len: usize, additional: usize) -> usize
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:19-23`](../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L19-L23)*

[`HeapBuffer`](#heapbuffer) grows at an amortized rates of 1.5x

Note: this is different than [`std::string::String`](../../../cargo_platform/index.md), which grows at a rate of 2x. It's debated
which is better, for now we'll stick with a rate of 1.5x

### `allocate_ptr`

```rust
fn allocate_ptr(capacity: usize) -> Result<(super::capacity::Capacity, ptr::NonNull<u8>), crate::ReserveError>
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:264-299`](../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L264-L299)*

Allocates a buffer on the heap that we can use to store a string, optionally stores the capacity
of said buffer on the heap.

Returns a [`Capacity`](../capacity/index.md) that either indicates the capacity is stored on the heap, or is stored
in the `Capacity` itself.

### `deallocate_ptr`

```rust
fn deallocate_ptr(ptr: ptr::NonNull<u8>, cap: super::capacity::Capacity)
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:303-328`](../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L303-L328)*

Deallocates a buffer on the heap, handling when the capacity is also stored on the heap

### `do_alloc`

```rust
unsafe fn do_alloc(layout: core::alloc::Layout) -> Result<ptr::NonNull<u8>, crate::ReserveError>
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:332-343`](../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L332-L343)*

SAFETY: `layout` must not be zero sized

## Type Aliases

### `StrBuffer`

```rust
type StrBuffer = [u8; 0];
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:12`](../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L12)*

## Constants

### `MIN_HEAP_SIZE`
```rust
const MIN_HEAP_SIZE: usize = 32usize;
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:9`](../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L9)*

The minimum size we'll allocate on the heap is one usize larger than our max inline size

### `UNKNOWN`
```rust
const UNKNOWN: usize = 0usize;
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:11`](../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L11)*

