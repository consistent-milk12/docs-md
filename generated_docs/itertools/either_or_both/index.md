*[itertools](../index.md) / [either_or_both](index.md)*

---

# Module `either_or_both`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EitherOrBoth`](#eitherorboth) | enum | Value that either holds a single A or B, or both. |

## Enums

### `EitherOrBoth<A, B>`

```rust
enum EitherOrBoth<A, B> {
    Both(A, B),
    Left(A),
    Right(B),
}
```

*Defined in [`itertools-0.14.0/src/either_or_both.rs:9-16`](../../../.source_1765894658/itertools-0.14.0/src/either_or_both.rs#L9-L16)*

Value that either holds a single A or B, or both.

#### Variants

- **`Both`**

  Both values are present.

- **`Left`**

  Only the left value of type `A` is present.

- **`Right`**

  Only the right value of type `B` is present.

#### Implementations

- <span id="eitherorboth-has-left"></span>`fn has_left(&self) -> bool`

  If `Left`, or `Both`, return true. Otherwise, return false.

- <span id="eitherorboth-has-right"></span>`fn has_right(&self) -> bool`

  If `Right`, or `Both`, return true, otherwise, return false.

- <span id="eitherorboth-is-left"></span>`fn is_left(&self) -> bool`

  If `Left`, return true. Otherwise, return false.
  Exclusive version of [`has_left`](EitherOrBoth::has_left).

- <span id="eitherorboth-is-right"></span>`fn is_right(&self) -> bool`

  If `Right`, return true. Otherwise, return false.
  Exclusive version of [`has_right`](EitherOrBoth::has_right).

- <span id="eitherorboth-is-both"></span>`fn is_both(&self) -> bool`

  If `Both`, return true. Otherwise, return false.

- <span id="eitherorboth-left"></span>`fn left(self) -> Option<A>`

  If `Left`, or `Both`, return `Some` with the left value. Otherwise, return `None`.

- <span id="eitherorboth-right"></span>`fn right(self) -> Option<B>`

  If `Right`, or `Both`, return `Some` with the right value. Otherwise, return `None`.

- <span id="eitherorboth-left-and-right"></span>`fn left_and_right(self) -> (Option<A>, Option<B>)`

  Return tuple of options corresponding to the left and right value respectively
  
  If `Left` return `(Some(..), None)`, if `Right` return `(None,Some(..))`, else return
  `(Some(..),Some(..))`

- <span id="eitherorboth-just-left"></span>`fn just_left(self) -> Option<A>`

  If `Left`, return `Some` with the left value. If `Right` or `Both`, return `None`.
  
  # Examples
  
  ```rust
  // On the `Left` variant.
  use itertools::{EitherOrBoth, EitherOrBoth::{Left, Right, Both}};
  let x: EitherOrBoth<_, ()> = Left("bonjour");
  assert_eq!(x.just_left(), Some("bonjour"));
  
  // On the `Right` variant.
  let x: EitherOrBoth<(), _> = Right("hola");
  assert_eq!(x.just_left(), None);
  
  // On the `Both` variant.
  let x = Both("bonjour", "hola");
  assert_eq!(x.just_left(), None);
  ```

- <span id="eitherorboth-just-right"></span>`fn just_right(self) -> Option<B>`

  If `Right`, return `Some` with the right value. If `Left` or `Both`, return `None`.
  
  # Examples
  
  ```rust
  // On the `Left` variant.
  use itertools::{EitherOrBoth::{Left, Right, Both}, EitherOrBoth};
  let x: EitherOrBoth<_, ()> = Left("auf wiedersehen");
  assert_eq!(x.just_left(), Some("auf wiedersehen"));
  
  // On the `Right` variant.
  let x: EitherOrBoth<(), _> = Right("adios");
  assert_eq!(x.just_left(), None);
  
  // On the `Both` variant.
  let x = Both("auf wiedersehen", "adios");
  assert_eq!(x.just_left(), None);
  ```

- <span id="eitherorboth-both"></span>`fn both(self) -> Option<(A, B)>`

  If `Both`, return `Some` containing the left and right values. Otherwise, return `None`.

- <span id="eitherorboth-into-left"></span>`fn into_left(self) -> A`

  If `Left` or `Both`, return the left value. Otherwise, convert the right value and return it.

- <span id="eitherorboth-into-right"></span>`fn into_right(self) -> B`

  If `Right` or `Both`, return the right value. Otherwise, convert the left value and return it.

- <span id="eitherorboth-as-ref"></span>`fn as_ref(&self) -> EitherOrBoth<&A, &B>` — [`EitherOrBoth`](#eitherorboth)

  Converts from `&EitherOrBoth<A, B>` to `EitherOrBoth<&A, &B>`.

- <span id="eitherorboth-as-mut"></span>`fn as_mut(&mut self) -> EitherOrBoth<&mut A, &mut B>` — [`EitherOrBoth`](#eitherorboth)

  Converts from `&mut EitherOrBoth<A, B>` to `EitherOrBoth<&mut A, &mut B>`.

- <span id="eitherorboth-as-deref"></span>`fn as_deref(&self) -> EitherOrBoth<&<A as >::Target, &<B as >::Target>` — [`EitherOrBoth`](#eitherorboth)

  Converts from `&EitherOrBoth<A, B>` to `EitherOrBoth<&_, &_>` using the `Deref` trait.

- <span id="eitherorboth-as-deref-mut"></span>`fn as_deref_mut(&mut self) -> EitherOrBoth<&mut <A as >::Target, &mut <B as >::Target>` — [`EitherOrBoth`](#eitherorboth)

  Converts from `&mut EitherOrBoth<A, B>` to `EitherOrBoth<&mut _, &mut _>` using the `DerefMut` trait.

- <span id="eitherorboth-flip"></span>`fn flip(self) -> EitherOrBoth<B, A>` — [`EitherOrBoth`](#eitherorboth)

  Convert `EitherOrBoth<A, B>` to `EitherOrBoth<B, A>`.

- <span id="eitherorboth-map-left"></span>`fn map_left<F, M>(self, f: F) -> EitherOrBoth<M, B>` — [`EitherOrBoth`](#eitherorboth)

  Apply the function `f` on the value `a` in `Left(a)` or `Both(a, b)` variants. If it is
  present rewrapping the result in `self`'s original variant.

- <span id="eitherorboth-map-right"></span>`fn map_right<F, M>(self, f: F) -> EitherOrBoth<A, M>` — [`EitherOrBoth`](#eitherorboth)

  Apply the function `f` on the value `b` in `Right(b)` or `Both(a, b)` variants.
  If it is present rewrapping the result in `self`'s original variant.

- <span id="eitherorboth-map-any"></span>`fn map_any<F, L, G, R>(self, f: F, g: G) -> EitherOrBoth<L, R>` — [`EitherOrBoth`](#eitherorboth)

  Apply the functions `f` and `g` on the value `a` and `b` respectively;
  found in `Left(a)`, `Right(b)`, or `Both(a, b)` variants.
  The Result is rewrapped `self`'s original variant.

- <span id="eitherorboth-left-and-then"></span>`fn left_and_then<F, L>(self, f: F) -> EitherOrBoth<L, B>` — [`EitherOrBoth`](#eitherorboth)

  Apply the function `f` on the value `a` in `Left(a)` or `Both(a, _)` variants if it is
  present.

- <span id="eitherorboth-right-and-then"></span>`fn right_and_then<F, R>(self, f: F) -> EitherOrBoth<A, R>` — [`EitherOrBoth`](#eitherorboth)

  Apply the function `f` on the value `b`
  in `Right(b)` or `Both(_, b)` variants if it is present.

- <span id="eitherorboth-or"></span>`fn or(self, l: A, r: B) -> (A, B)`

  Returns a tuple consisting of the `l` and `r` in `Both(l, r)`, if present.
  Otherwise, returns the wrapped value for the present element, and the supplied
  value for the other. The first (`l`) argument is used for a missing `Left`
  value. The second (`r`) argument is used for a missing `Right` value.
  
  Arguments passed to `or` are eagerly evaluated; if you are passing
  the result of a function call, it is recommended to use `or_else`,
  which is lazily evaluated.
  
  # Examples
  
  ```rust
  use itertools::EitherOrBoth;
  assert_eq!(EitherOrBoth::Both("tree", 1).or("stone", 5), ("tree", 1));
  assert_eq!(EitherOrBoth::Left("tree").or("stone", 5), ("tree", 5));
  assert_eq!(EitherOrBoth::Right(1).or("stone", 5), ("stone", 1));
  ```

- <span id="eitherorboth-or-default"></span>`fn or_default(self) -> (A, B)`

  Returns a tuple consisting of the `l` and `r` in `Both(l, r)`, if present.
  Otherwise, returns the wrapped value for the present element, and the [`default`](Default::default)
  for the other.

- <span id="eitherorboth-or-else"></span>`fn or_else<L: FnOnce() -> A, R: FnOnce() -> B>(self, l: L, r: R) -> (A, B)`

  Returns a tuple consisting of the `l` and `r` in `Both(l, r)`, if present.
  Otherwise, returns the wrapped value for the present element, and computes the
  missing value with the supplied closure. The first argument (`l`) is used for a
  missing `Left` value. The second argument (`r`) is used for a missing `Right` value.
  
  # Examples
  
  ```rust
  use itertools::EitherOrBoth;
  let k = 10;
  assert_eq!(EitherOrBoth::Both("tree", 1).or_else(|| "stone", || 2 * k), ("tree", 1));
  assert_eq!(EitherOrBoth::Left("tree").or_else(|| "stone", || 2 * k), ("tree", 20));
  assert_eq!(EitherOrBoth::Right(1).or_else(|| "stone", || 2 * k), ("stone", 1));
  ```

- <span id="eitherorboth-left-or-insert"></span>`fn left_or_insert(&mut self, val: A) -> &mut A`

  Returns a mutable reference to the left value. If the left value is not present,
  it is replaced with `val`.

- <span id="eitherorboth-right-or-insert"></span>`fn right_or_insert(&mut self, val: B) -> &mut B`

  Returns a mutable reference to the right value. If the right value is not present,
  it is replaced with `val`.

- <span id="eitherorboth-left-or-insert-with"></span>`fn left_or_insert_with<F>(&mut self, f: F) -> &mut A`

  If the left value is not present, replace it the value computed by the closure `f`.
  Returns a mutable reference to the now-present left value.

- <span id="eitherorboth-right-or-insert-with"></span>`fn right_or_insert_with<F>(&mut self, f: F) -> &mut B`

  If the right value is not present, replace it the value computed by the closure `f`.
  Returns a mutable reference to the now-present right value.

- <span id="eitherorboth-insert-left"></span>`fn insert_left(&mut self, val: A) -> &mut A`

  Sets the `left` value of this instance, and returns a mutable reference to it.
  Does not affect the `right` value.
  
  # Examples
  ```rust
  use itertools::{EitherOrBoth, EitherOrBoth::{Left, Right, Both}};
  
  // Overwriting a pre-existing value.
  let mut either: EitherOrBoth<_, ()> = Left(0_u32);
  assert_eq!(*either.insert_left(69), 69);
  
  // Inserting a second value.
  let mut either = Right("no");
  assert_eq!(*either.insert_left("yes"), "yes");
  assert_eq!(either, Both("yes", "no"));
  ```

- <span id="eitherorboth-insert-right"></span>`fn insert_right(&mut self, val: B) -> &mut B`

  Sets the `right` value of this instance, and returns a mutable reference to it.
  Does not affect the `left` value.
  
  # Examples
  ```rust
  use itertools::{EitherOrBoth, EitherOrBoth::{Left, Both}};
  // Overwriting a pre-existing value.
  let mut either: EitherOrBoth<_, ()> = Left(0_u32);
  assert_eq!(*either.insert_left(69), 69);
  
  // Inserting a second value.
  let mut either = Left("what's");
  assert_eq!(*either.insert_right(9 + 10), 21 - 2);
  assert_eq!(either, Both("what's", 9+10));
  ```

- <span id="eitherorboth-insert-both"></span>`fn insert_both(&mut self, left: A, right: B) -> (&mut A, &mut B)`

  Set `self` to `Both(..)`, containing the specified left and right values,
  and returns a mutable reference to those values.

#### Trait Implementations

##### `impl Any for EitherOrBoth<A, B>`

- <span id="eitherorboth-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EitherOrBoth<A, B>`

- <span id="eitherorboth-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EitherOrBoth<A, B>`

- <span id="eitherorboth-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: clone::Clone, B: clone::Clone> Clone for EitherOrBoth<A, B>`

- <span id="eitherorboth-clone"></span>`fn clone(&self) -> EitherOrBoth<A, B>` — [`EitherOrBoth`](#eitherorboth)

##### `impl CloneToUninit for EitherOrBoth<A, B>`

- <span id="eitherorboth-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for EitherOrBoth<A, B>`

- <span id="eitherorboth-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: cmp::Eq, B: cmp::Eq> Eq for EitherOrBoth<A, B>`

##### `impl<T> From for EitherOrBoth<A, B>`

- <span id="eitherorboth-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A: hash::Hash, B: hash::Hash> Hash for EitherOrBoth<A, B>`

- <span id="eitherorboth-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for EitherOrBoth<A, B>`

- <span id="eitherorboth-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for EitherOrBoth<A, B>`

##### `impl<A: cmp::PartialEq, B: cmp::PartialEq> PartialEq for EitherOrBoth<A, B>`

- <span id="eitherorboth-partialeq-eq"></span>`fn eq(&self, other: &EitherOrBoth<A, B>) -> bool` — [`EitherOrBoth`](#eitherorboth)

##### `impl<A, B> StructuralPartialEq for EitherOrBoth<A, B>`

##### `impl ToOwned for EitherOrBoth<A, B>`

- <span id="eitherorboth-toowned-type-owned"></span>`type Owned = T`

- <span id="eitherorboth-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="eitherorboth-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EitherOrBoth<A, B>`

- <span id="eitherorboth-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="eitherorboth-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EitherOrBoth<A, B>`

- <span id="eitherorboth-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="eitherorboth-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

