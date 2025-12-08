*[rayon](../../index.md) / [iter](../index.md) / [try_fold](index.md)*

---

# Module `try_fold`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryFold`](#tryfold) | struct | `TryFold` is an iterator that applies a function over an iterator producing a single value. |
| [`TryFoldConsumer`](#tryfoldconsumer) | struct |  |
| [`TryFoldFolder`](#tryfoldfolder) | struct |  |
| [`TryFoldWith`](#tryfoldwith) | struct | `TryFoldWith` is an iterator that applies a function over an iterator producing a single value. |
| [`TryFoldWithConsumer`](#tryfoldwithconsumer) | struct |  |

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

- <span id="tryfold-new"></span>`fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, U: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for TryFold<I, U, ID, F>`

- <span id="tryfold-clone"></span>`fn clone(&self) -> TryFold<I, U, ID, F>` — [`TryFold`](../index.md)

##### `impl<U, I: ParallelIterator + Debug, ID, F> Debug for TryFold<I, U, ID, F>`

- <span id="tryfold-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TryFold<I, U, ID, F>`

##### `impl<T> IntoParallelIterator for TryFold<I, U, ID, F>`

- <span id="tryfold-iter"></span>`type Iter = T`

- <span id="tryfold-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="tryfold-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, ID, F> ParallelIterator for TryFold<I, U, ID, F>`

- <span id="tryfold-item"></span>`type Item = U`

- <span id="tryfold-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for TryFold<I, U, ID, F>`

- <span id="tryfold-align"></span>`const ALIGN: usize`

- <span id="tryfold-init"></span>`type Init = T`

- <span id="tryfold-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfold-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfold-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfold-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="tryfoldconsumer-folder"></span>`type Folder = TryFoldFolder<'r, <C as Consumer>::Folder, U, F>`

- <span id="tryfoldconsumer-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="tryfoldconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="tryfoldconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="tryfoldconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="tryfoldconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for TryFoldConsumer<'c, U, C, ID, F>`

##### `impl<T> Pointable for TryFoldConsumer<'c, U, C, ID, F>`

- <span id="tryfoldconsumer-align"></span>`const ALIGN: usize`

- <span id="tryfoldconsumer-init"></span>`type Init = T`

- <span id="tryfoldconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfoldconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfoldconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfoldconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'r, U, T, C, ID, F> UnindexedConsumer for TryFoldConsumer<'r, U, C, ID, F>`

- <span id="tryfoldconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="tryfoldconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

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

- <span id="tryfoldfolder-result"></span>`type Result = <C as Folder>::Result`

- <span id="tryfoldfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="tryfoldfolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="tryfoldfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for TryFoldFolder<'r, C, U, F>`

##### `impl<T> Pointable for TryFoldFolder<'r, C, U, F>`

- <span id="tryfoldfolder-align"></span>`const ALIGN: usize`

- <span id="tryfoldfolder-init"></span>`type Init = T`

- <span id="tryfoldfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfoldfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfoldfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfoldfolder-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="tryfoldwith-new"></span>`fn new(base: I, item: <U as >::Output, fold_op: F) -> Self` — [`Try`](../private/index.md)

#### Trait Implementations

##### `impl<I: clone::Clone, U: clone::Clone + Try, F: clone::Clone> Clone for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-clone"></span>`fn clone(&self) -> TryFoldWith<I, U, F>` — [`TryFoldWith`](../index.md)

##### `impl<I, U, F> Debug for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TryFoldWith<I, U, F>`

##### `impl<T> IntoParallelIterator for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-iter"></span>`type Iter = T`

- <span id="tryfoldwith-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="tryfoldwith-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, F> ParallelIterator for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-item"></span>`type Item = U`

- <span id="tryfoldwith-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-align"></span>`const ALIGN: usize`

- <span id="tryfoldwith-init"></span>`type Init = T`

- <span id="tryfoldwith-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfoldwith-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfoldwith-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfoldwith-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="tryfoldwithconsumer-folder"></span>`type Folder = TryFoldFolder<'r, <C as Consumer>::Folder, U, F>`

- <span id="tryfoldwithconsumer-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="tryfoldwithconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="tryfoldwithconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="tryfoldwithconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="tryfoldwithconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for TryFoldWithConsumer<'c, C, U, F>`

##### `impl<T> Pointable for TryFoldWithConsumer<'c, C, U, F>`

- <span id="tryfoldwithconsumer-align"></span>`const ALIGN: usize`

- <span id="tryfoldwithconsumer-init"></span>`type Init = T`

- <span id="tryfoldwithconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfoldwithconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfoldwithconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfoldwithconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'r, U, T, C, F> UnindexedConsumer for TryFoldWithConsumer<'r, C, U, F>`

- <span id="tryfoldwithconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="tryfoldwithconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

