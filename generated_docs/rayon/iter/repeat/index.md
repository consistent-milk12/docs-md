*[rayon](../../index.md) / [iter](../index.md) / [repeat](index.md)*

---

# Module `repeat`

## Contents

- [Structs](#structs)
  - [`Repeat`](#repeat)
  - [`RepeatProducer`](#repeatproducer)
  - [`RepeatN`](#repeatn)
- [Enums](#enums)
  - [`RepeatNProducer`](#repeatnproducer)
- [Functions](#functions)
  - [`repeat`](#repeat)
  - [`repeat_n`](#repeat-n)
  - [`repeatn`](#repeatn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Repeat`](#repeat) | struct | Iterator adaptor for [the `repeat()` function]. |
| [`RepeatProducer`](#repeatproducer) | struct | Unindexed producer for `Repeat`. |
| [`RepeatN`](#repeatn) | struct | Iterator adaptor for [the `repeat_n()` function]. |
| [`RepeatNProducer`](#repeatnproducer) | enum | Producer for `RepeatN`. |
| [`repeat`](#repeat) | fn | Creates a parallel iterator that endlessly repeats `element` (by cloning it). |
| [`repeat_n`](#repeat-n) | fn | Creates a parallel iterator that produces `n` repeats of `element` (by cloning it). |
| [`repeatn`](#repeatn) | fn | Creates a parallel iterator that produces `n` repeats of `element` (by cloning it). |

## Structs

### `Repeat<T>`

```rust
struct Repeat<T> {
    element: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:10-12`](../../../../.source_1765633015/rayon-1.11.0/src/iter/repeat.rs#L10-L12)*

Iterator adaptor for [the `repeat()` function].


#### Implementations

- <span id="repeat-take"></span>`fn take(self, n: usize) -> RepeatN<T>` — [`RepeatN`](#repeatn)

  Takes only `n` repeats of the element, similar to the general

  `take()`.

  

  The resulting `RepeatN` is an `IndexedParallelIterator`, allowing

  more functionality than `Repeat` alone.

- <span id="repeat-zip"></span>`fn zip<Z>(self, zip_op: Z) -> Zip<RepeatN<T>, <Z as >::Iter>` — [`Zip`](../zip/index.md#zip), [`RepeatN`](#repeatn), [`IntoParallelIterator`](../index.md#intoparalleliterator)

  Iterates tuples, repeating the element with items from another

  iterator, similar to the general `zip()`.

#### Trait Implementations

##### `impl<T> Any for Repeat<T>`

- <span id="repeat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Repeat<T>`

- <span id="repeat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Repeat<T>`

- <span id="repeat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Repeat<T>`

- <span id="repeat-clone"></span>`fn clone(&self) -> Repeat<T>` — [`Repeat`](#repeat)

##### `impl<T> CloneToUninit for Repeat<T>`

- <span id="repeat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Repeat<T>`

- <span id="repeat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Repeat<T>`

- <span id="repeat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Repeat<T>`

- <span id="repeat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Repeat<T>`

##### `impl<T> IntoParallelIterator for Repeat<T>`

- <span id="repeat-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="repeat-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="repeat-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T> ParallelIterator for Repeat<T>`

- <span id="repeat-paralleliterator-type-item"></span>`type Item = T`

- <span id="repeat-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl<T> Pointable for Repeat<T>`

- <span id="repeat-pointable-const-align"></span>`const ALIGN: usize`

- <span id="repeat-pointable-type-init"></span>`type Init = T`

- <span id="repeat-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeat-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeat-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeat-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Repeat<T>`

- <span id="repeat-toowned-type-owned"></span>`type Owned = T`

- <span id="repeat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repeat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Repeat<T>`

- <span id="repeat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repeat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Repeat<T>`

- <span id="repeat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repeat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RepeatProducer<T: Clone + Send>`

```rust
struct RepeatProducer<T: Clone + Send> {
    element: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:81-83`](../../../../.source_1765633015/rayon-1.11.0/src/iter/repeat.rs#L81-L83)*

Unindexed producer for `Repeat`.

#### Trait Implementations

##### `impl<T> Any for RepeatProducer<T>`

- <span id="repeatproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RepeatProducer<T>`

- <span id="repeatproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RepeatProducer<T>`

- <span id="repeatproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for RepeatProducer<T>`

- <span id="repeatproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RepeatProducer<T>`

- <span id="repeatproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RepeatProducer<T>`

##### `impl<T> Pointable for RepeatProducer<T>`

- <span id="repeatproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="repeatproducer-pointable-type-init"></span>`type Init = T`

- <span id="repeatproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeatproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeatproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeatproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for RepeatProducer<T>`

- <span id="repeatproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repeatproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RepeatProducer<T>`

- <span id="repeatproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repeatproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: Clone + Send> UnindexedProducer for RepeatProducer<T>`

- <span id="repeatproducer-unindexedproducer-type-item"></span>`type Item = T`

- <span id="repeatproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="repeatproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `RepeatN<T>`

```rust
struct RepeatN<T> {
    inner: RepeatNProducer<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:111-113`](../../../../.source_1765633015/rayon-1.11.0/src/iter/repeat.rs#L111-L113)*

Iterator adaptor for [the `repeat_n()` function].


#### Trait Implementations

##### `impl<T> Any for RepeatN<T>`

- <span id="repeatn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RepeatN<T>`

- <span id="repeatn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RepeatN<T>`

- <span id="repeatn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for RepeatN<T>`

- <span id="repeatn-clone"></span>`fn clone(&self) -> RepeatN<T>` — [`RepeatN`](#repeatn)

##### `impl<T> CloneToUninit for RepeatN<T>`

- <span id="repeatn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for RepeatN<T>`

- <span id="repeatn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RepeatN<T>`

- <span id="repeatn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T> IndexedParallelIterator for RepeatN<T>`

- <span id="repeatn-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="repeatn-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

- <span id="repeatn-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

##### `impl<T, U> Into for RepeatN<T>`

- <span id="repeatn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RepeatN<T>`

##### `impl<T> IntoParallelIterator for RepeatN<T>`

- <span id="repeatn-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="repeatn-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="repeatn-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T> ParallelIterator for RepeatN<T>`

- <span id="repeatn-paralleliterator-type-item"></span>`type Item = T`

- <span id="repeatn-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="repeatn-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RepeatN<T>`

- <span id="repeatn-pointable-const-align"></span>`const ALIGN: usize`

- <span id="repeatn-pointable-type-init"></span>`type Init = T`

- <span id="repeatn-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeatn-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeatn-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeatn-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for RepeatN<T>`

- <span id="repeatn-toowned-type-owned"></span>`type Owned = T`

- <span id="repeatn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repeatn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RepeatN<T>`

- <span id="repeatn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repeatn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RepeatN<T>`

- <span id="repeatn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repeatn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `RepeatNProducer<T>`

```rust
enum RepeatNProducer<T> {
    Repeats(T, std::num::NonZeroUsize),
    Empty,
}
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:199-202`](../../../../.source_1765633015/rayon-1.11.0/src/iter/repeat.rs#L199-L202)*

Producer for `RepeatN`.

#### Trait Implementations

##### `impl<T> Any for RepeatNProducer<T>`

- <span id="repeatnproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RepeatNProducer<T>`

- <span id="repeatnproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RepeatNProducer<T>`

- <span id="repeatnproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for RepeatNProducer<T>`

- <span id="repeatnproducer-clone"></span>`fn clone(&self) -> RepeatNProducer<T>` — [`RepeatNProducer`](#repeatnproducer)

##### `impl<T> CloneToUninit for RepeatNProducer<T>`

- <span id="repeatnproducer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: Clone> DoubleEndedIterator for RepeatNProducer<T>`

- <span id="repeatnproducer-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<T>`

- <span id="repeatnproducer-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<T>`

##### `impl<T: Clone> ExactSizeIterator for RepeatNProducer<T>`

- <span id="repeatnproducer-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for RepeatNProducer<T>`

- <span id="repeatnproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RepeatNProducer<T>`

- <span id="repeatnproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RepeatNProducer<T>`

##### `impl IntoIterator for RepeatNProducer<T>`

- <span id="repeatnproducer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="repeatnproducer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="repeatnproducer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: Clone> Iterator for RepeatNProducer<T>`

- <span id="repeatnproducer-iterator-type-item"></span>`type Item = T`

- <span id="repeatnproducer-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="repeatnproducer-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<T>`

- <span id="repeatnproducer-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for RepeatNProducer<T>`

- <span id="repeatnproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="repeatnproducer-pointable-type-init"></span>`type Init = T`

- <span id="repeatnproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeatnproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeatnproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeatnproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Clone + Send> Producer for RepeatNProducer<T>`

- <span id="repeatnproducer-producer-type-item"></span>`type Item = T`

- <span id="repeatnproducer-producer-type-intoiter"></span>`type IntoIter = RepeatNProducer<T>`

- <span id="repeatnproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="repeatnproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T> ToOwned for RepeatNProducer<T>`

- <span id="repeatnproducer-toowned-type-owned"></span>`type Owned = T`

- <span id="repeatnproducer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repeatnproducer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RepeatNProducer<T>`

- <span id="repeatnproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repeatnproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RepeatNProducer<T>`

- <span id="repeatnproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repeatnproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `repeat`

```rust
fn repeat<T: Clone + Send>(element: T) -> Repeat<T>
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:30-32`](../../../../.source_1765633015/rayon-1.11.0/src/iter/repeat.rs#L30-L32)*

Creates a parallel iterator that endlessly repeats `element` (by
cloning it). Note that this iterator has "infinite" length, so
typically you would want to use `zip` or `take` or some other
means to shorten it, or consider using
[the `repeat_n()` function] instead.

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::repeat;
let x: Vec<(i32, i32)> = repeat(22).zip(0..3).collect();
assert_eq!(x, vec![(22, 0), (22, 1), (22, 2)]);
```

### `repeat_n`

```rust
fn repeat_n<T: Clone + Send>(element: T, n: usize) -> RepeatN<T>
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:126-132`](../../../../.source_1765633015/rayon-1.11.0/src/iter/repeat.rs#L126-L132)*

Creates a parallel iterator that produces `n` repeats of `element`
(by cloning it).

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::repeat_n;
let x: Vec<(i32, i32)> = repeat_n(22, 3).zip(0..3).collect();
assert_eq!(x, vec![(22, 0), (22, 1), (22, 2)]);
```

### `repeatn`

```rust
fn repeatn<T: Clone + Send>(element: T, n: usize) -> RepeatN<T>
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:139-141`](../../../../.source_1765633015/rayon-1.11.0/src/iter/repeat.rs#L139-L141)*

Creates a parallel iterator that produces `n` repeats of `element`
(by cloning it).

Deprecated in favor of [`repeat_n`](#repeat-n) for consistency with the standard library.

