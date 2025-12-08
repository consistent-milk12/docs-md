*[compact_str](../../../index.md) / [repr](../../index.md) / [heap](../index.md) / [inline_capacity](index.md)*

---

# Module `inline_capacity`

## Structs

### `HeapBufferInnerInlineCapacity`

```rust
struct HeapBufferInnerInlineCapacity {
    buffer: [u8; 0],
}
```

## Functions

### `alloc`

```rust
unsafe fn alloc(capacity: usize) -> Result<ptr::NonNull<u8>, crate::ReserveError>
```

# SAFETY:
* `capacity` must be > 0

### `dealloc`

```rust
unsafe fn dealloc(ptr: ptr::NonNull<u8>, capacity: usize)
```

Deallocates a pointer which references a `HeapBuffer` whose capacity is stored inline

# Safety
* `ptr` must point to the start of a `HeapBuffer` whose capacity is on the inline

### `layout`

```rust
fn layout(capacity: usize) -> alloc::Layout
```

