*[rayon](../../index.md) / [iter](../index.md) / [unzip](index.md)*

---

# Module `unzip`

## Structs

### `Unzip`

```rust
struct Unzip;
```

An `UnzipOp` that splits a tuple directly into the two consumers.

#### Trait Implementations

##### `impl<T> IntoEither for Unzip`

##### `impl<T> Pointable for Unzip`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<A: Send, B: Send> UnzipOp for Unzip`

- `type Left = A`

- `type Right = B`

- `fn consume<FA, FB>(self: &Self, item: (A, B), left: FA, right: FB) -> (FA, FB)`

- `fn indexable() -> bool`

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

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<P, T> UnzipOp for Partition<P>`

- `type Left = T`

- `type Right = T`

- `fn consume<FA, FB>(self: &Self, item: T, left: FA, right: FB) -> (FA, FB)`

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

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<P, L, R, T> UnzipOp for PartitionMap<P>`

- `type Left = L`

- `type Right = R`

- `fn consume<FA, FB>(self: &Self, item: T, left: FA, right: FB) -> (FA, FB)`

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

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'b, I, OP, FromB> ParallelIterator for UnzipA<'b, I, OP, FromB>`

- `type Item = <OP as UnzipOp>::Left`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for UnzipA<'b, I, OP, FromB>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'r, I, OP, CA> ParallelIterator for UnzipB<'r, I, OP, CA>`

- `type Item = <OP as UnzipOp>::Right`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for UnzipB<'r, I, OP, CA>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `type Folder = UnzipFolder<'a, OP, <CA as Consumer>::Folder, <CB as Consumer>::Folder>`

- `type Reducer = UnzipReducer<<CA as Consumer>::Reducer, <CB as Consumer>::Reducer>`

- `type Result = (<CA as Consumer>::Result, <CB as Consumer>::Result)`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for UnzipConsumer<'a, OP, CA, CB>`

##### `impl<T> Pointable for UnzipConsumer<'a, OP, CA, CB>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'a, T, OP, CA, CB> UnindexedConsumer for UnzipConsumer<'a, OP, CA, CB>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

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

- `type Result = (<FA as Folder>::Result, <FB as Folder>::Result)`

- `fn consume(self: Self, item: T) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for UnzipFolder<'a, OP, FA, FB>`

##### `impl<T> Pointable for UnzipFolder<'a, OP, FA, FB>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<A, B, RA, RB> Reducer for UnzipReducer<RA, RB>`

- `fn reduce(self: Self, left: (A, B), right: (A, B)) -> (A, B)`

### `UnEither`

```rust
struct UnEither;
```

An `UnzipOp` that routes items depending on their `Either` variant.

#### Trait Implementations

##### `impl<T> IntoEither for UnEither`

##### `impl<T> Pointable for UnEither`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<L, R> UnzipOp for UnEither`

- `type Left = L`

- `type Right = R`

- `fn consume<FL, FR>(self: &Self, item: Either<L, R>, left: FL, right: FR) -> (FL, FR)` — [`Either`](../index.md)

### `Collector<FromT>`

```rust
struct Collector<FromT> {
    result: Option<FromT>,
}
```

Shim to implement a one-time `ParallelExtend` using `FromParallelIterator`.

#### Trait Implementations

##### `impl<FromT> Default for Collector<FromT>`

- `fn default() -> Self`

##### `impl<T> IntoEither for Collector<FromT>`

##### `impl<T, FromT> ParallelExtend for Collector<FromT>`

- `fn par_extend<I>(self: &mut Self, pi: I)`

##### `impl<T> Pointable for Collector<FromT>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Traits

### `UnzipOp<T>`

```rust
trait UnzipOp<T>: Sync + Send { ... }
```

This trait abstracts the different ways we can "unzip" one parallel
iterator into two distinct consumers, which we can handle almost
identically apart from how to process the individual items.

#### Required Methods

- `type Left: 1`

- `type Right: 1`

- `fn consume<FA, FB>(self: &Self, item: T, left: FA, right: FB) -> (FA, FB)`

  Consumes one item and feeds it to one or both of the underlying folders.

- `fn indexable() -> bool`

  Reports whether this op may support indexed consumers.

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

