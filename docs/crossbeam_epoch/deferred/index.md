*[crossbeam_epoch](../index.md) / [deferred](index.md)*

---

# Module `deferred`

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

- `const NO_OP: Self`

- `fn new<F: FnOnce()>(f: F) -> Self`

- `fn call(self: Self)`

#### Trait Implementations

##### `impl Debug for Deferred`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl<T> Pointable for Deferred`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

