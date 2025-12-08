*[tinyvec](../index.md) / [arrayvec_drain](index.md)*

---

# Module `arrayvec_drain`

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

- `fn new<A, R>(arr: &'a mut ArrayVec<A>, range: R) -> Self` — [`ArrayVec`](../index.md)

#### Trait Implementations

##### `impl<'a, T: 'a + Default> DoubleEndedIterator for ArrayVecDrain<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

- `fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

##### `impl<'a, T: 'a + Default> ExactSizeIterator for ArrayVecDrain<'a, T>`

##### `impl<'a, T: 'a + Default> FusedIterator for ArrayVecDrain<'a, T>`

##### `impl<I> IntoIterator for ArrayVecDrain<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T: 'a + Default> Iterator for ArrayVecDrain<'a, T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn nth(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

- `fn last(self: Self) -> Option<<Self as >::Item>`

- `fn for_each<F>(self: Self, f: F)`

