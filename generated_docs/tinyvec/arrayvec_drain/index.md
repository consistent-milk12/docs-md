*[tinyvec](../index.md) / [arrayvec_drain](index.md)*

---

# Module `arrayvec_drain`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArrayVecDrain`](#arrayvecdrain) | struct | Draining iterator for [`ArrayVec`] |

## Structs

### `ArrayVecDrain<'a, T: 'a + Default>`

```rust
struct ArrayVecDrain<'a, T: 'a + Default> {
    iter: slice::IterMut<'a, T>,
}
```

*Defined in [`tinyvec-1.10.0/src/arrayvec_drain.rs:11-13`](../../../.source_1765521767/tinyvec-1.10.0/src/arrayvec_drain.rs#L11-L13)*

Draining iterator for [`ArrayVec`](../index.md)

See [`ArrayVec::drain`](ArrayVec::drain)

#### Implementations

- <span id="arrayvecdrain-new"></span>`fn new<A, R>(arr: &'a mut ArrayVec<A>, range: R) -> Self` — [`ArrayVec`](../index.md#arrayvec)

#### Trait Implementations

##### `impl<T> Any for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: 'a + Default> DoubleEndedIterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<T: 'a + Default> ExactSizeIterator for ArrayVecDrain<'a, T>`

##### `impl<T> From for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: 'a + Default> FusedIterator for ArrayVecDrain<'a, T>`

##### `impl<T, U> Into for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arrayvecdrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arrayvecdrain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: 'a + Default> Iterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-iterator-type-item"></span>`type Item = T`

- <span id="arrayvecdrain-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="arrayvecdrain-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-iterator-for-each"></span>`fn for_each<F>(self, f: F)`

##### `impl<T, U> TryFrom for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arrayvecdrain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arrayvecdrain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

