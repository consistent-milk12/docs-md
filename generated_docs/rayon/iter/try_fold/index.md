*[rayon](../../index.md) / [iter](../index.md) / [try_fold](index.md)*

---

# Module `try_fold`

## Structs

### `TryFold<I, U, ID, F>`

```rust
struct TryFold<I, U, ID, F> {
    base: I,
    identity: ID,
    fold_op: F,
    marker: std::marker::PhantomData<U>,
}
```

`TryFold` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `try_fold()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, U: $crate::clone::Clone, ID: $crate::clone::Clone, F: $crate::clone::Clone> Clone for TryFold<I, U, ID, F>`

- `fn clone(self: &Self) -> TryFold<I, U, ID, F>` — [`TryFold`](../index.md)

##### `impl<U, I: ParallelIterator + Debug, ID, F> Debug for TryFold<I, U, ID, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TryFold<I, U, ID, F>`

##### `impl<T> IntoParallelIterator for TryFold<I, U, ID, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<U, I, ID, F> ParallelIterator for TryFold<I, U, ID, F>`

- `type Item = U`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for TryFold<I, U, ID, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `TryFoldConsumer<'c, U, C, ID, F>`

```rust
struct TryFoldConsumer<'c, U, C, ID, F> {
    base: C,
    identity: &'c ID,
    fold_op: &'c F,
    marker: std::marker::PhantomData<U>,
}
```

#### Trait Implementations

##### `impl<'r, U, T, C, ID, F> Consumer for TryFoldConsumer<'r, U, C, ID, F>`

- `type Folder = TryFoldFolder<'r, <C as Consumer>::Folder, U, F>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for TryFoldConsumer<'c, U, C, ID, F>`

##### `impl<T> Pointable for TryFoldConsumer<'c, U, C, ID, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'r, U, T, C, ID, F> UnindexedConsumer for TryFoldConsumer<'r, U, C, ID, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `TryFoldFolder<'r, C, U: Try, F>`

```rust
struct TryFoldFolder<'r, C, U: Try, F> {
    base: C,
    fold_op: &'r F,
    control: std::ops::ControlFlow<<U as >::Residual, <U as >::Output>,
}
```

#### Trait Implementations

##### `impl<'r, C, U, F, T> Folder for TryFoldFolder<'r, C, U, F>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for TryFoldFolder<'r, C, U, F>`

##### `impl<T> Pointable for TryFoldFolder<'r, C, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `TryFoldWith<I, U: Try, F>`

```rust
struct TryFoldWith<I, U: Try, F> {
    base: I,
    item: <U as >::Output,
    fold_op: F,
}
```

`TryFoldWith` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `try_fold_with()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, item: <U as >::Output, fold_op: F) -> Self` — [`Try`](../private/index.md)

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, U: $crate::clone::Clone + Try, F: $crate::clone::Clone> Clone for TryFoldWith<I, U, F>`

- `fn clone(self: &Self) -> TryFoldWith<I, U, F>` — [`TryFoldWith`](../index.md)

##### `impl<I, U, F> Debug for TryFoldWith<I, U, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TryFoldWith<I, U, F>`

##### `impl<T> IntoParallelIterator for TryFoldWith<I, U, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<U, I, F> ParallelIterator for TryFoldWith<I, U, F>`

- `type Item = U`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for TryFoldWith<I, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `TryFoldWithConsumer<'c, C, U: Try, F>`

```rust
struct TryFoldWithConsumer<'c, C, U: Try, F> {
    base: C,
    item: <U as >::Output,
    fold_op: &'c F,
}
```

#### Trait Implementations

##### `impl<'r, U, T, C, F> Consumer for TryFoldWithConsumer<'r, C, U, F>`

- `type Folder = TryFoldFolder<'r, <C as Consumer>::Folder, U, F>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for TryFoldWithConsumer<'c, C, U, F>`

##### `impl<T> Pointable for TryFoldWithConsumer<'c, C, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'r, U, T, C, F> UnindexedConsumer for TryFoldWithConsumer<'r, C, U, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

