*[crossbeam_epoch](../index.md) / [deferred](index.md)*

---

# Module `deferred`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Deferred`](#deferred) | struct | A `FnOnce()` that is stored inline if small, or otherwise boxed on the heap. |
| [`Data`](#data) | type | Some space to keep a `FnOnce()` object on the stack. |
| [`DATA_WORDS`](#data-words) | const | Number of words a piece of `Data` can hold. |

## Structs

### `Deferred`

```rust
struct Deferred {
    call: fn(*mut u8),
    data: core::mem::MaybeUninit<[usize; 3]>,
    _marker: core::marker::PhantomData<*mut ()>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/deferred.rs:19-23`](../../../.source_1765521767/crossbeam-epoch-0.9.18/src/deferred.rs#L19-L23)*

A `FnOnce()` that is stored inline if small, or otherwise boxed on the heap.

This is a handy way of keeping an unsized `FnOnce()` within a sized structure.

#### Implementations

- <span id="deferred-const-no-op"></span>`const NO_OP: Self`

- <span id="deferred-new"></span>`fn new<F: FnOnce()>(f: F) -> Self`

  Constructs a new `Deferred` from a `FnOnce()`.

- <span id="deferred-call"></span>`fn call(self)`

  Calls the function.

#### Trait Implementations

##### `impl Any for Deferred`

- <span id="deferred-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Deferred`

- <span id="deferred-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Deferred`

- <span id="deferred-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Deferred`

- <span id="deferred-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl<T> From for Deferred`

- <span id="deferred-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Deferred`

- <span id="deferred-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Deferred`

- <span id="deferred-pointable-const-align"></span>`const ALIGN: usize`

- <span id="deferred-pointable-type-init"></span>`type Init = T`

- <span id="deferred-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="deferred-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="deferred-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="deferred-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Deferred`

- <span id="deferred-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deferred-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Deferred`

- <span id="deferred-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deferred-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `Data`

```rust
type Data = [usize; 3];
```

*Defined in [`crossbeam-epoch-0.9.18/src/deferred.rs:14`](../../../.source_1765521767/crossbeam-epoch-0.9.18/src/deferred.rs#L14)*

Some space to keep a `FnOnce()` object on the stack.

## Constants

### `DATA_WORDS`
```rust
const DATA_WORDS: usize = 3usize;
```

*Defined in [`crossbeam-epoch-0.9.18/src/deferred.rs:11`](../../../.source_1765521767/crossbeam-epoch-0.9.18/src/deferred.rs#L11)*

Number of words a piece of `Data` can hold.

Three words should be enough for the majority of cases. For example, you can fit inside it the
function pointer together with a fat pointer representing an object that needs to be destroyed.

