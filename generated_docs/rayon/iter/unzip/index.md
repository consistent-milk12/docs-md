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
  - [`execute_into`](#execute-into)
  - [`unzip`](#unzip)
  - [`unzip_indexed`](#unzip-indexed)
  - [`partition`](#partition)
  - [`partition_map`](#partition-map)

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
| [`UnzipOp`](#unzipop) | trait | This trait abstracts the different ways we can "unzip" one parallel iterator into two distinct consumers, which we can handle almost identically apart from how to process the individual items. |
| [`execute`](#execute) | fn | Runs an unzip-like operation into default `ParallelExtend` collections. |
| [`execute_into`](#execute-into) | fn | Runs an unzip-like operation into `ParallelExtend` collections. |
| [`unzip`](#unzip) | fn | Unzips the items of a parallel iterator into a pair of arbitrary `ParallelExtend` containers. |
| [`unzip_indexed`](#unzip-indexed) | fn | Unzips an `IndexedParallelIterator` into two arbitrary `Consumer`s. |
| [`partition`](#partition) | fn | Partitions the items of a parallel iterator into a pair of arbitrary `ParallelExtend` containers. |
| [`partition_map`](#partition-map) | fn | Partitions and maps the items of a parallel iterator into a pair of arbitrary `ParallelExtend` containers. |

## Structs

### `Unzip`

```rust
struct Unzip;
```

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:92`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L92)*

An `UnzipOp` that splits a tuple directly into the two consumers.

#### Trait Implementations

##### `impl Any for Unzip`

- <span id="unzip-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unzip`

- <span id="unzip-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unzip`

- <span id="unzip-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Unzip`

- <span id="unzip-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Unzip`

- <span id="unzip-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Unzip`

##### `impl Pointable for Unzip`

- <span id="unzip-pointable-const-align"></span>`const ALIGN: usize`

- <span id="unzip-pointable-type-init"></span>`type Init = T`

- <span id="unzip-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzip-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzip-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzip-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Unzip`

- <span id="unzip-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unzip-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Unzip`

- <span id="unzip-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unzip-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<A: Send, B: Send> UnzipOp for Unzip`

- <span id="unzip-unzipop-type-left"></span>`type Left = A`

- <span id="unzip-unzipop-type-right"></span>`type Right = B`

- <span id="unzip-unzipop-consume"></span>`fn consume<FA, FB>(&self, item: (A, B), left: FA, right: FB) -> (FA, FB)`

- <span id="unzip-unzipop-indexable"></span>`fn indexable() -> bool`

### `Partition<P>`

```rust
struct Partition<P> {
    predicate: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:126-128`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L126-L128)*

An `UnzipOp` that routes items depending on a predicate function.

#### Trait Implementations

##### `impl Any for Partition<P>`

- <span id="partition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Partition<P>`

- <span id="partition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Partition<P>`

- <span id="partition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Partition<P>`

- <span id="partition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Partition<P>`

- <span id="partition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Partition<P>`

##### `impl Pointable for Partition<P>`

- <span id="partition-pointable-const-align"></span>`const ALIGN: usize`

- <span id="partition-pointable-type-init"></span>`type Init = T`

- <span id="partition-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="partition-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="partition-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="partition-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Partition<P>`

- <span id="partition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="partition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Partition<P>`

- <span id="partition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="partition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<P, T> UnzipOp for Partition<P>`

- <span id="partition-unzipop-type-left"></span>`type Left = T`

- <span id="partition-unzipop-type-right"></span>`type Right = T`

- <span id="partition-unzipop-consume"></span>`fn consume<FA, FB>(&self, item: T, left: FA, right: FB) -> (FA, FB)`

### `PartitionMap<P>`

```rust
struct PartitionMap<P> {
    predicate: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:168-170`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L168-L170)*

An `UnzipOp` that routes items depending on how they are mapped `Either`.

#### Trait Implementations

##### `impl Any for PartitionMap<P>`

- <span id="partitionmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PartitionMap<P>`

- <span id="partitionmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PartitionMap<P>`

- <span id="partitionmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for PartitionMap<P>`

- <span id="partitionmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PartitionMap<P>`

- <span id="partitionmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PartitionMap<P>`

##### `impl Pointable for PartitionMap<P>`

- <span id="partitionmap-pointable-const-align"></span>`const ALIGN: usize`

- <span id="partitionmap-pointable-type-init"></span>`type Init = T`

- <span id="partitionmap-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="partitionmap-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="partitionmap-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="partitionmap-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for PartitionMap<P>`

- <span id="partitionmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="partitionmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PartitionMap<P>`

- <span id="partitionmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="partitionmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<P, T> UnzipOp for PartitionMap<P>`

- <span id="partitionmap-unzipop-type-left"></span>`type Left = L`

- <span id="partitionmap-unzipop-type-right"></span>`type Right = R`

- <span id="partitionmap-unzipop-consume"></span>`fn consume<FA, FB>(&self, item: T, left: FA, right: FB) -> (FA, FB)`

### `UnzipA<'b, I, OP, FromB>`

```rust
struct UnzipA<'b, I, OP, FromB> {
    base: I,
    op: OP,
    b: &'b mut FromB,
}
```

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:194-198`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L194-L198)*

A fake iterator to intercept the `Consumer` for type `A`.

#### Trait Implementations

##### `impl Any for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UnzipA<'b, I, OP, FromB>`

##### `impl IntoParallelIterator for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="unzipa-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="unzipa-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, OP, FromB> ParallelIterator for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-paralleliterator-type-item"></span>`type Item = <OP as UnzipOp>::Left`

- <span id="unzipa-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="unzipa-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-pointable-const-align"></span>`const ALIGN: usize`

- <span id="unzipa-pointable-type-init"></span>`type Init = T`

- <span id="unzipa-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzipa-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzipa-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzipa-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unzipa-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnzipA<'b, I, OP, FromB>`

- <span id="unzipa-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unzipa-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:239-249`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L239-L249)*

A fake iterator to intercept the `Consumer` for type `B`.

#### Trait Implementations

##### `impl Any for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UnzipB<'r, I, OP, CA>`

##### `impl IntoParallelIterator for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="unzipb-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="unzipb-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, OP, CA> ParallelIterator for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-paralleliterator-type-item"></span>`type Item = <OP as UnzipOp>::Right`

- <span id="unzipb-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="unzipb-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-pointable-const-align"></span>`const ALIGN: usize`

- <span id="unzipb-pointable-type-init"></span>`type Init = T`

- <span id="unzipb-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzipb-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzipb-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzipb-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unzipb-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnzipB<'r, I, OP, CA>`

- <span id="unzipb-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unzipb-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnzipConsumer<'a, OP, CA, CB>`

```rust
struct UnzipConsumer<'a, OP, CA, CB> {
    op: &'a OP,
    left: CA,
    right: CB,
}
```

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:285-289`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L285-L289)*

`Consumer` that unzips into two other `Consumer`s

#### Trait Implementations

##### `impl Any for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, OP, CA, CB> Consumer for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-consumer-type-folder"></span>`type Folder = UnzipFolder<'a, OP, <CA as Consumer>::Folder, <CB as Consumer>::Folder>`

- <span id="unzipconsumer-consumer-type-reducer"></span>`type Reducer = UnzipReducer<<CA as Consumer>::Reducer, <CB as Consumer>::Reducer>`

- <span id="unzipconsumer-consumer-type-result"></span>`type Result = (<CA as Consumer>::Result, <CB as Consumer>::Result)`

- <span id="unzipconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="unzipconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="unzipconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UnzipConsumer<'a, OP, CA, CB>`

##### `impl Pointable for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="unzipconsumer-pointable-type-init"></span>`type Init = T`

- <span id="unzipconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzipconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzipconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzipconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unzipconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unzipconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, OP, CA, CB> UnindexedConsumer for UnzipConsumer<'a, OP, CA, CB>`

- <span id="unzipconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="unzipconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `UnzipFolder<'a, OP, FA, FB>`

```rust
struct UnzipFolder<'a, OP, FA, FB> {
    op: &'a OP,
    left: FA,
    right: FB,
}
```

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:360-364`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L360-L364)*

`Folder` that unzips into two other `Folder`s

#### Trait Implementations

##### `impl Any for UnzipFolder<'a, OP, FA, FB>`

- <span id="unzipfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnzipFolder<'a, OP, FA, FB>`

- <span id="unzipfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnzipFolder<'a, OP, FA, FB>`

- <span id="unzipfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, OP, FA, FB> Folder for UnzipFolder<'a, OP, FA, FB>`

- <span id="unzipfolder-folder-type-result"></span>`type Result = (<FA as Folder>::Result, <FB as Folder>::Result)`

- <span id="unzipfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="unzipfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="unzipfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for UnzipFolder<'a, OP, FA, FB>`

- <span id="unzipfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnzipFolder<'a, OP, FA, FB>`

- <span id="unzipfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UnzipFolder<'a, OP, FA, FB>`

##### `impl Pointable for UnzipFolder<'a, OP, FA, FB>`

- <span id="unzipfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="unzipfolder-pointable-type-init"></span>`type Init = T`

- <span id="unzipfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzipfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzipfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzipfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for UnzipFolder<'a, OP, FA, FB>`

- <span id="unzipfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unzipfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnzipFolder<'a, OP, FA, FB>`

- <span id="unzipfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unzipfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnzipReducer<RA, RB>`

```rust
struct UnzipReducer<RA, RB> {
    left: RA,
    right: RB,
}
```

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:394-397`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L394-L397)*

`Reducer` that unzips into two other `Reducer`s

#### Trait Implementations

##### `impl Any for UnzipReducer<RA, RB>`

- <span id="unzipreducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnzipReducer<RA, RB>`

- <span id="unzipreducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnzipReducer<RA, RB>`

- <span id="unzipreducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for UnzipReducer<RA, RB>`

- <span id="unzipreducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnzipReducer<RA, RB>`

- <span id="unzipreducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UnzipReducer<RA, RB>`

##### `impl Pointable for UnzipReducer<RA, RB>`

- <span id="unzipreducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="unzipreducer-pointable-type-init"></span>`type Init = T`

- <span id="unzipreducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unzipreducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unzipreducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unzipreducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<A, B, RA, RB> Reducer for UnzipReducer<RA, RB>`

- <span id="unzipreducer-reducer-reduce"></span>`fn reduce(self, left: (A, B), right: (A, B)) -> (A, B)`

##### `impl<U> TryFrom for UnzipReducer<RA, RB>`

- <span id="unzipreducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unzipreducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnzipReducer<RA, RB>`

- <span id="unzipreducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unzipreducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnEither`

```rust
struct UnEither;
```

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:443`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L443)*

An `UnzipOp` that routes items depending on their `Either` variant.

#### Trait Implementations

##### `impl Any for UnEither`

- <span id="uneither-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnEither`

- <span id="uneither-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnEither`

- <span id="uneither-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for UnEither`

- <span id="uneither-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnEither`

- <span id="uneither-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UnEither`

##### `impl Pointable for UnEither`

- <span id="uneither-pointable-const-align"></span>`const ALIGN: usize`

- <span id="uneither-pointable-type-init"></span>`type Init = T`

- <span id="uneither-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="uneither-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="uneither-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="uneither-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for UnEither`

- <span id="uneither-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uneither-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnEither`

- <span id="uneither-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uneither-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<L, R> UnzipOp for UnEither`

- <span id="uneither-unzipop-type-left"></span>`type Left = L`

- <span id="uneither-unzipop-type-right"></span>`type Right = R`

- <span id="uneither-unzipop-consume"></span>`fn consume<FL, FR>(&self, item: Either<L, R>, left: FL, right: FR) -> (FL, FR)` — [`Either`](../index.md#either)

### `Collector<FromT>`

```rust
struct Collector<FromT> {
    result: Option<FromT>,
}
```

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:502-504`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L502-L504)*

Shim to implement a one-time `ParallelExtend` using `FromParallelIterator`.

#### Trait Implementations

##### `impl Any for Collector<FromT>`

- <span id="collector-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Collector<FromT>`

- <span id="collector-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Collector<FromT>`

- <span id="collector-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<FromT> Default for Collector<FromT>`

- <span id="collector-default"></span>`fn default() -> Self`

##### `impl<T> From for Collector<FromT>`

- <span id="collector-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Collector<FromT>`

- <span id="collector-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Collector<FromT>`

##### `impl<T, FromT> ParallelExtend for Collector<FromT>`

- <span id="collector-parallelextend-par-extend"></span>`fn par_extend<I>(&mut self, pi: I)`

##### `impl Pointable for Collector<FromT>`

- <span id="collector-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collector-pointable-type-init"></span>`type Init = T`

- <span id="collector-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collector-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collector-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collector-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Collector<FromT>`

- <span id="collector-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="collector-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Collector<FromT>`

- <span id="collector-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="collector-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `UnzipOp<T>`

```rust
trait UnzipOp<T>: Sync + Send { ... }
```

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:7-26`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L7-L26)*

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

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:29-40`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L29-L40)*

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

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:43-55`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L43-L55)*

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

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:61-70`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L61-L70)*

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

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:75-89`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L75-L89)*

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

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:115-123`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L115-L123)*

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

*Defined in [`rayon-1.11.0/src/iter/unzip.rs:155-165`](../../../../.source_1765521767/rayon-1.11.0/src/iter/unzip.rs#L155-L165)*

Partitions and maps the items of a parallel iterator into a pair of
arbitrary `ParallelExtend` containers.

This called by `ParallelIterator::partition_map`.

