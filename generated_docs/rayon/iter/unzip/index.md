*[rayon](../../index.md) / [iter](../index.md) / [unzip](index.md)*

---

# Module `unzip`

## Contents

- [Structs](#structs)
  - [`Unzip`](#unzip)
  - [`Partition`](#partition)
  - [`PartitionMap`](#partitionmap)
  - [`UnzipA`](#unzipa)
  - [`UnzipB`](#unzipb)
  - [`UnzipConsumer`](#unzipconsumer)
  - [`UnzipFolder`](#unzipfolder)
  - [`UnzipReducer`](#unzipreducer)
  - [`UnEither`](#uneither)
  - [`Collector`](#collector)
- [Traits](#traits)
  - [`UnzipOp`](#unzipop)
- [Functions](#functions)
  - [`execute`](#execute)
  - [`execute_into`](#execute_into)
  - [`unzip`](#unzip)
  - [`unzip_indexed`](#unzip_indexed)
  - [`partition`](#partition)
  - [`partition_map`](#partition_map)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Unzip`](#unzip) | struct | An `UnzipOp` that splits a tuple directly into the two consumers. |
| [`Partition`](#partition) | struct | An `UnzipOp` that routes items depending on a predicate function. |
| [`PartitionMap`](#partitionmap) | struct | An `UnzipOp` that routes items depending on how they are mapped `Either`. |
| [`UnzipA`](#unzipa) | struct | A fake iterator to intercept the `Consumer` for type `A`. |
| [`UnzipB`](#unzipb) | struct | A fake iterator to intercept the `Consumer` for type `B`. |
| [`UnzipConsumer`](#unzipconsumer) | struct | `Consumer` that unzips into two other `Consumer`s |
| [`UnzipFolder`](#unzipfolder) | struct | `Folder` that unzips into two other `Folder`s |
| [`UnzipReducer`](#unzipreducer) | struct | `Reducer` that unzips into two other `Reducer`s |
| [`UnEither`](#uneither) | struct | An `UnzipOp` that routes items depending on their `Either` variant. |
| [`Collector`](#collector) | struct | Shim to implement a one-time `ParallelExtend` using `FromParallelIterator`. |
| [`UnzipOp`](#unzipop) | trait | This trait abstracts the different ways we can "unzip" one parallel |
| [`execute`](#execute) | fn | Runs an unzip-like operation into default `ParallelExtend` collections. |
| [`execute_into`](#execute_into) | fn | Runs an unzip-like operation into `ParallelExtend` collections. |
| [`unzip`](#unzip) | fn | Unzips the items of a parallel iterator into a pair of arbitrary |
| [`unzip_indexed`](#unzip_indexed) | fn | Unzips an `IndexedParallelIterator` into two arbitrary `Consumer`s. |
| [`partition`](#partition) | fn | Partitions the items of a parallel iterator into a pair of arbitrary |
| [`partition_map`](#partition_map) | fn | Partitions and maps the items of a parallel iterator into a pair of |

## Structs

### `Unzip`

```rust
struct Unzip;
```

An `UnzipOp` that splits a tuple directly into the two consumers.

#### Trait Implementations

##### `impl<T> IntoEither for Unzip`

##### `impl<T> Pointable for Unzip`

- <span id="unzip-align"></span>`const ALIGN: usize`

- <span id="unzip-init"></span>`type Init = T`

- <span id="unzip-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzip-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzip-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzip-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<A: Send, B: Send> UnzipOp for Unzip`

- <span id="unzip-left"></span>`type Left = A`

- <span id="unzip-right"></span>`type Right = B`

- <span id="unzip-consume"></span>`fn consume<FA, FB>(&self, item: (A, B), left: FA, right: FB) -> (FA, FB)`

- <span id="unzip-indexable"></span>`fn indexable() -> bool`

### `Partition<P>`

```rust
struct Partition<P> {
    predicate: P,
}
```

An `UnzipOp` that routes items depending on a predicate function.

#### Trait Implementations

##### `impl<T> IntoEither for Partition<P>`

##### `impl<T> Pointable for Partition<P>`

- <span id="partition-align"></span>`const ALIGN: usize`

- <span id="partition-init"></span>`type Init = T`

- <span id="partition-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="partition-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="partition-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="partition-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, T> UnzipOp for Partition<P>`

- <span id="partition-left"></span>`type Left = T`

- <span id="partition-right"></span>`type Right = T`

- <span id="partition-consume"></span>`fn consume<FA, FB>(&self, item: T, left: FA, right: FB) -> (FA, FB)`

### `PartitionMap<P>`

```rust
struct PartitionMap<P> {
    predicate: P,
}
```

An `UnzipOp` that routes items depending on how they are mapped `Either`.

#### Trait Implementations

##### `impl<T> IntoEither for PartitionMap<P>`

##### `impl<T> Pointable for PartitionMap<P>`

- <span id="partitionmap-align"></span>`const ALIGN: usize`

- <span id="partitionmap-init"></span>`type Init = T`

- <span id="partitionmap-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="partitionmap-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="partitionmap-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="partitionmap-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, L, R, T> UnzipOp for PartitionMap<P>`

- <span id="partitionmap-left"></span>`type Left = L`

- <span id="partitionmap-right"></span>`type Right = R`

- <span id="partitionmap-consume"></span>`fn consume<FA, FB>(&self, item: T, left: FA, right: FB) -> (FA, FB)`

### `UnzipA<'b, I, OP, FromB>`

```rust
struct UnzipA<'b, I, OP, FromB> {
    base: I,
    op: OP,
    b: &'b mut FromB,
}
```

A fake iterator to intercept the `Consumer` for type `A`.

#### Trait Implementations

##### `impl<T> IntoEither for UnzipA<'b, I, OP, FromB>`

##### `impl<T> IntoParallelIterator for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-iter"></span>`type Iter = T`

- <span id="unzipa-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="unzipa-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'b, I, OP, FromB> ParallelIterator for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-item"></span>`type Item = <OP as UnzipOp>::Left`

- <span id="unzipa-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="unzipa-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-align"></span>`const ALIGN: usize`

- <span id="unzipa-init"></span>`type Init = T`

- <span id="unzipa-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzipa-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzipa-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzipa-drop"></span>`unsafe fn drop(ptr: usize)`

### `UnzipB<'r, I, OP, CA>`

```rust
struct UnzipB<'r, I, OP, CA>
where
    I: ParallelIterator,
    OP: UnzipOp<<I as >::Item>,
    CA: UnindexedConsumer<<OP as >::Left> {
    base: I,
    op: OP,
    left_consumer: CA,
    left_result: &'r mut Option<<CA as >::Result>,
}
```

A fake iterator to intercept the `Consumer` for type `B`.

#### Trait Implementations

##### `impl<T> IntoEither for UnzipB<'r, I, OP, CA>`

##### `impl<T> IntoParallelIterator for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-iter"></span>`type Iter = T`

- <span id="unzipb-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="unzipb-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'r, I, OP, CA> ParallelIterator for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-item"></span>`type Item = <OP as UnzipOp>::Right`

- <span id="unzipb-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="unzipb-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-align"></span>`const ALIGN: usize`

- <span id="unzipb-init"></span>`type Init = T`

- <span id="unzipb-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzipb-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzipb-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzipb-drop"></span>`unsafe fn drop(ptr: usize)`

### `UnzipConsumer<'a, OP, CA, CB>`

```rust
struct UnzipConsumer<'a, OP, CA, CB> {
    op: &'a OP,
    left: CA,
    right: CB,
}
```

`Consumer` that unzips into two other `Consumer`s

#### Trait Implementations

##### `impl<'a, T, OP, CA, CB> Consumer for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-folder"></span>`type Folder = UnzipFolder<'a, OP, <CA as Consumer>::Folder, <CB as Consumer>::Folder>`

- <span id="unzipconsumer-reducer"></span>`type Reducer = UnzipReducer<<CA as Consumer>::Reducer, <CB as Consumer>::Reducer>`

- <span id="unzipconsumer-result"></span>`type Result = (<CA as Consumer>::Result, <CB as Consumer>::Result)`

- <span id="unzipconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="unzipconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="unzipconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for UnzipConsumer<'a, OP, CA, CB>`

##### `impl<T> Pointable for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-align"></span>`const ALIGN: usize`

- <span id="unzipconsumer-init"></span>`type Init = T`

- <span id="unzipconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzipconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzipconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzipconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'a, T, OP, CA, CB> UnindexedConsumer for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="unzipconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `UnzipFolder<'a, OP, FA, FB>`

```rust
struct UnzipFolder<'a, OP, FA, FB> {
    op: &'a OP,
    left: FA,
    right: FB,
}
```

`Folder` that unzips into two other `Folder`s

#### Trait Implementations

##### `impl<'a, T, OP, FA, FB> Folder for UnzipFolder<'a, OP, FA, FB>`

- <span id="unzipfolder-result"></span>`type Result = (<FA as Folder>::Result, <FB as Folder>::Result)`

- <span id="unzipfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="unzipfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="unzipfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for UnzipFolder<'a, OP, FA, FB>`

##### `impl<T> Pointable for UnzipFolder<'a, OP, FA, FB>`

- <span id="unzipfolder-align"></span>`const ALIGN: usize`

- <span id="unzipfolder-init"></span>`type Init = T`

- <span id="unzipfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzipfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzipfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzipfolder-drop"></span>`unsafe fn drop(ptr: usize)`

### `UnzipReducer<RA, RB>`

```rust
struct UnzipReducer<RA, RB> {
    left: RA,
    right: RB,
}
```

`Reducer` that unzips into two other `Reducer`s

#### Trait Implementations

##### `impl<T> IntoEither for UnzipReducer<RA, RB>`

##### `impl<T> Pointable for UnzipReducer<RA, RB>`

- <span id="unzipreducer-align"></span>`const ALIGN: usize`

- <span id="unzipreducer-init"></span>`type Init = T`

- <span id="unzipreducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzipreducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzipreducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzipreducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<A, B, RA, RB> Reducer for UnzipReducer<RA, RB>`

- <span id="unzipreducer-reduce"></span>`fn reduce(self, left: (A, B), right: (A, B)) -> (A, B)`

### `UnEither`

```rust
struct UnEither;
```

An `UnzipOp` that routes items depending on their `Either` variant.

#### Trait Implementations

##### `impl<T> IntoEither for UnEither`

##### `impl<T> Pointable for UnEither`

- <span id="uneither-align"></span>`const ALIGN: usize`

- <span id="uneither-init"></span>`type Init = T`

- <span id="uneither-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="uneither-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="uneither-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="uneither-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<L, R> UnzipOp for UnEither`

- <span id="uneither-left"></span>`type Left = L`

- <span id="uneither-right"></span>`type Right = R`

- <span id="uneither-consume"></span>`fn consume<FL, FR>(&self, item: Either<L, R>, left: FL, right: FR) -> (FL, FR)` — [`Either`](../index.md)

### `Collector<FromT>`

```rust
struct Collector<FromT> {
    result: Option<FromT>,
}
```

Shim to implement a one-time `ParallelExtend` using `FromParallelIterator`.

#### Trait Implementations

##### `impl<FromT> Default for Collector<FromT>`

- <span id="collector-default"></span>`fn default() -> Self`

##### `impl<T> IntoEither for Collector<FromT>`

##### `impl<T, FromT> ParallelExtend for Collector<FromT>`

- <span id="collector-par-extend"></span>`fn par_extend<I>(&mut self, pi: I)`

##### `impl<T> Pointable for Collector<FromT>`

- <span id="collector-align"></span>`const ALIGN: usize`

- <span id="collector-init"></span>`type Init = T`

- <span id="collector-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collector-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collector-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collector-drop"></span>`unsafe fn drop(ptr: usize)`

## Traits

### `UnzipOp<T>`

```rust
trait UnzipOp<T>: Sync + Send { ... }
```

This trait abstracts the different ways we can "unzip" one parallel
iterator into two distinct consumers, which we can handle almost
identically apart from how to process the individual items.

#### Associated Types

- `type Left: 1`

- `type Right: 1`

#### Required Methods

- `fn consume<FA, FB>(&self, item: T, left: FA, right: FB) -> (FA, FB)`

  Consumes one item and feeds it to one or both of the underlying folders.

#### Provided Methods

- `fn indexable() -> bool`

  Reports whether this op may support indexed consumers.

#### Implementors

- [`PartitionMap`](#partitionmap)
- [`Partition`](#partition)
- [`UnEither`](#uneither)
- [`Unzip`](#unzip)

## Functions

### `execute`

```rust
fn execute<I, OP, FromA, FromB>(pi: I, op: OP) -> (FromA, FromB)
where
    I: ParallelIterator,
    OP: UnzipOp<<I as >::Item>,
    FromA: Default + Send + ParallelExtend<<OP as >::Left>,
    FromB: Default + Send + ParallelExtend<<OP as >::Right>
```

Runs an unzip-like operation into default `ParallelExtend` collections.

### `execute_into`

```rust
fn execute_into<I, OP, FromA, FromB>(a: &mut FromA, b: &mut FromB, pi: I, op: OP)
where
    I: ParallelIterator,
    OP: UnzipOp<<I as >::Item>,
    FromA: Send + ParallelExtend<<OP as >::Left>,
    FromB: Send + ParallelExtend<<OP as >::Right>
```

Runs an unzip-like operation into `ParallelExtend` collections.

### `unzip`

```rust
fn unzip<I, A, B, FromA, FromB>(pi: I) -> (FromA, FromB)
where
    I: ParallelIterator<Item = (A, B)>,
    FromA: Default + Send + ParallelExtend<A>,
    FromB: Default + Send + ParallelExtend<B>,
    A: Send,
    B: Send
```

Unzips the items of a parallel iterator into a pair of arbitrary
`ParallelExtend` containers.

This is called by `ParallelIterator::unzip`.

### `unzip_indexed`

```rust
fn unzip_indexed<I, A, B, CA, CB>(pi: I, left: CA, right: CB) -> (<CA as >::Result, <CB as >::Result)
where
    I: IndexedParallelIterator<Item = (A, B)>,
    CA: Consumer<A>,
    CB: Consumer<B>,
    A: Send,
    B: Send
```

Unzips an `IndexedParallelIterator` into two arbitrary `Consumer`s.

This is called by `super::collect::unzip_into_vecs`.

### `partition`

```rust
fn partition<I, A, B, P>(pi: I, predicate: P) -> (A, B)
where
    I: ParallelIterator,
    A: Default + Send + ParallelExtend<<I as >::Item>,
    B: Default + Send + ParallelExtend<<I as >::Item>,
    P: Fn(&<I as >::Item) -> bool + Sync + Send
```

Partitions the items of a parallel iterator into a pair of arbitrary
`ParallelExtend` containers.

This is called by `ParallelIterator::partition`.

### `partition_map`

```rust
fn partition_map<I, A, B, P, L, R>(pi: I, predicate: P) -> (A, B)
where
    I: ParallelIterator,
    A: Default + Send + ParallelExtend<L>,
    B: Default + Send + ParallelExtend<R>,
    P: Fn(<I as >::Item) -> Either<L, R> + Sync + Send,
    L: Send,
    R: Send
```

Partitions and maps the items of a parallel iterator into a pair of
arbitrary `ParallelExtend` containers.

This called by `ParallelIterator::partition_map`.

