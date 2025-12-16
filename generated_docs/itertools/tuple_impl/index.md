*[itertools](../index.md) / [tuple_impl](index.md)*

---

# Module `tuple_impl`

Some iterator that produces tuples

## Contents

- [Structs](#structs)
  - [`TupleBuffer`](#tuplebuffer)
  - [`Tuples`](#tuples)
  - [`TupleWindows`](#tuplewindows)
  - [`CircularTupleWindows`](#circulartuplewindows)
- [Traits](#traits)
  - [`HomogeneousTuple`](#homogeneoustuple)
  - [`TupleCollect`](#tuplecollect)
- [Functions](#functions)
  - [`tuples`](#tuples)
  - [`add_then_div`](#add-then-div)
  - [`tuple_windows`](#tuple-windows)
  - [`circular_tuple_windows`](#circular-tuple-windows)
- [Macros](#macros)
  - [`rev_for_each_ident!`](#rev-for-each-ident)
  - [`impl_tuple_collect!`](#impl-tuple-collect)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TupleBuffer`](#tuplebuffer) | struct | An iterator over a incomplete tuple. |
| [`Tuples`](#tuples) | struct | An iterator that groups the items in tuples of a specific size. |
| [`TupleWindows`](#tuplewindows) | struct | An iterator over all contiguous windows that produces tuples of a specific size. |
| [`CircularTupleWindows`](#circulartuplewindows) | struct | An iterator over all windows, wrapping back to the first elements when the window would otherwise exceed the length of the iterator, producing tuples of a specific size. |
| [`HomogeneousTuple`](#homogeneoustuple) | trait | Implemented for homogeneous tuples of size up to 12. |
| [`TupleCollect`](#tuplecollect) | trait |  |
| [`tuples`](#tuples) | fn | Create a new tuples iterator. |
| [`add_then_div`](#add-then-div) | fn | `(n + a) / d` avoiding overflow when possible, returns `None` if it overflows. |
| [`tuple_windows`](#tuple-windows) | fn | Create a new tuple windows iterator. |
| [`circular_tuple_windows`](#circular-tuple-windows) | fn |  |
| [`rev_for_each_ident!`](#rev-for-each-ident) | macro |  |
| [`impl_tuple_collect!`](#impl-tuple-collect) | macro |  |

## Structs

### `TupleBuffer<T>`

```rust
struct TupleBuffer<T>
where
    T: HomogeneousTuple {
    cur: usize,
    buf: <T as >::Buffer,
}
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:24-30`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L24-L30)*

An iterator over a incomplete tuple.

See [`.tuples()`](crate::Itertools::tuples) and
`Tuples::into_buffer()`.

#### Implementations

- <span id="tuplebuffer-new"></span>`fn new(buf: <T as >::Buffer) -> Self` — [`TupleCollect`](#tuplecollect)

#### Trait Implementations

##### `impl<T> Any for TupleBuffer<T>`

- <span id="tuplebuffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TupleBuffer<T>`

- <span id="tuplebuffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TupleBuffer<T>`

- <span id="tuplebuffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for TupleBuffer<T>`

- <span id="tuplebuffer-clone"></span>`fn clone(&self) -> TupleBuffer<T>` — [`TupleBuffer`](#tuplebuffer)

##### `impl<T> CloneToUninit for TupleBuffer<T>`

- <span id="tuplebuffer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Debug for TupleBuffer<T>`

- <span id="tuplebuffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ExactSizeIterator for TupleBuffer<T>`

##### `impl<T> From for TupleBuffer<T>`

- <span id="tuplebuffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for TupleBuffer<T>`

- <span id="tuplebuffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for TupleBuffer<T>`

##### `impl IntoIterator for TupleBuffer<T>`

- <span id="tuplebuffer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuplebuffer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuplebuffer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for TupleBuffer<T>`

- <span id="tuplebuffer-iterator-type-item"></span>`type Item = <T as TupleCollect>::Item`

- <span id="tuplebuffer-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuplebuffer-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Itertools for TupleBuffer<T>`

##### `impl MultiUnzip for TupleBuffer<T>`

- <span id="tuplebuffer-multiunzip"></span>`fn multiunzip(self)`

##### `impl<T> ToOwned for TupleBuffer<T>`

- <span id="tuplebuffer-toowned-type-owned"></span>`type Owned = T`

- <span id="tuplebuffer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuplebuffer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for TupleBuffer<T>`

- <span id="tuplebuffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuplebuffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for TupleBuffer<T>`

- <span id="tuplebuffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuplebuffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuples<I, T>`

```rust
struct Tuples<I, T>
where
    I: Iterator<Item = <T as >::Item>,
    T: HomogeneousTuple {
    iter: std::iter::Fuse<I>,
    buf: <T as >::Buffer,
}
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:78-85`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L78-L85)*

An iterator that groups the items in tuples of a specific size.

See [`.tuples()`](crate::Itertools::tuples) for more information.

#### Implementations

- <span id="tuples-into-buffer"></span>`fn into_buffer(self) -> TupleBuffer<T>` — [`TupleBuffer`](#tuplebuffer)

  Return a buffer with the produced items that was not enough to be grouped in a tuple.
  
  ```rust
  use itertools::Itertools;
  
  let mut iter = (0..5).tuples();
  assert_eq!(Some((0, 1, 2)), iter.next());
  assert_eq!(None, iter.next());
  itertools::assert_equal(vec![3, 4], iter.into_buffer());
  ```

#### Trait Implementations

##### `impl<T> Any for Tuples<I, T>`

- <span id="tuples-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuples<I, T>`

- <span id="tuples-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuples<I, T>`

- <span id="tuples-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, T> Clone for Tuples<I, T>`

- <span id="tuples-clone"></span>`fn clone(&self) -> Tuples<I, T>` — [`Tuples`](#tuples)

##### `impl<T> CloneToUninit for Tuples<I, T>`

- <span id="tuples-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, T> Debug for Tuples<I, T>`

- <span id="tuples-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, T> ExactSizeIterator for Tuples<I, T>`

##### `impl<T> From for Tuples<I, T>`

- <span id="tuples-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Tuples<I, T>`

- <span id="tuples-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Tuples<I, T>`

##### `impl<I> IntoIterator for Tuples<I, T>`

- <span id="tuples-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuples-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuples-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, T> Iterator for Tuples<I, T>`

- <span id="tuples-iterator-type-item"></span>`type Item = T`

- <span id="tuples-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuples-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Itertools for Tuples<I, T>`

##### `impl<FromA> MultiUnzip for Tuples<I, T>`

- <span id="tuples-multiunzip"></span>`fn multiunzip(self) -> (FromA)`

##### `impl<T> ToOwned for Tuples<I, T>`

- <span id="tuples-toowned-type-owned"></span>`type Owned = T`

- <span id="tuples-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuples-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Tuples<I, T>`

- <span id="tuples-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuples-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Tuples<I, T>`

- <span id="tuples-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuples-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TupleWindows<I, T>`

```rust
struct TupleWindows<I, T>
where
    I: Iterator<Item = <T as >::Item>,
    T: HomogeneousTuple {
    iter: I,
    last: Option<T>,
}
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:166-173`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L166-L173)*

An iterator over all contiguous windows that produces tuples of a specific size.

See [`.tuple_windows()`](crate::Itertools::tuple_windows) for more
information.

#### Trait Implementations

##### `impl<T> Any for TupleWindows<I, T>`

- <span id="tuplewindows-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TupleWindows<I, T>`

- <span id="tuplewindows-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TupleWindows<I, T>`

- <span id="tuplewindows-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, T> Clone for TupleWindows<I, T>`

- <span id="tuplewindows-clone"></span>`fn clone(&self) -> TupleWindows<I, T>` — [`TupleWindows`](#tuplewindows)

##### `impl<T> CloneToUninit for TupleWindows<I, T>`

- <span id="tuplewindows-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, T> Debug for TupleWindows<I, T>`

- <span id="tuplewindows-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, T> ExactSizeIterator for TupleWindows<I, T>`

##### `impl<T> From for TupleWindows<I, T>`

- <span id="tuplewindows-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, T> FusedIterator for TupleWindows<I, T>`

##### `impl<T, U> Into for TupleWindows<I, T>`

- <span id="tuplewindows-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for TupleWindows<I, T>`

##### `impl<I> IntoIterator for TupleWindows<I, T>`

- <span id="tuplewindows-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuplewindows-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuplewindows-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, T> Iterator for TupleWindows<I, T>`

- <span id="tuplewindows-iterator-type-item"></span>`type Item = T`

- <span id="tuplewindows-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuplewindows-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Itertools for TupleWindows<I, T>`

##### `impl<FromA> MultiUnzip for TupleWindows<I, T>`

- <span id="tuplewindows-multiunzip"></span>`fn multiunzip(self) -> (FromA)`

##### `impl<T> ToOwned for TupleWindows<I, T>`

- <span id="tuplewindows-toowned-type-owned"></span>`type Owned = T`

- <span id="tuplewindows-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuplewindows-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for TupleWindows<I, T>`

- <span id="tuplewindows-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuplewindows-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for TupleWindows<I, T>`

- <span id="tuplewindows-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuplewindows-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CircularTupleWindows<I, T>`

```rust
struct CircularTupleWindows<I, T>
where
    I: Iterator<Item = <T as >::Item> + Clone,
    T: TupleCollect + Clone {
    iter: TupleWindows<std::iter::Cycle<I>, T>,
    len: usize,
}
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:247-254`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L247-L254)*

An iterator over all windows, wrapping back to the first elements when the
window would otherwise exceed the length of the iterator, producing tuples
of a specific size.

See [`.circular_tuple_windows()`](crate::Itertools::circular_tuple_windows) for more
information.

#### Trait Implementations

##### `impl<T> Any for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, T> Clone for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-clone"></span>`fn clone(&self) -> CircularTupleWindows<I, T>` — [`CircularTupleWindows`](#circulartuplewindows)

##### `impl<T> CloneToUninit for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, T> Debug for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, T> ExactSizeIterator for CircularTupleWindows<I, T>`

##### `impl<T> From for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, T> FusedIterator for CircularTupleWindows<I, T>`

##### `impl<T, U> Into for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for CircularTupleWindows<I, T>`

##### `impl<I> IntoIterator for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="circulartuplewindows-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="circulartuplewindows-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, T> Iterator for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-iterator-type-item"></span>`type Item = T`

- <span id="circulartuplewindows-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="circulartuplewindows-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Itertools for CircularTupleWindows<I, T>`

##### `impl<FromA> MultiUnzip for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-multiunzip"></span>`fn multiunzip(self) -> (FromA)`

##### `impl<T> ToOwned for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-toowned-type-owned"></span>`type Owned = T`

- <span id="circulartuplewindows-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="circulartuplewindows-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="circulartuplewindows-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="circulartuplewindows-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `HomogeneousTuple`

```rust
trait HomogeneousTuple: TupleCollect { ... }
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:15`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L15)*

Implemented for homogeneous tuples of size up to 12.

#### Implementors

- `T`

### `TupleCollect`

```rust
trait TupleCollect: Sized { ... }
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:306-326`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L306-L326)*

#### Associated Types

- `type Item`

- `type Buffer: 3`

#### Required Methods

- `fn collect_from_iter<I>(iter: I, buf: &mut <Self as >::Buffer) -> Option<Self>`

- `fn collect_from_iter_no_buf<I>(iter: I) -> Option<Self>`

- `fn num_items() -> usize`

- `fn left_shift_push(&mut self, item: <Self as >::Item)`

#### Provided Methods

- `fn buffer_len(buf: &<Self as >::Buffer) -> usize`

#### Implementors

- `(A)`
- `(A, A)`
- `(A, A, A)`
- `(A, A, A, A)`
- `(A, A, A, A, A)`
- `(A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A, A, A, A, A, A)`

## Functions

### `tuples`

```rust
fn tuples<I, T>(iter: I) -> Tuples<I, T>
where
    I: Iterator<Item = <T as >::Item>,
    T: HomogeneousTuple
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:88-97`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L88-L97)*

Create a new tuples iterator.

### `add_then_div`

```rust
fn add_then_div(n: usize, a: usize, d: usize) -> Option<usize>
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:128-131`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L128-L131)*

`(n + a) / d` avoiding overflow when possible, returns `None` if it overflows.

### `tuple_windows`

```rust
fn tuple_windows<I, T>(iter: I) -> TupleWindows<I, T>
where
    I: Iterator<Item = <T as >::Item>,
    T: HomogeneousTuple,
    <T as >::Item: Clone
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:176-183`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L176-L183)*

Create a new tuple windows iterator.

### `circular_tuple_windows`

```rust
fn circular_tuple_windows<I, T>(iter: I) -> CircularTupleWindows<I, T>
where
    I: Iterator<Item = <T as >::Item> + Clone + ExactSizeIterator,
    T: TupleCollect + Clone,
    <T as >::Item: Clone
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:256-266`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L256-L266)*

## Macros

### `rev_for_each_ident!`

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:328-334`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L328-L334)*

### `impl_tuple_collect!`

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:336-400`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L336-L400)*

