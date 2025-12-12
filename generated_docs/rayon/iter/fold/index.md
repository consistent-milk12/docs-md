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

*Defined in [`rayon-1.11.0/src/iter/fold.rs:22-26`](../../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L22-L26)*

`Fold` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="fold-new"></span>`fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for Fold<I, ID, F>`

- <span id="fold-clone"></span>`fn clone(&self) -> Fold<I, ID, F>` — [`Fold`](#fold)

##### `impl<I: Debug, ID, F> Debug for Fold<I, ID, F>`

- <span id="fold-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for Fold<I, ID, F>`

##### `impl IntoParallelIterator for Fold<I, ID, F>`

- <span id="fold-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="fold-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="fold-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, ID, F> ParallelIterator for Fold<I, ID, F>`

- <span id="fold-paralleliterator-type-item"></span>`type Item = U`

- <span id="fold-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for Fold<I, ID, F>`

- <span id="fold-pointable-const-align"></span>`const ALIGN: usize`

- <span id="fold-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/iter/fold.rs:56-60`](../../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L56-L60)*

#### Trait Implementations

##### `impl<T, C, ID, F> Consumer for FoldConsumer<'r, C, ID, F>`

- <span id="foldconsumer-consumer-type-folder"></span>`type Folder = FoldFolder<'r, <C as Consumer>::Folder, U, F>`

- <span id="foldconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="foldconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="foldconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for FoldConsumer<'c, C, ID, F>`

##### `impl Pointable for FoldConsumer<'c, C, ID, F>`

- <span id="foldconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldconsumer-pointable-type-init"></span>`type Init = T`

- <span id="foldconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, C, ID, F> UnindexedConsumer for FoldConsumer<'r, C, ID, F>`

- <span id="foldconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="foldconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FoldFolder<'r, C, ID, F>`

```rust
struct FoldFolder<'r, C, ID, F> {
    base: C,
    fold_op: &'r F,
    item: ID,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold.rs:117-121`](../../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L117-L121)*

#### Trait Implementations

##### `impl<C, ID, F, T> Folder for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="foldfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="foldfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="foldfolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="foldfolder-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for FoldFolder<'r, C, ID, F>`

##### `impl Pointable for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldfolder-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/iter/fold.rs:191-195`](../../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L191-L195)*

`FoldWith` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold_with()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="foldwith-new"></span>`fn new(base: I, item: U, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, U: clone::Clone, F: clone::Clone> Clone for FoldWith<I, U, F>`

- <span id="foldwith-clone"></span>`fn clone(&self) -> FoldWith<I, U, F>` — [`FoldWith`](#foldwith)

##### `impl<I: Debug, U: Debug, F> Debug for FoldWith<I, U, F>`

- <span id="foldwith-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for FoldWith<I, U, F>`

##### `impl IntoParallelIterator for FoldWith<I, U, F>`

- <span id="foldwith-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="foldwith-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldwith-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, F> ParallelIterator for FoldWith<I, U, F>`

- <span id="foldwith-paralleliterator-type-item"></span>`type Item = U`

- <span id="foldwith-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for FoldWith<I, U, F>`

- <span id="foldwith-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldwith-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/iter/fold.rs:227-231`](../../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L227-L231)*

#### Trait Implementations

##### `impl<U, T, C, F> Consumer for FoldWithConsumer<'r, C, U, F>`

- <span id="foldwithconsumer-consumer-type-folder"></span>`type Folder = FoldFolder<'r, <C as Consumer>::Folder, U, F>`

- <span id="foldwithconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="foldwithconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="foldwithconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldwithconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldwithconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for FoldWithConsumer<'c, C, U, F>`

##### `impl Pointable for FoldWithConsumer<'c, C, U, F>`

- <span id="foldwithconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldwithconsumer-pointable-type-init"></span>`type Init = T`

- <span id="foldwithconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldwithconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldwithconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldwithconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U, T, C, F> UnindexedConsumer for FoldWithConsumer<'r, C, U, F>`

- <span id="foldwithconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="foldwithconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

