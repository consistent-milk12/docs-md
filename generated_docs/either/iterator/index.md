*[either](../index.md) / [iterator](index.md)*

---

# Module `iterator`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IterEither`](#itereither) | struct | Iterator that maps left or right iterators to corresponding `Either`-wrapped items. |
| [`wrap_either!`](#wrap-either) | macro |  |

## Structs

### `IterEither<L, R>`

```rust
struct IterEither<L, R> {
    inner: super::Either<L, R>,
}
```

*Defined in [`either-1.15.0/src/iterator.rs:19-21`](../../../.source_1765633015/either-1.15.0/src/iterator.rs#L19-L21)*

Iterator that maps left or right iterators to corresponding `Either`-wrapped items.

This struct is created by the `Either::factor_into_iter`,
`factor_iter`,
and `factor_iter_mut` methods.

#### Implementations

- <span id="itereither-new"></span>`fn new(inner: Either<L, R>) -> Self` — [`Either`](../index.md#either)

#### Trait Implementations

##### `impl Any for IterEither<L, R>`

- <span id="itereither-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterEither<L, R>`

- <span id="itereither-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterEither<L, R>`

- <span id="itereither-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<L: clone::Clone, R: clone::Clone> Clone for IterEither<L, R>`

- <span id="itereither-clone"></span>`fn clone(&self) -> IterEither<L, R>` — [`IterEither`](#itereither)

##### `impl CloneToUninit for IterEither<L, R>`

- <span id="itereither-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<L: fmt::Debug, R: fmt::Debug> Debug for IterEither<L, R>`

- <span id="itereither-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<L, R> DoubleEndedIterator for IterEither<L, R>`

- <span id="itereither-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="itereither-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="itereither-doubleendediterator-rfold"></span>`fn rfold<Acc, G>(self, init: Acc, f: G) -> Acc`

- <span id="itereither-doubleendediterator-rfind"></span>`fn rfind<P>(&mut self, predicate: P) -> Option<<Self as >::Item>`

##### `impl<L, R> ExactSizeIterator for IterEither<L, R>`

- <span id="itereither-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IterEither<L, R>`

- <span id="itereither-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<L, R> FusedIterator for IterEither<L, R>`

##### `impl<U> Into for IterEither<L, R>`

- <span id="itereither-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IterEither<L, R>`

##### `impl IntoIterator for IterEither<L, R>`

- <span id="itereither-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itereither-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itereither-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<L, R> Iterator for IterEither<L, R>`

- <span id="itereither-iterator-type-item"></span>`type Item = Either<<L as Iterator>::Item, <R as Iterator>::Item>`

- <span id="itereither-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itereither-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="itereither-iterator-fold"></span>`fn fold<Acc, G>(self, init: Acc, f: G) -> Acc`

- <span id="itereither-iterator-for-each"></span>`fn for_each<F>(self, f: F)`

- <span id="itereither-iterator-count"></span>`fn count(self) -> usize`

- <span id="itereither-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="itereither-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="itereither-iterator-collect"></span>`fn collect<B>(self) -> B`

- <span id="itereither-iterator-partition"></span>`fn partition<B, F>(self, f: F) -> (B, B)`

- <span id="itereither-iterator-all"></span>`fn all<F>(&mut self, f: F) -> bool`

- <span id="itereither-iterator-any"></span>`fn any<F>(&mut self, f: F) -> bool`

- <span id="itereither-iterator-find"></span>`fn find<P>(&mut self, predicate: P) -> Option<<Self as >::Item>`

- <span id="itereither-iterator-find-map"></span>`fn find_map<B, F>(&mut self, f: F) -> Option<B>`

- <span id="itereither-iterator-position"></span>`fn position<P>(&mut self, predicate: P) -> Option<usize>`

##### `impl<U> TryFrom for IterEither<L, R>`

- <span id="itereither-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itereither-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterEither<L, R>`

- <span id="itereither-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itereither-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `wrap_either!`

*Defined in [`either-1.15.0/src/iterator.rs:4-11`](../../../.source_1765633015/either-1.15.0/src/iterator.rs#L4-L11)*

