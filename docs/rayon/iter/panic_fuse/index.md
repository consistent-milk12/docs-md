*[rayon](../../index.md) / [iter](../index.md) / [panic_fuse](index.md)*

---

# Module `panic_fuse`

## Structs

### `PanicFuse<I>`

```rust
struct PanicFuse<I> {
    base: I,
}
```

`PanicFuse` is an adaptor that wraps an iterator with a fuse in case
of panics, to halt all threads as soon as possible.

This struct is created by the `panic_fuse()` method on [`ParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I) -> PanicFuse<I>` — [`PanicFuse`](#panicfuse)

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for PanicFuse<I>`

- `fn clone(self: &Self) -> PanicFuse<I>` — [`PanicFuse`](#panicfuse)

##### `impl<I: $crate::fmt::Debug> Debug for PanicFuse<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for PanicFuse<I>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for PanicFuse<I>`

##### `impl<T> IntoParallelIterator for PanicFuse<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for PanicFuse<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for PanicFuse<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Fuse<'a>`

```rust
struct Fuse<'a>(&'a std::sync::atomic::AtomicBool);
```

Helper that sets a bool to `true` if dropped while unwinding.

#### Implementations

- `fn panicked(self: &Self) -> bool`

#### Trait Implementations

##### `impl<'a> Clone for Fuse<'a>`

- `fn clone(self: &Self) -> Fuse<'a>` — [`Fuse`](#fuse)

##### `impl<'a> Drop for Fuse<'a>`

- `fn drop(self: &mut Self)`

##### `impl<T> IntoEither for Fuse<'a>`

##### `impl<T> Pointable for Fuse<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `PanicFuseProducer<'a, P>`

```rust
struct PanicFuseProducer<'a, P> {
    base: P,
    fuse: Fuse<'a>,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for PanicFuseProducer<'a, P>`

##### `impl<T> Pointable for PanicFuseProducer<'a, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'a, P> Producer for PanicFuseProducer<'a, P>`

- `type Item = <P as Producer>::Item`

- `type IntoIter = PanicFuseIter<'a, <P as Producer>::IntoIter>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<G>(self: Self, folder: G) -> G`

### `PanicFuseIter<'a, I>`

```rust
struct PanicFuseIter<'a, I> {
    base: I,
    fuse: Fuse<'a>,
}
```

#### Trait Implementations

##### `impl<'a, I> DoubleEndedIterator for PanicFuseIter<'a, I>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, I> ExactSizeIterator for PanicFuseIter<'a, I>`

- `fn len(self: &Self) -> usize`

##### `impl<T> IntoEither for PanicFuseIter<'a, I>`

##### `impl<I> IntoIterator for PanicFuseIter<'a, I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, I> Iterator for PanicFuseIter<'a, I>`

- `type Item = <I as Iterator>::Item`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for PanicFuseIter<'a, I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `PanicFuseConsumer<'a, C>`

```rust
struct PanicFuseConsumer<'a, C> {
    base: C,
    fuse: Fuse<'a>,
}
```

#### Trait Implementations

##### `impl<'a, T, C> Consumer for PanicFuseConsumer<'a, C>`

- `type Folder = PanicFuseFolder<'a, <C as Consumer>::Folder>`

- `type Reducer = PanicFuseReducer<'a, <C as Consumer>::Reducer>`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for PanicFuseConsumer<'a, C>`

##### `impl<T> Pointable for PanicFuseConsumer<'a, C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'a, T, C> UnindexedConsumer for PanicFuseConsumer<'a, C>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `PanicFuseFolder<'a, C>`

```rust
struct PanicFuseFolder<'a, C> {
    base: C,
    fuse: Fuse<'a>,
}
```

#### Trait Implementations

##### `impl<'a, T, C> Folder for PanicFuseFolder<'a, C>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for PanicFuseFolder<'a, C>`

##### `impl<T> Pointable for PanicFuseFolder<'a, C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `PanicFuseReducer<'a, C>`

```rust
struct PanicFuseReducer<'a, C> {
    base: C,
    _fuse: Fuse<'a>,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for PanicFuseReducer<'a, C>`

##### `impl<T> Pointable for PanicFuseReducer<'a, C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'a, T, C> Reducer for PanicFuseReducer<'a, C>`

- `fn reduce(self: Self, left: T, right: T) -> T`

