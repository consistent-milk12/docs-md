*[itertools](../index.md) / [next_array](index.md)*

---

# Module `next_array`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArrayBuilder`](#arraybuilder) | struct | An array of at most `N` elements. |
| [`slice_assume_init_mut`](#slice-assume-init-mut) | fn | Assuming all the elements are initialized, get a mutable slice to them. |
| [`next_array`](#next-array) | fn | Equivalent to `it.next_array()`. |

## Structs

### `ArrayBuilder<T, const N: usize>`

```rust
struct ArrayBuilder<T, const N: usize> {
    arr: [core::mem::MaybeUninit<T>; N],
    len: usize,
}
```

*Defined in [`itertools-0.14.0/src/next_array.rs:4-14`](../../../.source_1765900590/itertools-0.14.0/src/next_array.rs#L4-L14)*

An array of at most `N` elements.

#### Fields

- **`arr`**: `[core::mem::MaybeUninit<T>; N]`

  The (possibly uninitialized) elements of the `ArrayBuilder`.
  
  # Safety
  
  The elements of `arr[..len]` are valid `T`s.

- **`len`**: `usize`

  The number of leading elements of `arr` that are valid `T`s, len <= N.

#### Implementations

- <span id="arraybuilder-new"></span>`fn new() -> Self`

  Initializes a new, empty `ArrayBuilder`.

- <span id="arraybuilder-push"></span>`fn push(&mut self, value: T)`

  Pushes `value` onto the end of the array.
  
  ##### Panics
  
  This panics if `self.len >= N`.

- <span id="arraybuilder-take"></span>`fn take(&mut self) -> Option<[T; N]>`

  Consumes the elements in the `ArrayBuilder` and returns them as an array
  `[T; N]`.
  
  If `self.len() < N`, this returns `None`.

#### Trait Implementations

##### `impl<T> Any for ArrayBuilder<T, N>`

- <span id="arraybuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> AsMut for ArrayBuilder<T, N>`

- <span id="arraybuilder-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [T]`

##### `impl<T> Borrow for ArrayBuilder<T, N>`

- <span id="arraybuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArrayBuilder<T, N>`

- <span id="arraybuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Drop for ArrayBuilder<T, N>`

- <span id="arraybuilder-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for ArrayBuilder<T, N>`

- <span id="arraybuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ArrayBuilder<T, N>`

- <span id="arraybuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ArrayBuilder<T, N>`

##### `impl<T, U> TryFrom for ArrayBuilder<T, N>`

- <span id="arraybuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arraybuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ArrayBuilder<T, N>`

- <span id="arraybuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arraybuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `slice_assume_init_mut`

```rust
unsafe fn slice_assume_init_mut<T>(slice: &mut [core::mem::MaybeUninit<T>]) -> &mut [T]
```

*Defined in [`itertools-0.14.0/src/next_array.rs:128-134`](../../../.source_1765900590/itertools-0.14.0/src/next_array.rs#L128-L134)*

Assuming all the elements are initialized, get a mutable slice to them.

# Safety

The caller guarantees that the elements `T` referenced by `slice` are in a
valid state.

### `next_array`

```rust
fn next_array<I, const N: usize>(it: &mut I) -> Option<[<I as >::Item; N]>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/next_array.rs:137-146`](../../../.source_1765900590/itertools-0.14.0/src/next_array.rs#L137-L146)*

Equivalent to `it.next_array()`.

