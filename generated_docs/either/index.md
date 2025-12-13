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
  - [`into_either`](#into-either)
- [Structs](#structs)
  - [`IterEither`](#itereither)
- [Enums](#enums)
  - [`Either`](#either)
- [Traits](#traits)
  - [`IntoEither`](#intoeither)
- [Functions](#functions)
  - [`_unsized_ref_propagation`](#unsized-ref-propagation)
- [Macros](#macros)
  - [`map_either!`](#map-either)
  - [`impl_specific_ref_and_mut!`](#impl-specific-ref-and-mut)
  - [`check_t!`](#check-t)
  - [`for_both!`](#for-both)
  - [`try_left!`](#try-left)
  - [`try_right!`](#try-right)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`iterator`](#iterator) | mod |  |
| [`into_either`](#into-either) | mod | The trait [`IntoEither`] provides methods for converting a type `Self`, whose size is constant and known at compile-time, into an [`Either`] variant. |
| [`IterEither`](#itereither) | struct |  |
| [`Either`](#either) | enum | The enum `Either` with variants `Left` and `Right` is a general purpose sum type with two cases. |
| [`IntoEither`](#intoeither) | trait |  |
| [`_unsized_ref_propagation`](#unsized-ref-propagation) | fn |  |
| [`map_either!`](#map-either) | macro |  |
| [`impl_specific_ref_and_mut!`](#impl-specific-ref-and-mut) | macro |  |
| [`check_t!`](#check-t) | macro | A helper macro to check if AsRef and AsMut are implemented for a given type. |
| [`for_both!`](#for-both) | macro | Evaluate the provided expression for both [`Either::Left`] and [`Either::Right`]. |
| [`try_left!`](#try-left) | macro | Macro for unwrapping the left side of an [`Either`], which fails early with the opposite side. |
| [`try_right!`](#try-right) | macro | Dual to [`try_left!`], see its documentation for more information. |

## Modules

- [`iterator`](iterator/index.md)
- [`into_either`](into_either/index.md) — The trait [`IntoEither`] provides methods for converting a type `Self`, whose

## Structs

### `IterEither<L, R>`

```rust
struct IterEither<L, R> {
    inner: super::Either<L, R>,
}
```

*Defined in [`either-1.15.0/src/iterator.rs:19-21`](../../.source_1765633015/either-1.15.0/src/iterator.rs#L19-L21)*

Iterator that maps left or right iterators to corresponding `Either`-wrapped items.

This struct is created by the `Either::factor_into_iter`,
`factor_iter`,
and `factor_iter_mut` methods.

#### Implementations

- <span id="itereither-new"></span>`fn new(inner: Either<L, R>) -> Self` — [`Either`](#either)

#### Trait Implementations

##### `impl Any for IterEither<L, R>`

- <span id="itereither-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterEither<L, R>`

- <span id="itereither-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterEither<L, R>`

- <span id="itereither-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<L: clone::Clone, R: clone::Clone> Clone for IterEither<L, R>`

- <span id="itereither-clone"></span>`fn clone(&self) -> IterEither<L, R>` — [`IterEither`](iterator/index.md#itereither)

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

## Enums

### `Either<L, R>`

```rust
enum Either<L, R> {
    Left(L),
    Right(R),
}
```

*Defined in [`either-1.15.0/src/lib.rs:49-54`](../../.source_1765633015/either-1.15.0/src/lib.rs#L49-L54)*

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

- <span id="either-is-left"></span>`fn is_left(&self) -> bool`

  Return true if the value is the `Left` variant.

  

  ```rust

  use either::*;

  

  let values = [Left(1), Right("the right value")];

  assert_eq!(values[0].is_left(), true);

  assert_eq!(values[1].is_left(), false);

  ```

- <span id="either-is-right"></span>`fn is_right(&self) -> bool`

  Return true if the value is the `Right` variant.

  

  ```rust

  use either::*;

  

  let values = [Left(1), Right("the right value")];

  assert_eq!(values[0].is_right(), false);

  assert_eq!(values[1].is_right(), true);

  ```

- <span id="either-left"></span>`fn left(self) -> Option<L>`

  Convert the left side of `Either<L, R>` to an `Option<L>`.

  

  ```rust

  use either::*;

  

  let left: Either<_, ()> = Left("some value");

  assert_eq!(left.left(),  Some("some value"));

  

  let right: Either<(), _> = Right(321);

  assert_eq!(right.left(), None);

  ```

- <span id="either-right"></span>`fn right(self) -> Option<R>`

  Convert the right side of `Either<L, R>` to an `Option<R>`.

  

  ```rust

  use either::*;

  

  let left: Either<_, ()> = Left("some value");

  assert_eq!(left.right(),  None);

  

  let right: Either<(), _> = Right(321);

  assert_eq!(right.right(), Some(321));

  ```

- <span id="either-as-ref"></span>`fn as_ref(&self) -> Either<&L, &R>` — [`Either`](#either)

  Convert `&Either<L, R>` to `Either<&L, &R>`.

  

  ```rust

  use either::*;

  

  let left: Either<_, ()> = Left("some value");

  assert_eq!(left.as_ref(), Left(&"some value"));

  

  let right: Either<(), _> = Right("some value");

  assert_eq!(right.as_ref(), Right(&"some value"));

  ```

- <span id="either-as-mut"></span>`fn as_mut(&mut self) -> Either<&mut L, &mut R>` — [`Either`](#either)

  Convert `&mut Either<L, R>` to `Either<&mut L, &mut R>`.

  

  ```rust

  use either::*;

  

  fn mutate_left(value: &mut Either<u32, u32>) {

      if let Some(l) = value.as_mut().left() {

          *l = 999;

      }

  }

  

  let mut left = Left(123);

  let mut right = Right(123);

  mutate_left(&mut left);

  mutate_left(&mut right);

  assert_eq!(left, Left(999));

  assert_eq!(right, Right(123));

  ```

- <span id="either-as-pin-ref"></span>`fn as_pin_ref(self: Pin<&Self>) -> Either<Pin<&L>, Pin<&R>>` — [`Either`](#either)

  Convert `Pin<&Either<L, R>>` to `Either<Pin<&L>, Pin<&R>>`,

  pinned projections of the inner variants.

- <span id="either-as-pin-mut"></span>`fn as_pin_mut(self: Pin<&mut Self>) -> Either<Pin<&mut L>, Pin<&mut R>>` — [`Either`](#either)

  Convert `Pin<&mut Either<L, R>>` to `Either<Pin<&mut L>, Pin<&mut R>>`,

  pinned projections of the inner variants.

- <span id="either-flip"></span>`fn flip(self) -> Either<R, L>` — [`Either`](#either)

  Convert `Either<L, R>` to `Either<R, L>`.

  

  ```rust

  use either::*;

  

  let left: Either<_, ()> = Left(123);

  assert_eq!(left.flip(), Right(123));

  

  let right: Either<(), _> = Right("some value");

  assert_eq!(right.flip(), Left("some value"));

  ```

- <span id="either-map-left"></span>`fn map_left<F, M>(self, f: F) -> Either<M, R>` — [`Either`](#either)

  Apply the function `f` on the value in the `Left` variant if it is present rewrapping the

  result in `Left`.

  

  ```rust

  use either::*;

  

  let left: Either<_, u32> = Left(123);

  assert_eq!(left.map_left(|x| x * 2), Left(246));

  

  let right: Either<u32, _> = Right(123);

  assert_eq!(right.map_left(|x| x * 2), Right(123));

  ```

- <span id="either-map-right"></span>`fn map_right<F, S>(self, f: F) -> Either<L, S>` — [`Either`](#either)

  Apply the function `f` on the value in the `Right` variant if it is present rewrapping the

  result in `Right`.

  

  ```rust

  use either::*;

  

  let left: Either<_, u32> = Left(123);

  assert_eq!(left.map_right(|x| x * 2), Left(123));

  

  let right: Either<u32, _> = Right(123);

  assert_eq!(right.map_right(|x| x * 2), Right(246));

  ```

- <span id="either-map-either"></span>`fn map_either<F, G, M, S>(self, f: F, g: G) -> Either<M, S>` — [`Either`](#either)

  Apply the functions `f` and `g` to the `Left` and `Right` variants

  respectively. This is equivalent to

  [bimap](https://hackage.haskell.org/package/bifunctors-5/docs/Data-Bifunctor.html)

  in functional programming.

  

  ```rust

  use either::*;

  

  let f = |s: String| s.len();

  let g = |u: u8| u.to_string();

  

  let left: Either<String, u8> = Left("loopy".into());

  assert_eq!(left.map_either(f, g), Left(5));

  

  let right: Either<String, u8> = Right(42);

  assert_eq!(right.map_either(f, g), Right("42".into()));

  ```

- <span id="either-map-either-with"></span>`fn map_either_with<Ctx, F, G, M, S>(self, ctx: Ctx, f: F, g: G) -> Either<M, S>` — [`Either`](#either)

  Similar to `map_either`, with an added context `ctx` accessible to

  both functions.

  

  ```rust

  use either::*;

  

  let mut sum = 0;

  

  // Both closures want to update the same value, so pass it as context.

  let mut f = |sum: &mut usize, s: String| { *sum += s.len(); s.to_uppercase() };

  let mut g = |sum: &mut usize, u: usize| { *sum += u; u.to_string() };

  

  let left: Either<String, usize> = Left("loopy".into());

  assert_eq!(left.map_either_with(&mut sum, &mut f, &mut g), Left("LOOPY".into()));

  

  let right: Either<String, usize> = Right(42);

  assert_eq!(right.map_either_with(&mut sum, &mut f, &mut g), Right("42".into()));

  

  assert_eq!(sum, 47);

  ```

- <span id="either-either"></span>`fn either<F, G, T>(self, f: F, g: G) -> T`

  Apply one of two functions depending on contents, unifying their result. If the value is

  `Left(L)` then the first function `f` is applied; if it is `Right(R)` then the second

  function `g` is applied.

  

  ```rust

  use either::*;

  

  fn square(n: u32) -> i32 { (n * n) as i32 }

  fn negate(n: i32) -> i32 { -n }

  

  let left: Either<u32, i32> = Left(4);

  assert_eq!(left.either(square, negate), 16);

  

  let right: Either<u32, i32> = Right(-4);

  assert_eq!(right.either(square, negate), 4);

  ```

- <span id="either-either-with"></span>`fn either_with<Ctx, F, G, T>(self, ctx: Ctx, f: F, g: G) -> T`

  Like `either`, but provide some context to whichever of the

  functions ends up being called.

  

  ```rust

  // In this example, the context is a mutable reference

  use either::*;

  

  let mut result = Vec::new();

  

  let values = vec![Left(2), Right(2.7)];

  

  for value in values {

      value.either_with(&mut result,

                        |ctx, integer| ctx.push(integer),

                        |ctx, real| ctx.push(f64::round(real) as i32));

  }

  

  assert_eq!(result, vec![2, 3]);

  ```

- <span id="either-left-and-then"></span>`fn left_and_then<F, S>(self, f: F) -> Either<S, R>` — [`Either`](#either)

  Apply the function `f` on the value in the `Left` variant if it is present.

  

  ```rust

  use either::*;

  

  let left: Either<_, u32> = Left(123);

  assert_eq!(left.left_and_then::<_,()>(|x| Right(x * 2)), Right(246));

  

  let right: Either<u32, _> = Right(123);

  assert_eq!(right.left_and_then(|x| Right::<(), _>(x * 2)), Right(123));

  ```

- <span id="either-right-and-then"></span>`fn right_and_then<F, S>(self, f: F) -> Either<L, S>` — [`Either`](#either)

  Apply the function `f` on the value in the `Right` variant if it is present.

  

  ```rust

  use either::*;

  

  let left: Either<_, u32> = Left(123);

  assert_eq!(left.right_and_then(|x| Right(x * 2)), Left(123));

  

  let right: Either<u32, _> = Right(123);

  assert_eq!(right.right_and_then(|x| Right(x * 2)), Right(246));

  ```

- <span id="either-into-iter"></span>`fn into_iter(self) -> Either<<L as >::IntoIter, <R as >::IntoIter>` — [`Either`](#either)

  Convert the inner value to an iterator.

  

  This requires the `Left` and `Right` iterators to have the same item type.

  See `factor_into_iter` to iterate different types.

  

  ```rust

  use either::*;

  

  let left: Either<_, Vec<u32>> = Left(vec![1, 2, 3, 4, 5]);

  let mut right: Either<Vec<u32>, _> = Right(vec![]);

  right.extend(left.into_iter());

  assert_eq!(right, Right(vec![1, 2, 3, 4, 5]));

  ```

- <span id="either-iter"></span>`fn iter(&self) -> Either<<&L as IntoIterator>::IntoIter, <&R as IntoIterator>::IntoIter>` — [`Either`](#either)

  Borrow the inner value as an iterator.

  

  This requires the `Left` and `Right` iterators to have the same item type.

  See `factor_iter` to iterate different types.

  

  ```rust

  use either::*;

  

  let left: Either<_, &[u32]> = Left(vec![2, 3]);

  let mut right: Either<Vec<u32>, _> = Right(&[4, 5][..]);

  let mut all = vec![1];

  all.extend(left.iter());

  all.extend(right.iter());

  assert_eq!(all, vec![1, 2, 3, 4, 5]);

  ```

- <span id="either-iter-mut"></span>`fn iter_mut(&mut self) -> Either<<&mut L as IntoIterator>::IntoIter, <&mut R as IntoIterator>::IntoIter>` — [`Either`](#either)

  Mutably borrow the inner value as an iterator.

  

  This requires the `Left` and `Right` iterators to have the same item type.

  See `factor_iter_mut` to iterate different types.

  

  ```rust

  use either::*;

  

  let mut left: Either<_, &mut [u32]> = Left(vec![2, 3]);

  for l in left.iter_mut() {

      *l *= *l

  }

  assert_eq!(left, Left(vec![4, 9]));

  

  let mut inner = [4, 5];

  let mut right: Either<Vec<u32>, _> = Right(&mut inner[..]);

  for r in right.iter_mut() {

      *r *= *r

  }

  assert_eq!(inner, [16, 25]);

  ```

- <span id="either-factor-into-iter"></span>`fn factor_into_iter(self) -> IterEither<<L as >::IntoIter, <R as >::IntoIter>` — [`IterEither`](iterator/index.md#itereither)

  Converts an `Either` of `Iterator`s to be an `Iterator` of `Either`s

  

  Unlike `into_iter`, this does not require the

  `Left` and `Right` iterators to have the same item type.

  

  ```rust

  use either::*;

  let left: Either<_, Vec<u8>> = Left(&["hello"]);

  assert_eq!(left.factor_into_iter().next(), Some(Left(&"hello")));

  

  let right: Either<&[&str], _> = Right(vec![0, 1]);

  assert_eq!(right.factor_into_iter().collect::<Vec<_>>(), vec![Right(0), Right(1)]);

  

  ```

- <span id="either-factor-iter"></span>`fn factor_iter(&self) -> IterEither<<&L as IntoIterator>::IntoIter, <&R as IntoIterator>::IntoIter>` — [`IterEither`](iterator/index.md#itereither)

  Borrows an `Either` of `Iterator`s to be an `Iterator` of `Either`s

  

  Unlike `iter`, this does not require the

  `Left` and `Right` iterators to have the same item type.

  

  ```rust

  use either::*;

  let left: Either<_, Vec<u8>> = Left(["hello"]);

  assert_eq!(left.factor_iter().next(), Some(Left(&"hello")));

  

  let right: Either<[&str; 2], _> = Right(vec![0, 1]);

  assert_eq!(right.factor_iter().collect::<Vec<_>>(), vec![Right(&0), Right(&1)]);

  

  ```

- <span id="either-factor-iter-mut"></span>`fn factor_iter_mut(&mut self) -> IterEither<<&mut L as IntoIterator>::IntoIter, <&mut R as IntoIterator>::IntoIter>` — [`IterEither`](iterator/index.md#itereither)

  Mutably borrows an `Either` of `Iterator`s to be an `Iterator` of `Either`s

  

  Unlike `iter_mut`, this does not require the

  `Left` and `Right` iterators to have the same item type.

  

  ```rust

  use either::*;

  let mut left: Either<_, Vec<u8>> = Left(["hello"]);

  left.factor_iter_mut().for_each(|x| *x.unwrap_left() = "goodbye");

  assert_eq!(left, Left(["goodbye"]));

  

  let mut right: Either<[&str; 2], _> = Right(vec![0, 1, 2]);

  right.factor_iter_mut().for_each(|x| if let Right(r) = x { *r = -*r; });

  assert_eq!(right, Right(vec![0, -1, -2]));

  

  ```

- <span id="either-left-or"></span>`fn left_or(self, other: L) -> L`

  Return left value or given value

  

  Arguments passed to `left_or` are eagerly evaluated; if you are passing

  the result of a function call, it is recommended to use

  `left_or_else`, which is lazily evaluated.

  

  # Examples

  

  ```rust

  use either::*;

  let left: Either<&str, &str> = Left("left");

  assert_eq!(left.left_or("foo"), "left");

  

  let right: Either<&str, &str> = Right("right");

  assert_eq!(right.left_or("left"), "left");

  ```

- <span id="either-left-or-default"></span>`fn left_or_default(self) -> L`

  Return left or a default

  

  # Examples

  

  ```rust

  use either::*;

  let left: Either<String, u32> = Left("left".to_string());

  assert_eq!(left.left_or_default(), "left");

  

  let right: Either<String, u32> = Right(42);

  assert_eq!(right.left_or_default(), String::default());

  ```

- <span id="either-left-or-else"></span>`fn left_or_else<F>(self, f: F) -> L`

  Returns left value or computes it from a closure

  

  # Examples

  

  ```rust

  use either::*;

  let left: Either<String, u32> = Left("3".to_string());

  assert_eq!(left.left_or_else(|_| unreachable!()), "3");

  

  let right: Either<String, u32> = Right(3);

  assert_eq!(right.left_or_else(|x| x.to_string()), "3");

  ```

- <span id="either-right-or"></span>`fn right_or(self, other: R) -> R`

  Return right value or given value

  

  Arguments passed to `right_or` are eagerly evaluated; if you are passing

  the result of a function call, it is recommended to use

  `right_or_else`, which is lazily evaluated.

  

  # Examples

  

  ```rust

  use either::*;

  let right: Either<&str, &str> = Right("right");

  assert_eq!(right.right_or("foo"), "right");

  

  let left: Either<&str, &str> = Left("left");

  assert_eq!(left.right_or("right"), "right");

  ```

- <span id="either-right-or-default"></span>`fn right_or_default(self) -> R`

  Return right or a default

  

  # Examples

  

  ```rust

  use either::*;

  let left: Either<String, u32> = Left("left".to_string());

  assert_eq!(left.right_or_default(), u32::default());

  

  let right: Either<String, u32> = Right(42);

  assert_eq!(right.right_or_default(), 42);

  ```

- <span id="either-right-or-else"></span>`fn right_or_else<F>(self, f: F) -> R`

  Returns right value or computes it from a closure

  

  # Examples

  

  ```rust

  use either::*;

  let left: Either<String, u32> = Left("3".to_string());

  assert_eq!(left.right_or_else(|x| x.parse().unwrap()), 3);

  

  let right: Either<String, u32> = Right(3);

  assert_eq!(right.right_or_else(|_| unreachable!()), 3);

  ```

- <span id="either-unwrap-left"></span>`fn unwrap_left(self) -> L`

  Returns the left value

  

  # Examples

  

  ```rust

  use either::*;

  let left: Either<_, ()> = Left(3);

  assert_eq!(left.unwrap_left(), 3);

  ```

  

  # Panics

  

  When `Either` is a `Right` value

  

  ```should_panic

  use either::*;

  let right: Either<(), _> = Right(3);

  right.unwrap_left();

  ```

- <span id="either-unwrap-right"></span>`fn unwrap_right(self) -> R`

  Returns the right value

  

  # Examples

  

  ```rust

  use either::*;

  let right: Either<(), _> = Right(3);

  assert_eq!(right.unwrap_right(), 3);

  ```

  

  # Panics

  

  When `Either` is a `Left` value

  

  ```should_panic

  use either::*;

  let left: Either<_, ()> = Left(3);

  left.unwrap_right();

  ```

- <span id="either-expect-left"></span>`fn expect_left(self, msg: &str) -> L`

  Returns the left value

  

  # Examples

  

  ```rust

  use either::*;

  let left: Either<_, ()> = Left(3);

  assert_eq!(left.expect_left("value was Right"), 3);

  ```

  

  # Panics

  

  When `Either` is a `Right` value

  

  ```should_panic

  use either::*;

  let right: Either<(), _> = Right(3);

  right.expect_left("value was Right");

  ```

- <span id="either-expect-right"></span>`fn expect_right(self, msg: &str) -> R`

  Returns the right value

  

  # Examples

  

  ```rust

  use either::*;

  let right: Either<(), _> = Right(3);

  assert_eq!(right.expect_right("value was Left"), 3);

  ```

  

  # Panics

  

  When `Either` is a `Left` value

  

  ```should_panic

  use either::*;

  let left: Either<_, ()> = Left(3);

  left.expect_right("value was Right");

  ```

- <span id="either-either-into"></span>`fn either_into<T>(self) -> T`

  Convert the contained value into `T`

  

  # Examples

  

  ```rust

  use either::*;

  // Both u16 and u32 can be converted to u64.

  let left: Either<u16, u32> = Left(3u16);

  assert_eq!(left.either_into::<u64>(), 3u64);

  let right: Either<u16, u32> = Right(7u32);

  assert_eq!(right.either_into::<u64>(), 7u64);

  ```

#### Trait Implementations

##### `impl Any for Either<L, R>`

- <span id="either-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<L, R> AsMut for Either<L, R>`

- <span id="either-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut str`

##### `impl<L, R, Target> AsRef for Either<L, R>`

- <span id="either-asref-as-ref"></span>`fn as_ref(&self) -> &Target`

##### `impl<T> Borrow for Either<L, R>`

- <span id="either-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Either<L, R>`

- <span id="either-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<L: Clone, R: Clone> Clone for Either<L, R>`

- <span id="either-clone"></span>`fn clone(&self) -> Self`

- <span id="either-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl CloneToUninit for Either<L, R>`

- <span id="either-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<L: marker::Copy, R: marker::Copy> Copy for Either<L, R>`

##### `impl<L: fmt::Debug, R: fmt::Debug> Debug for Either<L, R>`

- <span id="either-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<L, R> Deref for Either<L, R>`

- <span id="either-deref-type-target"></span>`type Target = <L as Deref>::Target`

- <span id="either-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<L, R> DerefMut for Either<L, R>`

- <span id="either-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<L, R> Display for Either<L, R>`

- <span id="either-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<L, R> DoubleEndedIterator for super::Either<L, R>`

- <span id="supereither-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="supereither-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="supereither-doubleendediterator-rfold"></span>`fn rfold<Acc, G>(self, init: Acc, f: G) -> Acc`

- <span id="supereither-doubleendediterator-rfind"></span>`fn rfind<P>(&mut self, predicate: P) -> Option<<Self as >::Item>`

##### `impl<L: cmp::Eq, R: cmp::Eq> Eq for Either<L, R>`

##### `impl<L, R> ExactSizeIterator for super::Either<L, R>`

- <span id="supereither-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<L, R, A> Extend for super::Either<L, R>`

- <span id="supereither-extend"></span>`fn extend<T>(&mut self, iter: T)`

##### `impl<T> From for Either<L, R>`

- <span id="either-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<L, R> FusedIterator for super::Either<L, R>`

##### `impl<L, R> Future for Either<L, R>`

- <span id="either-future-type-output"></span>`type Output = <L as Future>::Output`

- <span id="either-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>`

##### `impl<L: hash::Hash, R: hash::Hash> Hash for Either<L, R>`

- <span id="either-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Either<L, R>`

- <span id="either-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Either<L, R>`

##### `impl IntoFuture for Either<L, R>`

- <span id="either-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="either-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="either-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl IntoIterator for Either<L, R>`

- <span id="either-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="either-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="either-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<L, R> Iterator for super::Either<L, R>`

- <span id="supereither-iterator-type-item"></span>`type Item = <L as Iterator>::Item`

- <span id="supereither-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="supereither-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="supereither-iterator-fold"></span>`fn fold<Acc, G>(self, init: Acc, f: G) -> Acc`

- <span id="supereither-iterator-for-each"></span>`fn for_each<F>(self, f: F)`

- <span id="supereither-iterator-count"></span>`fn count(self) -> usize`

- <span id="supereither-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="supereither-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="supereither-iterator-collect"></span>`fn collect<B>(self) -> B`

- <span id="supereither-iterator-partition"></span>`fn partition<B, F>(self, f: F) -> (B, B)`

- <span id="supereither-iterator-all"></span>`fn all<F>(&mut self, f: F) -> bool`

- <span id="supereither-iterator-any"></span>`fn any<F>(&mut self, f: F) -> bool`

- <span id="supereither-iterator-find"></span>`fn find<P>(&mut self, predicate: P) -> Option<<Self as >::Item>`

- <span id="supereither-iterator-find-map"></span>`fn find_map<B, F>(&mut self, f: F) -> Option<B>`

- <span id="supereither-iterator-position"></span>`fn position<P>(&mut self, predicate: P) -> Option<usize>`

##### `impl<L: cmp::Ord, R: cmp::Ord> Ord for Either<L, R>`

- <span id="either-ord-cmp"></span>`fn cmp(&self, other: &Either<L, R>) -> cmp::Ordering` — [`Either`](#either)

##### `impl<L: cmp::PartialEq, R: cmp::PartialEq> PartialEq for Either<L, R>`

- <span id="either-partialeq-eq"></span>`fn eq(&self, other: &Either<L, R>) -> bool` — [`Either`](#either)

##### `impl<L: cmp::PartialOrd, R: cmp::PartialOrd> PartialOrd for Either<L, R>`

- <span id="either-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Either<L, R>) -> option::Option<cmp::Ordering>` — [`Either`](#either)

##### `impl Receiver for Either<L, R>`

- <span id="either-receiver-type-target"></span>`type Target = T`

##### `impl<L, R> StructuralPartialEq for Either<L, R>`

##### `impl<U> TryFrom for Either<L, R>`

- <span id="either-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="either-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Either<L, R>`

- <span id="either-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="either-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<L, R> Write for Either<L, R>`

- <span id="either-write-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

- <span id="either-write-write-char"></span>`fn write_char(&mut self, c: char) -> fmt::Result`

- <span id="either-write-write-fmt"></span>`fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result`

## Traits

### `IntoEither`

```rust
trait IntoEither: Sized { ... }
```

*Defined in [`either-1.15.0/src/into_either.rs:14-62`](../../.source_1765633015/either-1.15.0/src/into_either.rs#L14-L62)*

Provides methods for converting a type `Self` into either a [`Left`](#left) or [`Right`](#right)
variant of [`Either<Self, Self>`](Either).

The [`into_either`](IntoEither::into_either) method takes a `bool` to determine
whether to convert to [`Left`](#left) or [`Right`](#right).

The [`into_either_with`](IntoEither::into_either_with) method takes a
[predicate function](FnOnce) to determine whether to convert to [`Left`](#left) or [`Right`](#right).

#### Provided Methods

- `fn into_either(self, into_left: bool) -> Either<Self, Self>`

  Converts `self` into a [`Left`](#left) variant of [`Either<Self, Self>`](Either)

- `fn into_either_with<F>(self, into_left: F) -> Either<Self, Self>`

  Converts `self` into a [`Left`](#left) variant of [`Either<Self, Self>`](Either)

#### Implementors

- `T`

## Functions

### `_unsized_ref_propagation`

```rust
fn _unsized_ref_propagation()
```

*Defined in [`either-1.15.0/src/lib.rs:1540-1553`](../../.source_1765633015/either-1.15.0/src/lib.rs#L1540-L1553)*

## Macros

### `map_either!`

*Defined in [`either-1.15.0/src/lib.rs:133-140`](../../.source_1765633015/either-1.15.0/src/lib.rs#L133-L140)*

### `impl_specific_ref_and_mut!`

*Defined in [`either-1.15.0/src/lib.rs:1257-1277`](../../.source_1765633015/either-1.15.0/src/lib.rs#L1257-L1277)*

### `check_t!`

*Defined in [`either-1.15.0/src/lib.rs:1526-1537`](../../.source_1765633015/either-1.15.0/src/lib.rs#L1526-L1537)*

A helper macro to check if AsRef and AsMut are implemented for a given type.

### `for_both!`

*Defined in [`either-1.15.0/src/lib.rs:81-88`](../../.source_1765633015/either-1.15.0/src/lib.rs#L81-L88)*

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

*Defined in [`either-1.15.0/src/lib.rs:113-120`](../../.source_1765633015/either-1.15.0/src/lib.rs#L113-L120)*

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

*Defined in [`either-1.15.0/src/lib.rs:124-131`](../../.source_1765633015/either-1.15.0/src/lib.rs#L124-L131)*

Dual to [`try_left!`](#try-left), see its documentation for more information.

