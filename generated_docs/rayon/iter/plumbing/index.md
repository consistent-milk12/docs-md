*[rayon](../../index.md) / [iter](../index.md) / [plumbing](index.md)*

---

# Module `plumbing`

Traits and functions used to implement parallel iteration.  These are
low-level details -- users of parallel iterators should not need to
interact with them directly.  See [the `plumbing` README][r] for a general overview.


## Contents

- [Structs](#structs)
  - [`Splitter`](#splitter)
  - [`LengthSplitter`](#lengthsplitter)
- [Traits](#traits)
  - [`ProducerCallback`](#producercallback)
  - [`Producer`](#producer)
  - [`Consumer`](#consumer)
  - [`Folder`](#folder)
  - [`Reducer`](#reducer)
  - [`UnindexedConsumer`](#unindexedconsumer)
  - [`UnindexedProducer`](#unindexedproducer)
- [Functions](#functions)
  - [`bridge`](#bridge)
  - [`bridge_producer_consumer`](#bridge-producer-consumer)
  - [`bridge_unindexed`](#bridge-unindexed)
  - [`bridge_unindexed_producer_consumer`](#bridge-unindexed-producer-consumer)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Splitter`](#splitter) | struct | A splitter controls the policy for splitting into smaller work items. |
| [`LengthSplitter`](#lengthsplitter) | struct | The length splitter is built on thief-splitting, but additionally takes into account the remaining length of the iterator. |
| [`ProducerCallback`](#producercallback) | trait | The `ProducerCallback` trait is a kind of generic closure, [analogous to `FnOnce`][FnOnce]. |
| [`Producer`](#producer) | trait | A `Producer` is effectively a "splittable `IntoIterator`". |
| [`Consumer`](#consumer) | trait | A consumer is effectively a [generalized "fold" operation][fold], and in fact each consumer will eventually be converted into a [`Folder`]. |
| [`Folder`](#folder) | trait | The `Folder` trait encapsulates [the standard fold operation][fold]. |
| [`Reducer`](#reducer) | trait | The reducer is the final step of a `Consumer` -- after a consumer has been split into two parts, and each of those parts has been fully processed, we are left with two results. |
| [`UnindexedConsumer`](#unindexedconsumer) | trait | A stateless consumer can be freely copied. |
| [`UnindexedProducer`](#unindexedproducer) | trait | A variant on `Producer` which does not know its exact length or cannot represent it in a `usize`. |
| [`bridge`](#bridge) | fn | This helper function is used to "connect" a parallel iterator to a consumer. |
| [`bridge_producer_consumer`](#bridge-producer-consumer) | fn | This helper function is used to "connect" a producer and a consumer. |
| [`bridge_unindexed`](#bridge-unindexed) | fn | A variant of [`bridge_producer_consumer()`] where the producer is an unindexed producer. |
| [`bridge_unindexed_producer_consumer`](#bridge-unindexed-producer-consumer) | fn |  |

## Structs

### `Splitter`

```rust
struct Splitter {
    splits: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:251-256`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L251-L256)*

A splitter controls the policy for splitting into smaller work items.

Thief-splitting is an adaptive policy that starts by splitting into
enough jobs for every worker thread, and then resets itself whenever a
job is actually stolen into a different thread.

#### Fields

- **`splits`**: `usize`

  The `splits` tell us approximately how many remaining times we'd
  like to split this job.  We always just divide it by two though, so
  the effective number of pieces will be `next_power_of_two()`.

#### Implementations

- <span id="splitter-new"></span>`fn new() -> Splitter` — [`Splitter`](#splitter)

- <span id="splitter-try-split"></span>`fn try_split(&mut self, stolen: bool) -> bool`

#### Trait Implementations

##### `impl Any for Splitter`

- <span id="splitter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Splitter`

- <span id="splitter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Splitter`

- <span id="splitter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Splitter`

- <span id="splitter-clone"></span>`fn clone(&self) -> Splitter` — [`Splitter`](#splitter)

##### `impl CloneToUninit for Splitter`

- <span id="splitter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Splitter`

##### `impl<T> From for Splitter`

- <span id="splitter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Splitter`

- <span id="splitter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Splitter`

##### `impl Pointable for Splitter`

- <span id="splitter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitter-pointable-type-init"></span>`type Init = T`

- <span id="splitter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Splitter`

- <span id="splitter-toowned-type-owned"></span>`type Owned = T`

- <span id="splitter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="splitter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Splitter`

- <span id="splitter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Splitter`

- <span id="splitter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LengthSplitter`

```rust
struct LengthSplitter {
    inner: Splitter,
    min: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:289-295`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L289-L295)*

The length splitter is built on thief-splitting, but additionally takes
into account the remaining length of the iterator.

#### Fields

- **`min`**: `usize`

  The smallest we're willing to divide into.  Usually this is just 1,
  but you can choose a larger working size with `with_min_len()`.

#### Implementations

- <span id="lengthsplitter-new"></span>`fn new(min: usize, max: usize, len: usize) -> LengthSplitter` — [`LengthSplitter`](#lengthsplitter)

  Creates a new splitter based on lengths.

  

  The `min` is a hard lower bound.  We'll never split below that, but

  of course an iterator might start out smaller already.

  

  The `max` is an upper bound on the working size, used to determine

  the minimum number of times we need to split to get under that limit.

  The adaptive algorithm may very well split even further, but never

  smaller than the `min`.

- <span id="lengthsplitter-try-split"></span>`fn try_split(&mut self, len: usize, stolen: bool) -> bool`

#### Trait Implementations

##### `impl Any for LengthSplitter`

- <span id="lengthsplitter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LengthSplitter`

- <span id="lengthsplitter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LengthSplitter`

- <span id="lengthsplitter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LengthSplitter`

- <span id="lengthsplitter-clone"></span>`fn clone(&self) -> LengthSplitter` — [`LengthSplitter`](#lengthsplitter)

##### `impl CloneToUninit for LengthSplitter`

- <span id="lengthsplitter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LengthSplitter`

##### `impl<T> From for LengthSplitter`

- <span id="lengthsplitter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LengthSplitter`

- <span id="lengthsplitter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for LengthSplitter`

##### `impl Pointable for LengthSplitter`

- <span id="lengthsplitter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="lengthsplitter-pointable-type-init"></span>`type Init = T`

- <span id="lengthsplitter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="lengthsplitter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="lengthsplitter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="lengthsplitter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for LengthSplitter`

- <span id="lengthsplitter-toowned-type-owned"></span>`type Owned = T`

- <span id="lengthsplitter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lengthsplitter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LengthSplitter`

- <span id="lengthsplitter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lengthsplitter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LengthSplitter`

- <span id="lengthsplitter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lengthsplitter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ProducerCallback<T>`

```rust
trait ProducerCallback<T> { ... }
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:17-30`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L17-L30)*

The `ProducerCallback` trait is a kind of generic closure,
[analogous to `FnOnce`][FnOnce]. See [the corresponding section in
the plumbing README][r] for more details.



#### Associated Types

- `type Output`

#### Required Methods

- `fn callback<P>(self, producer: P) -> <Self as >::Output`

  Invokes the callback with the given producer as argument. The

#### Implementors

- [`BlocksCallback`](../blocks/index.md#blockscallback)

### `Producer`

```rust
trait Producer: Send + Sized { ... }
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:56-109`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L56-L109)*

A `Producer` is effectively a "splittable `IntoIterator`". That
is, a producer is a value which can be converted into an iterator
at any time: at that point, it simply produces items on demand,
like any iterator. But what makes a `Producer` special is that,
*before* we convert to an iterator, we can also **split** it at a
particular point using the `split_at` method. This will yield up
two producers, one producing the items before that point, and one
producing the items after that point (these two producers can then
independently be split further, or be converted into iterators).
In Rayon, this splitting is used to divide between threads.
See [the `plumbing` README][r] for further details.

Note that each producer will always produce a fixed number of
items N. However, this number N is not queryable through the API;
the consumer is expected to track it.

NB. You might expect `Producer` to extend the `IntoIterator`
trait.  However, [rust-lang/rust#20671][20671] prevents us from
declaring the DoubleEndedIterator and ExactSizeIterator
constraints on a required IntoIterator trait, so we inline
IntoIterator here until that issue is fixed.



#### Associated Types

- `type Item`

- `type IntoIter: 3`

#### Required Methods

- `fn into_iter(self) -> <Self as >::IntoIter`

  Convert `self` into an iterator; at this point, no more parallel splits

- `fn split_at(self, index: usize) -> (Self, Self)`

  Split into two producers; one produces items `0..index`, the

#### Provided Methods

- `fn min_len(&self) -> usize`

  The minimum number of items that we will process

- `fn max_len(&self) -> usize`

  The maximum number of items that we will process

- `fn fold_with<F>(self, folder: F) -> F`

  Iterate the producer, feeding each element to `folder`, and

#### Implementors

- [`ChainProducer`](../chain/index.md#chainproducer)
- [`ChunkProducer`](../chunks/index.md#chunkproducer)
- [`ChunksExactMutProducer`](../../slice/chunks/index.md#chunksexactmutproducer)
- [`ChunksExactProducer`](../../slice/chunks/index.md#chunksexactproducer)
- [`ChunksMutProducer`](../../slice/chunks/index.md#chunksmutproducer)
- [`ChunksProducer`](../../slice/chunks/index.md#chunksproducer)
- [`ClonedProducer`](../cloned/index.md#clonedproducer)
- [`CopiedProducer`](../copied/index.md#copiedproducer)
- [`DrainProducer`](../../vec/index.md#drainproducer)
- [`EmptyProducer`](../empty/index.md#emptyproducer)
- [`EnumerateProducer`](../enumerate/index.md#enumerateproducer)
- [`InspectProducer`](../inspect/index.md#inspectproducer)
- [`InterleaveProducer`](../interleave/index.md#interleaveproducer)
- [`IntersperseProducer`](../intersperse/index.md#intersperseproducer)
- [`IterMutProducer`](../../slice/index.md#itermutproducer)
- [`IterProducer`](../../range/index.md#iterproducer)
- [`IterProducer`](../../slice/index.md#iterproducer)
- [`MapInitProducer`](../map_with/index.md#mapinitproducer)
- [`MapProducer`](../map/index.md#mapproducer)
- [`MapWithProducer`](../map_with/index.md#mapwithproducer)
- [`MaxLenProducer`](../len/index.md#maxlenproducer)
- [`MinLenProducer`](../len/index.md#minlenproducer)
- [`OptionProducer`](../../option/index.md#optionproducer)
- [`PanicFuseProducer`](../panic_fuse/index.md#panicfuseproducer)
- [`RChunksExactMutProducer`](../../slice/rchunks/index.md#rchunksexactmutproducer)
- [`RChunksExactProducer`](../../slice/rchunks/index.md#rchunksexactproducer)
- [`RChunksMutProducer`](../../slice/rchunks/index.md#rchunksmutproducer)
- [`RChunksProducer`](../../slice/rchunks/index.md#rchunksproducer)
- [`RepeatNProducer`](../repeat/index.md#repeatnproducer)
- [`RevProducer`](../rev/index.md#revproducer)
- [`StepByProducer`](../step_by/index.md#stepbyproducer)
- [`UpdateProducer`](../update/index.md#updateproducer)
- [`WindowsProducer`](../../slice/index.md#windowsproducer)
- [`ZipProducer`](../zip/index.md#zipproducer)

### `Consumer<Item>`

```rust
trait Consumer<Item>: Send + Sized { ... }
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:123-146`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L123-L146)*

A consumer is effectively a [generalized "fold" operation][`fold`](../fold/index.md),
and in fact each consumer will eventually be converted into a
[`Folder`](#folder). What makes a consumer special is that, like a
[`Producer`](#producer), it can be **split** into multiple consumers using
the `split_at` method. When a consumer is split, it produces two
consumers, as well as a **reducer**. The two consumers can be fed
items independently, and when they are done the reducer is used to
combine their two results into one. See [the `plumbing`
README][r] for further details.



#### Associated Types

- `type Folder: 1`

- `type Reducer: 1`

- `type Result: 1`

#### Required Methods

- `fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)`

  Divide the consumer into two consumers, one processing items

- `fn into_folder(self) -> <Self as >::Folder`

  Convert the consumer into a folder that can consume items

- `fn full(&self) -> bool`

  Hint whether this `Consumer` would like to stop processing

#### Implementors

- [`ClonedConsumer`](../cloned/index.md#clonedconsumer)
- [`CollectConsumer`](../collect/consumer/index.md#collectconsumer)
- [`CopiedConsumer`](../copied/index.md#copiedconsumer)
- [`FilterConsumer`](../filter/index.md#filterconsumer)
- [`FilterMapConsumer`](../filter_map/index.md#filtermapconsumer)
- [`FindConsumer`](../find/index.md#findconsumer)
- [`FindConsumer`](../find_first_last/index.md#findconsumer)
- [`FlatMapConsumer`](../flat_map/index.md#flatmapconsumer)
- [`FlatMapIterConsumer`](../flat_map_iter/index.md#flatmapiterconsumer)
- [`FlattenConsumer`](../flatten/index.md#flattenconsumer)
- [`FlattenIterConsumer`](../flatten_iter/index.md#flatteniterconsumer)
- [`FoldConsumer`](../fold/index.md#foldconsumer)
- [`FoldWithConsumer`](../fold/index.md#foldwithconsumer)
- [`ForEachConsumer`](../for_each/index.md#foreachconsumer)
- [`InspectConsumer`](../inspect/index.md#inspectconsumer)
- [`IntersperseConsumer`](../intersperse/index.md#intersperseconsumer)
- [`ListConsumer`](../extend/index.md#listconsumer)
- [`ListStringConsumer`](../extend/index.md#liststringconsumer)
- [`ListVecConsumer`](../extend/index.md#listvecconsumer)
- [`MapConsumer`](../map/index.md#mapconsumer)
- [`MapInitConsumer`](../map_with/index.md#mapinitconsumer)
- [`MapWithConsumer`](../map_with/index.md#mapwithconsumer)
- [`NoopConsumer`](../noop/index.md#noopconsumer)
- [`PanicFuseConsumer`](../panic_fuse/index.md#panicfuseconsumer)
- [`PositionsConsumer`](../positions/index.md#positionsconsumer)
- [`ProductConsumer`](../product/index.md#productconsumer)
- [`ReduceConsumer`](../reduce/index.md#reduceconsumer)
- [`SkipAnyConsumer`](../skip_any/index.md#skipanyconsumer)
- [`SkipAnyWhileConsumer`](../skip_any_while/index.md#skipanywhileconsumer)
- [`SumConsumer`](../sum/index.md#sumconsumer)
- [`TakeAnyConsumer`](../take_any/index.md#takeanyconsumer)
- [`TakeAnyWhileConsumer`](../take_any_while/index.md#takeanywhileconsumer)
- [`TryFoldConsumer`](../try_fold/index.md#tryfoldconsumer)
- [`TryFoldWithConsumer`](../try_fold/index.md#tryfoldwithconsumer)
- [`TryReduceConsumer`](../try_reduce/index.md#tryreduceconsumer)
- [`TryReduceWithConsumer`](../try_reduce_with/index.md#tryreducewithconsumer)
- [`UnzipConsumer`](../unzip/index.md#unzipconsumer)
- [`UpdateConsumer`](../update/index.md#updateconsumer)
- [`WhileSomeConsumer`](../while_some/index.md#whilesomeconsumer)

### `Folder<Item>`

```rust
trait Folder<Item>: Sized { ... }
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:154-188`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L154-L188)*

The `Folder` trait encapsulates [the standard fold
operation][`fold`](../fold/index.md).  It can be fed many items using the `consume`
method. At the end, once all items have been consumed, it can then
be converted (using `complete`) into a final value.


#### Associated Types

- `type Result`

#### Required Methods

- `fn consume(self, item: Item) -> Self`

  Consume next item and return new sequential state.

- `fn complete(self) -> <Self as >::Result`

  Finish consuming items, produce final result.

- `fn full(&self) -> bool`

  Hint whether this `Folder` would like to stop processing

#### Provided Methods

- `fn consume_iter<I>(self, iter: I) -> Self`

  Consume items from the iterator until full, and return new sequential state.

#### Implementors

- [`ClonedFolder`](../cloned/index.md#clonedfolder)
- [`CollectResult`](../collect/consumer/index.md#collectresult)
- [`CopiedFolder`](../copied/index.md#copiedfolder)
- [`FilterFolder`](../filter/index.md#filterfolder)
- [`FilterMapFolder`](../filter_map/index.md#filtermapfolder)
- [`FindFolder`](../find/index.md#findfolder)
- [`FindFolder`](../find_first_last/index.md#findfolder)
- [`FlatMapFolder`](../flat_map/index.md#flatmapfolder)
- [`FlatMapIterFolder`](../flat_map_iter/index.md#flatmapiterfolder)
- [`FlattenFolder`](../flatten/index.md#flattenfolder)
- [`FlattenIterFolder`](../flatten_iter/index.md#flatteniterfolder)
- [`FoldFolder`](../fold/index.md#foldfolder)
- [`ForEachConsumer`](../for_each/index.md#foreachconsumer)
- [`InspectFolder`](../inspect/index.md#inspectfolder)
- [`IntersperseFolder`](../intersperse/index.md#interspersefolder)
- [`ListFolder`](../extend/index.md#listfolder)
- [`ListStringFolder`](../extend/index.md#liststringfolder)
- [`ListVecFolder`](../extend/index.md#listvecfolder)
- [`MapFolder`](../map/index.md#mapfolder)
- [`MapWithFolder`](../map_with/index.md#mapwithfolder)
- [`NoopConsumer`](../noop/index.md#noopconsumer)
- [`PanicFuseFolder`](../panic_fuse/index.md#panicfusefolder)
- [`PositionsFolder`](../positions/index.md#positionsfolder)
- [`ProductFolder`](../product/index.md#productfolder)
- [`ReduceFolder`](../reduce/index.md#reducefolder)
- [`SkipAnyFolder`](../skip_any/index.md#skipanyfolder)
- [`SkipAnyWhileFolder`](../skip_any_while/index.md#skipanywhilefolder)
- [`SumFolder`](../sum/index.md#sumfolder)
- [`TakeAnyFolder`](../take_any/index.md#takeanyfolder)
- [`TakeAnyWhileFolder`](../take_any_while/index.md#takeanywhilefolder)
- [`TryFoldFolder`](../try_fold/index.md#tryfoldfolder)
- [`TryReduceFolder`](../try_reduce/index.md#tryreducefolder)
- [`TryReduceWithFolder`](../try_reduce_with/index.md#tryreducewithfolder)
- [`UnzipFolder`](../unzip/index.md#unzipfolder)
- [`UpdateFolder`](../update/index.md#updatefolder)
- [`WhileSomeFolder`](../while_some/index.md#whilesomefolder)

### `Reducer<Result>`

```rust
trait Reducer<Result> { ... }
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:197-201`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L197-L201)*

The reducer is the final step of a `Consumer` -- after a consumer
has been split into two parts, and each of those parts has been
fully processed, we are left with two results. The reducer is then
used to combine those two results into one. See [the `plumbing`
README][r] for further details.


#### Required Methods

- `fn reduce(self, left: Result, right: Result) -> Result`

  Reduce two final results into one; this is executed after a

#### Implementors

- [`CollectReducer`](../collect/consumer/index.md#collectreducer)
- [`FindReducer`](../find/index.md#findreducer)
- [`FindReducer`](../find_first_last/index.md#findreducer)
- [`ListReducer`](../extend/index.md#listreducer)
- [`NoopReducer`](../noop/index.md#noopreducer)
- [`PanicFuseReducer`](../panic_fuse/index.md#panicfusereducer)
- [`ProductConsumer`](../product/index.md#productconsumer)
- [`ReduceConsumer`](../reduce/index.md#reduceconsumer)
- [`SumConsumer`](../sum/index.md#sumconsumer)
- [`TryReduceConsumer`](../try_reduce/index.md#tryreduceconsumer)
- [`TryReduceWithConsumer`](../try_reduce_with/index.md#tryreducewithconsumer)
- [`UnzipReducer`](../unzip/index.md#unzipreducer)

### `UnindexedConsumer<I>`

```rust
trait UnindexedConsumer<I>: Consumer<I> { ... }
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:208-221`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L208-L221)*

A stateless consumer can be freely copied. These consumers can be
used like regular consumers, but they also support a
`split_off_left` method that does not take an index to split, but
simply splits at some arbitrary point (`for_each`, for example,
produces an unindexed consumer).

#### Required Methods

- `fn split_off_left(&self) -> Self`

  Splits off a "left" consumer and returns it. The `self`

- `fn to_reducer(&self) -> <Self as >::Reducer`

  Creates a reducer that can be used to combine the results from

#### Implementors

- [`ClonedConsumer`](../cloned/index.md#clonedconsumer)
- [`CollectConsumer`](../collect/consumer/index.md#collectconsumer)
- [`CopiedConsumer`](../copied/index.md#copiedconsumer)
- [`FilterConsumer`](../filter/index.md#filterconsumer)
- [`FilterMapConsumer`](../filter_map/index.md#filtermapconsumer)
- [`FindConsumer`](../find/index.md#findconsumer)
- [`FindConsumer`](../find_first_last/index.md#findconsumer)
- [`FlatMapConsumer`](../flat_map/index.md#flatmapconsumer)
- [`FlatMapIterConsumer`](../flat_map_iter/index.md#flatmapiterconsumer)
- [`FlattenConsumer`](../flatten/index.md#flattenconsumer)
- [`FlattenIterConsumer`](../flatten_iter/index.md#flatteniterconsumer)
- [`FoldConsumer`](../fold/index.md#foldconsumer)
- [`FoldWithConsumer`](../fold/index.md#foldwithconsumer)
- [`ForEachConsumer`](../for_each/index.md#foreachconsumer)
- [`InspectConsumer`](../inspect/index.md#inspectconsumer)
- [`IntersperseConsumer`](../intersperse/index.md#intersperseconsumer)
- [`ListConsumer`](../extend/index.md#listconsumer)
- [`ListStringConsumer`](../extend/index.md#liststringconsumer)
- [`ListVecConsumer`](../extend/index.md#listvecconsumer)
- [`MapConsumer`](../map/index.md#mapconsumer)
- [`MapInitConsumer`](../map_with/index.md#mapinitconsumer)
- [`MapWithConsumer`](../map_with/index.md#mapwithconsumer)
- [`NoopConsumer`](../noop/index.md#noopconsumer)
- [`PanicFuseConsumer`](../panic_fuse/index.md#panicfuseconsumer)
- [`ProductConsumer`](../product/index.md#productconsumer)
- [`ReduceConsumer`](../reduce/index.md#reduceconsumer)
- [`SkipAnyConsumer`](../skip_any/index.md#skipanyconsumer)
- [`SkipAnyWhileConsumer`](../skip_any_while/index.md#skipanywhileconsumer)
- [`SumConsumer`](../sum/index.md#sumconsumer)
- [`TakeAnyConsumer`](../take_any/index.md#takeanyconsumer)
- [`TakeAnyWhileConsumer`](../take_any_while/index.md#takeanywhileconsumer)
- [`TryFoldConsumer`](../try_fold/index.md#tryfoldconsumer)
- [`TryFoldWithConsumer`](../try_fold/index.md#tryfoldwithconsumer)
- [`TryReduceConsumer`](../try_reduce/index.md#tryreduceconsumer)
- [`TryReduceWithConsumer`](../try_reduce_with/index.md#tryreducewithconsumer)
- [`UnzipConsumer`](../unzip/index.md#unzipconsumer)
- [`UpdateConsumer`](../update/index.md#updateconsumer)
- [`WhileSomeConsumer`](../while_some/index.md#whilesomeconsumer)

### `UnindexedProducer`

```rust
trait UnindexedProducer: Send + Sized { ... }
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:231-243`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L231-L243)*

A variant on `Producer` which does not know its exact length or
cannot represent it in a `usize`. These producers act like
ordinary producers except that they cannot be told to split at a
particular point. Instead, you just ask them to split 'somewhere'.

(In principle, `Producer` could extend this trait; however, it
does not because to do so would require producers to carry their
own length with them.)

#### Associated Types

- `type Item`

#### Required Methods

- `fn split(self) -> (Self, Option<Self>)`

  Split midway into a new producer if possible, otherwise return `None`.

- `fn fold_with<F>(self, folder: F) -> F`

  Iterate the producer, feeding each element to `folder`, and

#### Implementors

- [`BytesProducer`](../../str/index.md#bytesproducer)
- [`CharIndicesProducer`](../../str/index.md#charindicesproducer)
- [`CharsProducer`](../../str/index.md#charsproducer)
- [`ChunkByProducer`](../../slice/chunk_by/index.md#chunkbyproducer)
- [`EncodeUtf16Producer`](../../str/index.md#encodeutf16producer)
- [`IterProducer`](../../range/index.md#iterproducer)
- [`MatchIndicesProducer`](../../str/index.md#matchindicesproducer)
- [`MatchesProducer`](../../str/index.md#matchesproducer)
- [`RepeatProducer`](../repeat/index.md#repeatproducer)
- [`SplitProducer`](../../split_producer/index.md#splitproducer)
- [`SplitProducer`](../splitter/index.md#splitproducer)
- [`SplitTerminatorProducer`](../../str/index.md#splitterminatorproducer)
- [`WalkTreePostfixProducer`](../walk_tree/index.md#walktreepostfixproducer)
- [`WalkTreePrefixProducer`](../walk_tree/index.md#walktreeprefixproducer)
- `&IterParallelProducer<'_, Iter>`

## Functions

### `bridge`

```rust
fn bridge<I, C>(par_iter: I, consumer: C) -> <C as >::Result
where
    I: IndexedParallelIterator,
    C: Consumer<<I as >::Item>
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:346-371`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L346-L371)*

This helper function is used to "connect" a parallel iterator to a
consumer. It will convert the `par_iter` into a producer P and
then pull items from P and feed them to `consumer`, splitting and
creating parallel threads as needed.

This is useful when you are implementing your own parallel
iterators: it is often used as the definition of the
`drive_unindexed` or `drive` methods.



### `bridge_producer_consumer`

```rust
fn bridge_producer_consumer<P, C>(len: usize, producer: P, consumer: C) -> <C as >::Result
where
    P: Producer,
    C: Consumer<<P as >::Item>
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:385-435`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L385-L435)*

This helper function is used to "connect" a producer and a
consumer. You may prefer to call [`bridge()`](#bridge), which wraps this
function. This function will draw items from `producer` and feed
them to `consumer`, splitting and creating parallel tasks when
needed.

This is useful when you are implementing your own parallel
iterators: it is often used as the definition of the
`drive_unindexed` or `drive` methods.



### `bridge_unindexed`

```rust
fn bridge_unindexed<P, C>(producer: P, consumer: C) -> <C as >::Result
where
    P: UnindexedProducer,
    C: UnindexedConsumer<<P as >::Item>
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:438-445`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L438-L445)*

A variant of [`bridge_producer_consumer()`](#bridge-producer-consumer) where the producer is an unindexed producer.

### `bridge_unindexed_producer_consumer`

```rust
fn bridge_unindexed_producer_consumer<P, C>(migrated: bool, splitter: Splitter, producer: P, consumer: C) -> <C as >::Result
where
    P: UnindexedProducer,
    C: UnindexedConsumer<<P as >::Item>
```

*Defined in [`rayon-1.11.0/src/iter/plumbing/mod.rs:447-476`](../../../../.source_1765521767/rayon-1.11.0/src/iter/plumbing/mod.rs#L447-L476)*

