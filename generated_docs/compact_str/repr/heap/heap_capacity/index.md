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

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:367-370`](../../../../../.source_1765633015/compact_str-0.9.0/src/repr/heap.rs#L367-L370)*

#### Trait Implementations

##### `impl Any for HeapBufferInnerHeapCapacity`

- <span id="heapbufferinnerheapcapacity-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HeapBufferInnerHeapCapacity`

- <span id="heapbufferinnerheapcapacity-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HeapBufferInnerHeapCapacity`

- <span id="heapbufferinnerheapcapacity-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for HeapBufferInnerHeapCapacity`

- <span id="heapbufferinnerheapcapacity-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HeapBufferInnerHeapCapacity`

- <span id="heapbufferinnerheapcapacity-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for HeapBufferInnerHeapCapacity`

- <span id="heapbufferinnerheapcapacity-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="heapbufferinnerheapcapacity-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HeapBufferInnerHeapCapacity`

- <span id="heapbufferinnerheapcapacity-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="heapbufferinnerheapcapacity-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `alloc`

```rust
unsafe fn alloc(capacity: usize) -> Result<ptr::NonNull<u8>, crate::ReserveError>
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:352-354`](../../../../../.source_1765633015/compact_str-0.9.0/src/repr/heap.rs#L352-L354)*

SAFETY: `capacity` must not be zero

### `dealloc`

```rust
unsafe fn dealloc(ptr: ptr::NonNull<u8>, capacity: usize)
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:361-364`](../../../../../.source_1765633015/compact_str-0.9.0/src/repr/heap.rs#L361-L364)*

Deallocates a pointer which references a `HeapBuffer` whose capacity is on the heap

# Safety
* `ptr` must point to the start of a `HeapBuffer` whose capacity is on the heap. i.e. we
  must have `ptr -> [cap<usize> ; string<bytes>]`

### `layout`

```rust
fn layout(capacity: usize) -> alloc::Layout
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:373-380`](../../../../../.source_1765633015/compact_str-0.9.0/src/repr/heap.rs#L373-L380)*

