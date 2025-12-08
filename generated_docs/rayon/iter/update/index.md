*[rayon](../../index.md) / [iter](../index.md) / [update](index.md)*

---

# Module `update`

## Structs

### `Update<I, F>`

```rust
struct Update<I, F> {
    base: I,
    update_op: F,
}
```

`Update` is an iterator that mutates the elements of an
underlying iterator before they are yielded.

This struct is created by the `update()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, update_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, F: $crate::clone::Clone> Clone for Update<I, F>`

- `fn clone(self: &Self) -> Update<I, F>` — [`Update`](../index.md)

##### `impl<I: Debug, F> Debug for Update<I, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> IndexedParallelIterator for Update<I, F>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Update<I, F>`

##### `impl<T> IntoParallelIterator for Update<I, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, F> ParallelIterator for Update<I, F>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Update<I, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `UpdateProducer<'f, P, F>`

```rust
struct UpdateProducer<'f, P, F> {
    base: P,
    update_op: &'f F,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for UpdateProducer<'f, P, F>`

##### `impl<T> Pointable for UpdateProducer<'f, P, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, P, F> Producer for UpdateProducer<'f, P, F>`

- `type Item = <P as Producer>::Item`

- `type IntoIter = UpdateSeq<<P as Producer>::IntoIter, &'f F>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<G>(self: Self, folder: G) -> G`

### `UpdateConsumer<'f, C, F>`

```rust
struct UpdateConsumer<'f, C, F> {
    base: C,
    update_op: &'f F,
}
```

#### Implementations

- `fn new(base: C, update_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, C, F> Consumer for UpdateConsumer<'f, C, F>`

- `type Folder = UpdateFolder<'f, <C as Consumer>::Folder, F>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for UpdateConsumer<'f, C, F>`

##### `impl<T> Pointable for UpdateConsumer<'f, C, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, T, C, F> UnindexedConsumer for UpdateConsumer<'f, C, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `UpdateFolder<'f, C, F>`

```rust
struct UpdateFolder<'f, C, F> {
    base: C,
    update_op: &'f F,
}
```

#### Trait Implementations

##### `impl<'f, T, C, F> Folder for UpdateFolder<'f, C, F>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for UpdateFolder<'f, C, F>`

##### `impl<T> Pointable for UpdateFolder<'f, C, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `UpdateSeq<I, F>`

```rust
struct UpdateSeq<I, F> {
    base: I,
    update_op: F,
}
```

Standard Update adaptor, based on `itertools::adaptors::Update`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, F: $crate::clone::Clone> Clone for UpdateSeq<I, F>`

- `fn clone(self: &Self) -> UpdateSeq<I, F>` — [`UpdateSeq`](#updateseq)

##### `impl<I: $crate::fmt::Debug, F: $crate::fmt::Debug> Debug for UpdateSeq<I, F>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I, F> DoubleEndedIterator for UpdateSeq<I, F>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<I, F> ExactSizeIterator for UpdateSeq<I, F>`

##### `impl<T> IntoEither for UpdateSeq<I, F>`

##### `impl<I> IntoIterator for UpdateSeq<I, F>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I, F> Iterator for UpdateSeq<I, F>`

- `type Item = <I as Iterator>::Item`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<Acc, G>(self: Self, init: Acc, g: G) -> Acc`

- `fn collect<C>(self: Self) -> C`

##### `impl<T> Pointable for UpdateSeq<I, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `apply`

```rust
fn apply<T>(update_op: impl Fn(&mut T)) -> impl Fn(T) -> T
```

