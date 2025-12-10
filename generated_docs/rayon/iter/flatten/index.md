*[rayon](../../index.md) / [iter](../index.md) / [flatten](index.md)*

---

# Module `flatten`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Flatten`](#flatten) | struct | `Flatten` turns each element to a parallel iterator, then flattens these iterators together. |
| [`FlattenConsumer`](#flattenconsumer) | struct |  |
| [`FlattenFolder`](#flattenfolder) | struct |  |

## Structs

### `Flatten<I>`

```rust
struct Flatten<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten.rs:10-12`](../../../../.source_1765210505/rayon-1.11.0/src/iter/flatten.rs#L10-L12)*

`Flatten` turns each element to a parallel iterator, then flattens these iterators
together. This struct is created by the `flatten()` method on [`ParallelIterator`](../index.md).


#### Implementations

- <span id="flatten-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Flatten<I>`

- <span id="flatten-clone"></span>`fn clone(&self) -> Flatten<I>` — [`Flatten`](#flatten)

##### `impl<I: fmt::Debug> Debug for Flatten<I>`

- <span id="flatten-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Flatten<I>`

##### `impl<T> IntoParallelIterator for Flatten<I>`

- <span id="flatten-type-iter"></span>`type Iter = T`

- <span id="flatten-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatten-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Flatten<I>`

- <span id="flatten-type-item"></span>`type Item = <<I as ParallelIterator>::Item as IntoParallelIterator>::Item`

- <span id="flatten-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl<T> Pointable for Flatten<I>`

- <span id="flatten-const-align"></span>`const ALIGN: usize`

- <span id="flatten-type-init"></span>`type Init = T`

- <span id="flatten-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatten-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatten-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatten-drop"></span>`unsafe fn drop(ptr: usize)`

### `FlattenConsumer<C>`

```rust
struct FlattenConsumer<C> {
    base: C,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten.rs:39-41`](../../../../.source_1765210505/rayon-1.11.0/src/iter/flatten.rs#L39-L41)*

#### Implementations

- <span id="flattenconsumer-new"></span>`fn new(base: C) -> Self`

#### Trait Implementations

##### `impl<T, C> Consumer for FlattenConsumer<C>`

- <span id="flattenconsumer-type-folder"></span>`type Folder = FlattenFolder<C, <C as Consumer>::Result>`

- <span id="flattenconsumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="flattenconsumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flattenconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flattenconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flattenconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FlattenConsumer<C>`

##### `impl<T> Pointable for FlattenConsumer<C>`

- <span id="flattenconsumer-const-align"></span>`const ALIGN: usize`

- <span id="flattenconsumer-type-init"></span>`type Init = T`

- <span id="flattenconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flattenconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flattenconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flattenconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, C> UnindexedConsumer for FlattenConsumer<C>`

- <span id="flattenconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="flattenconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FlattenFolder<C, R>`

```rust
struct FlattenFolder<C, R> {
    base: C,
    previous: Option<R>,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten.rs:93-96`](../../../../.source_1765210505/rayon-1.11.0/src/iter/flatten.rs#L93-L96)*

#### Trait Implementations

##### `impl<T, C> Folder for FlattenFolder<C, <C as >::Result>`

- <span id="flattenfolder-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flattenfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="flattenfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="flattenfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FlattenFolder<C, R>`

##### `impl<T> Pointable for FlattenFolder<C, R>`

- <span id="flattenfolder-const-align"></span>`const ALIGN: usize`

- <span id="flattenfolder-type-init"></span>`type Init = T`

- <span id="flattenfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flattenfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flattenfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flattenfolder-drop"></span>`unsafe fn drop(ptr: usize)`

