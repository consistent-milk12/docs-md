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

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:367-370`](../../../../../.source_1765210505/compact_str-0.9.0/src/repr/heap.rs#L367-L370)*

## Functions

### `alloc`

```rust
unsafe fn alloc(capacity: usize) -> Result<ptr::NonNull<u8>, crate::ReserveError>
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:352-354`](../../../../../.source_1765210505/compact_str-0.9.0/src/repr/heap.rs#L352-L354)*

SAFETY: `capacity` must not be zero

### `dealloc`

```rust
unsafe fn dealloc(ptr: ptr::NonNull<u8>, capacity: usize)
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:361-364`](../../../../../.source_1765210505/compact_str-0.9.0/src/repr/heap.rs#L361-L364)*

Deallocates a pointer which references a `HeapBuffer` whose capacity is on the heap

# Safety
* `ptr` must point to the start of a `HeapBuffer` whose capacity is on the heap. i.e. we
  must have `ptr -> [cap<usize> ; string<bytes>]`

### `layout`

```rust
fn layout(capacity: usize) -> alloc::Layout
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:373-380`](../../../../../.source_1765210505/compact_str-0.9.0/src/repr/heap.rs#L373-L380)*

