*[rayon](../../index.md) / [iter](../index.md) / [inspect](index.md)*

---

# Module `inspect`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Inspect`](#inspect) | struct | `Inspect` is an iterator that calls a function with a reference to each element before yielding it. |
| [`InspectProducer`](#inspectproducer) | struct |  |
| [`InspectConsumer`](#inspectconsumer) | struct |  |
| [`InspectFolder`](#inspectfolder) | struct |  |

## Structs

### `Inspect<I, F>`

```rust
struct Inspect<I, F> {
    base: I,
    inspect_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/inspect.rs:15-18`](../../../../.source_1765210505/rayon-1.11.0/src/iter/inspect.rs#L15-L18)*

`Inspect` is an iterator that calls a function with a reference to each
element before yielding it.

This struct is created by the `inspect()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="inspect-new"></span>`fn new(base: I, inspect_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Inspect<I, F>`

- <span id="inspect-clone"></span>`fn clone(&self) -> Inspect<I, F>` — [`Inspect`](#inspect)

##### `impl<I: Debug, F> Debug for Inspect<I, F>`

- <span id="inspect-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> IndexedParallelIterator for Inspect<I, F>`

- <span id="inspect-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="inspect-len"></span>`fn len(&self) -> usize`

- <span id="inspect-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Inspect<I, F>`

##### `impl<T> IntoParallelIterator for Inspect<I, F>`

- <span id="inspect-type-iter"></span>`type Iter = T`

- <span id="inspect-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="inspect-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for Inspect<I, F>`

- <span id="inspect-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="inspect-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="inspect-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Inspect<I, F>`

- <span id="inspect-const-align"></span>`const ALIGN: usize`

- <span id="inspect-type-init"></span>`type Init = T`

- <span id="inspect-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inspect-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inspect-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inspect-drop"></span>`unsafe fn drop(ptr: usize)`

### `InspectProducer<'f, P, F>`

```rust
struct InspectProducer<'f, P, F> {
    base: P,
    inspect_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/inspect.rs:107-110`](../../../../.source_1765210505/rayon-1.11.0/src/iter/inspect.rs#L107-L110)*

#### Trait Implementations

##### `impl<T> IntoEither for InspectProducer<'f, P, F>`

##### `impl<T> Pointable for InspectProducer<'f, P, F>`

- <span id="inspectproducer-const-align"></span>`const ALIGN: usize`

- <span id="inspectproducer-type-init"></span>`type Init = T`

- <span id="inspectproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inspectproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inspectproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inspectproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'f, P, F> Producer for InspectProducer<'f, P, F>`

- <span id="inspectproducer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="inspectproducer-type-intoiter"></span>`type IntoIter = Inspect<<P as Producer>::IntoIter, &'f F>`

- <span id="inspectproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="inspectproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="inspectproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="inspectproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="inspectproducer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

### `InspectConsumer<'f, C, F>`

```rust
struct InspectConsumer<'f, C, F> {
    base: C,
    inspect_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/inspect.rs:161-164`](../../../../.source_1765210505/rayon-1.11.0/src/iter/inspect.rs#L161-L164)*

#### Implementations

- <span id="inspectconsumer-new"></span>`fn new(base: C, inspect_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, C, F> Consumer for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-type-folder"></span>`type Folder = InspectFolder<'f, <C as Consumer>::Folder, F>`

- <span id="inspectconsumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="inspectconsumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="inspectconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="inspectconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="inspectconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for InspectConsumer<'f, C, F>`

##### `impl<T> Pointable for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-const-align"></span>`const ALIGN: usize`

- <span id="inspectconsumer-type-init"></span>`type Init = T`

- <span id="inspectconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inspectconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inspectconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inspectconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'f, T, C, F> UnindexedConsumer for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="inspectconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `InspectFolder<'f, C, F>`

```rust
struct InspectFolder<'f, C, F> {
    base: C,
    inspect_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/inspect.rs:216-219`](../../../../.source_1765210505/rayon-1.11.0/src/iter/inspect.rs#L216-L219)*

#### Trait Implementations

##### `impl<'f, T, C, F> Folder for InspectFolder<'f, C, F>`

- <span id="inspectfolder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="inspectfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="inspectfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="inspectfolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="inspectfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for InspectFolder<'f, C, F>`

##### `impl<T> Pointable for InspectFolder<'f, C, F>`

- <span id="inspectfolder-const-align"></span>`const ALIGN: usize`

- <span id="inspectfolder-type-init"></span>`type Init = T`

- <span id="inspectfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inspectfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inspectfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inspectfolder-drop"></span>`unsafe fn drop(ptr: usize)`

