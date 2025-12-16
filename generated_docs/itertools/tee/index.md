*[itertools](../index.md) / [tee](index.md)*

---

# Module `tee`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TeeBuffer`](#teebuffer) | struct | Common buffer object for the two tee halves |
| [`Tee`](#tee) | struct | One half of an iterator pair where both return the same elements. |
| [`new`](#new) | fn |  |

## Structs

### `TeeBuffer<A, I>`

```rust
struct TeeBuffer<A, I> {
    backlog: alloc::collections::VecDeque<A>,
    iter: I,
    owner: bool,
}
```

*Defined in [`itertools-0.14.0/src/tee.rs:9-14`](../../../.source_1765900590/itertools-0.14.0/src/tee.rs#L9-L14)*

Common buffer object for the two tee halves

#### Fields

- **`owner`**: `bool`

  The owner field indicates which id should read from the backlog

#### Trait Implementations

##### `impl Any for TeeBuffer<A, I>`

- <span id="teebuffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TeeBuffer<A, I>`

- <span id="teebuffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TeeBuffer<A, I>`

- <span id="teebuffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: fmt::Debug, I: fmt::Debug> Debug for TeeBuffer<A, I>`

- <span id="teebuffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TeeBuffer<A, I>`

- <span id="teebuffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TeeBuffer<A, I>`

- <span id="teebuffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TeeBuffer<A, I>`

##### `impl<U> TryFrom for TeeBuffer<A, I>`

- <span id="teebuffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="teebuffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TeeBuffer<A, I>`

- <span id="teebuffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="teebuffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tee<I>`

```rust
struct Tee<I>
where
    I: Iterator {
    rcbuffer: alloc::rc::Rc<std::cell::RefCell<TeeBuffer<<I as >::Item, I>>>,
    id: bool,
}
```

*Defined in [`itertools-0.14.0/src/tee.rs:21-27`](../../../.source_1765900590/itertools-0.14.0/src/tee.rs#L21-L27)*

One half of an iterator pair where both return the same elements.

See [`.tee()`](crate::Itertools::tee) for more information.

#### Trait Implementations

##### `impl Any for Tee<I>`

- <span id="tee-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tee<I>`

- <span id="tee-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tee<I>`

- <span id="tee-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Debug for Tee<I>`

- <span id="tee-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> ExactSizeIterator for Tee<I>`

##### `impl<T> From for Tee<I>`

- <span id="tee-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tee<I>`

- <span id="tee-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tee<I>`

##### `impl<I> IntoIterator for Tee<I>`

- <span id="tee-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tee-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tee-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tee<I>`

- <span id="tee-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tee-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tee-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for Tee<I>`

##### `impl MultiUnzip for Tee<I>`

- <span id="tee-multiunzip"></span>`fn multiunzip(self)`

##### `impl<U> TryFrom for Tee<I>`

- <span id="tee-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tee-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tee<I>`

- <span id="tee-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tee-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `new`

```rust
fn new<I>(iter: I) -> (Tee<I>, Tee<I>)
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/tee.rs:29-47`](../../../.source_1765900590/itertools-0.14.0/src/tee.rs#L29-L47)*

