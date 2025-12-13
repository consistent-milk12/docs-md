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

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:405-407`](../../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L405-L407)*

#### Trait Implementations

##### `impl Any for HeapBufferInnerInlineCapacity`

- <span id="heapbufferinnerinlinecapacity-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HeapBufferInnerInlineCapacity`

- <span id="heapbufferinnerinlinecapacity-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HeapBufferInnerInlineCapacity`

- <span id="heapbufferinnerinlinecapacity-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for HeapBufferInnerInlineCapacity`

- <span id="heapbufferinnerinlinecapacity-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HeapBufferInnerInlineCapacity`

- <span id="heapbufferinnerinlinecapacity-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for HeapBufferInnerInlineCapacity`

- <span id="heapbufferinnerinlinecapacity-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="heapbufferinnerinlinecapacity-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HeapBufferInnerInlineCapacity`

- <span id="heapbufferinnerinlinecapacity-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="heapbufferinnerinlinecapacity-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `alloc`

```rust
unsafe fn alloc(capacity: usize) -> Result<ptr::NonNull<u8>, crate::ReserveError>
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:391-393`](../../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L391-L393)*

# SAFETY:
* `capacity` must be > 0

### `dealloc`

```rust
unsafe fn dealloc(ptr: ptr::NonNull<u8>, capacity: usize)
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:399-402`](../../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L399-L402)*

Deallocates a pointer which references a `HeapBuffer` whose capacity is stored inline

# Safety
* `ptr` must point to the start of a `HeapBuffer` whose capacity is on the inline

### `layout`

```rust
fn layout(capacity: usize) -> alloc::Layout
```

*Defined in [`compact_str-0.9.0/src/repr/heap.rs:410-417`](../../../../../.source_1765521767/compact_str-0.9.0/src/repr/heap.rs#L410-L417)*

