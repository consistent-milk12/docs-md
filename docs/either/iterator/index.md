*[either](../index.md) / [iterator](index.md)*

---

# Module `iterator`

## Structs

### `IterEither<L, R>`

```rust
struct IterEither<L, R> {
    inner: super::Either<L, R>,
}
```

Iterator that maps left or right iterators to corresponding `Either`-wrapped items.

This struct is created by the `Either::factor_into_iter`,
`factor_iter`,
and `factor_iter_mut` methods.

#### Implementations

- `fn new(inner: Either<L, R>) -> Self` — [`Either`](../index.md)

#### Trait Implementations

##### `impl<L: $crate::clone::Clone, R: $crate::clone::Clone> Clone for IterEither<L, R>`

- `fn clone(self: &Self) -> IterEither<L, R>` — [`IterEither`](#itereither)

##### `impl<L: $crate::fmt::Debug, R: $crate::fmt::Debug> Debug for IterEither<L, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<L, R> DoubleEndedIterator for IterEither<L, R>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

- `fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

- `fn rfold<Acc, G>(self: Self, init: Acc, f: G) -> Acc`

- `fn rfind<P>(self: &mut Self, predicate: P) -> Option<<Self as >::Item>`

##### `impl<L, R> ExactSizeIterator for IterEither<L, R>`

- `fn len(self: &Self) -> usize`

##### `impl<L, R> FusedIterator for IterEither<L, R>`

##### `impl<T> IntoEither for IterEither<L, R>`

##### `impl<I> IntoIterator for IterEither<L, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<L, R> Iterator for IterEither<L, R>`

- `type Item = Either<<L as Iterator>::Item, <R as Iterator>::Item>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<Acc, G>(self: Self, init: Acc, f: G) -> Acc`

- `fn for_each<F>(self: Self, f: F)`

- `fn count(self: Self) -> usize`

- `fn last(self: Self) -> Option<<Self as >::Item>`

- `fn nth(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

- `fn collect<B>(self: Self) -> B`

- `fn partition<B, F>(self: Self, f: F) -> (B, B)`

- `fn all<F>(self: &mut Self, f: F) -> bool`

- `fn any<F>(self: &mut Self, f: F) -> bool`

- `fn find<P>(self: &mut Self, predicate: P) -> Option<<Self as >::Item>`

- `fn find_map<B, F>(self: &mut Self, f: F) -> Option<B>`

- `fn position<P>(self: &mut Self, predicate: P) -> Option<usize>`

## Macros

### `wrap_either!`

