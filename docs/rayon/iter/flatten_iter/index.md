*[rayon](../../index.md) / [iter](../index.md) / [flatten_iter](index.md)*

---

# Module `flatten_iter`

## Structs

### `FlattenIter<I>`

```rust
struct FlattenIter<I> {
    base: I,
}
```

`FlattenIter` turns each element to a serial iterator, then flattens these iterators
together. This struct is created by the `flatten_iter()` method on [`ParallelIterator`](../index.md).


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for FlattenIter<I>`

- `fn clone(self: &Self) -> FlattenIter<I>` — [`FlattenIter`](#flatteniter)

##### `impl<I: $crate::fmt::Debug> Debug for FlattenIter<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for FlattenIter<I>`

##### `impl<T> IntoParallelIterator for FlattenIter<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for FlattenIter<I>`

- `type Item = <<I as ParallelIterator>::Item as IntoIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for FlattenIter<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FlattenIterConsumer<C>`

```rust
struct FlattenIterConsumer<C> {
    base: C,
}
```

#### Implementations

- `fn new(base: C) -> Self`

#### Trait Implementations

##### `impl<T, C> Consumer for FlattenIterConsumer<C>`

- `type Folder = FlattenIterFolder<<C as Consumer>::Folder>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FlattenIterConsumer<C>`

##### `impl<T> Pointable for FlattenIterConsumer<C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T, C> UnindexedConsumer for FlattenIterConsumer<C>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FlattenIterFolder<C>`

```rust
struct FlattenIterFolder<C> {
    base: C,
}
```

#### Trait Implementations

##### `impl<T, C> Folder for FlattenIterFolder<C>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FlattenIterFolder<C>`

##### `impl<T> Pointable for FlattenIterFolder<C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

