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

Draining iterator for [`ArrayVec`](../index.md)

See [`ArrayVec::drain`](ArrayVec::drain)

#### Implementations

- <span id="arrayvecdrain-new"></span>`fn new<A, R>(arr: &'a mut ArrayVec<A>, range: R) -> Self` — [`ArrayVec`](../index.md)

#### Trait Implementations

##### `impl<'a, T: 'a + Default> DoubleEndedIterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<'a, T: 'a + Default> ExactSizeIterator for ArrayVecDrain<'a, T>`

##### `impl<'a, T: 'a + Default> FusedIterator for ArrayVecDrain<'a, T>`

##### `impl<I> IntoIterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arrayvecdrain-intoiter"></span>`type IntoIter = I`

- <span id="arrayvecdrain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T: 'a + Default> Iterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-item"></span>`type Item = T`

- <span id="arrayvecdrain-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="arrayvecdrain-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-for-each"></span>`fn for_each<F>(self, f: F)`

