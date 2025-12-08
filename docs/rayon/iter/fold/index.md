*[rayon](../../index.md) / [iter](../index.md) / [fold](index.md)*

---

# Module `fold`

## Structs

### `Fold<I, ID, F>`

```rust
struct Fold<I, ID, F> {
    base: I,
    identity: ID,
    fold_op: F,
}
```

`Fold` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, ID: $crate::clone::Clone, F: $crate::clone::Clone> Clone for Fold<I, ID, F>`

- `fn clone(self: &Self) -> Fold<I, ID, F>` — [`Fold`](../index.md)

##### `impl<I: Debug, ID, F> Debug for Fold<I, ID, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Fold<I, ID, F>`

##### `impl<T> IntoParallelIterator for Fold<I, ID, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<U, I, ID, F> ParallelIterator for Fold<I, ID, F>`

- `type Item = U`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for Fold<I, ID, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FoldConsumer<'c, C, ID, F>`

```rust
struct FoldConsumer<'c, C, ID, F> {
    base: C,
    fold_op: &'c F,
    identity: &'c ID,
}
```

#### Trait Implementations

##### `impl<'r, U, T, C, ID, F> Consumer for FoldConsumer<'r, C, ID, F>`

- `type Folder = FoldFolder<'r, <C as Consumer>::Folder, U, F>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FoldConsumer<'c, C, ID, F>`

##### `impl<T> Pointable for FoldConsumer<'c, C, ID, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'r, U, T, C, ID, F> UnindexedConsumer for FoldConsumer<'r, C, ID, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FoldFolder<'r, C, ID, F>`

```rust
struct FoldFolder<'r, C, ID, F> {
    base: C,
    fold_op: &'r F,
    item: ID,
}
```

#### Trait Implementations

##### `impl<'r, C, ID, F, T> Folder for FoldFolder<'r, C, ID, F>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FoldFolder<'r, C, ID, F>`

##### `impl<T> Pointable for FoldFolder<'r, C, ID, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FoldWith<I, U, F>`

```rust
struct FoldWith<I, U, F> {
    base: I,
    item: U,
    fold_op: F,
}
```

`FoldWith` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold_with()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, item: U, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, U: $crate::clone::Clone, F: $crate::clone::Clone> Clone for FoldWith<I, U, F>`

- `fn clone(self: &Self) -> FoldWith<I, U, F>` — [`FoldWith`](../index.md)

##### `impl<I: Debug, U: Debug, F> Debug for FoldWith<I, U, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FoldWith<I, U, F>`

##### `impl<T> IntoParallelIterator for FoldWith<I, U, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<U, I, F> ParallelIterator for FoldWith<I, U, F>`

- `type Item = U`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for FoldWith<I, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FoldWithConsumer<'c, C, U, F>`

```rust
struct FoldWithConsumer<'c, C, U, F> {
    base: C,
    item: U,
    fold_op: &'c F,
}
```

#### Trait Implementations

##### `impl<'r, U, T, C, F> Consumer for FoldWithConsumer<'r, C, U, F>`

- `type Folder = FoldFolder<'r, <C as Consumer>::Folder, U, F>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FoldWithConsumer<'c, C, U, F>`

##### `impl<T> Pointable for FoldWithConsumer<'c, C, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'r, U, T, C, F> UnindexedConsumer for FoldWithConsumer<'r, C, U, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

