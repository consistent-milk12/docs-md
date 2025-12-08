*[rayon](../../index.md) / [iter](../index.md) / [cloned](index.md)*

---

# Module `cloned`

## Structs

### `Cloned<I>`

```rust
struct Cloned<I> {
    base: I,
}
```

`Cloned` is an iterator that clones the elements of an underlying iterator.

This struct is created by the `cloned()` method on [`ParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Cloned<I>`

- `fn clone(self: &Self) -> Cloned<I>` — [`Cloned`](#cloned)

##### `impl<I: $crate::fmt::Debug> Debug for Cloned<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, T, I> IndexedParallelIterator for Cloned<I>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Cloned<I>`

##### `impl<T> IntoParallelIterator for Cloned<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'a, T, I> ParallelIterator for Cloned<I>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Cloned<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ClonedProducer<P>`

```rust
struct ClonedProducer<P> {
    base: P,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for ClonedProducer<P>`

##### `impl<T> Pointable for ClonedProducer<P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'a, T, P> Producer for ClonedProducer<P>`

- `type Item = T`

- `type IntoIter = Cloned<<P as Producer>::IntoIter>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

### `ClonedConsumer<C>`

```rust
struct ClonedConsumer<C> {
    base: C,
}
```

#### Implementations

- `fn new(base: C) -> Self`

#### Trait Implementations

##### `impl<'a, T, C> Consumer for ClonedConsumer<C>`

- `type Folder = ClonedFolder<<C as Consumer>::Folder>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ClonedConsumer<C>`

##### `impl<T> Pointable for ClonedConsumer<C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'a, T, C> UnindexedConsumer for ClonedConsumer<C>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `ClonedFolder<F>`

```rust
struct ClonedFolder<F> {
    base: F,
}
```

#### Trait Implementations

##### `impl<'a, T, F> Folder for ClonedFolder<F>`

- `type Result = <F as Folder>::Result`

- `fn consume(self: Self, item: &'a T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <F as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ClonedFolder<F>`

##### `impl<T> Pointable for ClonedFolder<F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

