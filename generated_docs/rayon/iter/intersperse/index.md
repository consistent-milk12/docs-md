*[rayon](../../index.md) / [iter](../index.md) / [intersperse](index.md)*

---

# Module `intersperse`

## Structs

### `Intersperse<I>`

```rust
struct Intersperse<I>
where
    I: ParallelIterator {
    base: I,
    item: <I as >::Item,
}
```

`Intersperse` is an iterator that inserts a particular item between each
item of the adapted iterator.  This struct is created by the
`intersperse()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, item: <I as >::Item) -> Self` — [`ParallelIterator`](../../prelude/index.md)

#### Trait Implementations

##### `impl<I> Clone for Intersperse<I>`

- `fn clone(self: &Self) -> Intersperse<I>` — [`Intersperse`](../index.md)

##### `impl<I> Debug for Intersperse<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for Intersperse<I>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Intersperse<I>`

##### `impl<T> IntoParallelIterator for Intersperse<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Intersperse<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Intersperse<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `IntersperseProducer<P>`

```rust
struct IntersperseProducer<P>
where
    P: Producer {
    base: P,
    item: <P as >::Item,
    len: usize,
    clone_first: bool,
}
```

#### Implementations

- `fn new(base: P, item: <P as >::Item, len: usize) -> Self` — [`Producer`](../plumbing/index.md)

#### Trait Implementations

##### `impl<T> IntoEither for IntersperseProducer<P>`

##### `impl<T> Pointable for IntersperseProducer<P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for IntersperseProducer<P>`

- `type Item = <P as Producer>::Item`

- `type IntoIter = IntersperseIter<<P as Producer>::IntoIter>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

### `IntersperseIter<I>`

```rust
struct IntersperseIter<I>
where
    I: Iterator {
    base: std::iter::Fuse<I>,
    item: <I as >::Item,
    clone_first: bool,
    clone_last: bool,
}
```

#### Trait Implementations

##### `impl<I> DoubleEndedIterator for IntersperseIter<I>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<I> ExactSizeIterator for IntersperseIter<I>`

- `fn len(self: &Self) -> usize`

##### `impl<T> IntoEither for IntersperseIter<I>`

##### `impl<I> IntoIterator for IntersperseIter<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I> Iterator for IntersperseIter<I>`

- `type Item = <I as Iterator>::Item`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for IntersperseIter<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `IntersperseConsumer<C, T>`

```rust
struct IntersperseConsumer<C, T> {
    base: C,
    item: T,
    clone_first: std::cell::Cell<bool>,
}
```

#### Implementations

- `fn new(base: C, item: T) -> Self`

#### Trait Implementations

##### `impl<C, T> Consumer for IntersperseConsumer<C, T>`

- `type Folder = IntersperseFolder<<C as Consumer>::Folder, T>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for IntersperseConsumer<C, T>`

##### `impl<T> Pointable for IntersperseConsumer<C, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<C, T> UnindexedConsumer for IntersperseConsumer<C, T>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `IntersperseFolder<C, T>`

```rust
struct IntersperseFolder<C, T> {
    base: C,
    item: T,
    clone_first: bool,
}
```

#### Trait Implementations

##### `impl<C, T> Folder for IntersperseFolder<C, T>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for IntersperseFolder<C, T>`

##### `impl<T> Pointable for IntersperseFolder<C, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

