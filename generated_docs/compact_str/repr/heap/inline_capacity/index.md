*[compact_str](../../../index.md) / [repr](../../index.md) / [heap](../index.md) / [inline_capacity](index.md)*

---

# Module `inline_capacity`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HeapBufferInnerInlineCapacity`](#heapbufferinnerinlinecapacity) | struct |  |
| [`alloc`](#alloc) | fn | # SAFETY: * `capacity` must be > 0 |
| [`dealloc`](#dealloc) | fn | Deallocates a pointer which references a `HeapBuffer` whose capacity is stored inline |
| [`layout`](#layout) | fn |  |

## Structs

### `HeapBufferInnerInlineCapacity`

```rust
struct HeapBufferInnerInlineCapacity {
    buffer: [u8; 0],
}
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:405-407`](../../../../../.source_1765210505/compact_str-0.9.0/src/repr/heap.rs#L405-L407)*

## Functions

### `alloc`

```rust
unsafe fn alloc(capacity: usize) -> Result<ptr::NonNull<u8>, crate::ReserveError>
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:391-393`](../../../../../.source_1765210505/compact_str-0.9.0/src/repr/heap.rs#L391-L393)*

# SAFETY:
* `capacity` must be > 0

### `dealloc`

```rust
unsafe fn dealloc(ptr: ptr::NonNull<u8>, capacity: usize)
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:399-402`](../../../../../.source_1765210505/compact_str-0.9.0/src/repr/heap.rs#L399-L402)*

Deallocates a pointer which references a `HeapBuffer` whose capacity is stored inline

# Safety
* `ptr` must point to the start of a `HeapBuffer` whose capacity is on the inline

### `layout`

```rust
fn layout(capacity: usize) -> alloc::Layout
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:410-417`](../../../../../.source_1765210505/compact_str-0.9.0/src/repr/heap.rs#L410-L417)*

