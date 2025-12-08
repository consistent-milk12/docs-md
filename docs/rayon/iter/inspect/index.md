*[rayon](../../index.md) / [iter](../index.md) / [inspect](index.md)*

---

# Module `inspect`

## Structs

### `Inspect<I, F>`

```rust
struct Inspect<I, F> {
    base: I,
    inspect_op: F,
}
```

`Inspect` is an iterator that calls a function with a reference to each
element before yielding it.

This struct is created by the `inspect()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, inspect_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, F: $crate::clone::Clone> Clone for Inspect<I, F>`

- `fn clone(self: &Self) -> Inspect<I, F>` — [`Inspect`](../index.md)

##### `impl<I: Debug, F> Debug for Inspect<I, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> IndexedParallelIterator for Inspect<I, F>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Inspect<I, F>`

##### `impl<T> IntoParallelIterator for Inspect<I, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, F> ParallelIterator for Inspect<I, F>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Inspect<I, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `InspectProducer<'f, P, F>`

```rust
struct InspectProducer<'f, P, F> {
    base: P,
    inspect_op: &'f F,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for InspectProducer<'f, P, F>`

##### `impl<T> Pointable for InspectProducer<'f, P, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, P, F> Producer for InspectProducer<'f, P, F>`

- `type Item = <P as Producer>::Item`

- `type IntoIter = Inspect<<P as Producer>::IntoIter, &'f F>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<G>(self: Self, folder: G) -> G`

### `InspectConsumer<'f, C, F>`

```rust
struct InspectConsumer<'f, C, F> {
    base: C,
    inspect_op: &'f F,
}
```

#### Implementations

- `fn new(base: C, inspect_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, C, F> Consumer for InspectConsumer<'f, C, F>`

- `type Folder = InspectFolder<'f, <C as Consumer>::Folder, F>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for InspectConsumer<'f, C, F>`

##### `impl<T> Pointable for InspectConsumer<'f, C, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, T, C, F> UnindexedConsumer for InspectConsumer<'f, C, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `InspectFolder<'f, C, F>`

```rust
struct InspectFolder<'f, C, F> {
    base: C,
    inspect_op: &'f F,
}
```

#### Trait Implementations

##### `impl<'f, T, C, F> Folder for InspectFolder<'f, C, F>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for InspectFolder<'f, C, F>`

##### `impl<T> Pointable for InspectFolder<'f, C, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

