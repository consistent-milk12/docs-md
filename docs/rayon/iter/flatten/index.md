*[rayon](../../index.md) / [iter](../index.md) / [flatten](index.md)*

---

# Module `flatten`

## Structs

### `Flatten<I>`

```rust
struct Flatten<I> {
    base: I,
}
```

`Flatten` turns each element to a parallel iterator, then flattens these iterators
together. This struct is created by the `flatten()` method on [`ParallelIterator`](../index.md).


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Flatten<I>`

- `fn clone(self: &Self) -> Flatten<I>` — [`Flatten`](#flatten)

##### `impl<I: $crate::fmt::Debug> Debug for Flatten<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for Flatten<I>`

##### `impl<T> IntoParallelIterator for Flatten<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Flatten<I>`

- `type Item = <<I as ParallelIterator>::Item as IntoParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for Flatten<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FlattenConsumer<C>`

```rust
struct FlattenConsumer<C> {
    base: C,
}
```

#### Implementations

- `fn new(base: C) -> Self`

#### Trait Implementations

##### `impl<T, C> Consumer for FlattenConsumer<C>`

- `type Folder = FlattenFolder<C, <C as Consumer>::Result>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FlattenConsumer<C>`

##### `impl<T> Pointable for FlattenConsumer<C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T, C> UnindexedConsumer for FlattenConsumer<C>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FlattenFolder<C, R>`

```rust
struct FlattenFolder<C, R> {
    base: C,
    previous: Option<R>,
}
```

#### Trait Implementations

##### `impl<T, C> Folder for FlattenFolder<C, <C as >::Result>`

- `type Result = <C as Consumer>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FlattenFolder<C, R>`

##### `impl<T> Pointable for FlattenFolder<C, R>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

