*[itertools](../index.md) / [put_back_n_impl](index.md)*

---

# Module `put_back_n_impl`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PutBackN`](#putbackn) | struct | An iterator adaptor that allows putting multiple items in front of the iterator. |
| [`put_back_n`](#put-back-n) | fn | Create an iterator where you can put back multiple values to the front of the iteration. |

## Structs

### `PutBackN<I: Iterator>`

```rust
struct PutBackN<I: Iterator> {
    top: alloc::vec::Vec<<I as >::Item>,
    iter: I,
}
```

*Defined in [`itertools-0.14.0/src/put_back_n_impl.rs:11-14`](../../../.source_1765894658/itertools-0.14.0/src/put_back_n_impl.rs#L11-L14)*

An iterator adaptor that allows putting multiple
items in front of the iterator.

Iterator element type is `I::Item`.

#### Implementations

- <span id="putbackn-put-back"></span>`fn put_back(&mut self, x: <I as >::Item)`

  Puts `x` in front of the iterator.
  
  The values are yielded in order of the most recently put back
  values first.
  
  ```rust
  use itertools::put_back_n;
  
  let mut it = put_back_n(1..5);
  it.next();
  it.put_back(1);
  it.put_back(0);
  
  assert!(itertools::equal(it, 0..5));
  ```

#### Trait Implementations

##### `impl Any for PutBackN<I>`

- <span id="putbackn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PutBackN<I>`

- <span id="putbackn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PutBackN<I>`

- <span id="putbackn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for PutBackN<I>`

- <span id="putbackn-clone"></span>`fn clone(&self) -> PutBackN<I>` — [`PutBackN`](#putbackn)

##### `impl CloneToUninit for PutBackN<I>`

- <span id="putbackn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for PutBackN<I>`

- <span id="putbackn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PutBackN<I>`

- <span id="putbackn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PutBackN<I>`

- <span id="putbackn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PutBackN<I>`

##### `impl<I> IntoIterator for PutBackN<I>`

- <span id="putbackn-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="putbackn-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="putbackn-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator> Iterator for PutBackN<I>`

- <span id="putbackn-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="putbackn-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="putbackn-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="putbackn-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for PutBackN<I>`

##### `impl MultiUnzip for PutBackN<I>`

- <span id="putbackn-multiunzip"></span>`fn multiunzip(self)`

##### `impl<I> PeekingNext for crate::PutBackN<I>`

- <span id="crateputbackn-peekingnext-peeking-next"></span>`fn peeking_next<F>(&mut self, accept: F) -> Option<<Self as >::Item>`

##### `impl ToOwned for PutBackN<I>`

- <span id="putbackn-toowned-type-owned"></span>`type Owned = T`

- <span id="putbackn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="putbackn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PutBackN<I>`

- <span id="putbackn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="putbackn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PutBackN<I>`

- <span id="putbackn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="putbackn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `put_back_n`

```rust
fn put_back_n<I>(iterable: I) -> PutBackN<<I as >::IntoIter>
where
    I: IntoIterator
```

*Defined in [`itertools-0.14.0/src/put_back_n_impl.rs:20-28`](../../../.source_1765894658/itertools-0.14.0/src/put_back_n_impl.rs#L20-L28)*

Create an iterator where you can put back multiple values to the front
of the iteration.

Iterator element type is `I::Item`.

