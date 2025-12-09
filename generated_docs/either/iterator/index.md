*[either](../index.md) / [iterator](index.md)*

---

# Module `iterator`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IterEither`](#itereither) | struct | Iterator that maps left or right iterators to corresponding `Either`-wrapped items. |
| [`wrap_either!`](#wrap_either) | macro |  |

## Structs

### `IterEither<L, R>`

```rust
struct IterEither<L, R> {
    inner: super::Either<L, R>,
}
```

*Defined in [`either-1.15.0/src/iterator.rs:19-21`](../../../.source_1765210505/either-1.15.0/src/iterator.rs#L19-L21)*

Iterator that maps left or right iterators to corresponding `Either`-wrapped items.

This struct is created by the `Either::factor_into_iter`,
`factor_iter`,
and `factor_iter_mut` methods.

#### Implementations

- <span id="itereither-new"></span>`fn new(inner: Either<L, R>) -> Self` — [`Either`](../index.md)

#### Trait Implementations

##### `impl<L: clone::Clone, R: clone::Clone> Clone for IterEither<L, R>`

- <span id="itereither-clone"></span>`fn clone(&self) -> IterEither<L, R>` — [`IterEither`](#itereither)

##### `impl<L: fmt::Debug, R: fmt::Debug> Debug for IterEither<L, R>`

- <span id="itereither-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<L, R> DoubleEndedIterator for IterEither<L, R>`

- <span id="itereither-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="itereither-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="itereither-rfold"></span>`fn rfold<Acc, G>(self, init: Acc, f: G) -> Acc`

- <span id="itereither-rfind"></span>`fn rfind<P>(&mut self, predicate: P) -> Option<<Self as >::Item>`

##### `impl<L, R> ExactSizeIterator for IterEither<L, R>`

- <span id="itereither-len"></span>`fn len(&self) -> usize`

##### `impl<L, R> FusedIterator for IterEither<L, R>`

##### `impl<T> IntoEither for IterEither<L, R>`

##### `impl<I> IntoIterator for IterEither<L, R>`

- <span id="itereither-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itereither-type-intoiter"></span>`type IntoIter = I`

- <span id="itereither-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<L, R> Iterator for IterEither<L, R>`

- <span id="itereither-type-item"></span>`type Item = Either<<L as Iterator>::Item, <R as Iterator>::Item>`

- <span id="itereither-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itereither-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="itereither-fold"></span>`fn fold<Acc, G>(self, init: Acc, f: G) -> Acc`

- <span id="itereither-for-each"></span>`fn for_each<F>(self, f: F)`

- <span id="itereither-count"></span>`fn count(self) -> usize`

- <span id="itereither-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="itereither-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="itereither-collect"></span>`fn collect<B>(self) -> B`

- <span id="itereither-partition"></span>`fn partition<B, F>(self, f: F) -> (B, B)`

- <span id="itereither-all"></span>`fn all<F>(&mut self, f: F) -> bool`

- <span id="itereither-any"></span>`fn any<F>(&mut self, f: F) -> bool`

- <span id="itereither-find"></span>`fn find<P>(&mut self, predicate: P) -> Option<<Self as >::Item>`

- <span id="itereither-find-map"></span>`fn find_map<B, F>(&mut self, f: F) -> Option<B>`

- <span id="itereither-position"></span>`fn position<P>(&mut self, predicate: P) -> Option<usize>`

## Macros

### `wrap_either!`

*Defined in [`either-1.15.0/src/iterator.rs:4-11`](../../../.source_1765210505/either-1.15.0/src/iterator.rs#L4-L11)*

