*[rayon](../../index.md) / [iter](../index.md) / [fold](index.md)*

---

# Module `fold`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Fold`](#fold) | struct | `Fold` is an iterator that applies a function over an iterator producing a single value. |
| [`FoldConsumer`](#foldconsumer) | struct |  |
| [`FoldFolder`](#foldfolder) | struct |  |
| [`FoldWith`](#foldwith) | struct | `FoldWith` is an iterator that applies a function over an iterator producing a single value. |
| [`FoldWithConsumer`](#foldwithconsumer) | struct |  |

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

- <span id="fold-new"></span>`fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for Fold<I, ID, F>`

- <span id="fold-clone"></span>`fn clone(&self) -> Fold<I, ID, F>` — [`Fold`](../index.md)

##### `impl<I: Debug, ID, F> Debug for Fold<I, ID, F>`

- <span id="fold-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Fold<I, ID, F>`

##### `impl<T> IntoParallelIterator for Fold<I, ID, F>`

- <span id="fold-iter"></span>`type Iter = T`

- <span id="fold-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="fold-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, ID, F> ParallelIterator for Fold<I, ID, F>`

- <span id="fold-item"></span>`type Item = U`

- <span id="fold-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for Fold<I, ID, F>`

- <span id="fold-align"></span>`const ALIGN: usize`

- <span id="fold-init"></span>`type Init = T`

- <span id="fold-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fold-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fold-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fold-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="foldconsumer-folder"></span>`type Folder = FoldFolder<'r, <C as Consumer>::Folder, U, F>`

- <span id="foldconsumer-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="foldconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="foldconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="foldconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="foldconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FoldConsumer<'c, C, ID, F>`

##### `impl<T> Pointable for FoldConsumer<'c, C, ID, F>`

- <span id="foldconsumer-align"></span>`const ALIGN: usize`

- <span id="foldconsumer-init"></span>`type Init = T`

- <span id="foldconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'r, U, T, C, ID, F> UnindexedConsumer for FoldConsumer<'r, C, ID, F>`

- <span id="foldconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="foldconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

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

- <span id="foldfolder-result"></span>`type Result = <C as Folder>::Result`

- <span id="foldfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="foldfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="foldfolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="foldfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FoldFolder<'r, C, ID, F>`

##### `impl<T> Pointable for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-align"></span>`const ALIGN: usize`

- <span id="foldfolder-init"></span>`type Init = T`

- <span id="foldfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldfolder-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="foldwith-new"></span>`fn new(base: I, item: U, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, U: clone::Clone, F: clone::Clone> Clone for FoldWith<I, U, F>`

- <span id="foldwith-clone"></span>`fn clone(&self) -> FoldWith<I, U, F>` — [`FoldWith`](../index.md)

##### `impl<I: Debug, U: Debug, F> Debug for FoldWith<I, U, F>`

- <span id="foldwith-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FoldWith<I, U, F>`

##### `impl<T> IntoParallelIterator for FoldWith<I, U, F>`

- <span id="foldwith-iter"></span>`type Iter = T`

- <span id="foldwith-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldwith-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, F> ParallelIterator for FoldWith<I, U, F>`

- <span id="foldwith-item"></span>`type Item = U`

- <span id="foldwith-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for FoldWith<I, U, F>`

- <span id="foldwith-align"></span>`const ALIGN: usize`

- <span id="foldwith-init"></span>`type Init = T`

- <span id="foldwith-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldwith-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldwith-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldwith-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="foldwithconsumer-folder"></span>`type Folder = FoldFolder<'r, <C as Consumer>::Folder, U, F>`

- <span id="foldwithconsumer-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="foldwithconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="foldwithconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="foldwithconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="foldwithconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FoldWithConsumer<'c, C, U, F>`

##### `impl<T> Pointable for FoldWithConsumer<'c, C, U, F>`

- <span id="foldwithconsumer-align"></span>`const ALIGN: usize`

- <span id="foldwithconsumer-init"></span>`type Init = T`

- <span id="foldwithconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldwithconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldwithconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldwithconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'r, U, T, C, F> UnindexedConsumer for FoldWithConsumer<'r, C, U, F>`

- <span id="foldwithconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="foldwithconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

