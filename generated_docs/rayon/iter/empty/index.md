*[rayon](../../index.md) / [iter](../index.md) / [empty](index.md)*

---

# Module `empty`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Empty`](#empty) | struct | Iterator adaptor for [the `empty()` function]. |
| [`EmptyProducer`](#emptyproducer) | struct | Private empty producer |
| [`empty`](#empty) | fn | Creates a parallel iterator that produces nothing. |

## Structs

### `Empty<T>`

```rust
struct Empty<T> {
    marker: std::marker::PhantomData<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/empty.rs:33-35`](../../../../.source_1765521767/rayon-1.11.0/src/iter/empty.rs#L33-L35)*

Iterator adaptor for [the `empty()` function].


#### Trait Implementations

##### `impl<T> Clone for Empty<T>`

- <span id="empty-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: Send> Debug for Empty<T>`

- <span id="empty-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for Empty<T>`

- <span id="empty-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="empty-len"></span>`fn len(&self) -> usize`

- <span id="empty-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Empty<T>`

##### `impl<T> IntoParallelIterator for Empty<T>`

- <span id="empty-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="empty-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="empty-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Empty<T>`

- <span id="empty-paralleliterator-type-item"></span>`type Item = T`

- <span id="empty-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="empty-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Empty<T>`

- <span id="empty-pointable-const-align"></span>`const ALIGN: usize`

- <span id="empty-pointable-type-init"></span>`type Init = T`

- <span id="empty-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="empty-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="empty-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="empty-drop"></span>`unsafe fn drop(ptr: usize)`

### `EmptyProducer<T: Send>`

```rust
struct EmptyProducer<T: Send>(std::marker::PhantomData<T>);
```

*Defined in [`rayon-1.11.0/src/iter/empty.rs:87`](../../../../.source_1765521767/rayon-1.11.0/src/iter/empty.rs#L87)*

Private empty producer

#### Trait Implementations

##### `impl<T> IntoEither for EmptyProducer<T>`

##### `impl<T> Pointable for EmptyProducer<T>`

- <span id="emptyproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="emptyproducer-pointable-type-init"></span>`type Init = T`

- <span id="emptyproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="emptyproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="emptyproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="emptyproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Producer for EmptyProducer<T>`

- <span id="emptyproducer-producer-type-item"></span>`type Item = T`

- <span id="emptyproducer-producer-type-intoiter"></span>`type IntoIter = Empty<T>`

- <span id="emptyproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="emptyproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="emptyproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

## Functions

### `empty`

```rust
fn empty<T: Send>() -> Empty<T>
```

*Defined in [`rayon-1.11.0/src/iter/empty.rs:24-28`](../../../../.source_1765521767/rayon-1.11.0/src/iter/empty.rs#L24-L28)*

Creates a parallel iterator that produces nothing.

This admits no parallelism on its own, but it could be used for code that
deals with generic parallel iterators.

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::empty;

let pi = (0..1234).into_par_iter()
    .chain(empty())
    .chain(1234..10_000);

assert_eq!(pi.count(), 10_000);
```

