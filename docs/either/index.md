# Crate `either`

The enum [`Either`](#either) with variants `Left` and `Right` is a general purpose
sum type with two cases.

**Crate features:**

* `"std"`
  Enabled by default. Disable to make the library `#![no_std]`.

* `"serde"`
  Disabled by default. Enable to `#[derive(Serialize, Deserialize)]` for `Either`


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

- `fn new(inner: Either<L, R>) -> Self` — [`Either`](#either)

#### Trait Implementations

##### `impl<L: $crate::clone::Clone, R: $crate::clone::Clone> Clone for IterEither<L, R>`

- `fn clone(self: &Self) -> IterEither<L, R>` — [`IterEither`](iterator/index.md)

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

## Enums

### `Either<L, R>`

```rust
enum Either<L, R> {
    Left(L),
    Right(R),
}
```

The enum `Either` with variants `Left` and `Right` is a general purpose
sum type with two cases.

The `Either` type is symmetric and treats its variants the same way, without
preference.
(For representing success or error, use the regular `Result` enum instead.)

#### Variants

- **`Left`**

  A value of type `L`.

- **`Right`**

  A value of type `R`.

#### Implementations

- `fn factor_err(self: Self) -> Result<Either<L, R>, E>` — [`Either`](#either)

#### Trait Implementations

##### `impl<L, R, Target> AsMut for Either<L, R>`

- `fn as_mut(self: &mut Self) -> &mut Target`

##### `impl<L, R, Target> AsRef for Either<L, R>`

- `fn as_ref(self: &Self) -> &[Target]`

##### `impl<L: Clone, R: Clone> Clone for Either<L, R>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl<L: $crate::marker::Copy, R: $crate::marker::Copy> Copy for Either<L, R>`

##### `impl<L: $crate::fmt::Debug, R: $crate::fmt::Debug> Debug for Either<L, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<L, R> Deref for Either<L, R>`

- `type Target = <L as Deref>::Target`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<L, R> DerefMut for Either<L, R>`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl<L, R> Display for Either<L, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<L, R> DoubleEndedIterator for super::Either<L, R>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

- `fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

- `fn rfold<Acc, G>(self: Self, init: Acc, f: G) -> Acc`

- `fn rfind<P>(self: &mut Self, predicate: P) -> Option<<Self as >::Item>`

##### `impl<L: $crate::cmp::Eq, R: $crate::cmp::Eq> Eq for Either<L, R>`

##### `impl<L, R> ExactSizeIterator for super::Either<L, R>`

- `fn len(self: &Self) -> usize`

##### `impl<L, R, A> Extend for super::Either<L, R>`

- `fn extend<T>(self: &mut Self, iter: T)`

##### `impl<L, R> FusedIterator for super::Either<L, R>`

##### `impl<L, R> Future for Either<L, R>`

- `type Output = <L as Future>::Output`

- `fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>`

##### `impl<L: $crate::hash::Hash, R: $crate::hash::Hash> Hash for Either<L, R>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T> IntoEither for Either<L, R>`

##### `impl<F> IntoFuture for Either<L, R>`

- `type Output = <F as Future>::Output`

- `type IntoFuture = F`

- `fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture`

##### `impl<I> IntoIterator for Either<L, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<L, R> Iterator for super::Either<L, R>`

- `type Item = <L as Iterator>::Item`

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

##### `impl<L: $crate::cmp::Ord, R: $crate::cmp::Ord> Ord for Either<L, R>`

- `fn cmp(self: &Self, other: &Either<L, R>) -> $crate::cmp::Ordering` — [`Either`](#either)

##### `impl<L: $crate::cmp::PartialEq, R: $crate::cmp::PartialEq> PartialEq for Either<L, R>`

- `fn eq(self: &Self, other: &Either<L, R>) -> bool` — [`Either`](#either)

##### `impl<L: $crate::cmp::PartialOrd, R: $crate::cmp::PartialOrd> PartialOrd for Either<L, R>`

- `fn partial_cmp(self: &Self, other: &Either<L, R>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Either`](#either)

##### `impl<P, T> Receiver for Either<L, R>`

- `type Target = T`

##### `impl<L, R> StructuralPartialEq for Either<L, R>`

##### `impl<L, R> Write for Either<L, R>`

- `fn write_str(self: &mut Self, s: &str) -> fmt::Result`

- `fn write_char(self: &mut Self, c: char) -> fmt::Result`

- `fn write_fmt(self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result`

## Traits

## Macros

### `for_both!`

Evaluate the provided expression for both `Either::Left` and `Either::Right`.

This macro is useful in cases where both sides of [`Either`](#either) can be interacted with
in the same way even though the don't share the same type.

Syntax: `either::for_both!(` *expression* `,` *pattern* `=>` *expression* `)`

# Example

```rust
use either::Either;

fn length(owned_or_borrowed: Either<String, &'static str>) -> usize {
    either::for_both!(owned_or_borrowed, s => s.len())
}

fn main() {
    let borrowed = Either::Right("Hello world!");
    let owned = Either::Left("Hello world!".to_owned());

    assert_eq!(length(borrowed), 12);
    assert_eq!(length(owned), 12);
}
```

### `try_left!`

Macro for unwrapping the left side of an [`Either`](#either), which fails early
with the opposite side. Can only be used in functions that return
`Either` because of the early return of `Right` that it provides.

See also `try_right!` for its dual, which applies the same just to the
right side.

# Example

```rust
use either::{Either, Left, Right};

fn twice(wrapper: Either<u32, &str>) -> Either<u32, &str> {
    let value = either::try_left!(wrapper);
    Left(value * 2)
}

fn main() {
    assert_eq!(twice(Left(2)), Left(4));
    assert_eq!(twice(Right("ups")), Right("ups"));
}
```

### `try_right!`

Dual to `try_left!`, see its documentation for more information.

