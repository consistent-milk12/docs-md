*[rayon](../index.md) / [prelude](index.md)*

---

# Module `prelude`

The rayon prelude imports the various `ParallelIterator` traits.
The intention is that one can include `use rayon::prelude::*` and
have easy access to the various traits and methods you will need.

## Contents

- [Traits](#traits)
  - [`FromParallelIterator`](#fromparalleliterator)
  - [`IndexedParallelIterator`](#indexedparalleliterator)
  - [`IntoParallelIterator`](#intoparalleliterator)
  - [`IntoParallelRefIterator`](#intoparallelrefiterator)
  - [`IntoParallelRefMutIterator`](#intoparallelrefmutiterator)
  - [`ParallelBridge`](#parallelbridge)
  - [`ParallelDrainFull`](#paralleldrainfull)
  - [`ParallelDrainRange`](#paralleldrainrange)
  - [`ParallelExtend`](#parallelextend)
  - [`ParallelIterator`](#paralleliterator)
  - [`ParallelSlice`](#parallelslice)
  - [`ParallelSliceMut`](#parallelslicemut)
  - [`ParallelString`](#parallelstring)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FromParallelIterator`](#fromparalleliterator) | trait |  |
| [`IndexedParallelIterator`](#indexedparalleliterator) | trait |  |
| [`IntoParallelIterator`](#intoparalleliterator) | trait |  |
| [`IntoParallelRefIterator`](#intoparallelrefiterator) | trait |  |
| [`IntoParallelRefMutIterator`](#intoparallelrefmutiterator) | trait |  |
| [`ParallelBridge`](#parallelbridge) | trait |  |
| [`ParallelDrainFull`](#paralleldrainfull) | trait |  |
| [`ParallelDrainRange`](#paralleldrainrange) | trait |  |
| [`ParallelExtend`](#parallelextend) | trait |  |
| [`ParallelIterator`](#paralleliterator) | trait |  |
| [`ParallelSlice`](#parallelslice) | trait |  |
| [`ParallelSliceMut`](#parallelslicemut) | trait |  |
| [`ParallelString`](#parallelstring) | trait |  |

## Traits

### `FromParallelIterator<T>`

```rust
trait FromParallelIterator<T>
where
    T: Send { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3280-3303`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L3280-L3303)*

`FromParallelIterator` implements the creation of a collection
from a [`ParallelIterator`](../iter/index.md). By implementing
`FromParallelIterator` for a given type, you define how it will be
created from an iterator.

`FromParallelIterator` is used through [`ParallelIterator`](../iter/index.md)'s `collect()` method.

# Examples

Implementing `FromParallelIterator` for your type:

```rust
use rayon::prelude::*;

struct BlackHole {
    mass: usize,
}

impl<T: Send> FromParallelIterator<T> for BlackHole {
    fn from_par_iter<I>(par_iter: I) -> Self
        where I: IntoParallelIterator<Item = T>
    {
        let par_iter = par_iter.into_par_iter();
        BlackHole {
            mass: par_iter.count() * size_of::<T>(),
        }
    }
}

let bh: BlackHole = (0i32..1000).into_par_iter().collect();
assert_eq!(bh.mass, 4000);
```

#### Required Methods

- `fn from_par_iter<I>(par_iter: I) -> Self`

  Creates an instance of the collection from the parallel iterator `par_iter`.

#### Implementors

- `()`
- `(A, B)`
- `(FromA, FromB)`
- `Box<[T]>`
- `Box<str>`
- `Option<C>`
- `Result<C, E>`
- `String`
- `Vec<T>`
- `std::borrow::Cow<'a, C>`
- `std::collections::BTreeMap<K, V>`
- `std::collections::BTreeSet<V>`
- `std::collections::BinaryHeap<T>`
- `std::collections::HashMap<K, V, S>`
- `std::collections::HashSet<V, S>`
- `std::collections::LinkedList<T>`
- `std::collections::VecDeque<T>`
- `std::ffi::OsString`
- `std::rc::Rc<[T]>`
- `std::sync::Arc<[T]>`

### `IndexedParallelIterator`

```rust
trait IndexedParallelIterator: ParallelIterator { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:2439-3244`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L2439-L3244)*

An iterator that supports "random access" to its data, meaning
that you can split it at arbitrary indices and draw data from
those points.

**Note:** Not implemented for `u64`, `i64`, `u128`, or `i128` ranges

#### Required Methods

- `fn len(&self) -> usize`

  Produces an exact count of how many items this iterator will

- `fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result`

  Internal method used to define the behavior of this parallel

- `fn with_producer<CB: ProducerCallback<<Self as >::Item>>(self, callback: CB) -> <CB as >::Output`

  Internal method used to define the behavior of this parallel

#### Provided Methods

- `fn by_exponential_blocks(self) -> ExponentialBlocks<Self>`

  Divides an iterator into sequential blocks of exponentially-increasing size.

- `fn by_uniform_blocks(self, block_size: usize) -> UniformBlocks<Self>`

  Divides an iterator into sequential blocks of the given size.

- `fn collect_into_vec(self, target: &mut Vec<<Self as >::Item>)`

  Collects the results of the iterator into the specified

- `fn unzip_into_vecs<A, B>(self, left: &mut Vec<A>, right: &mut Vec<B>)`

  Unzips the results of the iterator into the specified

- `fn zip<Z>(self, zip_op: Z) -> Zip<Self, <Z as >::Iter>`

  Iterates over tuples `(A, B)`, where the items `A` are from

- `fn zip_eq<Z>(self, zip_op: Z) -> ZipEq<Self, <Z as >::Iter>`

  The same as `Zip`, but requires that both iterators have the same length.

- `fn interleave<I>(self, other: I) -> Interleave<Self, <I as >::Iter>`

  Interleaves elements of this iterator and the other given

- `fn interleave_shortest<I>(self, other: I) -> InterleaveShortest<Self, <I as >::Iter>`

  Interleaves elements of this iterator and the other given

- `fn chunks(self, chunk_size: usize) -> Chunks<Self>`

  Splits an iterator up into fixed-size chunks.

- `fn fold_chunks<T, ID, F>(self, chunk_size: usize, identity: ID, fold_op: F) -> FoldChunks<Self, ID, F>`

  Splits an iterator into fixed-size chunks, performing a sequential `fold()` on

- `fn fold_chunks_with<T, F>(self, chunk_size: usize, init: T, fold_op: F) -> FoldChunksWith<Self, T, F>`

  Splits an iterator into fixed-size chunks, performing a sequential `fold()` on

- `fn cmp<I>(self, other: I) -> Ordering`

  Lexicographically compares the elements of this `ParallelIterator` with those of

- `fn partial_cmp<I>(self, other: I) -> Option<Ordering>`

  Lexicographically compares the elements of this `ParallelIterator` with those of

- `fn eq<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn ne<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn lt<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn le<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn gt<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn ge<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn enumerate(self) -> Enumerate<Self>`

  Yields an index along with each item.

- `fn step_by(self, step: usize) -> StepBy<Self>`

   Creates an iterator that steps by the given amount

- `fn skip(self, n: usize) -> Skip<Self>`

  Creates an iterator that skips the first `n` elements.

- `fn take(self, n: usize) -> Take<Self>`

  Creates an iterator that yields the first `n` elements.

- `fn position_any<P>(self, predicate: P) -> Option<usize>`

  Searches for **some** item in the parallel iterator that

- `fn position_first<P>(self, predicate: P) -> Option<usize>`

  Searches for the sequentially **first** item in the parallel iterator

- `fn position_last<P>(self, predicate: P) -> Option<usize>`

  Searches for the sequentially **last** item in the parallel iterator

- `fn positions<P>(self, predicate: P) -> Positions<Self, P>`

  Searches for items in the parallel iterator that match the given

- `fn rev(self) -> Rev<Self>`

  Produces a new iterator with the elements of this iterator in

- `fn with_min_len(self, min: usize) -> MinLen<Self>`

  Sets the minimum length of iterators desired to process in each

- `fn with_max_len(self, max: usize) -> MaxLen<Self>`

  Sets the maximum length of iterators desired to process in each

#### Implementors

- [`Chain`](../iter/chain/index.md#chain)
- [`ChunksExactMut`](../slice/chunks/index.md#chunksexactmut)
- [`ChunksExact`](../slice/chunks/index.md#chunksexact)
- [`ChunksMut`](../slice/chunks/index.md#chunksmut)
- [`Chunks`](../iter/chunks/index.md#chunks)
- [`Chunks`](../slice/chunks/index.md#chunks)
- [`Cloned`](../iter/cloned/index.md#cloned)
- [`Copied`](../iter/copied/index.md#copied)
- [`Drain`](../collections/binary_heap/index.md#drain)
- [`Drain`](../collections/vec_deque/index.md#drain)
- [`Drain`](../vec/index.md#drain)
- [`Either`](../iter/index.md#either)
- [`Empty`](../iter/empty/index.md#empty)
- [`Enumerate`](../iter/enumerate/index.md#enumerate)
- [`FoldChunksWith`](../iter/fold_chunks_with/index.md#foldchunkswith)
- [`FoldChunks`](../iter/fold_chunks/index.md#foldchunks)
- [`Inspect`](../iter/inspect/index.md#inspect)
- [`InterleaveShortest`](../iter/interleave_shortest/index.md#interleaveshortest)
- [`Interleave`](../iter/interleave/index.md#interleave)
- [`Intersperse`](../iter/intersperse/index.md#intersperse)
- [`IntoIter`](../array/index.md#intoiter)
- [`IntoIter`](../collections/binary_heap/index.md#intoiter)
- [`IntoIter`](../collections/vec_deque/index.md#intoiter)
- [`IntoIter`](../option/index.md#intoiter)
- [`IntoIter`](../result/index.md#intoiter)
- [`IntoIter`](../vec/index.md#intoiter)
- [`IterMut`](../collections/vec_deque/index.md#itermut)
- [`IterMut`](../option/index.md#itermut)
- [`IterMut`](../result/index.md#itermut)
- [`IterMut`](../slice/index.md#itermut)
- [`Iter`](../collections/binary_heap/index.md#iter)
- [`Iter`](../collections/vec_deque/index.md#iter)
- [`Iter`](../option/index.md#iter)
- [`Iter`](../range/index.md#iter)
- [`Iter`](../range_inclusive/index.md#iter)
- [`Iter`](../result/index.md#iter)
- [`Iter`](../slice/index.md#iter)
- [`MapInit`](../iter/map_with/index.md#mapinit)
- [`MapWith`](../iter/map_with/index.md#mapwith)
- [`Map`](../iter/map/index.md#map)
- [`MaxLen`](../iter/len/index.md#maxlen)
- [`MinLen`](../iter/len/index.md#minlen)
- [`MultiZip`](../iter/multizip/index.md#multizip)
- [`Once`](../iter/once/index.md#once)
- [`PanicFuse`](../iter/panic_fuse/index.md#panicfuse)
- [`RChunksExactMut`](../slice/rchunks/index.md#rchunksexactmut)
- [`RChunksExact`](../slice/rchunks/index.md#rchunksexact)
- [`RChunksMut`](../slice/rchunks/index.md#rchunksmut)
- [`RChunks`](../slice/rchunks/index.md#rchunks)
- [`RepeatN`](../iter/repeat/index.md#repeatn)
- [`Rev`](../iter/rev/index.md#rev)
- [`Skip`](../iter/skip/index.md#skip)
- [`StepBy`](../iter/step_by/index.md#stepby)
- [`Take`](../iter/take/index.md#take)
- [`Update`](../iter/update/index.md#update)
- [`Windows`](../slice/index.md#windows)
- [`ZipEq`](../iter/zip_eq/index.md#zipeq)
- [`Zip`](../iter/zip/index.md#zip)

### `IntoParallelIterator`

```rust
trait IntoParallelIterator { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:219-249`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L219-L249)*

`IntoParallelIterator` implements the conversion to a [`ParallelIterator`](../iter/index.md).

By implementing `IntoParallelIterator` for a type, you define how it will
transformed into an iterator. This is a parallel version of the standard
library's `std::iter::IntoIterator` trait.

#### Associated Types

- `type Iter: 1`

- `type Item: 1`

#### Required Methods

- `fn into_par_iter(self) -> <Self as >::Iter`

  Converts `self` into a parallel iterator.

#### Implementors

- `&'a (A)`
- `&'a (A, B)`
- `&'a (A, B, C)`
- `&'a (A, B, C, D)`
- `&'a (A, B, C, D, E)`
- `&'a (A, B, C, D, E, F)`
- `&'a (A, B, C, D, E, F, G)`
- `&'a (A, B, C, D, E, F, G, H)`
- `&'a (A, B, C, D, E, F, G, H, I)`
- `&'a (A, B, C, D, E, F, G, H, I, J)`
- `&'a (A, B, C, D, E, F, G, H, I, J, K)`
- `&'a (A, B, C, D, E, F, G, H, I, J, K, L)`
- `&'a Option<T>`
- `&'a Result<T, E>`
- `&'a mut (A)`
- `&'a mut (A, B)`
- `&'a mut (A, B, C)`
- `&'a mut (A, B, C, D)`
- `&'a mut (A, B, C, D, E)`
- `&'a mut (A, B, C, D, E, F)`
- `&'a mut (A, B, C, D, E, F, G)`
- `&'a mut (A, B, C, D, E, F, G, H)`
- `&'a mut (A, B, C, D, E, F, G, H, I)`
- `&'a mut (A, B, C, D, E, F, G, H, I, J)`
- `&'a mut (A, B, C, D, E, F, G, H, I, J, K)`
- `&'a mut (A, B, C, D, E, F, G, H, I, J, K, L)`
- `&'a mut Option<T>`
- `&'a mut Result<T, E>`
- `&'a mut std::collections::BTreeMap<K, V>`
- `&'a mut std::collections::HashMap<K, V, S>`
- `&'a mut std::collections::LinkedList<T>`
- `&'a mut std::collections::VecDeque<T>`
- `&'a std::collections::BTreeMap<K, V>`
- `&'a std::collections::BTreeSet<T>`
- `&'a std::collections::BinaryHeap<T>`
- `&'a std::collections::HashMap<K, V, S>`
- `&'a std::collections::HashSet<T, S>`
- `&'a std::collections::LinkedList<T>`
- `&'a std::collections::VecDeque<T>`
- `&'data Box<[T]>`
- `&'data Vec<T>`
- `&'data [T; N]`
- `&'data [T]`
- `&'data mut Box<[T]>`
- `&'data mut Vec<T>`
- `&'data mut [T; N]`
- `&'data mut [T]`
- `(A)`
- `(A, B)`
- `(A, B, C)`
- `(A, B, C, D)`
- `(A, B, C, D, E)`
- `(A, B, C, D, E, F)`
- `(A, B, C, D, E, F, G)`
- `(A, B, C, D, E, F, G, H)`
- `(A, B, C, D, E, F, G, H, I)`
- `(A, B, C, D, E, F, G, H, I, J)`
- `(A, B, C, D, E, F, G, H, I, J, K)`
- `(A, B, C, D, E, F, G, H, I, J, K, L)`
- `Box<[T]>`
- `Option<T>`
- `Result<T, E>`
- `T`
- `Vec<T>`
- `[T; N]`
- `std::collections::BTreeMap<K, V>`
- `std::collections::BTreeSet<T>`
- `std::collections::BinaryHeap<T>`
- `std::collections::HashMap<K, V, S>`
- `std::collections::HashSet<T, S>`
- `std::collections::LinkedList<T>`
- `std::collections::VecDeque<T>`
- `std::ops::Range<T>`
- `std::ops::RangeInclusive<T>`

### `IntoParallelRefIterator<'data>`

```rust
trait IntoParallelRefIterator<'data> { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:261-285`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L261-L285)*

`IntoParallelRefIterator` implements the conversion to a
[`ParallelIterator`](../iter/index.md), providing shared references to the data.

This is a parallel version of the `iter()` method
defined by various collections.

This trait is automatically implemented
`for I where &I: IntoParallelIterator`. In most cases, users
will want to implement [`IntoParallelIterator`](../iter/index.md) rather than implement
this trait directly.

#### Associated Types

- `type Iter: 1`

- `type Item: 2`

#### Required Methods

- `fn par_iter(self: &'data Self) -> <Self as >::Iter`

  Converts `self` into a parallel iterator.

#### Implementors

- `I`

### `IntoParallelRefMutIterator<'data>`

```rust
trait IntoParallelRefMutIterator<'data> { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:309-329`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L309-L329)*

`IntoParallelRefMutIterator` implements the conversion to a
[`ParallelIterator`](../iter/index.md), providing mutable references to the data.

This is a parallel version of the `iter_mut()` method
defined by various collections.

This trait is automatically implemented
`for I where &mut I: IntoParallelIterator`. In most cases, users
will want to implement [`IntoParallelIterator`](../iter/index.md) rather than implement
this trait directly.

#### Associated Types

- `type Iter: 1`

- `type Item: 2`

#### Required Methods

- `fn par_iter_mut(self: &'data mut Self) -> <Self as >::Iter`

  Creates the parallel iterator from `self`.

#### Implementors

- `I`

### `ParallelBridge`

```rust
trait ParallelBridge: Sized { ... }
```

*Defined in [`rayon-1.11.0/src/iter/par_bridge.rs:53-56`](../../../.source_1765521767/rayon-1.11.0/src/iter/par_bridge.rs#L53-L56)*

Conversion trait to convert an `Iterator` to a `ParallelIterator`.

This creates a "bridge" from a sequential iterator to a parallel one, by distributing its items
across the Rayon thread pool. This has the advantage of being able to parallelize just about
anything, but the resulting `ParallelIterator` can be less efficient than if you started with
`par_iter` instead. However, it can still be useful for iterators that are difficult to
parallelize by other means, like channels or file or network I/O.

Iterator items are pulled by `next()` one at a time, synchronized from each thread that is
ready for work, so this may become a bottleneck if the serial iterator can't keep up with the
parallel demand. The items are not buffered by `IterBridge`, so it's fine to use this with
large or even unbounded iterators.

The resulting iterator is not guaranteed to keep the order of the original iterator.

# Examples

To use this trait, take an existing `Iterator` and call `par_bridge` on it. After that, you can
use any of the `ParallelIterator` methods:

```rust
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::sync::mpsc::channel;

let rx = {
    let (tx, rx) = channel();

    tx.send("one!");
    tx.send("two!");
    tx.send("three!");

    rx
};

let mut output: Vec<&'static str> = rx.into_iter().par_bridge().collect();
output.sort_unstable();

assert_eq!(&*output, &["one!", "three!", "two!"]);
```

#### Required Methods

- `fn par_bridge(self) -> IterBridge<Self>`

  Creates a bridge from this type to a `ParallelIterator`.

#### Implementors

- `T`

### `ParallelDrainFull`

```rust
trait ParallelDrainFull { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3360-3394`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L3360-L3394)*

`ParallelDrainFull` creates a parallel iterator that moves all items
from a collection while retaining the original capacity.

Types which are indexable typically implement [`ParallelDrainRange`](../iter/index.md)
instead, where you can drain fully with `par_drain(..)`.

#### Associated Types

- `type Iter: 1`

- `type Item: 1`

#### Required Methods

- `fn par_drain(self) -> <Self as >::Iter`

  Returns a draining parallel iterator over an entire collection.

#### Implementors

- `&'a mut std::collections::BinaryHeap<T>`
- `&'a mut std::collections::HashMap<K, V, S>`
- `&'a mut std::collections::HashSet<T, S>`

### `ParallelDrainRange<Idx>`

```rust
trait ParallelDrainRange<Idx> { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3400-3467`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L3400-L3467)*

`ParallelDrainRange` creates a parallel iterator that moves a range of items
from a collection while retaining the original capacity.

Types which are not indexable may implement [`ParallelDrainFull`](../iter/index.md) instead.

#### Associated Types

- `type Iter: 1`

- `type Item: 1`

#### Required Methods

- `fn par_drain<R: RangeBounds<Idx>>(self, range: R) -> <Self as >::Iter`

  Returns a draining parallel iterator over a range of the collection.

#### Implementors

- `&'a mut DrainGuard<'_, T, C>`
- `&'a mut String`
- `&'a mut std::collections::VecDeque<T>`
- `&'data mut Vec<T>`

### `ParallelExtend<T>`

```rust
trait ParallelExtend<T>
where
    T: Send { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3333-3353`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L3333-L3353)*

`ParallelExtend` extends an existing collection with items from a [`ParallelIterator`](../iter/index.md).

# Examples

Implementing `ParallelExtend` for your type:

```rust
use rayon::prelude::*;

struct BlackHole {
    mass: usize,
}

impl<T: Send> ParallelExtend<T> for BlackHole {
    fn par_extend<I>(&mut self, par_iter: I)
        where I: IntoParallelIterator<Item = T>
    {
        let par_iter = par_iter.into_par_iter();
        self.mass += par_iter.count() * size_of::<T>();
    }
}

let mut bh = BlackHole { mass: 0 };
bh.par_extend(0i32..1000);
assert_eq!(bh.mass, 4000);
bh.par_extend(0i64..10);
assert_eq!(bh.mass, 4080);
```

#### Required Methods

- `fn par_extend<I>(&mut self, par_iter: I)`

  Extends an instance of the collection with the elements drawn

#### Implementors

- [`Collector`](../iter/unzip/index.md#collector)
- [`Either`](../iter/index.md#either)
- `()`
- `(A, B)`
- `(FromA, FromB)`
- `String`
- `Vec<T>`
- `std::collections::BTreeMap<K, V>`
- `std::collections::BTreeSet<T>`
- `std::collections::BinaryHeap<T>`
- `std::collections::HashMap<K, V, S>`
- `std::collections::HashSet<T, S>`
- `std::collections::LinkedList<T>`
- `std::collections::VecDeque<T>`
- `std::ffi::OsString`

### `ParallelIterator`

```rust
trait ParallelIterator: Sized + Send { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:356-2421`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L356-L2421)*

Parallel version of the standard iterator trait.

The combinators on this trait are available on **all** parallel
iterators.  Additional methods can be found on the
[`IndexedParallelIterator`](../iter/index.md) trait: those methods are only
available for parallel iterators where the number of items is
known in advance (so, e.g., after invoking `filter`, those methods
become unavailable).

For examples of using parallel iterators, see [the docs on the
`iter` module][`iter`](../iter/index.md).


#### Associated Types

- `type Item: 1`

#### Required Methods

- `fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result`

  Internal method used to define the behavior of this parallel

#### Provided Methods

- `fn for_each<OP>(self, op: OP)`

  Executes `OP` on each item produced by the iterator, in parallel.

- `fn for_each_with<OP, T>(self, init: T, op: OP)`

  Executes `OP` on the given `init` value with each item produced by

- `fn for_each_init<OP, INIT, T>(self, init: INIT, op: OP)`

  Executes `OP` on a value returned by `init` with each item produced by

- `fn try_for_each<OP, R>(self, op: OP) -> R`

  Executes a fallible `OP` on each item produced by the iterator, in parallel.

- `fn try_for_each_with<OP, T, R>(self, init: T, op: OP) -> R`

  Executes a fallible `OP` on the given `init` value with each item

- `fn try_for_each_init<OP, INIT, T, R>(self, init: INIT, op: OP) -> R`

  Executes a fallible `OP` on a value returned by `init` with each item

- `fn count(self) -> usize`

  Counts the number of items in this parallel iterator.

- `fn map<F, R>(self, map_op: F) -> Map<Self, F>`

  Applies `map_op` to each item of this iterator, producing a new

- `fn map_with<F, T, R>(self, init: T, map_op: F) -> MapWith<Self, T, F>`

  Applies `map_op` to the given `init` value with each item of this

- `fn map_init<F, INIT, T, R>(self, init: INIT, map_op: F) -> MapInit<Self, INIT, F>`

  Applies `map_op` to a value returned by `init` with each item of this

- `fn cloned<'a, T>(self) -> Cloned<Self>`

  Creates an iterator which clones all of its elements.  This may be

- `fn copied<'a, T>(self) -> Copied<Self>`

  Creates an iterator which copies all of its elements.  This may be

- `fn inspect<OP>(self, inspect_op: OP) -> Inspect<Self, OP>`

  Applies `inspect_op` to a reference to each item of this iterator,

- `fn update<F>(self, update_op: F) -> Update<Self, F>`

  Mutates each item of this iterator before yielding it.

- `fn filter<P>(self, filter_op: P) -> Filter<Self, P>`

  Applies `filter_op` to each item of this iterator, producing a new

- `fn filter_map<P, R>(self, filter_op: P) -> FilterMap<Self, P>`

  Applies `filter_op` to each item of this iterator to get an `Option`,

- `fn flat_map<F, PI>(self, map_op: F) -> FlatMap<Self, F>`

  Applies `map_op` to each item of this iterator to get nested parallel iterators,

- `fn flat_map_iter<F, SI>(self, map_op: F) -> FlatMapIter<Self, F>`

  Applies `map_op` to each item of this iterator to get nested serial iterators,

- `fn flatten(self) -> Flatten<Self>`

  An adaptor that flattens parallel-iterable `Item`s into one large iterator.

- `fn flatten_iter(self) -> FlattenIter<Self>`

  An adaptor that flattens serial-iterable `Item`s into one large iterator.

- `fn reduce<OP, ID>(self, identity: ID, op: OP) -> <Self as >::Item`

  Reduces the items in the iterator into one item using `op`.

- `fn reduce_with<OP>(self, op: OP) -> Option<<Self as >::Item>`

  Reduces the items in the iterator into one item using `op`.

- `fn try_reduce<T, OP, ID>(self, identity: ID, op: OP) -> <Self as >::Item`

  Reduces the items in the iterator into one item using a fallible `op`.

- `fn try_reduce_with<T, OP>(self, op: OP) -> Option<<Self as >::Item>`

  Reduces the items in the iterator into one item using a fallible `op`.

- `fn fold<T, ID, F>(self, identity: ID, fold_op: F) -> Fold<Self, ID, F>`

  Parallel fold is similar to sequential fold except that the

- `fn fold_with<F, T>(self, init: T, fold_op: F) -> FoldWith<Self, T, F>`

  Applies `fold_op` to the given `init` value with each item of this

- `fn try_fold<T, R, ID, F>(self, identity: ID, fold_op: F) -> TryFold<Self, R, ID, F>`

  Performs a fallible parallel fold.

- `fn try_fold_with<F, T, R>(self, init: T, fold_op: F) -> TryFoldWith<Self, R, F>`

  Performs a fallible parallel fold with a cloneable `init` value.

- `fn sum<S>(self) -> S`

  Sums up the items in the iterator.

- `fn product<P>(self) -> P`

  Multiplies all the items in the iterator.

- `fn min(self) -> Option<<Self as >::Item>`

  Computes the minimum of all the items in the iterator. If the

- `fn min_by<F>(self, f: F) -> Option<<Self as >::Item>`

  Computes the minimum of all the items in the iterator with respect to

- `fn min_by_key<K, F>(self, f: F) -> Option<<Self as >::Item>`

  Computes the item that yields the minimum value for the given

- `fn max(self) -> Option<<Self as >::Item>`

  Computes the maximum of all the items in the iterator. If the

- `fn max_by<F>(self, f: F) -> Option<<Self as >::Item>`

  Computes the maximum of all the items in the iterator with respect to

- `fn max_by_key<K, F>(self, f: F) -> Option<<Self as >::Item>`

  Computes the item that yields the maximum value for the given

- `fn chain<C>(self, chain: C) -> Chain<Self, <C as >::Iter>`

  Takes two iterators and creates a new iterator over both.

- `fn find_any<P>(self, predicate: P) -> Option<<Self as >::Item>`

  Searches for **some** item in the parallel iterator that

- `fn find_first<P>(self, predicate: P) -> Option<<Self as >::Item>`

  Searches for the sequentially **first** item in the parallel iterator

- `fn find_last<P>(self, predicate: P) -> Option<<Self as >::Item>`

  Searches for the sequentially **last** item in the parallel iterator

- `fn find_map_any<P, R>(self, predicate: P) -> Option<R>`

  Applies the given predicate to the items in the parallel iterator

- `fn find_map_first<P, R>(self, predicate: P) -> Option<R>`

  Applies the given predicate to the items in the parallel iterator and

- `fn find_map_last<P, R>(self, predicate: P) -> Option<R>`

  Applies the given predicate to the items in the parallel iterator and

- `fn any<P>(self, predicate: P) -> bool`

  Searches for **some** item in the parallel iterator that

- `fn all<P>(self, predicate: P) -> bool`

  Tests that every item in the parallel iterator matches the given

- `fn while_some<T>(self) -> WhileSome<Self>`

  Creates an iterator over the `Some` items of this iterator, halting

- `fn panic_fuse(self) -> PanicFuse<Self>`

  Wraps an iterator with a fuse in case of panics, to halt all threads

- `fn collect<C>(self) -> C`

  Creates a fresh collection containing all the elements produced

- `fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)`

  Unzips the items of a parallel iterator into a pair of arbitrary

- `fn partition<A, B, P>(self, predicate: P) -> (A, B)`

  Partitions the items of a parallel iterator into a pair of arbitrary

- `fn partition_map<A, B, P, L, R>(self, predicate: P) -> (A, B)`

  Partitions and maps the items of a parallel iterator into a pair of

- `fn intersperse(self, element: <Self as >::Item) -> Intersperse<Self>`

  Intersperses clones of an element between items of this iterator.

- `fn take_any(self, n: usize) -> TakeAny<Self>`

  Creates an iterator that yields `n` elements from *anywhere* in the original iterator.

- `fn skip_any(self, n: usize) -> SkipAny<Self>`

  Creates an iterator that skips `n` elements from *anywhere* in the original iterator.

- `fn take_any_while<P>(self, predicate: P) -> TakeAnyWhile<Self, P>`

  Creates an iterator that takes elements from *anywhere* in the original iterator

- `fn skip_any_while<P>(self, predicate: P) -> SkipAnyWhile<Self, P>`

  Creates an iterator that skips elements from *anywhere* in the original iterator

- `fn collect_vec_list(self) -> LinkedList<Vec<<Self as >::Item>>`

  Collects this iterator into a linked list of vectors.

- `fn opt_len(&self) -> Option<usize>`

  Internal method used to define the behavior of this parallel

#### Implementors

- [`Bytes`](../str/index.md#bytes)
- [`Chain`](../iter/chain/index.md#chain)
- [`CharIndices`](../str/index.md#charindices)
- [`Chars`](../str/index.md#chars)
- [`ChunkByMut`](../slice/chunk_by/index.md#chunkbymut)
- [`ChunkBy`](../slice/chunk_by/index.md#chunkby)
- [`ChunksExactMut`](../slice/chunks/index.md#chunksexactmut)
- [`ChunksExact`](../slice/chunks/index.md#chunksexact)
- [`ChunksMut`](../slice/chunks/index.md#chunksmut)
- [`Chunks`](../iter/chunks/index.md#chunks)
- [`Chunks`](../slice/chunks/index.md#chunks)
- [`Cloned`](../iter/cloned/index.md#cloned)
- [`Copied`](../iter/copied/index.md#copied)
- [`Drain`](../collections/binary_heap/index.md#drain)
- [`Drain`](../collections/hash_map/index.md#drain)
- [`Drain`](../collections/hash_set/index.md#drain)
- [`Drain`](../collections/vec_deque/index.md#drain)
- [`Drain`](../string/index.md#drain)
- [`Drain`](../vec/index.md#drain)
- [`Either`](../iter/index.md#either)
- [`Empty`](../iter/empty/index.md#empty)
- [`EncodeUtf16`](../str/index.md#encodeutf16)
- [`Enumerate`](../iter/enumerate/index.md#enumerate)
- [`ExponentialBlocks`](../iter/blocks/index.md#exponentialblocks)
- [`FilterMap`](../iter/filter_map/index.md#filtermap)
- [`Filter`](../iter/filter/index.md#filter)
- [`FlatMapIter`](../iter/flat_map_iter/index.md#flatmapiter)
- [`FlatMap`](../iter/flat_map/index.md#flatmap)
- [`FlattenIter`](../iter/flatten_iter/index.md#flatteniter)
- [`Flatten`](../iter/flatten/index.md#flatten)
- [`FoldChunksWith`](../iter/fold_chunks_with/index.md#foldchunkswith)
- [`FoldChunks`](../iter/fold_chunks/index.md#foldchunks)
- [`FoldWith`](../iter/fold/index.md#foldwith)
- [`Fold`](../iter/fold/index.md#fold)
- [`Inspect`](../iter/inspect/index.md#inspect)
- [`InterleaveShortest`](../iter/interleave_shortest/index.md#interleaveshortest)
- [`Interleave`](../iter/interleave/index.md#interleave)
- [`Intersperse`](../iter/intersperse/index.md#intersperse)
- [`IntoIter`](../array/index.md#intoiter)
- [`IntoIter`](../collections/binary_heap/index.md#intoiter)
- [`IntoIter`](../collections/btree_map/index.md#intoiter)
- [`IntoIter`](../collections/btree_set/index.md#intoiter)
- [`IntoIter`](../collections/hash_map/index.md#intoiter)
- [`IntoIter`](../collections/hash_set/index.md#intoiter)
- [`IntoIter`](../collections/linked_list/index.md#intoiter)
- [`IntoIter`](../collections/vec_deque/index.md#intoiter)
- [`IntoIter`](../option/index.md#intoiter)
- [`IntoIter`](../result/index.md#intoiter)
- [`IntoIter`](../vec/index.md#intoiter)
- [`IterBridge`](../iter/par_bridge/index.md#iterbridge)
- [`IterMut`](../collections/btree_map/index.md#itermut)
- [`IterMut`](../collections/hash_map/index.md#itermut)
- [`IterMut`](../collections/linked_list/index.md#itermut)
- [`IterMut`](../collections/vec_deque/index.md#itermut)
- [`IterMut`](../option/index.md#itermut)
- [`IterMut`](../result/index.md#itermut)
- [`IterMut`](../slice/index.md#itermut)
- [`Iter`](../collections/binary_heap/index.md#iter)
- [`Iter`](../collections/btree_map/index.md#iter)
- [`Iter`](../collections/btree_set/index.md#iter)
- [`Iter`](../collections/hash_map/index.md#iter)
- [`Iter`](../collections/hash_set/index.md#iter)
- [`Iter`](../collections/linked_list/index.md#iter)
- [`Iter`](../collections/vec_deque/index.md#iter)
- [`Iter`](../option/index.md#iter)
- [`Iter`](../range/index.md#iter)
- [`Iter`](../range_inclusive/index.md#iter)
- [`Iter`](../result/index.md#iter)
- [`Iter`](../slice/index.md#iter)
- [`Lines`](../str/index.md#lines)
- [`MapInit`](../iter/map_with/index.md#mapinit)
- [`MapWith`](../iter/map_with/index.md#mapwith)
- [`Map`](../iter/map/index.md#map)
- [`MatchIndices`](../str/index.md#matchindices)
- [`Matches`](../str/index.md#matches)
- [`MaxLen`](../iter/len/index.md#maxlen)
- [`MinLen`](../iter/len/index.md#minlen)
- [`MultiZip`](../iter/multizip/index.md#multizip)
- [`Once`](../iter/once/index.md#once)
- [`PanicFuse`](../iter/panic_fuse/index.md#panicfuse)
- [`Positions`](../iter/positions/index.md#positions)
- [`RChunksExactMut`](../slice/rchunks/index.md#rchunksexactmut)
- [`RChunksExact`](../slice/rchunks/index.md#rchunksexact)
- [`RChunksMut`](../slice/rchunks/index.md#rchunksmut)
- [`RChunks`](../slice/rchunks/index.md#rchunks)
- [`RepeatN`](../iter/repeat/index.md#repeatn)
- [`Repeat`](../iter/repeat/index.md#repeat)
- [`Rev`](../iter/rev/index.md#rev)
- [`SkipAnyWhile`](../iter/skip_any_while/index.md#skipanywhile)
- [`SkipAny`](../iter/skip_any/index.md#skipany)
- [`Skip`](../iter/skip/index.md#skip)
- [`SplitAsciiWhitespace`](../str/index.md#splitasciiwhitespace)
- [`SplitInclusiveMut`](../slice/index.md#splitinclusivemut)
- [`SplitInclusive`](../slice/index.md#splitinclusive)
- [`SplitInclusive`](../str/index.md#splitinclusive)
- [`SplitMut`](../slice/index.md#splitmut)
- [`SplitTerminator`](../str/index.md#splitterminator)
- [`SplitWhitespace`](../str/index.md#splitwhitespace)
- [`Split`](../iter/splitter/index.md#split)
- [`Split`](../slice/index.md#split)
- [`Split`](../str/index.md#split)
- [`StepBy`](../iter/step_by/index.md#stepby)
- [`TakeAnyWhile`](../iter/take_any_while/index.md#takeanywhile)
- [`TakeAny`](../iter/take_any/index.md#takeany)
- [`Take`](../iter/take/index.md#take)
- [`TryFoldWith`](../iter/try_fold/index.md#tryfoldwith)
- [`TryFold`](../iter/try_fold/index.md#tryfold)
- [`UniformBlocks`](../iter/blocks/index.md#uniformblocks)
- [`UnzipA`](../iter/unzip/index.md#unzipa)
- [`UnzipB`](../iter/unzip/index.md#unzipb)
- [`Update`](../iter/update/index.md#update)
- [`WalkTreePostfix`](../iter/walk_tree/index.md#walktreepostfix)
- [`WalkTreePrefix`](../iter/walk_tree/index.md#walktreeprefix)
- [`WalkTree`](../iter/walk_tree/index.md#walktree)
- [`WhileSome`](../iter/while_some/index.md#whilesome)
- [`Windows`](../slice/index.md#windows)
- [`ZipEq`](../iter/zip_eq/index.md#zipeq)
- [`Zip`](../iter/zip/index.md#zip)

### `ParallelSlice<T: Sync>`

```rust
trait ParallelSlice<T: Sync> { ... }
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:29-199`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L29-L199)*

Parallel extensions for slices.

#### Required Methods

- `fn as_parallel_slice(&self) -> &[T]`

  Returns a plain slice, which is used to implement the rest of the

#### Provided Methods

- `fn par_split<P>(&self, separator: P) -> Split<'_, T, P>`

  Returns a parallel iterator over subslices separated by elements that

- `fn par_split_inclusive<P>(&self, separator: P) -> SplitInclusive<'_, T, P>`

  Returns a parallel iterator over subslices separated by elements that

- `fn par_windows(&self, window_size: usize) -> Windows<'_, T>`

  Returns a parallel iterator over all contiguous windows of length

- `fn par_chunks(&self, chunk_size: usize) -> Chunks<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of

- `fn par_chunks_exact(&self, chunk_size: usize) -> ChunksExact<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of

- `fn par_rchunks(&self, chunk_size: usize) -> RChunks<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of `self` at a time,

- `fn par_rchunks_exact(&self, chunk_size: usize) -> RChunksExact<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of `self` at a time,

- `fn par_chunk_by<F>(&self, pred: F) -> ChunkBy<'_, T, F>`

  Returns a parallel iterator over the slice producing non-overlapping runs

#### Implementors

- `[T]`

### `ParallelSliceMut<T: Send>`

```rust
trait ParallelSliceMut<T: Send> { ... }
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:209-754`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L209-L754)*

Parallel extensions for mutable slices.

#### Required Methods

- `fn as_parallel_slice_mut(&mut self) -> &mut [T]`

  Returns a plain mutable slice, which is used to implement the rest of

#### Provided Methods

- `fn par_split_mut<P>(&mut self, separator: P) -> SplitMut<'_, T, P>`

  Returns a parallel iterator over mutable subslices separated by

- `fn par_split_inclusive_mut<P>(&mut self, separator: P) -> SplitInclusiveMut<'_, T, P>`

  Returns a parallel iterator over mutable subslices separated by elements

- `fn par_chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of

- `fn par_chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of

- `fn par_rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of `self` at a time,

- `fn par_rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of `self` at a time,

- `fn par_sort(&mut self)`

  Sorts the slice in parallel.

- `fn par_sort_by<F>(&mut self, compare: F)`

  Sorts the slice in parallel with a comparator function.

- `fn par_sort_by_key<K, F>(&mut self, f: F)`

  Sorts the slice in parallel with a key extraction function.

- `fn par_sort_by_cached_key<K, F>(&mut self, f: F)`

  Sorts the slice in parallel with a key extraction function.

- `fn par_sort_unstable(&mut self)`

  Sorts the slice in parallel, but might not preserve the order of equal elements.

- `fn par_sort_unstable_by<F>(&mut self, compare: F)`

  Sorts the slice in parallel with a comparator function, but might not preserve the order of

- `fn par_sort_unstable_by_key<K, F>(&mut self, f: F)`

  Sorts the slice in parallel with a key extraction function, but might not preserve the order

- `fn par_chunk_by_mut<F>(&mut self, pred: F) -> ChunkByMut<'_, T, F>`

  Returns a parallel iterator over the slice producing non-overlapping mutable

#### Implementors

- `[T]`

### `ParallelString`

```rust
trait ParallelString { ... }
```

*Defined in [`rayon-1.11.0/src/str.rs:58-342`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L58-L342)*

Parallel extensions for strings.

#### Required Methods

- `fn as_parallel_string(&self) -> &str`

  Returns a plain string slice, which is used to implement the rest of

#### Provided Methods

- `fn par_chars(&self) -> Chars<'_>`

  Returns a parallel iterator over the characters of a string.

- `fn par_char_indices(&self) -> CharIndices<'_>`

  Returns a parallel iterator over the characters of a string, with their positions.

- `fn par_bytes(&self) -> Bytes<'_>`

  Returns a parallel iterator over the bytes of a string.

- `fn par_encode_utf16(&self) -> EncodeUtf16<'_>`

  Returns a parallel iterator over a string encoded as UTF-16.

- `fn par_split<P: Pattern>(&self, separator: P) -> Split<'_, P>`

  Returns a parallel iterator over substrings separated by a

- `fn par_split_inclusive<P: Pattern>(&self, separator: P) -> SplitInclusive<'_, P>`

  Returns a parallel iterator over substrings separated by a

- `fn par_split_terminator<P: Pattern>(&self, terminator: P) -> SplitTerminator<'_, P>`

  Returns a parallel iterator over substrings terminated by a

- `fn par_lines(&self) -> Lines<'_>`

  Returns a parallel iterator over the lines of a string, ending with an

- `fn par_split_whitespace(&self) -> SplitWhitespace<'_>`

  Returns a parallel iterator over the sub-slices of a string that are

- `fn par_split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_>`

  Returns a parallel iterator over the sub-slices of a string that are

- `fn par_matches<P: Pattern>(&self, pattern: P) -> Matches<'_, P>`

  Returns a parallel iterator over substrings that match a

- `fn par_match_indices<P: Pattern>(&self, pattern: P) -> MatchIndices<'_, P>`

  Returns a parallel iterator over substrings that match a given character

#### Implementors

- `str`

