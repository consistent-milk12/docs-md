*[itertools](../../index.md) / [adaptors](../index.md) / [map](index.md)*

---

# Module `map`

## Contents

- [Structs](#structs)
  - [`MapSpecialCase`](#mapspecialcase)
  - [`MapSpecialCaseFnOk`](#mapspecialcasefnok)
  - [`MapSpecialCaseFnInto`](#mapspecialcasefninto)
- [Traits](#traits)
  - [`MapSpecialCaseFn`](#mapspecialcasefn)
- [Functions](#functions)
  - [`map_ok`](#map-ok)
  - [`map_into`](#map-into)
- [Type Aliases](#type-aliases)
  - [`MapOk`](#mapok)
  - [`MapInto`](#mapinto)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MapSpecialCase`](#mapspecialcase) | struct |  |
| [`MapSpecialCaseFnOk`](#mapspecialcasefnok) | struct |  |
| [`MapSpecialCaseFnInto`](#mapspecialcasefninto) | struct |  |
| [`MapSpecialCaseFn`](#mapspecialcasefn) | trait |  |
| [`map_ok`](#map-ok) | fn | Create a new `MapOk` iterator. |
| [`map_into`](#map-into) | fn | Create a new [`MapInto`] iterator. |
| [`MapOk`](#mapok) | type | An iterator adapter to apply a transformation within a nested `Result::Ok`. |
| [`MapInto`](#mapinto) | type | An iterator adapter to apply `Into` conversion to each element. |

## Structs

### `MapSpecialCase<I, F>`

```rust
struct MapSpecialCase<I, F> {
    iter: I,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:6-9`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/map.rs#L6-L9)*

#### Trait Implementations

##### `impl Any for MapSpecialCase<I, F>`

- <span id="mapspecialcase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapSpecialCase<I, F>`

- <span id="mapspecialcase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapSpecialCase<I, F>`

- <span id="mapspecialcase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for MapSpecialCase<I, F>`

- <span id="mapspecialcase-clone"></span>`fn clone(&self) -> MapSpecialCase<I, F>` — [`MapSpecialCase`](#mapspecialcase)

##### `impl CloneToUninit for MapSpecialCase<I, F>`

- <span id="mapspecialcase-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, F: fmt::Debug> Debug for MapSpecialCase<I, F>`

- <span id="mapspecialcase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, R> DoubleEndedIterator for MapSpecialCase<I, R>`

- <span id="mapspecialcase-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I, R> ExactSizeIterator for MapSpecialCase<I, R>`

##### `impl<T> From for MapSpecialCase<I, F>`

- <span id="mapspecialcase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapSpecialCase<I, F>`

- <span id="mapspecialcase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapSpecialCase<I, F>`

##### `impl<I> IntoIterator for MapSpecialCase<I, F>`

- <span id="mapspecialcase-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="mapspecialcase-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="mapspecialcase-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, R> Iterator for MapSpecialCase<I, R>`

- <span id="mapspecialcase-iterator-type-item"></span>`type Item = <R as MapSpecialCaseFn>::Out`

- <span id="mapspecialcase-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="mapspecialcase-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="mapspecialcase-iterator-fold"></span>`fn fold<Acc, Fold>(self, init: Acc, fold_f: Fold) -> Acc`

- <span id="mapspecialcase-iterator-collect"></span>`fn collect<C>(self) -> C`

##### `impl Itertools for MapSpecialCase<I, F>`

##### `impl MultiUnzip for MapSpecialCase<I, F>`

- <span id="mapspecialcase-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for MapSpecialCase<I, F>`

- <span id="mapspecialcase-toowned-type-owned"></span>`type Owned = T`

- <span id="mapspecialcase-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mapspecialcase-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MapSpecialCase<I, F>`

- <span id="mapspecialcase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapspecialcase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapSpecialCase<I, F>`

- <span id="mapspecialcase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapspecialcase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapSpecialCaseFnOk<F>`

```rust
struct MapSpecialCaseFnOk<F>(F);
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:81`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/map.rs#L81)*

#### Trait Implementations

##### `impl Any for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F: clone::Clone> Clone for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-clone"></span>`fn clone(&self) -> MapSpecialCaseFnOk<F>` — [`MapSpecialCaseFnOk`](#mapspecialcasefnok)

##### `impl CloneToUninit for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<F> Debug for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapSpecialCaseFnOk<F>`

##### `impl<F, T, E> MapSpecialCaseFn for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-mapspecialcasefn-type-out"></span>`type Out = Result<U, E>`

- <span id="mapspecialcasefnok-mapspecialcasefn-call"></span>`fn call(&mut self, t: Result<T, E>) -> <Self as >::Out` — [`MapSpecialCaseFn`](#mapspecialcasefn)

##### `impl ToOwned for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-toowned-type-owned"></span>`type Owned = T`

- <span id="mapspecialcasefnok-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mapspecialcasefnok-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapspecialcasefnok-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapSpecialCaseFnOk<F>`

- <span id="mapspecialcasefnok-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapspecialcasefnok-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapSpecialCaseFnInto<U>`

```rust
struct MapSpecialCaseFnInto<U>(std::marker::PhantomData<U>);
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:111`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/map.rs#L111)*

#### Trait Implementations

##### `impl Any for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<U> Clone for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<U> Debug for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapSpecialCaseFnInto<U>`

##### `impl<T: Into<U>, U> MapSpecialCaseFn for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-mapspecialcasefn-type-out"></span>`type Out = U`

- <span id="mapspecialcasefninto-mapspecialcasefn-call"></span>`fn call(&mut self, t: T) -> <Self as >::Out` — [`MapSpecialCaseFn`](#mapspecialcasefn)

##### `impl ToOwned for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-toowned-type-owned"></span>`type Owned = T`

- <span id="mapspecialcasefninto-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mapspecialcasefninto-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapspecialcasefninto-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapSpecialCaseFnInto<U>`

- <span id="mapspecialcasefninto-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapspecialcasefninto-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `MapSpecialCaseFn<T>`

```rust
trait MapSpecialCaseFn<T> { ... }
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:11-14`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/map.rs#L11-L14)*

#### Associated Types

- `type Out`

#### Required Methods

- `fn MapSpecialCaseFn::call(&mut self, t: T) -> <Self as >::Out`

#### Implementors

- [`ConsTuplesFn`](../../cons_tuples_impl/index.md#constuplesfn)
- [`GroupingMapFn`](../../grouping_map/index.md#groupingmapfn)
- [`MapSpecialCaseFnInto`](#mapspecialcasefninto)
- [`MapSpecialCaseFnOk`](#mapspecialcasefnok)

## Functions

### `map_ok`

```rust
fn map_ok<I, F, T, U, E>(iter: I, f: F) -> MapOk<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(T) -> U
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:88-97`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/map.rs#L88-L97)*

Create a new `MapOk` iterator.

### `map_into`

```rust
fn map_into<I, R>(iter: I) -> MapInto<I, R>
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:125-130`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/map.rs#L125-L130)*

Create a new [`MapInto`](#mapinto) iterator.

## Type Aliases

### `MapOk<I, F>`

```rust
type MapOk<I, F> = MapSpecialCase<I, MapSpecialCaseFnOk<F>>;
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:68`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/map.rs#L68)*

An iterator adapter to apply a transformation within a nested `Result::Ok`.

See [`.map_ok()`](crate::Itertools::map_ok) for more information.

### `MapInto<I, R>`

```rust
type MapInto<I, R> = MapSpecialCase<I, MapSpecialCaseFnInto<R>>;
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:102`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/map.rs#L102)*

An iterator adapter to apply `Into` conversion to each element.

See [`.map_into()`](crate::Itertools::map_into) for more information.

