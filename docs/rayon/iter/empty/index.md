*[rayon](../../index.md) / [iter](../index.md) / [empty](index.md)*

---

# Module `empty`

## Structs

### `Empty<T>`

```rust
struct Empty<T> {
    marker: std::marker::PhantomData<T>,
}
```

Iterator adaptor for [the `empty()` function].


#### Trait Implementations

##### `impl<T> Clone for Empty<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T: Send> Debug for Empty<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for Empty<T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Empty<T>`

##### `impl<T> IntoParallelIterator for Empty<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<T: Send> ParallelIterator for Empty<T>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Empty<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `EmptyProducer<T: Send>`

```rust
struct EmptyProducer<T: Send>(std::marker::PhantomData<T>);
```

Private empty producer

#### Trait Implementations

##### `impl<T> IntoEither for EmptyProducer<T>`

##### `impl<T> Pointable for EmptyProducer<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Producer for EmptyProducer<T>`

- `type Item = T`

- `type IntoIter = Empty<T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

## Functions

### `empty`

```rust
fn empty<T: Send>() -> Empty<T>
```

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

