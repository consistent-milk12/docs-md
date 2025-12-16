*[itertools](../index.md) / [kmerge_impl](index.md)*

---

# Module `kmerge_impl`

## Contents

- [Structs](#structs)
  - [`HeadTail`](#headtail)
  - [`KMergeByLt`](#kmergebylt)
  - [`KMergeBy`](#kmergeby)
- [Traits](#traits)
  - [`KMergePredicate`](#kmergepredicate)
- [Functions](#functions)
  - [`heapify`](#heapify)
  - [`sift_down`](#sift-down)
  - [`kmerge`](#kmerge)
  - [`kmerge_by`](#kmerge-by)
- [Type Aliases](#type-aliases)
  - [`KMerge`](#kmerge)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HeadTail`](#headtail) | struct | Head element and Tail iterator pair |
| [`KMergeByLt`](#kmergebylt) | struct |  |
| [`KMergeBy`](#kmergeby) | struct | An iterator adaptor that merges an abitrary number of base iterators according to an ordering function. |
| [`KMergePredicate`](#kmergepredicate) | trait |  |
| [`heapify`](#heapify) | fn | Make `data` a heap (min-heap w.r.t the sorting). |
| [`sift_down`](#sift-down) | fn | Sift down element at `index` (`heap` is a min-heap wrt the ordering) |
| [`kmerge`](#kmerge) | fn | Create an iterator that merges elements of the contained iterators using the ordering function. |
| [`kmerge_by`](#kmerge-by) | fn | Create an iterator that merges elements of the contained iterators. |
| [`KMerge`](#kmerge) | type | An iterator adaptor that merges an abitrary number of base iterators in ascending order. |

## Structs

### `HeadTail<I>`

```rust
struct HeadTail<I>
where
    I: Iterator {
    head: <I as >::Item,
    tail: I,
}
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:16-22`](../../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L16-L22)*

Head element and Tail iterator pair

`PartialEq`, `Eq`, `PartialOrd` and `Ord` are implemented by comparing sequences based on
first items (which are guaranteed to exist).

The meanings of `PartialOrd` and `Ord` are reversed so as to turn the heap used in
`KMerge` into a min-heap.

#### Implementations

- <span id="headtail-new"></span>`fn new(it: I) -> Option<Self>`

  Constructs a `HeadTail` from an `Iterator`. Returns `None` if the `Iterator` is empty.

- <span id="headtail-next"></span>`fn next(&mut self) -> Option<<I as >::Item>`

  Get the next element and update `head`, returning the old head in `Some`.
  
  Returns `None` when the tail is exhausted (only `head` then remains).

- <span id="headtail-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

  Hints at the size of the sequence, same as the `Iterator` method.

#### Trait Implementations

##### `impl Any for HeadTail<I>`

- <span id="headtail-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HeadTail<I>`

- <span id="headtail-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HeadTail<I>`

- <span id="headtail-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for HeadTail<I>`

- <span id="headtail-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for HeadTail<I>`

- <span id="headtail-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for HeadTail<I>`

- <span id="headtail-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HeadTail<I>`

- <span id="headtail-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HeadTail<I>`

- <span id="headtail-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for HeadTail<I>`

##### `impl ToOwned for HeadTail<I>`

- <span id="headtail-toowned-type-owned"></span>`type Owned = T`

- <span id="headtail-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="headtail-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for HeadTail<I>`

- <span id="headtail-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="headtail-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HeadTail<I>`

- <span id="headtail-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="headtail-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `KMergeByLt`

```rust
struct KMergeByLt;
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:113`](../../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L113)*

#### Trait Implementations

##### `impl Any for KMergeByLt`

- <span id="kmergebylt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for KMergeByLt`

- <span id="kmergebylt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for KMergeByLt`

- <span id="kmergebylt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for KMergeByLt`

- <span id="kmergebylt-clone"></span>`fn clone(&self) -> KMergeByLt` — [`KMergeByLt`](#kmergebylt)

##### `impl CloneToUninit for KMergeByLt`

- <span id="kmergebylt-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for KMergeByLt`

- <span id="kmergebylt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for KMergeByLt`

- <span id="kmergebylt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for KMergeByLt`

- <span id="kmergebylt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for KMergeByLt`

##### `impl<T: PartialOrd> KMergePredicate for KMergeByLt`

- <span id="kmergebylt-kmergepredicate-kmerge-pred"></span>`fn kmerge_pred(&mut self, a: &T, b: &T) -> bool`

##### `impl ToOwned for KMergeByLt`

- <span id="kmergebylt-toowned-type-owned"></span>`type Owned = T`

- <span id="kmergebylt-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="kmergebylt-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for KMergeByLt`

- <span id="kmergebylt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kmergebylt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for KMergeByLt`

- <span id="kmergebylt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kmergebylt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `KMergeBy<I, F>`

```rust
struct KMergeBy<I, F>
where
    I: Iterator {
    heap: alloc::vec::Vec<HeadTail<I>>,
    less_than: F,
}
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:157-163`](../../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L157-L163)*

An iterator adaptor that merges an abitrary number of base iterators
according to an ordering function.

Iterator element type is `I::Item`.

See [`.kmerge_by()`](crate::Itertools::kmerge_by) for more
information.

#### Trait Implementations

##### `impl Any for KMergeBy<I, F>`

- <span id="kmergeby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for KMergeBy<I, F>`

- <span id="kmergeby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for KMergeBy<I, F>`

- <span id="kmergeby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, F> Clone for KMergeBy<I, F>`

- <span id="kmergeby-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for KMergeBy<I, F>`

- <span id="kmergeby-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F> Debug for KMergeBy<I, F>`

- <span id="kmergeby-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for KMergeBy<I, F>`

- <span id="kmergeby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> FusedIterator for KMergeBy<I, F>`

##### `impl<U> Into for KMergeBy<I, F>`

- <span id="kmergeby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for KMergeBy<I, F>`

##### `impl<I> IntoIterator for KMergeBy<I, F>`

- <span id="kmergeby-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="kmergeby-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="kmergeby-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for KMergeBy<I, F>`

- <span id="kmergeby-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="kmergeby-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="kmergeby-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for KMergeBy<I, F>`

##### `impl MultiUnzip for KMergeBy<I, F>`

- <span id="kmergeby-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for KMergeBy<I, F>`

- <span id="kmergeby-toowned-type-owned"></span>`type Owned = T`

- <span id="kmergeby-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="kmergeby-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for KMergeBy<I, F>`

- <span id="kmergeby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kmergeby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for KMergeBy<I, F>`

- <span id="kmergeby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kmergeby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `KMergePredicate<T>`

```rust
trait KMergePredicate<T> { ... }
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:108-110`](../../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L108-L110)*

#### Required Methods

- `fn kmerge_pred(&mut self, a: &T, b: &T) -> bool`

#### Implementors

- [`KMergeByLt`](#kmergebylt)
- `F`

## Functions

### `heapify`

```rust
fn heapify<T, S>(data: &mut [T], less_than: S)
where
    S: FnMut(&T, &T) -> bool
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:60-67`](../../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L60-L67)*

Make `data` a heap (min-heap w.r.t the sorting).

### `sift_down`

```rust
fn sift_down<T, S>(heap: &mut [T], index: usize, less_than: S)
where
    S: FnMut(&T, &T) -> bool
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:70-98`](../../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L70-L98)*

Sift down element at `index` (`heap` is a min-heap wrt the ordering)

### `kmerge`

```rust
fn kmerge<I>(iterable: I) -> KMerge<<<I as >::Item as IntoIterator>::IntoIter>
where
    I: IntoIterator,
    <I as >::Item: IntoIterator,
    <<I as IntoIterator>::Item as IntoIterator>::Item: PartialOrd
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:140-147`](../../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L140-L147)*

Create an iterator that merges elements of the contained iterators using
the ordering function.

`IntoIterator` enabled version of [`Itertools::kmerge`](crate::Itertools::kmerge).

```rust
use itertools::kmerge;

for elt in kmerge(vec![vec![0, 2, 4], vec![1, 3, 5], vec![6, 7]]) {
    /* loop body */
    let _ = elt;
}
```

### `kmerge_by`

```rust
fn kmerge_by<I, F>(iterable: I, less_than: F) -> KMergeBy<<<I as >::Item as IntoIterator>::IntoIter, F>
where
    I: IntoIterator,
    <I as >::Item: IntoIterator,
    F: KMergePredicate<<<I as IntoIterator>::Item as IntoIterator>::Item>
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:176-191`](../../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L176-L191)*

Create an iterator that merges elements of the contained iterators.

`IntoIterator` enabled version of [`Itertools::kmerge_by`](crate::Itertools::kmerge_by).

## Type Aliases

### `KMerge<I>`

```rust
type KMerge<I> = KMergeBy<I, KMergeByLt>;
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:106`](../../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L106)*

An iterator adaptor that merges an abitrary number of base iterators in ascending order.
If all base iterators are sorted (ascending), the result is sorted.

Iterator element type is `I::Item`.

See [`.kmerge()`](crate::Itertools::kmerge) for more information.

