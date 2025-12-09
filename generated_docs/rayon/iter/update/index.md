*[rayon](../../index.md) / [iter](../index.md) / [update](index.md)*

---

# Module `update`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Update`](#update) | struct | `Update` is an iterator that mutates the elements of an underlying iterator before they are yielded. |
| [`UpdateProducer`](#updateproducer) | struct |  |
| [`UpdateConsumer`](#updateconsumer) | struct |  |
| [`UpdateFolder`](#updatefolder) | struct |  |
| [`UpdateSeq`](#updateseq) | struct | Standard Update adaptor, based on `itertools::adaptors::Update` |
| [`apply`](#apply) | fn |  |

## Structs

### `Update<I, F>`

```rust
struct Update<I, F> {
    base: I,
    update_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:14-17`](../../../../.source_1765210505/rayon-1.11.0/src/iter/update.rs#L14-L17)*

`Update` is an iterator that mutates the elements of an
underlying iterator before they are yielded.

This struct is created by the `update()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="update-new"></span>`fn new(base: I, update_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Update<I, F>`

- <span id="update-clone"></span>`fn clone(&self) -> Update<I, F>` — [`Update`](#update)

##### `impl<I: Debug, F> Debug for Update<I, F>`

- <span id="update-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> IndexedParallelIterator for Update<I, F>`

- <span id="update-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="update-len"></span>`fn len(&self) -> usize`

- <span id="update-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Update<I, F>`

##### `impl<T> IntoParallelIterator for Update<I, F>`

- <span id="update-type-iter"></span>`type Iter = T`

- <span id="update-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="update-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for Update<I, F>`

- <span id="update-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="update-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="update-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Update<I, F>`

- <span id="update-const-align"></span>`const ALIGN: usize`

- <span id="update-type-init"></span>`type Init = T`

- <span id="update-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="update-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="update-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="update-drop"></span>`unsafe fn drop(ptr: usize)`

### `UpdateProducer<'f, P, F>`

```rust
struct UpdateProducer<'f, P, F> {
    base: P,
    update_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:106-109`](../../../../.source_1765210505/rayon-1.11.0/src/iter/update.rs#L106-L109)*

#### Trait Implementations

##### `impl<T> IntoEither for UpdateProducer<'f, P, F>`

##### `impl<T> Pointable for UpdateProducer<'f, P, F>`

- <span id="updateproducer-const-align"></span>`const ALIGN: usize`

- <span id="updateproducer-type-init"></span>`type Init = T`

- <span id="updateproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="updateproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="updateproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="updateproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'f, P, F> Producer for UpdateProducer<'f, P, F>`

- <span id="updateproducer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="updateproducer-type-intoiter"></span>`type IntoIter = UpdateSeq<<P as Producer>::IntoIter, &'f F>`

- <span id="updateproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="updateproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="updateproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="updateproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="updateproducer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

### `UpdateConsumer<'f, C, F>`

```rust
struct UpdateConsumer<'f, C, F> {
    base: C,
    update_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:162-165`](../../../../.source_1765210505/rayon-1.11.0/src/iter/update.rs#L162-L165)*

#### Implementations

- <span id="updateconsumer-new"></span>`fn new(base: C, update_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, C, F> Consumer for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-type-folder"></span>`type Folder = UpdateFolder<'f, <C as Consumer>::Folder, F>`

- <span id="updateconsumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="updateconsumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="updateconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="updateconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="updateconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for UpdateConsumer<'f, C, F>`

##### `impl<T> Pointable for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-const-align"></span>`const ALIGN: usize`

- <span id="updateconsumer-type-init"></span>`type Init = T`

- <span id="updateconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="updateconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="updateconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="updateconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'f, T, C, F> UnindexedConsumer for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="updateconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `UpdateFolder<'f, C, F>`

```rust
struct UpdateFolder<'f, C, F> {
    base: C,
    update_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:217-220`](../../../../.source_1765210505/rayon-1.11.0/src/iter/update.rs#L217-L220)*

#### Trait Implementations

##### `impl<'f, T, C, F> Folder for UpdateFolder<'f, C, F>`

- <span id="updatefolder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="updatefolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="updatefolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="updatefolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="updatefolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for UpdateFolder<'f, C, F>`

##### `impl<T> Pointable for UpdateFolder<'f, C, F>`

- <span id="updatefolder-const-align"></span>`const ALIGN: usize`

- <span id="updatefolder-type-init"></span>`type Init = T`

- <span id="updatefolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="updatefolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="updatefolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="updatefolder-drop"></span>`unsafe fn drop(ptr: usize)`

### `UpdateSeq<I, F>`

```rust
struct UpdateSeq<I, F> {
    base: I,
    update_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:268-271`](../../../../.source_1765210505/rayon-1.11.0/src/iter/update.rs#L268-L271)*

Standard Update adaptor, based on `itertools::adaptors::Update`

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for UpdateSeq<I, F>`

- <span id="updateseq-clone"></span>`fn clone(&self) -> UpdateSeq<I, F>` — [`UpdateSeq`](#updateseq)

##### `impl<I: fmt::Debug, F: fmt::Debug> Debug for UpdateSeq<I, F>`

- <span id="updateseq-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> DoubleEndedIterator for UpdateSeq<I, F>`

- <span id="updateseq-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I, F> ExactSizeIterator for UpdateSeq<I, F>`

##### `impl<T> IntoEither for UpdateSeq<I, F>`

##### `impl<I> IntoIterator for UpdateSeq<I, F>`

- <span id="updateseq-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="updateseq-type-intoiter"></span>`type IntoIter = I`

- <span id="updateseq-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for UpdateSeq<I, F>`

- <span id="updateseq-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="updateseq-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="updateseq-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="updateseq-fold"></span>`fn fold<Acc, G>(self, init: Acc, g: G) -> Acc`

- <span id="updateseq-collect"></span>`fn collect<C>(self) -> C`

##### `impl<T> Pointable for UpdateSeq<I, F>`

- <span id="updateseq-const-align"></span>`const ALIGN: usize`

- <span id="updateseq-type-init"></span>`type Init = T`

- <span id="updateseq-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="updateseq-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="updateseq-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="updateseq-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `apply`

```rust
fn apply<T>(update_op: impl Fn(&mut T)) -> impl Fn(T) -> T
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:222-227`](../../../../.source_1765210505/rayon-1.11.0/src/iter/update.rs#L222-L227)*

