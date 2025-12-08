# Crate `either`

The enum [`Either`](#either) with variants `Left` and `Right` is a general purpose
sum type with two cases.

**Crate features:**

* `"std"`
  Enabled by default. Disable to make the library `#![no_std]`.

* `"serde"`
  Disabled by default. Enable to `#[derive(Serialize, Deserialize)]` for `Either`


## Contents

- [Modules](#modules)
  - [`iterator`](#iterator)
  - [`into_either`](#into_either)
- [Structs](#structs)
  - [`unnamed`](#unnamed)
- [Enums](#enums)
  - [`Either`](#either)
- [Traits](#traits)
  - [`unnamed`](#unnamed)
- [Functions](#functions)
  - [`_unsized_ref_propagation`](#_unsized_ref_propagation)
- [Macros](#macros)
  - [`map_either!`](#map_either)
  - [`impl_specific_ref_and_mut!`](#impl_specific_ref_and_mut)
  - [`check_t!`](#check_t)
  - [`for_both!`](#for_both)
  - [`try_left!`](#try_left)
  - [`try_right!`](#try_right)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`iterator`](#iterator) | mod |  |
| [`into_either`](#into_either) | mod | The trait [`IntoEither`] provides methods for converting a type `Self`, whose |
| [`unnamed`](#unnamed) | struct |  |
| [`Either`](#either) | enum | The enum `Either` with variants `Left` and `Right` is a general purpose |
| [`unnamed`](#unnamed) | trait |  |
| [`_unsized_ref_propagation`](#_unsized_ref_propagation) | fn |  |
| [`map_either!`](#map_either) | macro |  |
| [`impl_specific_ref_and_mut!`](#impl_specific_ref_and_mut) | macro |  |
| [`check_t!`](#check_t) | macro | A helper macro to check if AsRef and AsMut are implemented for a given type. |
| [`for_both!`](#for_both) | macro | Evaluate the provided expression for both [`Either::Left`] and [`Either::Right`]. |
| [`try_left!`](#try_left) | macro | Macro for unwrapping the left side of an [`Either`], which fails early |
| [`try_right!`](#try_right) | macro | Dual to [`try_left!`], see its documentation for more information. |

## Modules

- [`iterator`](iterator/index.md) - 
- [`into_either`](into_either/index.md) - The trait [`IntoEither`] provides methods for converting a type `Self`, whose

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

- <span id="itereither-new"></span>`fn new(inner: Either<L, R>) -> Self` — [`Either`](#either)

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

- <span id="itereither-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itereither-intoiter"></span>`type IntoIter = I`

- <span id="itereither-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<L, R> Iterator for IterEither<L, R>`

- <span id="itereither-item"></span>`type Item = Either<<L as Iterator>::Item, <R as Iterator>::Item>`

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

- <span id="either-factor-none"></span>`fn factor_none(self) -> Option<Either<L, R>>` — [`Either`](#either)

#### Trait Implementations

##### `impl<L, R, Target> AsMut for Either<L, R>`

- <span id="either-as-mut"></span>`fn as_mut(&mut self) -> &mut Target`

##### `impl<L, R, Target> AsRef for Either<L, R>`

- <span id="either-as-ref"></span>`fn as_ref(&self) -> &[Target]`

##### `impl<L: Clone, R: Clone> Clone for Either<L, R>`

- <span id="either-clone"></span>`fn clone(&self) -> Self`

- <span id="either-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<L: marker::Copy, R: marker::Copy> Copy for Either<L, R>`

##### `impl<L: fmt::Debug, R: fmt::Debug> Debug for Either<L, R>`

- <span id="either-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<L, R> Deref for Either<L, R>`

- <span id="either-target"></span>`type Target = <L as Deref>::Target`

- <span id="either-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<L, R> DerefMut for Either<L, R>`

- <span id="either-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<L, R> Display for Either<L, R>`

- <span id="either-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<L, R> DoubleEndedIterator for super::Either<L, R>`

- <span id="supereither-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="supereither-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="supereither-rfold"></span>`fn rfold<Acc, G>(self, init: Acc, f: G) -> Acc`

- <span id="supereither-rfind"></span>`fn rfind<P>(&mut self, predicate: P) -> Option<<Self as >::Item>`

##### `impl<L: cmp::Eq, R: cmp::Eq> Eq for Either<L, R>`

##### `impl<L, R> ExactSizeIterator for super::Either<L, R>`

- <span id="supereither-len"></span>`fn len(&self) -> usize`

##### `impl<L, R, A> Extend for super::Either<L, R>`

- <span id="supereither-extend"></span>`fn extend<T>(&mut self, iter: T)`

##### `impl<L, R> FusedIterator for super::Either<L, R>`

##### `impl<L, R> Future for Either<L, R>`

- <span id="either-output"></span>`type Output = <L as Future>::Output`

- <span id="either-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>`

##### `impl<L: hash::Hash, R: hash::Hash> Hash for Either<L, R>`

- <span id="either-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> IntoEither for Either<L, R>`

##### `impl<F> IntoFuture for Either<L, R>`

- <span id="either-output"></span>`type Output = <F as Future>::Output`

- <span id="either-intofuture"></span>`type IntoFuture = F`

- <span id="either-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<I> IntoIterator for Either<L, R>`

- <span id="either-item"></span>`type Item = <I as Iterator>::Item`

- <span id="either-intoiter"></span>`type IntoIter = I`

- <span id="either-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<L, R> Iterator for super::Either<L, R>`

- <span id="supereither-item"></span>`type Item = <L as Iterator>::Item`

- <span id="supereither-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="supereither-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="supereither-fold"></span>`fn fold<Acc, G>(self, init: Acc, f: G) -> Acc`

- <span id="supereither-for-each"></span>`fn for_each<F>(self, f: F)`

- <span id="supereither-count"></span>`fn count(self) -> usize`

- <span id="supereither-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="supereither-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="supereither-collect"></span>`fn collect<B>(self) -> B`

- <span id="supereither-partition"></span>`fn partition<B, F>(self, f: F) -> (B, B)`

- <span id="supereither-all"></span>`fn all<F>(&mut self, f: F) -> bool`

- <span id="supereither-any"></span>`fn any<F>(&mut self, f: F) -> bool`

- <span id="supereither-find"></span>`fn find<P>(&mut self, predicate: P) -> Option<<Self as >::Item>`

- <span id="supereither-find-map"></span>`fn find_map<B, F>(&mut self, f: F) -> Option<B>`

- <span id="supereither-position"></span>`fn position<P>(&mut self, predicate: P) -> Option<usize>`

##### `impl<L: cmp::Ord, R: cmp::Ord> Ord for Either<L, R>`

- <span id="either-cmp"></span>`fn cmp(&self, other: &Either<L, R>) -> cmp::Ordering` — [`Either`](#either)

##### `impl<L: cmp::PartialEq, R: cmp::PartialEq> PartialEq for Either<L, R>`

- <span id="either-eq"></span>`fn eq(&self, other: &Either<L, R>) -> bool` — [`Either`](#either)

##### `impl<L: cmp::PartialOrd, R: cmp::PartialOrd> PartialOrd for Either<L, R>`

- <span id="either-partial-cmp"></span>`fn partial_cmp(&self, other: &Either<L, R>) -> option::Option<cmp::Ordering>` — [`Either`](#either)

##### `impl<P, T> Receiver for Either<L, R>`

- <span id="either-target"></span>`type Target = T`

##### `impl<L, R> StructuralPartialEq for Either<L, R>`

##### `impl<L, R> Write for Either<L, R>`

- <span id="either-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

- <span id="either-write-char"></span>`fn write_char(&mut self, c: char) -> fmt::Result`

- <span id="either-write-fmt"></span>`fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result`

## Traits

## Functions

### `_unsized_ref_propagation`

```rust
fn _unsized_ref_propagation()
```

## Macros

### `map_either!`

### `impl_specific_ref_and_mut!`

### `check_t!`

A helper macro to check if AsRef and AsMut are implemented for a given type.

### `for_both!`

Evaluate the provided expression for both [`Either::Left`](#eitherleft) and [`Either::Right`](#eitherright).

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

See also [`try_right!`](#try-right) for its dual, which applies the same just to the
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

Dual to [`try_left!`](#try-left), see its documentation for more information.

