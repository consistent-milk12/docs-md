*[compact_str](../../../index.md) / [repr](../../index.md) / [heap](../index.md) / [heap_capacity](index.md)*

---

# Module `heap_capacity`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HeapBufferInnerHeapCapacity`](#heapbufferinnerheapcapacity) | struct |  |
| [`alloc`](#alloc) | fn | SAFETY: `capacity` must not be zero |
| [`dealloc`](#dealloc) | fn | Deallocates a pointer which references a `HeapBuffer` whose capacity is on the heap |
| [`layout`](#layout) | fn |  |

## Structs

### `HeapBufferInnerHeapCapacity`

```rust
struct HeapBufferInnerHeapCapacity {
    capacity: usize,
    buffer: [u8; 0],
}
```

## Functions

### `alloc`

```rust
unsafe fn alloc(capacity: usize) -> Result<ptr::NonNull<u8>, crate::ReserveError>
```

SAFETY: `capacity` must not be zero

### `dealloc`

```rust
unsafe fn dealloc(ptr: ptr::NonNull<u8>, capacity: usize)
```

Deallocates a pointer which references a `HeapBuffer` whose capacity is on the heap

# Safety
* `ptr` must point to the start of a `HeapBuffer` whose capacity is on the heap. i.e. we
  must have `ptr -> [cap<usize> ; string<bytes>]`

### `layout`

```rust
fn layout(capacity: usize) -> alloc::Layout
```

