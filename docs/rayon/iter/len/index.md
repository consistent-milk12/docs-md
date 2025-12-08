*[rayon](../../index.md) / [iter](../index.md) / [len](index.md)*

---

# Module `len`

## Structs

### `MinLen<I>`

```rust
struct MinLen<I> {
    base: I,
    min: usize,
}
```

`MinLen` is an iterator that imposes a minimum length on iterator splits.
This struct is created by the `with_min_len()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, min: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for MinLen<I>`

- `fn clone(self: &Self) -> MinLen<I>` — [`MinLen`](../index.md)

##### `impl<I: $crate::fmt::Debug> Debug for MinLen<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for MinLen<I>`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for MinLen<I>`

##### `impl<T> IntoParallelIterator for MinLen<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for MinLen<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for MinLen<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MinLenProducer<P>`

```rust
struct MinLenProducer<P> {
    base: P,
    min: usize,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for MinLenProducer<P>`

##### `impl<T> Pointable for MinLenProducer<P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for MinLenProducer<P>`

- `type Item = <P as Producer>::Item`

- `type IntoIter = <P as Producer>::IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

### `MaxLen<I>`

```rust
struct MaxLen<I> {
    base: I,
    max: usize,
}
```

`MaxLen` is an iterator that imposes a maximum length on iterator splits.
This struct is created by the `with_max_len()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, max: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for MaxLen<I>`

- `fn clone(self: &Self) -> MaxLen<I>` — [`MaxLen`](../index.md)

##### `impl<I: $crate::fmt::Debug> Debug for MaxLen<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for MaxLen<I>`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for MaxLen<I>`

##### `impl<T> IntoParallelIterator for MaxLen<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for MaxLen<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for MaxLen<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MaxLenProducer<P>`

```rust
struct MaxLenProducer<P> {
    base: P,
    max: usize,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for MaxLenProducer<P>`

##### `impl<T> Pointable for MaxLenProducer<P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for MaxLenProducer<P>`

- `type Item = <P as Producer>::Item`

- `type IntoIter = <P as Producer>::IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

