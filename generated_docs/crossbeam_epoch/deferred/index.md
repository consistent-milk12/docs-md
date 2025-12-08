*[crossbeam_epoch](../index.md) / [deferred](index.md)*

---

# Module `deferred`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Deferred`](#deferred) | struct | A `FnOnce()` that is stored inline if small, or otherwise boxed on the heap. |
| [`Data`](#data) | type | Some space to keep a `FnOnce()` object on the stack. |
| [`DATA_WORDS`](#data_words) | const | Number of words a piece of `Data` can hold. |

## Structs

### `Deferred`

```rust
struct Deferred {
    call: fn(*mut u8),
    data: core::mem::MaybeUninit<[usize; 3]>,
    _marker: core::marker::PhantomData<*mut ()>,
}
```

A `FnOnce()` that is stored inline if small, or otherwise boxed on the heap.

This is a handy way of keeping an unsized `FnOnce()` within a sized structure.

#### Implementations

- <span id="deferred-no-op"></span>`const NO_OP: Self`

- <span id="deferred-new"></span>`fn new<F: FnOnce()>(f: F) -> Self`

- <span id="deferred-call"></span>`fn call(self)`

#### Trait Implementations

##### `impl Debug for Deferred`

- <span id="deferred-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl<T> Pointable for Deferred`

- <span id="deferred-align"></span>`const ALIGN: usize`

- <span id="deferred-init"></span>`type Init = T`

- <span id="deferred-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- <span id="deferred-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="deferred-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="deferred-drop"></span>`unsafe fn drop(ptr: usize)`

## Type Aliases

### `Data`

```rust
type Data = [usize; 3];
```

Some space to keep a `FnOnce()` object on the stack.

## Constants

### `DATA_WORDS`

```rust
const DATA_WORDS: usize = 3usize;
```

Number of words a piece of `Data` can hold.

Three words should be enough for the majority of cases. For example, you can fit inside it the
function pointer together with a fat pointer representing an object that needs to be destroyed.

