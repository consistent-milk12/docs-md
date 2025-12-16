*[itertools](../index.md) / [intersperse](index.md)*

---

# Module `intersperse`

## Contents

- [Structs](#structs)
  - [`IntersperseElementSimple`](#intersperseelementsimple)
  - [`IntersperseWith`](#interspersewith)
- [Traits](#traits)
  - [`IntersperseElement`](#intersperseelement)
- [Functions](#functions)
  - [`intersperse`](#intersperse)
  - [`intersperse_with`](#intersperse-with)
- [Type Aliases](#type-aliases)
  - [`Intersperse`](#intersperse)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntersperseElementSimple`](#intersperseelementsimple) | struct |  |
| [`IntersperseWith`](#interspersewith) | struct | An iterator adaptor to insert a particular value created by a function between each element of the adapted iterator. |
| [`IntersperseElement`](#intersperseelement) | trait |  |
| [`intersperse`](#intersperse) | fn | Create a new Intersperse iterator |
| [`intersperse_with`](#intersperse-with) | fn | Create a new `IntersperseWith` iterator |
| [`Intersperse`](#intersperse) | type | An iterator adaptor to insert a particular value between each element of the adapted iterator. |

## Structs

### `IntersperseElementSimple<Item>`

```rust
struct IntersperseElementSimple<Item>(Item);
```

*Defined in [`itertools-0.14.0/src/intersperse.rs:9`](../../../.source_1765894658/itertools-0.14.0/src/intersperse.rs#L9)*

#### Trait Implementations

##### `impl Any for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Item: clone::Clone> Clone for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-clone"></span>`fn clone(&self) -> IntersperseElementSimple<Item>` — [`IntersperseElementSimple`](#intersperseelementsimple)

##### `impl CloneToUninit for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Item: fmt::Debug> Debug for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<Item: Clone> IntersperseElement for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-intersperseelement-generate"></span>`fn generate(&mut self) -> Item`

##### `impl<U> Into for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IntersperseElementSimple<Item>`

##### `impl ToOwned for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-toowned-type-owned"></span>`type Owned = T`

- <span id="intersperseelementsimple-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intersperseelementsimple-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intersperseelementsimple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntersperseElementSimple<Item>`

- <span id="intersperseelementsimple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intersperseelementsimple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntersperseWith<I, ElemF>`

```rust
struct IntersperseWith<I, ElemF>
where
    I: Iterator {
    element: ElemF,
    iter: std::iter::Fuse<I>,
    peek: Option<Option<<I as >::Item>>,
}
```

*Defined in [`itertools-0.14.0/src/intersperse.rs:51-61`](../../../.source_1765894658/itertools-0.14.0/src/intersperse.rs#L51-L61)*

An iterator adaptor to insert a particular value created by a function
between each element of the adapted iterator.

Iterator element type is `I::Item`

This iterator is *fused*.

See [`.intersperse_with()`](crate::Itertools::intersperse_with) for more information.

#### Fields

- **`peek`**: `Option<Option<<I as >::Item>>`

  `peek` is None while no item have been taken out of `iter` (at definition).
  Then `peek` will alternatively be `Some(None)` and `Some(Some(item))`,
  where `None` indicates it's time to generate from `element` (unless `iter` is empty).

#### Trait Implementations

##### `impl Any for IntersperseWith<I, ElemF>`

- <span id="interspersewith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntersperseWith<I, ElemF>`

- <span id="interspersewith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntersperseWith<I, ElemF>`

- <span id="interspersewith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, ElemF: clone::Clone> Clone for IntersperseWith<I, ElemF>`

- <span id="interspersewith-clone"></span>`fn clone(&self) -> IntersperseWith<I, ElemF>` — [`IntersperseWith`](#interspersewith)

##### `impl CloneToUninit for IntersperseWith<I, ElemF>`

- <span id="interspersewith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, ElemF: fmt::Debug> Debug for IntersperseWith<I, ElemF>`

- <span id="interspersewith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IntersperseWith<I, ElemF>`

- <span id="interspersewith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, ElemF> FusedIterator for IntersperseWith<I, ElemF>`

##### `impl<U> Into for IntersperseWith<I, ElemF>`

- <span id="interspersewith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IntersperseWith<I, ElemF>`

##### `impl<I> IntoIterator for IntersperseWith<I, ElemF>`

- <span id="interspersewith-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interspersewith-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="interspersewith-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, ElemF> Iterator for IntersperseWith<I, ElemF>`

- <span id="interspersewith-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interspersewith-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="interspersewith-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="interspersewith-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for IntersperseWith<I, ElemF>`

##### `impl MultiUnzip for IntersperseWith<I, ElemF>`

- <span id="interspersewith-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for IntersperseWith<I, ElemF>`

- <span id="interspersewith-toowned-type-owned"></span>`type Owned = T`

- <span id="interspersewith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="interspersewith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IntersperseWith<I, ElemF>`

- <span id="interspersewith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interspersewith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntersperseWith<I, ElemF>`

- <span id="interspersewith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interspersewith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `IntersperseElement<Item>`

```rust
trait IntersperseElement<Item> { ... }
```

*Defined in [`itertools-0.14.0/src/intersperse.rs:4-6`](../../../.source_1765894658/itertools-0.14.0/src/intersperse.rs#L4-L6)*

#### Required Methods

- `fn generate(&mut self) -> Item`

#### Implementors

- [`IntersperseElementSimple`](#intersperseelementsimple)
- `F`

## Functions

### `intersperse`

```rust
fn intersperse<I>(iter: I, elt: <I as >::Item) -> Intersperse<I>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/intersperse.rs:28-33`](../../../.source_1765894658/itertools-0.14.0/src/intersperse.rs#L28-L33)*

Create a new Intersperse iterator

### `intersperse_with`

```rust
fn intersperse_with<I, ElemF>(iter: I, elt: ElemF) -> IntersperseWith<I, ElemF>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/intersperse.rs:64-73`](../../../.source_1765894658/itertools-0.14.0/src/intersperse.rs#L64-L73)*

Create a new `IntersperseWith` iterator

## Type Aliases

### `Intersperse<I>`

```rust
type Intersperse<I> = IntersperseWith<I, IntersperseElementSimple<<I as Iterator>::Item>>;
```

*Defined in [`itertools-0.14.0/src/intersperse.rs:25`](../../../.source_1765894658/itertools-0.14.0/src/intersperse.rs#L25)*

An iterator adaptor to insert a particular value
between each element of the adapted iterator.

Iterator element type is `I::Item`

This iterator is *fused*.

See [`.intersperse()`](crate::Itertools::intersperse) for more information.

