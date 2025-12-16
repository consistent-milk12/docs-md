*[itertools](../index.md) / [rciter_impl](index.md)*

---

# Module `rciter_impl`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RcIter`](#rciter) | struct | A wrapper for `Rc<RefCell<I>>`, that implements the `Iterator` trait. |
| [`rciter`](#rciter) | fn | Return an iterator inside a `Rc<RefCell<_>>` wrapper. |

## Structs

### `RcIter<I>`

```rust
struct RcIter<I> {
    pub rciter: alloc::rc::Rc<std::cell::RefCell<I>>,
}
```

*Defined in [`itertools-0.14.0/src/rciter_impl.rs:8-11`](../../../.source_1765894658/itertools-0.14.0/src/rciter_impl.rs#L8-L11)*

A wrapper for `Rc<RefCell<I>>`, that implements the `Iterator` trait.

#### Fields

- **`rciter`**: `alloc::rc::Rc<std::cell::RefCell<I>>`

  The boxed iterator.

#### Trait Implementations

##### `impl Any for RcIter<I>`

- <span id="rciter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RcIter<I>`

- <span id="rciter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RcIter<I>`

- <span id="rciter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for RcIter<I>`

- <span id="rciter-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for RcIter<I>`

- <span id="rciter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for RcIter<I>`

- <span id="rciter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> DoubleEndedIterator for RcIter<I>`

- <span id="rciter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> From for RcIter<I>`

- <span id="rciter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for RcIter<I>`

##### `impl<U> Into for RcIter<I>`

- <span id="rciter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for RcIter<I>`

##### `impl<I> IntoIterator for RcIter<I>`

- <span id="rciter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rciter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rciter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for RcIter<I>`

- <span id="rciter-iterator-type-item"></span>`type Item = A`

- <span id="rciter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="rciter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for RcIter<I>`

##### `impl MultiUnzip for RcIter<I>`

- <span id="rciter-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for RcIter<I>`

- <span id="rciter-toowned-type-owned"></span>`type Owned = T`

- <span id="rciter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rciter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RcIter<I>`

- <span id="rciter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rciter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RcIter<I>`

- <span id="rciter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rciter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `rciter`

```rust
fn rciter<I>(iterable: I) -> RcIter<<I as >::IntoIter>
where
    I: IntoIterator
```

*Defined in [`itertools-0.14.0/src/rciter_impl.rs:47-54`](../../../.source_1765894658/itertools-0.14.0/src/rciter_impl.rs#L47-L54)*

Return an iterator inside a `Rc<RefCell<_>>` wrapper.

The returned `RcIter` can be cloned, and each clone will refer back to the
same original iterator.

`RcIter` allows doing interesting things like using `.zip()` on an iterator with
itself, at the cost of runtime borrow checking which may have a performance
penalty.

Iterator element type is `Self::Item`.

```rust
use itertools::rciter;
use itertools::zip;

// In this example a range iterator is created and we iterate it using
// three separate handles (two of them given to zip).
// We also use the IntoIterator implementation for `&RcIter`.

let mut iter = rciter(0..9);
let mut z = zip(&iter, &iter);

assert_eq!(z.next(), Some((0, 1)));
assert_eq!(z.next(), Some((2, 3)));
assert_eq!(z.next(), Some((4, 5)));
assert_eq!(iter.next(), Some(6));
assert_eq!(z.next(), Some((7, 8)));
assert_eq!(z.next(), None);
```

**Panics** in iterator methods if a borrow error is encountered in the
iterator methods. It can only happen if the `RcIter` is reentered in
`.next()`, i.e. if it somehow participates in an “iterator knot”
where it is an adaptor of itself.

