*[rayon](../index.md) / [iter](index.md)*

---

# Module `iter`

Traits for writing parallel programs using an iterator-style interface

You will rarely need to interact with this module directly unless you have
need to name one of the iterator types.

Parallel iterators make it easy to write iterator-like chains that
execute in parallel: typically all you have to do is convert the
first `.iter()` (or `iter_mut()`, `into_iter()`, etc) method into
`par_iter()` (or `par_iter_mut()`, `into_par_iter()`, etc). For
example, to compute the sum of the squares of a sequence of
integers, one might write:

```rust
use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter()
         .map(|i| i * i)
         .sum()
}
```

Or, to increment all the integers in a slice, you could write:

```rust
use rayon::prelude::*;
fn increment_all(input: &mut [i32]) {
    input.par_iter_mut()
         .for_each(|p| *p += 1);
}
```

To use parallel iterators, first import the traits by adding
something like `use rayon::prelude::*` to your module. You can
then call `par_iter`, `par_iter_mut`, or `into_par_iter` to get a
parallel iterator. Like a [regular iterator][], parallel
iterators work by first constructing a computation and then
executing it.

In addition to `par_iter()` and friends, some types offer other
ways to create (or consume) parallel iterators:

- Slices (`&[T]`, `&mut [T]`) offer methods like `par_split` and
  `par_windows`, as well as various parallel sorting
  operations. See [the `ParallelSlice` trait] for the full list.
- Strings (`&str`) offer methods like `par_split` and `par_lines`.
  See [the `ParallelString` trait] for the full list.
- Various collections offer `par_extend`, which grows a
  collection given a parallel iterator. (If you don't have a
  collection to extend, you can use `collect()` to create a new
  one from scratch.)




To see the full range of methods available on parallel iterators,
check out the [`ParallelIterator`](#paralleliterator) and [`IndexedParallelIterator`](#indexedparalleliterator)
traits.

If you'd like to build a custom parallel iterator, or to write your own
combinator, then check out the [`split`](splitter/index.md) function and the [`plumbing`](plumbing/index.md) module.



Note: Several of the `ParallelIterator` methods rely on a `Try` trait which
has been deliberately obscured from the public API.  This trait is intended
to mirror the unstable `std::ops::Try` with implementations for `Option` and
`Result`, where `Some`/`Ok` values will let those iterators continue, but
`None`/`Err` values will exit early.

A note about object safety: It is currently _not_ possible to wrap
a `ParallelIterator` (or any trait that depends on it) using a
`Box<dyn ParallelIterator>` or other kind of dynamic allocation,
because `ParallelIterator` is **not object-safe**.
(This keeps the implementation simpler and allows extra optimizations.)

## Contents

- [Modules](#modules)
  - [`plumbing`](#plumbing)
  - [`blocks`](#blocks)
  - [`chain`](#chain)
  - [`chunks`](#chunks)
  - [`cloned`](#cloned)
  - [`collect`](#collect)
  - [`copied`](#copied)
  - [`empty`](#empty)
  - [`enumerate`](#enumerate)
  - [`extend`](#extend)
  - [`filter`](#filter)
  - [`filter_map`](#filter_map)
  - [`find`](#find)
  - [`find_first_last`](#find_first_last)
  - [`flat_map`](#flat_map)
  - [`flat_map_iter`](#flat_map_iter)
  - [`flatten`](#flatten)
  - [`flatten_iter`](#flatten_iter)
  - [`fold`](#fold)
  - [`fold_chunks`](#fold_chunks)
  - [`fold_chunks_with`](#fold_chunks_with)
  - [`for_each`](#for_each)
  - [`from_par_iter`](#from_par_iter)
  - [`inspect`](#inspect)
  - [`interleave`](#interleave)
  - [`interleave_shortest`](#interleave_shortest)
  - [`intersperse`](#intersperse)
  - [`len`](#len)
  - [`map`](#map)
  - [`map_with`](#map_with)
  - [`multizip`](#multizip)
  - [`noop`](#noop)
  - [`once`](#once)
  - [`panic_fuse`](#panic_fuse)
  - [`par_bridge`](#par_bridge)
  - [`positions`](#positions)
  - [`product`](#product)
  - [`reduce`](#reduce)
  - [`repeat`](#repeat)
  - [`rev`](#rev)
  - [`skip`](#skip)
  - [`skip_any`](#skip_any)
  - [`skip_any_while`](#skip_any_while)
  - [`splitter`](#splitter)
  - [`step_by`](#step_by)
  - [`sum`](#sum)
  - [`take`](#take)
  - [`take_any`](#take_any)
  - [`take_any_while`](#take_any_while)
  - [`try_fold`](#try_fold)
  - [`try_reduce`](#try_reduce)
  - [`try_reduce_with`](#try_reduce_with)
  - [`unzip`](#unzip)
  - [`update`](#update)
  - [`walk_tree`](#walk_tree)
  - [`while_some`](#while_some)
  - [`zip`](#zip)
  - [`zip_eq`](#zip_eq)
  - [`private`](#private)
- [Structs](#structs)
  - [`ExponentialBlocks`](#exponentialblocks)
  - [`UniformBlocks`](#uniformblocks)
  - [`Chain`](#chain)
  - [`Chunks`](#chunks)
  - [`Cloned`](#cloned)
  - [`Copied`](#copied)
  - [`Empty`](#empty)
  - [`Enumerate`](#enumerate)
  - [`Filter`](#filter)
  - [`FilterMap`](#filtermap)
  - [`FlatMap`](#flatmap)
  - [`FlatMapIter`](#flatmapiter)
  - [`Flatten`](#flatten)
  - [`FlattenIter`](#flatteniter)
  - [`Fold`](#fold)
  - [`FoldWith`](#foldwith)
  - [`FoldChunks`](#foldchunks)
  - [`FoldChunksWith`](#foldchunkswith)
  - [`Inspect`](#inspect)
  - [`Interleave`](#interleave)
  - [`InterleaveShortest`](#interleaveshortest)
  - [`Intersperse`](#intersperse)
  - [`MaxLen`](#maxlen)
  - [`MinLen`](#minlen)
  - [`Map`](#map)
  - [`MapInit`](#mapinit)
  - [`MapWith`](#mapwith)
  - [`MultiZip`](#multizip)
  - [`Once`](#once)
  - [`PanicFuse`](#panicfuse)
  - [`IterBridge`](#iterbridge)
  - [`Positions`](#positions)
  - [`Repeat`](#repeat)
  - [`RepeatN`](#repeatn)
  - [`Rev`](#rev)
  - [`Skip`](#skip)
  - [`SkipAny`](#skipany)
  - [`SkipAnyWhile`](#skipanywhile)
  - [`Split`](#split)
  - [`StepBy`](#stepby)
  - [`Take`](#take)
  - [`TakeAny`](#takeany)
  - [`TakeAnyWhile`](#takeanywhile)
  - [`TryFold`](#tryfold)
  - [`TryFoldWith`](#tryfoldwith)
  - [`Update`](#update)
  - [`WalkTree`](#walktree)
  - [`WalkTreePostfix`](#walktreepostfix)
  - [`WalkTreePrefix`](#walktreeprefix)
  - [`WhileSome`](#whilesome)
  - [`Zip`](#zip)
  - [`ZipEq`](#zipeq)
- [Traits](#traits)
  - [`ParallelBridge`](#parallelbridge)
  - [`IntoParallelIterator`](#intoparalleliterator)
  - [`IntoParallelRefIterator`](#intoparallelrefiterator)
  - [`IntoParallelRefMutIterator`](#intoparallelrefmutiterator)
  - [`ParallelIterator`](#paralleliterator)
  - [`IndexedParallelIterator`](#indexedparalleliterator)
  - [`FromParallelIterator`](#fromparalleliterator)
  - [`ParallelExtend`](#parallelextend)
  - [`ParallelDrainFull`](#paralleldrainfull)
  - [`ParallelDrainRange`](#paralleldrainrange)
- [Functions](#functions)
  - [`empty`](#empty)
  - [`once`](#once)
  - [`repeat`](#repeat)
  - [`repeat_n`](#repeat_n)
  - [`split`](#split)
  - [`walk_tree`](#walk_tree)
  - [`walk_tree_postfix`](#walk_tree_postfix)
  - [`walk_tree_prefix`](#walk_tree_prefix)
  - [`repeatn`](#repeatn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`plumbing`](#plumbing) | mod | Traits and functions used to implement parallel iteration. |
| [`blocks`](#blocks) | mod |  |
| [`chain`](#chain) | mod |  |
| [`chunks`](#chunks) | mod |  |
| [`cloned`](#cloned) | mod |  |
| [`collect`](#collect) | mod |  |
| [`copied`](#copied) | mod |  |
| [`empty`](#empty) | mod |  |
| [`enumerate`](#enumerate) | mod |  |
| [`extend`](#extend) | mod |  |
| [`filter`](#filter) | mod |  |
| [`filter_map`](#filter_map) | mod |  |
| [`find`](#find) | mod |  |
| [`find_first_last`](#find_first_last) | mod |  |
| [`flat_map`](#flat_map) | mod |  |
| [`flat_map_iter`](#flat_map_iter) | mod |  |
| [`flatten`](#flatten) | mod |  |
| [`flatten_iter`](#flatten_iter) | mod |  |
| [`fold`](#fold) | mod |  |
| [`fold_chunks`](#fold_chunks) | mod |  |
| [`fold_chunks_with`](#fold_chunks_with) | mod |  |
| [`for_each`](#for_each) | mod |  |
| [`from_par_iter`](#from_par_iter) | mod |  |
| [`inspect`](#inspect) | mod |  |
| [`interleave`](#interleave) | mod |  |
| [`interleave_shortest`](#interleave_shortest) | mod |  |
| [`intersperse`](#intersperse) | mod |  |
| [`len`](#len) | mod |  |
| [`map`](#map) | mod |  |
| [`map_with`](#map_with) | mod |  |
| [`multizip`](#multizip) | mod |  |
| [`noop`](#noop) | mod |  |
| [`once`](#once) | mod |  |
| [`panic_fuse`](#panic_fuse) | mod |  |
| [`par_bridge`](#par_bridge) | mod |  |
| [`positions`](#positions) | mod |  |
| [`product`](#product) | mod |  |
| [`reduce`](#reduce) | mod |  |
| [`repeat`](#repeat) | mod |  |
| [`rev`](#rev) | mod |  |
| [`skip`](#skip) | mod |  |
| [`skip_any`](#skip_any) | mod |  |
| [`skip_any_while`](#skip_any_while) | mod |  |
| [`splitter`](#splitter) | mod |  |
| [`step_by`](#step_by) | mod |  |
| [`sum`](#sum) | mod |  |
| [`take`](#take) | mod |  |
| [`take_any`](#take_any) | mod |  |
| [`take_any_while`](#take_any_while) | mod |  |
| [`try_fold`](#try_fold) | mod |  |
| [`try_reduce`](#try_reduce) | mod |  |
| [`try_reduce_with`](#try_reduce_with) | mod |  |
| [`unzip`](#unzip) | mod |  |
| [`update`](#update) | mod |  |
| [`walk_tree`](#walk_tree) | mod |  |
| [`while_some`](#while_some) | mod |  |
| [`zip`](#zip) | mod |  |
| [`zip_eq`](#zip_eq) | mod |  |
| [`private`](#private) | mod | We hide the `Try` trait in a private module, as it's only meant to be a stable clone of the standard library's `Try` trait, as yet unstable. |
| [`ExponentialBlocks`](#exponentialblocks) | struct |  |
| [`UniformBlocks`](#uniformblocks) | struct |  |
| [`Chain`](#chain) | struct |  |
| [`Chunks`](#chunks) | struct |  |
| [`Cloned`](#cloned) | struct |  |
| [`Copied`](#copied) | struct |  |
| [`Empty`](#empty) | struct |  |
| [`Enumerate`](#enumerate) | struct |  |
| [`Filter`](#filter) | struct |  |
| [`FilterMap`](#filtermap) | struct |  |
| [`FlatMap`](#flatmap) | struct |  |
| [`FlatMapIter`](#flatmapiter) | struct |  |
| [`Flatten`](#flatten) | struct |  |
| [`FlattenIter`](#flatteniter) | struct |  |
| [`Fold`](#fold) | struct |  |
| [`FoldWith`](#foldwith) | struct |  |
| [`FoldChunks`](#foldchunks) | struct |  |
| [`FoldChunksWith`](#foldchunkswith) | struct |  |
| [`Inspect`](#inspect) | struct |  |
| [`Interleave`](#interleave) | struct |  |
| [`InterleaveShortest`](#interleaveshortest) | struct |  |
| [`Intersperse`](#intersperse) | struct |  |
| [`MaxLen`](#maxlen) | struct |  |
| [`MinLen`](#minlen) | struct |  |
| [`Map`](#map) | struct |  |
| [`MapInit`](#mapinit) | struct |  |
| [`MapWith`](#mapwith) | struct |  |
| [`MultiZip`](#multizip) | struct |  |
| [`Once`](#once) | struct |  |
| [`PanicFuse`](#panicfuse) | struct |  |
| [`IterBridge`](#iterbridge) | struct |  |
| [`Positions`](#positions) | struct |  |
| [`Repeat`](#repeat) | struct |  |
| [`RepeatN`](#repeatn) | struct |  |
| [`Rev`](#rev) | struct |  |
| [`Skip`](#skip) | struct |  |
| [`SkipAny`](#skipany) | struct |  |
| [`SkipAnyWhile`](#skipanywhile) | struct |  |
| [`Split`](#split) | struct |  |
| [`StepBy`](#stepby) | struct |  |
| [`Take`](#take) | struct |  |
| [`TakeAny`](#takeany) | struct |  |
| [`TakeAnyWhile`](#takeanywhile) | struct |  |
| [`TryFold`](#tryfold) | struct |  |
| [`TryFoldWith`](#tryfoldwith) | struct |  |
| [`Update`](#update) | struct |  |
| [`WalkTree`](#walktree) | struct |  |
| [`WalkTreePostfix`](#walktreepostfix) | struct |  |
| [`WalkTreePrefix`](#walktreeprefix) | struct |  |
| [`WhileSome`](#whilesome) | struct |  |
| [`Zip`](#zip) | struct |  |
| [`ZipEq`](#zipeq) | struct |  |
| [`ParallelBridge`](#parallelbridge) | trait |  |
| [`IntoParallelIterator`](#intoparalleliterator) | trait | `IntoParallelIterator` implements the conversion to a [`ParallelIterator`]. |
| [`IntoParallelRefIterator`](#intoparallelrefiterator) | trait | `IntoParallelRefIterator` implements the conversion to a [`ParallelIterator`], providing shared references to the data. |
| [`IntoParallelRefMutIterator`](#intoparallelrefmutiterator) | trait | `IntoParallelRefMutIterator` implements the conversion to a [`ParallelIterator`], providing mutable references to the data. |
| [`ParallelIterator`](#paralleliterator) | trait | Parallel version of the standard iterator trait. |
| [`IndexedParallelIterator`](#indexedparalleliterator) | trait | An iterator that supports "random access" to its data, meaning that you can split it at arbitrary indices and draw data from those points. |
| [`FromParallelIterator`](#fromparalleliterator) | trait | `FromParallelIterator` implements the creation of a collection from a [`ParallelIterator`]. |
| [`ParallelExtend`](#parallelextend) | trait | `ParallelExtend` extends an existing collection with items from a [`ParallelIterator`]. |
| [`ParallelDrainFull`](#paralleldrainfull) | trait | `ParallelDrainFull` creates a parallel iterator that moves all items from a collection while retaining the original capacity. |
| [`ParallelDrainRange`](#paralleldrainrange) | trait | `ParallelDrainRange` creates a parallel iterator that moves a range of items from a collection while retaining the original capacity. |
| [`empty`](#empty) | fn |  |
| [`once`](#once) | fn |  |
| [`repeat`](#repeat) | fn |  |
| [`repeat_n`](#repeat_n) | fn |  |
| [`split`](#split) | fn |  |
| [`walk_tree`](#walk_tree) | fn |  |
| [`walk_tree_postfix`](#walk_tree_postfix) | fn |  |
| [`walk_tree_prefix`](#walk_tree_prefix) | fn |  |
| [`repeatn`](#repeatn) | fn |  |

## Modules

- [`plumbing`](plumbing/index.md) — Traits and functions used to implement parallel iteration.  These are
- [`blocks`](blocks/index.md)
- [`chain`](chain/index.md)
- [`chunks`](chunks/index.md)
- [`cloned`](cloned/index.md)
- [`collect`](collect/index.md)
- [`copied`](copied/index.md)
- [`empty`](empty/index.md)
- [`enumerate`](enumerate/index.md)
- [`extend`](extend/index.md)
- [`filter`](filter/index.md)
- [`filter_map`](filter_map/index.md)
- [`find`](find/index.md)
- [`find_first_last`](find_first_last/index.md)
- [`flat_map`](flat_map/index.md)
- [`flat_map_iter`](flat_map_iter/index.md)
- [`flatten`](flatten/index.md)
- [`flatten_iter`](flatten_iter/index.md)
- [`fold`](fold/index.md)
- [`fold_chunks`](fold_chunks/index.md)
- [`fold_chunks_with`](fold_chunks_with/index.md)
- [`for_each`](for_each/index.md)
- [`from_par_iter`](from_par_iter/index.md)
- [`inspect`](inspect/index.md)
- [`interleave`](interleave/index.md)
- [`interleave_shortest`](interleave_shortest/index.md)
- [`intersperse`](intersperse/index.md)
- [`len`](len/index.md)
- [`map`](map/index.md)
- [`map_with`](map_with/index.md)
- [`multizip`](multizip/index.md)
- [`noop`](noop/index.md)
- [`once`](once/index.md)
- [`panic_fuse`](panic_fuse/index.md)
- [`par_bridge`](par_bridge/index.md)
- [`positions`](positions/index.md)
- [`product`](product/index.md)
- [`reduce`](reduce/index.md)
- [`repeat`](repeat/index.md)
- [`rev`](rev/index.md)
- [`skip`](skip/index.md)
- [`skip_any`](skip_any/index.md)
- [`skip_any_while`](skip_any_while/index.md)
- [`splitter`](splitter/index.md)
- [`step_by`](step_by/index.md)
- [`sum`](sum/index.md)
- [`take`](take/index.md)
- [`take_any`](take_any/index.md)
- [`take_any_while`](take_any_while/index.md)
- [`try_fold`](try_fold/index.md)
- [`try_reduce`](try_reduce/index.md)
- [`try_reduce_with`](try_reduce_with/index.md)
- [`unzip`](unzip/index.md)
- [`update`](update/index.md)
- [`walk_tree`](walk_tree/index.md)
- [`while_some`](while_some/index.md)
- [`zip`](zip/index.md)
- [`zip_eq`](zip_eq/index.md)
- [`private`](private/index.md) — We hide the `Try` trait in a private module, as it's only meant to be a

## Structs

### `ExponentialBlocks<I>`

```rust
struct ExponentialBlocks<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/blocks.rs:59-61`](../../../.source_1765210505/rayon-1.11.0/src/iter/blocks.rs#L59-L61)*

`ExponentialBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of increasing sizes (exponentially).

This struct is created by the `by_exponential_blocks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="exponentialblocks-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for ExponentialBlocks<I>`

- <span id="exponentialblocks-clone"></span>`fn clone(&self) -> ExponentialBlocks<I>` — [`ExponentialBlocks`](blocks/index.md)

##### `impl<I: fmt::Debug> Debug for ExponentialBlocks<I>`

- <span id="exponentialblocks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for ExponentialBlocks<I>`

##### `impl<T> IntoParallelIterator for ExponentialBlocks<I>`

- <span id="exponentialblocks-type-iter"></span>`type Iter = T`

- <span id="exponentialblocks-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="exponentialblocks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for ExponentialBlocks<I>`

- <span id="exponentialblocks-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="exponentialblocks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for ExponentialBlocks<I>`

- <span id="exponentialblocks-const-align"></span>`const ALIGN: usize`

- <span id="exponentialblocks-type-init"></span>`type Init = T`

- <span id="exponentialblocks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="exponentialblocks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="exponentialblocks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="exponentialblocks-drop"></span>`unsafe fn drop(ptr: usize)`

### `UniformBlocks<I>`

```rust
struct UniformBlocks<I> {
    base: I,
    block_size: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/blocks.rs:101-104`](../../../.source_1765210505/rayon-1.11.0/src/iter/blocks.rs#L101-L104)*

`UniformBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of constant sizes.

This struct is created by the `by_uniform_blocks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="uniformblocks-new"></span>`fn new(base: I, block_size: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for UniformBlocks<I>`

- <span id="uniformblocks-clone"></span>`fn clone(&self) -> UniformBlocks<I>` — [`UniformBlocks`](blocks/index.md)

##### `impl<I: fmt::Debug> Debug for UniformBlocks<I>`

- <span id="uniformblocks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for UniformBlocks<I>`

##### `impl<T> IntoParallelIterator for UniformBlocks<I>`

- <span id="uniformblocks-type-iter"></span>`type Iter = T`

- <span id="uniformblocks-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="uniformblocks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for UniformBlocks<I>`

- <span id="uniformblocks-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="uniformblocks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for UniformBlocks<I>`

- <span id="uniformblocks-const-align"></span>`const ALIGN: usize`

- <span id="uniformblocks-type-init"></span>`type Init = T`

- <span id="uniformblocks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="uniformblocks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="uniformblocks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="uniformblocks-drop"></span>`unsafe fn drop(ptr: usize)`

### `Chain<A, B>`

```rust
struct Chain<A, B> {
    a: A,
    b: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/chain.rs:12-15`](../../../.source_1765210505/rayon-1.11.0/src/iter/chain.rs#L12-L15)*

`Chain` is an iterator that joins `b` after `a` in one continuous iterator.
This struct is created by the `chain()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="chain-new"></span>`fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: clone::Clone, B: clone::Clone> Clone for Chain<A, B>`

- <span id="chain-clone"></span>`fn clone(&self) -> Chain<A, B>` — [`Chain`](chain/index.md)

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Chain<A, B>`

- <span id="chain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> IndexedParallelIterator for Chain<A, B>`

- <span id="chain-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="chain-len"></span>`fn len(&self) -> usize`

- <span id="chain-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Chain<A, B>`

##### `impl<T> IntoParallelIterator for Chain<A, B>`

- <span id="chain-type-iter"></span>`type Iter = T`

- <span id="chain-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chain-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for Chain<A, B>`

- <span id="chain-type-item"></span>`type Item = <A as ParallelIterator>::Item`

- <span id="chain-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="chain-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Chain<A, B>`

- <span id="chain-const-align"></span>`const ALIGN: usize`

- <span id="chain-type-init"></span>`type Init = T`

- <span id="chain-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chain-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chain-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chain-drop"></span>`unsafe fn drop(ptr: usize)`

### `Chunks<I>`

```rust
struct Chunks<I> {
    size: usize,
    i: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/chunks.rs:11-14`](../../../.source_1765210505/rayon-1.11.0/src/iter/chunks.rs#L11-L14)*

`Chunks` is an iterator that groups elements of an underlying iterator.

This struct is created by the `chunks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="chunks-new"></span>`fn new(i: I, size: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Chunks<I>`

- <span id="chunks-clone"></span>`fn clone(&self) -> Chunks<I>` — [`Chunks`](chunks/index.md)

##### `impl<I: fmt::Debug> Debug for Chunks<I>`

- <span id="chunks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Chunks<I>`

- <span id="chunks-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="chunks-len"></span>`fn len(&self) -> usize`

- <span id="chunks-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Chunks<I>`

##### `impl<T> IntoParallelIterator for Chunks<I>`

- <span id="chunks-type-iter"></span>`type Iter = T`

- <span id="chunks-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Chunks<I>`

- <span id="chunks-type-item"></span>`type Item = Vec<<I as ParallelIterator>::Item>`

- <span id="chunks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="chunks-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Chunks<I>`

- <span id="chunks-const-align"></span>`const ALIGN: usize`

- <span id="chunks-type-init"></span>`type Init = T`

- <span id="chunks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunks-drop"></span>`unsafe fn drop(ptr: usize)`

### `Cloned<I>`

```rust
struct Cloned<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/cloned.rs:13-15`](../../../.source_1765210505/rayon-1.11.0/src/iter/cloned.rs#L13-L15)*

`Cloned` is an iterator that clones the elements of an underlying iterator.

This struct is created by the `cloned()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="cloned-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Cloned<I>`

- <span id="cloned-clone"></span>`fn clone(&self) -> Cloned<I>` — [`Cloned`](cloned/index.md)

##### `impl<I: fmt::Debug> Debug for Cloned<I>`

- <span id="cloned-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T, I> IndexedParallelIterator for Cloned<I>`

- <span id="cloned-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="cloned-len"></span>`fn len(&self) -> usize`

- <span id="cloned-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Cloned<I>`

##### `impl<T> IntoParallelIterator for Cloned<I>`

- <span id="cloned-type-iter"></span>`type Iter = T`

- <span id="cloned-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="cloned-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'a, T, I> ParallelIterator for Cloned<I>`

- <span id="cloned-type-item"></span>`type Item = T`

- <span id="cloned-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="cloned-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Cloned<I>`

- <span id="cloned-const-align"></span>`const ALIGN: usize`

- <span id="cloned-type-init"></span>`type Init = T`

- <span id="cloned-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cloned-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cloned-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cloned-drop"></span>`unsafe fn drop(ptr: usize)`

### `Copied<I>`

```rust
struct Copied<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/copied.rs:13-15`](../../../.source_1765210505/rayon-1.11.0/src/iter/copied.rs#L13-L15)*

`Copied` is an iterator that copies the elements of an underlying iterator.

This struct is created by the `copied()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="copied-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Copied<I>`

- <span id="copied-clone"></span>`fn clone(&self) -> Copied<I>` — [`Copied`](copied/index.md)

##### `impl<I: fmt::Debug> Debug for Copied<I>`

- <span id="copied-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T, I> IndexedParallelIterator for Copied<I>`

- <span id="copied-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="copied-len"></span>`fn len(&self) -> usize`

- <span id="copied-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Copied<I>`

##### `impl<T> IntoParallelIterator for Copied<I>`

- <span id="copied-type-iter"></span>`type Iter = T`

- <span id="copied-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="copied-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'a, T, I> ParallelIterator for Copied<I>`

- <span id="copied-type-item"></span>`type Item = T`

- <span id="copied-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="copied-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Copied<I>`

- <span id="copied-const-align"></span>`const ALIGN: usize`

- <span id="copied-type-init"></span>`type Init = T`

- <span id="copied-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="copied-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="copied-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="copied-drop"></span>`unsafe fn drop(ptr: usize)`

### `Empty<T>`

```rust
struct Empty<T> {
    marker: std::marker::PhantomData<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/empty.rs:33-35`](../../../.source_1765210505/rayon-1.11.0/src/iter/empty.rs#L33-L35)*

Iterator adaptor for [the `empty()` function].


#### Trait Implementations

##### `impl<T> Clone for Empty<T>`

- <span id="empty-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: Send> Debug for Empty<T>`

- <span id="empty-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for Empty<T>`

- <span id="empty-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="empty-len"></span>`fn len(&self) -> usize`

- <span id="empty-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Empty<T>`

##### `impl<T> IntoParallelIterator for Empty<T>`

- <span id="empty-type-iter"></span>`type Iter = T`

- <span id="empty-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="empty-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Empty<T>`

- <span id="empty-type-item"></span>`type Item = T`

- <span id="empty-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="empty-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Empty<T>`

- <span id="empty-const-align"></span>`const ALIGN: usize`

- <span id="empty-type-init"></span>`type Init = T`

- <span id="empty-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="empty-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="empty-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="empty-drop"></span>`unsafe fn drop(ptr: usize)`

### `Enumerate<I>`

```rust
struct Enumerate<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/enumerate.rs:12-14`](../../../.source_1765210505/rayon-1.11.0/src/iter/enumerate.rs#L12-L14)*

`Enumerate` is an iterator that returns the current count along with the element.
This struct is created by the `enumerate()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="enumerate-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Enumerate<I>`

- <span id="enumerate-clone"></span>`fn clone(&self) -> Enumerate<I>` — [`Enumerate`](enumerate/index.md)

##### `impl<I: fmt::Debug> Debug for Enumerate<I>`

- <span id="enumerate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Enumerate<I>`

- <span id="enumerate-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="enumerate-len"></span>`fn len(&self) -> usize`

- <span id="enumerate-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Enumerate<I>`

##### `impl<T> IntoParallelIterator for Enumerate<I>`

- <span id="enumerate-type-iter"></span>`type Iter = T`

- <span id="enumerate-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="enumerate-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Enumerate<I>`

- <span id="enumerate-type-item"></span>`type Item = (usize, <I as ParallelIterator>::Item)`

- <span id="enumerate-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="enumerate-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Enumerate<I>`

- <span id="enumerate-const-align"></span>`const ALIGN: usize`

- <span id="enumerate-type-init"></span>`type Init = T`

- <span id="enumerate-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="enumerate-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="enumerate-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="enumerate-drop"></span>`unsafe fn drop(ptr: usize)`

### `Filter<I, P>`

```rust
struct Filter<I, P> {
    base: I,
    filter_op: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter.rs:12-15`](../../../.source_1765210505/rayon-1.11.0/src/iter/filter.rs#L12-L15)*

`Filter` takes a predicate `filter_op` and filters out elements that match.
This struct is created by the `filter()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="filter-new"></span>`fn new(base: I, filter_op: P) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for Filter<I, P>`

- <span id="filter-clone"></span>`fn clone(&self) -> Filter<I, P>` — [`Filter`](filter/index.md)

##### `impl<I: Debug, P> Debug for Filter<I, P>`

- <span id="filter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Filter<I, P>`

##### `impl<T> IntoParallelIterator for Filter<I, P>`

- <span id="filter-type-iter"></span>`type Iter = T`

- <span id="filter-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="filter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for Filter<I, P>`

- <span id="filter-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="filter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Filter<I, P>`

- <span id="filter-const-align"></span>`const ALIGN: usize`

- <span id="filter-type-init"></span>`type Init = T`

- <span id="filter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filter-drop"></span>`unsafe fn drop(ptr: usize)`

### `FilterMap<I, P>`

```rust
struct FilterMap<I, P> {
    base: I,
    filter_op: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter_map.rs:12-15`](../../../.source_1765210505/rayon-1.11.0/src/iter/filter_map.rs#L12-L15)*

`FilterMap` creates an iterator that uses `filter_op` to both filter and map elements.
This struct is created by the `filter_map()` method on [`ParallelIterator`](#paralleliterator).


#### Implementations

- <span id="filtermap-new"></span>`fn new(base: I, filter_op: P) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for FilterMap<I, P>`

- <span id="filtermap-clone"></span>`fn clone(&self) -> FilterMap<I, P>` — [`FilterMap`](filter_map/index.md)

##### `impl<I: Debug, P> Debug for FilterMap<I, P>`

- <span id="filtermap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FilterMap<I, P>`

##### `impl<T> IntoParallelIterator for FilterMap<I, P>`

- <span id="filtermap-type-iter"></span>`type Iter = T`

- <span id="filtermap-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="filtermap-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P, R> ParallelIterator for FilterMap<I, P>`

- <span id="filtermap-type-item"></span>`type Item = R`

- <span id="filtermap-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for FilterMap<I, P>`

- <span id="filtermap-const-align"></span>`const ALIGN: usize`

- <span id="filtermap-type-init"></span>`type Init = T`

- <span id="filtermap-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filtermap-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filtermap-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filtermap-drop"></span>`unsafe fn drop(ptr: usize)`

### `FlatMap<I, F>`

```rust
struct FlatMap<I, F> {
    base: I,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map.rs:12-15`](../../../.source_1765210505/rayon-1.11.0/src/iter/flat_map.rs#L12-L15)*

`FlatMap` maps each element to a parallel iterator, then flattens these iterators together.
This struct is created by the `flat_map()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="flatmap-new"></span>`fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for FlatMap<I, F>`

- <span id="flatmap-clone"></span>`fn clone(&self) -> FlatMap<I, F>` — [`FlatMap`](flat_map/index.md)

##### `impl<I: Debug, F> Debug for FlatMap<I, F>`

- <span id="flatmap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FlatMap<I, F>`

##### `impl<T> IntoParallelIterator for FlatMap<I, F>`

- <span id="flatmap-type-iter"></span>`type Iter = T`

- <span id="flatmap-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatmap-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F, PI> ParallelIterator for FlatMap<I, F>`

- <span id="flatmap-type-item"></span>`type Item = <PI as IntoParallelIterator>::Item`

- <span id="flatmap-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for FlatMap<I, F>`

- <span id="flatmap-const-align"></span>`const ALIGN: usize`

- <span id="flatmap-type-init"></span>`type Init = T`

- <span id="flatmap-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmap-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmap-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmap-drop"></span>`unsafe fn drop(ptr: usize)`

### `FlatMapIter<I, F>`

```rust
struct FlatMapIter<I, F> {
    base: I,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map_iter.rs:12-15`](../../../.source_1765210505/rayon-1.11.0/src/iter/flat_map_iter.rs#L12-L15)*

`FlatMapIter` maps each element to a serial iterator, then flattens these iterators together.
This struct is created by the `flat_map_iter()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="flatmapiter-new"></span>`fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for FlatMapIter<I, F>`

- <span id="flatmapiter-clone"></span>`fn clone(&self) -> FlatMapIter<I, F>` — [`FlatMapIter`](flat_map_iter/index.md)

##### `impl<I: Debug, F> Debug for FlatMapIter<I, F>`

- <span id="flatmapiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FlatMapIter<I, F>`

##### `impl<T> IntoParallelIterator for FlatMapIter<I, F>`

- <span id="flatmapiter-type-iter"></span>`type Iter = T`

- <span id="flatmapiter-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatmapiter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F, SI> ParallelIterator for FlatMapIter<I, F>`

- <span id="flatmapiter-type-item"></span>`type Item = <SI as IntoIterator>::Item`

- <span id="flatmapiter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for FlatMapIter<I, F>`

- <span id="flatmapiter-const-align"></span>`const ALIGN: usize`

- <span id="flatmapiter-type-init"></span>`type Init = T`

- <span id="flatmapiter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapiter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapiter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapiter-drop"></span>`unsafe fn drop(ptr: usize)`

### `Flatten<I>`

```rust
struct Flatten<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten.rs:10-12`](../../../.source_1765210505/rayon-1.11.0/src/iter/flatten.rs#L10-L12)*

`Flatten` turns each element to a parallel iterator, then flattens these iterators
together. This struct is created by the `flatten()` method on [`ParallelIterator`](#paralleliterator).


#### Implementations

- <span id="flatten-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Flatten<I>`

- <span id="flatten-clone"></span>`fn clone(&self) -> Flatten<I>` — [`Flatten`](flatten/index.md)

##### `impl<I: fmt::Debug> Debug for Flatten<I>`

- <span id="flatten-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Flatten<I>`

##### `impl<T> IntoParallelIterator for Flatten<I>`

- <span id="flatten-type-iter"></span>`type Iter = T`

- <span id="flatten-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatten-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Flatten<I>`

- <span id="flatten-type-item"></span>`type Item = <<I as ParallelIterator>::Item as IntoParallelIterator>::Item`

- <span id="flatten-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Flatten<I>`

- <span id="flatten-const-align"></span>`const ALIGN: usize`

- <span id="flatten-type-init"></span>`type Init = T`

- <span id="flatten-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatten-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatten-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatten-drop"></span>`unsafe fn drop(ptr: usize)`

### `FlattenIter<I>`

```rust
struct FlattenIter<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten_iter.rs:10-12`](../../../.source_1765210505/rayon-1.11.0/src/iter/flatten_iter.rs#L10-L12)*

`FlattenIter` turns each element to a serial iterator, then flattens these iterators
together. This struct is created by the `flatten_iter()` method on [`ParallelIterator`](#paralleliterator).


#### Implementations

- <span id="flatteniter-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for FlattenIter<I>`

- <span id="flatteniter-clone"></span>`fn clone(&self) -> FlattenIter<I>` — [`FlattenIter`](flatten_iter/index.md)

##### `impl<I: fmt::Debug> Debug for FlattenIter<I>`

- <span id="flatteniter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FlattenIter<I>`

##### `impl<T> IntoParallelIterator for FlattenIter<I>`

- <span id="flatteniter-type-iter"></span>`type Iter = T`

- <span id="flatteniter-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatteniter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for FlattenIter<I>`

- <span id="flatteniter-type-item"></span>`type Item = <<I as ParallelIterator>::Item as IntoIterator>::Item`

- <span id="flatteniter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for FlattenIter<I>`

- <span id="flatteniter-const-align"></span>`const ALIGN: usize`

- <span id="flatteniter-type-init"></span>`type Init = T`

- <span id="flatteniter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatteniter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatteniter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatteniter-drop"></span>`unsafe fn drop(ptr: usize)`

### `Fold<I, ID, F>`

```rust
struct Fold<I, ID, F> {
    base: I,
    identity: ID,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold.rs:22-26`](../../../.source_1765210505/rayon-1.11.0/src/iter/fold.rs#L22-L26)*

`Fold` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="fold-new"></span>`fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for Fold<I, ID, F>`

- <span id="fold-clone"></span>`fn clone(&self) -> Fold<I, ID, F>` — [`Fold`](fold/index.md)

##### `impl<I: Debug, ID, F> Debug for Fold<I, ID, F>`

- <span id="fold-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Fold<I, ID, F>`

##### `impl<T> IntoParallelIterator for Fold<I, ID, F>`

- <span id="fold-type-iter"></span>`type Iter = T`

- <span id="fold-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="fold-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, ID, F> ParallelIterator for Fold<I, ID, F>`

- <span id="fold-type-item"></span>`type Item = U`

- <span id="fold-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Fold<I, ID, F>`

- <span id="fold-const-align"></span>`const ALIGN: usize`

- <span id="fold-type-init"></span>`type Init = T`

- <span id="fold-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fold-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fold-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fold-drop"></span>`unsafe fn drop(ptr: usize)`

### `FoldWith<I, U, F>`

```rust
struct FoldWith<I, U, F> {
    base: I,
    item: U,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold.rs:191-195`](../../../.source_1765210505/rayon-1.11.0/src/iter/fold.rs#L191-L195)*

`FoldWith` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold_with()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="foldwith-new"></span>`fn new(base: I, item: U, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, U: clone::Clone, F: clone::Clone> Clone for FoldWith<I, U, F>`

- <span id="foldwith-clone"></span>`fn clone(&self) -> FoldWith<I, U, F>` — [`FoldWith`](fold/index.md)

##### `impl<I: Debug, U: Debug, F> Debug for FoldWith<I, U, F>`

- <span id="foldwith-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FoldWith<I, U, F>`

##### `impl<T> IntoParallelIterator for FoldWith<I, U, F>`

- <span id="foldwith-type-iter"></span>`type Iter = T`

- <span id="foldwith-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldwith-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, F> ParallelIterator for FoldWith<I, U, F>`

- <span id="foldwith-type-item"></span>`type Item = U`

- <span id="foldwith-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for FoldWith<I, U, F>`

- <span id="foldwith-const-align"></span>`const ALIGN: usize`

- <span id="foldwith-type-init"></span>`type Init = T`

- <span id="foldwith-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldwith-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldwith-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldwith-drop"></span>`unsafe fn drop(ptr: usize)`

### `FoldChunks<I, ID, F>`

```rust
struct FoldChunks<I, ID, F> {
    base: I,
    chunk_size: usize,
    fold_op: F,
    identity: ID,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold_chunks.rs:15-20`](../../../.source_1765210505/rayon-1.11.0/src/iter/fold_chunks.rs#L15-L20)*

`FoldChunks` is an iterator that groups elements of an underlying iterator and applies a
function over them, producing a single value for each group.

This struct is created by the `fold_chunks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="foldchunks-new"></span>`fn new(base: I, chunk_size: usize, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for FoldChunks<I, ID, F>`

- <span id="foldchunks-clone"></span>`fn clone(&self) -> FoldChunks<I, ID, F>` — [`FoldChunks`](fold_chunks/index.md)

##### `impl<I: Debug, ID, F> Debug for FoldChunks<I, ID, F>`

- <span id="foldchunks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, ID, U, F> IndexedParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-len"></span>`fn len(&self) -> usize`

- <span id="foldchunks-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="foldchunks-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for FoldChunks<I, ID, F>`

##### `impl<T> IntoParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-type-iter"></span>`type Iter = T`

- <span id="foldchunks-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldchunks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, ID, U, F> ParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-type-item"></span>`type Item = U`

- <span id="foldchunks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="foldchunks-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for FoldChunks<I, ID, F>`

- <span id="foldchunks-const-align"></span>`const ALIGN: usize`

- <span id="foldchunks-type-init"></span>`type Init = T`

- <span id="foldchunks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldchunks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldchunks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldchunks-drop"></span>`unsafe fn drop(ptr: usize)`

### `FoldChunksWith<I, U, F>`

```rust
struct FoldChunksWith<I, U, F> {
    base: I,
    chunk_size: usize,
    item: U,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold_chunks_with.rs:15-20`](../../../.source_1765210505/rayon-1.11.0/src/iter/fold_chunks_with.rs#L15-L20)*

`FoldChunksWith` is an iterator that groups elements of an underlying iterator and applies a
function over them, producing a single value for each group.

This struct is created by the `fold_chunks_with()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="foldchunkswith-new"></span>`fn new(base: I, chunk_size: usize, item: U, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, U: clone::Clone, F: clone::Clone> Clone for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-clone"></span>`fn clone(&self) -> FoldChunksWith<I, U, F>` — [`FoldChunksWith`](fold_chunks_with/index.md)

##### `impl<I: Debug, U: Debug, F> Debug for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, U, F> IndexedParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-len"></span>`fn len(&self) -> usize`

- <span id="foldchunkswith-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="foldchunkswith-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for FoldChunksWith<I, U, F>`

##### `impl<T> IntoParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-type-iter"></span>`type Iter = T`

- <span id="foldchunkswith-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldchunkswith-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, U, F> ParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-type-item"></span>`type Item = U`

- <span id="foldchunkswith-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="foldchunkswith-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-const-align"></span>`const ALIGN: usize`

- <span id="foldchunkswith-type-init"></span>`type Init = T`

- <span id="foldchunkswith-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldchunkswith-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldchunkswith-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldchunkswith-drop"></span>`unsafe fn drop(ptr: usize)`

### `Inspect<I, F>`

```rust
struct Inspect<I, F> {
    base: I,
    inspect_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/inspect.rs:15-18`](../../../.source_1765210505/rayon-1.11.0/src/iter/inspect.rs#L15-L18)*

`Inspect` is an iterator that calls a function with a reference to each
element before yielding it.

This struct is created by the `inspect()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="inspect-new"></span>`fn new(base: I, inspect_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Inspect<I, F>`

- <span id="inspect-clone"></span>`fn clone(&self) -> Inspect<I, F>` — [`Inspect`](inspect/index.md)

##### `impl<I: Debug, F> Debug for Inspect<I, F>`

- <span id="inspect-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> IndexedParallelIterator for Inspect<I, F>`

- <span id="inspect-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="inspect-len"></span>`fn len(&self) -> usize`

- <span id="inspect-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Inspect<I, F>`

##### `impl<T> IntoParallelIterator for Inspect<I, F>`

- <span id="inspect-type-iter"></span>`type Iter = T`

- <span id="inspect-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="inspect-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for Inspect<I, F>`

- <span id="inspect-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="inspect-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="inspect-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Inspect<I, F>`

- <span id="inspect-const-align"></span>`const ALIGN: usize`

- <span id="inspect-type-init"></span>`type Init = T`

- <span id="inspect-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inspect-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inspect-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inspect-drop"></span>`unsafe fn drop(ptr: usize)`

### `Interleave<I, J>`

```rust
struct Interleave<I, J> {
    i: I,
    j: J,
}
```

*Defined in [`rayon-1.11.0/src/iter/interleave.rs:12-15`](../../../.source_1765210505/rayon-1.11.0/src/iter/interleave.rs#L12-L15)*

`Interleave` is an iterator that interleaves elements of iterators
`i` and `j` in one continuous iterator. This struct is created by
the `interleave()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="interleave-new"></span>`fn new(i: I, j: J) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, J: clone::Clone> Clone for Interleave<I, J>`

- <span id="interleave-clone"></span>`fn clone(&self) -> Interleave<I, J>` — [`Interleave`](interleave/index.md)

##### `impl<I: fmt::Debug, J: fmt::Debug> Debug for Interleave<I, J>`

- <span id="interleave-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, J> IndexedParallelIterator for Interleave<I, J>`

- <span id="interleave-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="interleave-len"></span>`fn len(&self) -> usize`

- <span id="interleave-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Interleave<I, J>`

##### `impl<T> IntoParallelIterator for Interleave<I, J>`

- <span id="interleave-type-iter"></span>`type Iter = T`

- <span id="interleave-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="interleave-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, J> ParallelIterator for Interleave<I, J>`

- <span id="interleave-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="interleave-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="interleave-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Interleave<I, J>`

- <span id="interleave-const-align"></span>`const ALIGN: usize`

- <span id="interleave-type-init"></span>`type Init = T`

- <span id="interleave-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interleave-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interleave-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interleave-drop"></span>`unsafe fn drop(ptr: usize)`

### `InterleaveShortest<I, J>`

```rust
struct InterleaveShortest<I, J> {
    interleave: Interleave<Take<I>, Take<J>>,
}
```

*Defined in [`rayon-1.11.0/src/iter/interleave_shortest.rs:14-16`](../../../.source_1765210505/rayon-1.11.0/src/iter/interleave_shortest.rs#L14-L16)*

`InterleaveShortest` is an iterator that works similarly to
`Interleave`, but this version stops returning elements once one
of the iterators run out.

This struct is created by the `interleave_shortest()` method on
[`IndexedParallelIterator`](#indexedparalleliterator).


#### Implementations

- <span id="interleaveshortest-new"></span>`fn new(i: I, j: J) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, J: clone::Clone> Clone for InterleaveShortest<I, J>`

- <span id="interleaveshortest-clone"></span>`fn clone(&self) -> InterleaveShortest<I, J>` — [`InterleaveShortest`](interleave_shortest/index.md)

##### `impl<I: fmt::Debug, J: fmt::Debug> Debug for InterleaveShortest<I, J>`

- <span id="interleaveshortest-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, J> IndexedParallelIterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="interleaveshortest-len"></span>`fn len(&self) -> usize`

- <span id="interleaveshortest-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for InterleaveShortest<I, J>`

##### `impl<T> IntoParallelIterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-type-iter"></span>`type Iter = T`

- <span id="interleaveshortest-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="interleaveshortest-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, J> ParallelIterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="interleaveshortest-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="interleaveshortest-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for InterleaveShortest<I, J>`

- <span id="interleaveshortest-const-align"></span>`const ALIGN: usize`

- <span id="interleaveshortest-type-init"></span>`type Init = T`

- <span id="interleaveshortest-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interleaveshortest-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interleaveshortest-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interleaveshortest-drop"></span>`unsafe fn drop(ptr: usize)`

### `Intersperse<I>`

```rust
struct Intersperse<I>
where
    I: ParallelIterator {
    base: I,
    item: <I as >::Item,
}
```

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:13-19`](../../../.source_1765210505/rayon-1.11.0/src/iter/intersperse.rs#L13-L19)*

`Intersperse` is an iterator that inserts a particular item between each
item of the adapted iterator.  This struct is created by the
`intersperse()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="intersperse-new"></span>`fn new(base: I, item: <I as >::Item) -> Self` — [`ParallelIterator`](#paralleliterator)

#### Trait Implementations

##### `impl<I> Clone for Intersperse<I>`

- <span id="intersperse-clone"></span>`fn clone(&self) -> Intersperse<I>` — [`Intersperse`](intersperse/index.md)

##### `impl<I> Debug for Intersperse<I>`

- <span id="intersperse-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Intersperse<I>`

- <span id="intersperse-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="intersperse-len"></span>`fn len(&self) -> usize`

- <span id="intersperse-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Intersperse<I>`

##### `impl<T> IntoParallelIterator for Intersperse<I>`

- <span id="intersperse-type-iter"></span>`type Iter = T`

- <span id="intersperse-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intersperse-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Intersperse<I>`

- <span id="intersperse-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="intersperse-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="intersperse-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Intersperse<I>`

- <span id="intersperse-const-align"></span>`const ALIGN: usize`

- <span id="intersperse-type-init"></span>`type Init = T`

- <span id="intersperse-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intersperse-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intersperse-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intersperse-drop"></span>`unsafe fn drop(ptr: usize)`

### `MaxLen<I>`

```rust
struct MaxLen<I> {
    base: I,
    max: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/len.rs:140-143`](../../../.source_1765210505/rayon-1.11.0/src/iter/len.rs#L140-L143)*

`MaxLen` is an iterator that imposes a maximum length on iterator splits.
This struct is created by the `with_max_len()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="maxlen-new"></span>`fn new(base: I, max: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for MaxLen<I>`

- <span id="maxlen-clone"></span>`fn clone(&self) -> MaxLen<I>` — [`MaxLen`](len/index.md)

##### `impl<I: fmt::Debug> Debug for MaxLen<I>`

- <span id="maxlen-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for MaxLen<I>`

- <span id="maxlen-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="maxlen-len"></span>`fn len(&self) -> usize`

- <span id="maxlen-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for MaxLen<I>`

##### `impl<T> IntoParallelIterator for MaxLen<I>`

- <span id="maxlen-type-iter"></span>`type Iter = T`

- <span id="maxlen-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="maxlen-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for MaxLen<I>`

- <span id="maxlen-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="maxlen-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="maxlen-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MaxLen<I>`

- <span id="maxlen-const-align"></span>`const ALIGN: usize`

- <span id="maxlen-type-init"></span>`type Init = T`

- <span id="maxlen-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="maxlen-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="maxlen-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="maxlen-drop"></span>`unsafe fn drop(ptr: usize)`

### `MinLen<I>`

```rust
struct MinLen<I> {
    base: I,
    min: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/len.rs:10-13`](../../../.source_1765210505/rayon-1.11.0/src/iter/len.rs#L10-L13)*

`MinLen` is an iterator that imposes a minimum length on iterator splits.
This struct is created by the `with_min_len()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="minlen-new"></span>`fn new(base: I, min: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for MinLen<I>`

- <span id="minlen-clone"></span>`fn clone(&self) -> MinLen<I>` — [`MinLen`](len/index.md)

##### `impl<I: fmt::Debug> Debug for MinLen<I>`

- <span id="minlen-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for MinLen<I>`

- <span id="minlen-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="minlen-len"></span>`fn len(&self) -> usize`

- <span id="minlen-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for MinLen<I>`

##### `impl<T> IntoParallelIterator for MinLen<I>`

- <span id="minlen-type-iter"></span>`type Iter = T`

- <span id="minlen-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="minlen-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for MinLen<I>`

- <span id="minlen-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="minlen-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="minlen-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MinLen<I>`

- <span id="minlen-const-align"></span>`const ALIGN: usize`

- <span id="minlen-type-init"></span>`type Init = T`

- <span id="minlen-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="minlen-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="minlen-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="minlen-drop"></span>`unsafe fn drop(ptr: usize)`

### `Map<I, F>`

```rust
struct Map<I, F> {
    base: I,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map.rs:14-17`](../../../.source_1765210505/rayon-1.11.0/src/iter/map.rs#L14-L17)*

`Map` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="map-new"></span>`fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Map<I, F>`

- <span id="map-clone"></span>`fn clone(&self) -> Map<I, F>` — [`Map`](map/index.md)

##### `impl<I: Debug, F> Debug for Map<I, F>`

- <span id="map-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F, R> IndexedParallelIterator for Map<I, F>`

- <span id="map-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="map-len"></span>`fn len(&self) -> usize`

- <span id="map-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Map<I, F>`

##### `impl<T> IntoParallelIterator for Map<I, F>`

- <span id="map-type-iter"></span>`type Iter = T`

- <span id="map-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="map-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F, R> ParallelIterator for Map<I, F>`

- <span id="map-type-item"></span>`type Item = <F as FnOnce>::Output`

- <span id="map-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="map-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Map<I, F>`

- <span id="map-const-align"></span>`const ALIGN: usize`

- <span id="map-type-init"></span>`type Init = T`

- <span id="map-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="map-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="map-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="map-drop"></span>`unsafe fn drop(ptr: usize)`

### `MapInit<I, INIT, F>`

```rust
struct MapInit<I, INIT, F> {
    base: I,
    init: INIT,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:341-345`](../../../.source_1765210505/rayon-1.11.0/src/iter/map_with.rs#L341-L345)*

`MapInit` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_init()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="mapinit-new"></span>`fn new(base: I, init: INIT, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, INIT: clone::Clone, F: clone::Clone> Clone for MapInit<I, INIT, F>`

- <span id="mapinit-clone"></span>`fn clone(&self) -> MapInit<I, INIT, F>` — [`MapInit`](map_with/index.md)

##### `impl<I: Debug, INIT, F> Debug for MapInit<I, INIT, F>`

- <span id="mapinit-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, INIT, T, F, R> IndexedParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="mapinit-len"></span>`fn len(&self) -> usize`

- <span id="mapinit-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for MapInit<I, INIT, F>`

##### `impl<T> IntoParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-type-iter"></span>`type Iter = T`

- <span id="mapinit-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="mapinit-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, INIT, T, F, R> ParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-type-item"></span>`type Item = R`

- <span id="mapinit-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="mapinit-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MapInit<I, INIT, F>`

- <span id="mapinit-const-align"></span>`const ALIGN: usize`

- <span id="mapinit-type-init"></span>`type Init = T`

- <span id="mapinit-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapinit-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapinit-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapinit-drop"></span>`unsafe fn drop(ptr: usize)`

### `MapWith<I, T, F>`

```rust
struct MapWith<I, T, F> {
    base: I,
    item: T,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:13-17`](../../../.source_1765210505/rayon-1.11.0/src/iter/map_with.rs#L13-L17)*

`MapWith` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_with()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="mapwith-new"></span>`fn new(base: I, item: T, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, T: clone::Clone, F: clone::Clone> Clone for MapWith<I, T, F>`

- <span id="mapwith-clone"></span>`fn clone(&self) -> MapWith<I, T, F>` — [`MapWith`](map_with/index.md)

##### `impl<I: Debug, T: Debug, F> Debug for MapWith<I, T, F>`

- <span id="mapwith-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, T, F, R> IndexedParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="mapwith-len"></span>`fn len(&self) -> usize`

- <span id="mapwith-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for MapWith<I, T, F>`

##### `impl<T> IntoParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-type-iter"></span>`type Iter = T`

- <span id="mapwith-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="mapwith-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, T, F, R> ParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-type-item"></span>`type Item = R`

- <span id="mapwith-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="mapwith-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MapWith<I, T, F>`

- <span id="mapwith-const-align"></span>`const ALIGN: usize`

- <span id="mapwith-type-init"></span>`type Init = T`

- <span id="mapwith-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwith-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwith-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwith-drop"></span>`unsafe fn drop(ptr: usize)`

### `MultiZip<T>`

```rust
struct MultiZip<T> {
    tuple: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/multizip.rs:79-81`](../../../.source_1765210505/rayon-1.11.0/src/iter/multizip.rs#L79-L81)*

`MultiZip` is an iterator that zips up a tuple of parallel iterators to
produce tuples of their items.

It is created by calling `into_par_iter()` on a tuple of types that
implement `IntoParallelIterator`, or `par_iter()`/`par_iter_mut()` with
types that are iterable by reference.

The implementation currently support tuples up to length 12.

# Examples

```rust
use rayon::prelude::*;

// This will iterate `r` by mutable reference, like `par_iter_mut()`, while
// ranges are all iterated by value like `into_par_iter()`.
// Note that the zipped iterator is only as long as the shortest input.
let mut r = vec![0; 3];
(&mut r, 1..10, 10..100, 100..1000).into_par_iter()
    .for_each(|(r, x, y, z)| *r = x * y + z);

assert_eq!(&r, &[1 * 10 + 100, 2 * 11 + 101, 3 * 12 + 102]);
```

For a group that should all be iterated by reference, you can use a tuple reference.

```rust
use rayon::prelude::*;

let xs: Vec<_> = (1..10).collect();
let ys: Vec<_> = (10..100).collect();
let zs: Vec<_> = (100..1000).collect();

// Reference each input separately with `IntoParallelIterator`:
let r1: Vec<_> = (&xs, &ys, &zs).into_par_iter()
    .map(|(x, y, z)| x * y + z)
    .collect();

// Reference them all together with `IntoParallelRefIterator`:
let r2: Vec<_> = (xs, ys, zs).par_iter()
    .map(|(x, y, z)| x * y + z)
    .collect();

assert_eq!(r1, r2);
```

Mutable references to a tuple will work similarly.

```rust
use rayon::prelude::*;

let mut xs: Vec<_> = (1..4).collect();
let mut ys: Vec<_> = (-4..-1).collect();
let mut zs = vec![0; 3];

// Mutably reference each input separately with `IntoParallelIterator`:
(&mut xs, &mut ys, &mut zs).into_par_iter().for_each(|(x, y, z)| {
    *z += *x + *y;
    std::mem::swap(x, y);
});

assert_eq!(xs, (vec![-4, -3, -2]));
assert_eq!(ys, (vec![1, 2, 3]));
assert_eq!(zs, (vec![-3, -1, 1]));

// Mutably reference them all together with `IntoParallelRefMutIterator`:
let mut tuple = (xs, ys, zs);
tuple.par_iter_mut().for_each(|(x, y, z)| {
    *z += *x + *y;
    std::mem::swap(x, y);
});

assert_eq!(tuple, (vec![1, 2, 3], vec![-4, -3, -2], vec![-6, -2, 2]));
```

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for MultiZip<T>`

- <span id="multizip-clone"></span>`fn clone(&self) -> MultiZip<T>` — [`MultiZip`](multizip/index.md)

##### `impl<T: fmt::Debug> Debug for MultiZip<T>`

- <span id="multizip-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A> IndexedParallelIterator for MultiZip<(A)>`

- <span id="multizip-drive"></span>`fn drive<CONSUMER>(self, consumer: CONSUMER) -> <CONSUMER as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="multizip-len"></span>`fn len(&self) -> usize`

- <span id="multizip-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for MultiZip<T>`

##### `impl<T> IntoParallelIterator for MultiZip<T>`

- <span id="multizip-type-iter"></span>`type Iter = T`

- <span id="multizip-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="multizip-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A> ParallelIterator for MultiZip<(A)>`

- <span id="multizip-type-item"></span>`type Item = (<A as ParallelIterator>::Item)`

- <span id="multizip-drive-unindexed"></span>`fn drive_unindexed<CONSUMER>(self, consumer: CONSUMER) -> <CONSUMER as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="multizip-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MultiZip<T>`

- <span id="multizip-const-align"></span>`const ALIGN: usize`

- <span id="multizip-type-init"></span>`type Init = T`

- <span id="multizip-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multizip-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multizip-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multizip-drop"></span>`unsafe fn drop(ptr: usize)`

### `Once<T>`

```rust
struct Once<T> {
    item: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/once.rs:32-34`](../../../.source_1765210505/rayon-1.11.0/src/iter/once.rs#L32-L34)*

Iterator adaptor for [the `once()` function].


#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Once<T>`

- <span id="once-clone"></span>`fn clone(&self) -> Once<T>` — [`Once`](once/index.md)

##### `impl<T: fmt::Debug> Debug for Once<T>`

- <span id="once-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for Once<T>`

- <span id="once-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="once-len"></span>`fn len(&self) -> usize`

- <span id="once-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Once<T>`

##### `impl<T> IntoParallelIterator for Once<T>`

- <span id="once-type-iter"></span>`type Iter = T`

- <span id="once-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="once-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Once<T>`

- <span id="once-type-item"></span>`type Item = T`

- <span id="once-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="once-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Once<T>`

- <span id="once-const-align"></span>`const ALIGN: usize`

- <span id="once-type-init"></span>`type Init = T`

- <span id="once-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="once-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="once-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="once-drop"></span>`unsafe fn drop(ptr: usize)`

### `PanicFuse<I>`

```rust
struct PanicFuse<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:14-16`](../../../.source_1765210505/rayon-1.11.0/src/iter/panic_fuse.rs#L14-L16)*

`PanicFuse` is an adaptor that wraps an iterator with a fuse in case
of panics, to halt all threads as soon as possible.

This struct is created by the `panic_fuse()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="panicfuse-new"></span>`fn new(base: I) -> PanicFuse<I>` — [`PanicFuse`](panic_fuse/index.md)

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for PanicFuse<I>`

- <span id="panicfuse-clone"></span>`fn clone(&self) -> PanicFuse<I>` — [`PanicFuse`](panic_fuse/index.md)

##### `impl<I: fmt::Debug> Debug for PanicFuse<I>`

- <span id="panicfuse-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for PanicFuse<I>`

- <span id="panicfuse-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="panicfuse-len"></span>`fn len(&self) -> usize`

- <span id="panicfuse-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for PanicFuse<I>`

##### `impl<T> IntoParallelIterator for PanicFuse<I>`

- <span id="panicfuse-type-iter"></span>`type Iter = T`

- <span id="panicfuse-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="panicfuse-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for PanicFuse<I>`

- <span id="panicfuse-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="panicfuse-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="panicfuse-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for PanicFuse<I>`

- <span id="panicfuse-const-align"></span>`const ALIGN: usize`

- <span id="panicfuse-type-init"></span>`type Init = T`

- <span id="panicfuse-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuse-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuse-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuse-drop"></span>`unsafe fn drop(ptr: usize)`

### `IterBridge<Iter>`

```rust
struct IterBridge<Iter> {
    iter: Iter,
}
```

*Defined in [`rayon-1.11.0/src/iter/par_bridge.rs:72-74`](../../../.source_1765210505/rayon-1.11.0/src/iter/par_bridge.rs#L72-L74)*

`IterBridge` is a parallel iterator that wraps a sequential iterator.

This type is created when using the `par_bridge` method on `ParallelBridge`. See the
[`ParallelBridge`](par_bridge/index.md) documentation for details.

#### Trait Implementations

##### `impl<Iter: clone::Clone> Clone for IterBridge<Iter>`

- <span id="iterbridge-clone"></span>`fn clone(&self) -> IterBridge<Iter>` — [`IterBridge`](par_bridge/index.md)

##### `impl<Iter: fmt::Debug> Debug for IterBridge<Iter>`

- <span id="iterbridge-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for IterBridge<Iter>`

##### `impl<T> IntoParallelIterator for IterBridge<Iter>`

- <span id="iterbridge-type-iter"></span>`type Iter = T`

- <span id="iterbridge-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iterbridge-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<Iter> ParallelIterator for IterBridge<Iter>`

- <span id="iterbridge-type-item"></span>`type Item = <Iter as Iterator>::Item`

- <span id="iterbridge-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for IterBridge<Iter>`

- <span id="iterbridge-const-align"></span>`const ALIGN: usize`

- <span id="iterbridge-type-init"></span>`type Init = T`

- <span id="iterbridge-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iterbridge-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iterbridge-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iterbridge-drop"></span>`unsafe fn drop(ptr: usize)`

### `Positions<I, P>`

```rust
struct Positions<I, P> {
    base: I,
    predicate: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/positions.rs:14-17`](../../../.source_1765210505/rayon-1.11.0/src/iter/positions.rs#L14-L17)*

`Positions` takes a predicate `predicate` and filters out elements that match,
yielding their indices.

This struct is created by the `positions()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="positions-new"></span>`fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for Positions<I, P>`

- <span id="positions-clone"></span>`fn clone(&self) -> Positions<I, P>` — [`Positions`](positions/index.md)

##### `impl<I: Debug, P> Debug for Positions<I, P>`

- <span id="positions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Positions<I, P>`

##### `impl<T> IntoParallelIterator for Positions<I, P>`

- <span id="positions-type-iter"></span>`type Iter = T`

- <span id="positions-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="positions-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for Positions<I, P>`

- <span id="positions-type-item"></span>`type Item = usize`

- <span id="positions-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Positions<I, P>`

- <span id="positions-const-align"></span>`const ALIGN: usize`

- <span id="positions-type-init"></span>`type Init = T`

- <span id="positions-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="positions-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="positions-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="positions-drop"></span>`unsafe fn drop(ptr: usize)`

### `Repeat<T>`

```rust
struct Repeat<T> {
    element: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:10-12`](../../../.source_1765210505/rayon-1.11.0/src/iter/repeat.rs#L10-L12)*

Iterator adaptor for [the `repeat()` function].


#### Implementations

- <span id="repeat-take"></span>`fn take(self, n: usize) -> RepeatN<T>` — [`RepeatN`](repeat/index.md)

- <span id="repeat-zip"></span>`fn zip<Z>(self, zip_op: Z) -> Zip<RepeatN<T>, <Z as >::Iter>` — [`Zip`](zip/index.md), [`RepeatN`](repeat/index.md), [`IntoParallelIterator`](#intoparalleliterator)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Repeat<T>`

- <span id="repeat-clone"></span>`fn clone(&self) -> Repeat<T>` — [`Repeat`](repeat/index.md)

##### `impl<T: fmt::Debug> Debug for Repeat<T>`

- <span id="repeat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Repeat<T>`

##### `impl<T> IntoParallelIterator for Repeat<T>`

- <span id="repeat-type-iter"></span>`type Iter = T`

- <span id="repeat-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="repeat-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T> ParallelIterator for Repeat<T>`

- <span id="repeat-type-item"></span>`type Item = T`

- <span id="repeat-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Repeat<T>`

- <span id="repeat-const-align"></span>`const ALIGN: usize`

- <span id="repeat-type-init"></span>`type Init = T`

- <span id="repeat-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeat-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeat-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeat-drop"></span>`unsafe fn drop(ptr: usize)`

### `RepeatN<T>`

```rust
struct RepeatN<T> {
    inner: RepeatNProducer<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:111-113`](../../../.source_1765210505/rayon-1.11.0/src/iter/repeat.rs#L111-L113)*

Iterator adaptor for [the `repeat_n()` function].


#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RepeatN<T>`

- <span id="repeatn-clone"></span>`fn clone(&self) -> RepeatN<T>` — [`RepeatN`](repeat/index.md)

##### `impl<T: fmt::Debug> Debug for RepeatN<T>`

- <span id="repeatn-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IndexedParallelIterator for RepeatN<T>`

- <span id="repeatn-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="repeatn-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

- <span id="repeatn-len"></span>`fn len(&self) -> usize`

##### `impl<T> IntoEither for RepeatN<T>`

##### `impl<T> IntoParallelIterator for RepeatN<T>`

- <span id="repeatn-type-iter"></span>`type Iter = T`

- <span id="repeatn-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="repeatn-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T> ParallelIterator for RepeatN<T>`

- <span id="repeatn-type-item"></span>`type Item = T`

- <span id="repeatn-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="repeatn-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RepeatN<T>`

- <span id="repeatn-const-align"></span>`const ALIGN: usize`

- <span id="repeatn-type-init"></span>`type Init = T`

- <span id="repeatn-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeatn-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeatn-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeatn-drop"></span>`unsafe fn drop(ptr: usize)`

### `Rev<I>`

```rust
struct Rev<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/rev.rs:11-13`](../../../.source_1765210505/rayon-1.11.0/src/iter/rev.rs#L11-L13)*

`Rev` is an iterator that produces elements in reverse order. This struct
is created by the `rev()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="rev-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Rev<I>`

- <span id="rev-clone"></span>`fn clone(&self) -> Rev<I>` — [`Rev`](rev/index.md)

##### `impl<I: fmt::Debug> Debug for Rev<I>`

- <span id="rev-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Rev<I>`

- <span id="rev-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="rev-len"></span>`fn len(&self) -> usize`

- <span id="rev-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Rev<I>`

##### `impl<T> IntoParallelIterator for Rev<I>`

- <span id="rev-type-iter"></span>`type Iter = T`

- <span id="rev-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rev-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Rev<I>`

- <span id="rev-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="rev-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="rev-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Rev<I>`

- <span id="rev-const-align"></span>`const ALIGN: usize`

- <span id="rev-type-init"></span>`type Init = T`

- <span id="rev-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rev-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rev-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rev-drop"></span>`unsafe fn drop(ptr: usize)`

### `Skip<I>`

```rust
struct Skip<I> {
    base: I,
    n: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip.rs:11-14`](../../../.source_1765210505/rayon-1.11.0/src/iter/skip.rs#L11-L14)*

`Skip` is an iterator that skips over the first `n` elements.
This struct is created by the `skip()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="skip-new"></span>`fn new(base: I, n: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Skip<I>`

- <span id="skip-clone"></span>`fn clone(&self) -> Skip<I>` — [`Skip`](skip/index.md)

##### `impl<I: fmt::Debug> Debug for Skip<I>`

- <span id="skip-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Skip<I>`

- <span id="skip-len"></span>`fn len(&self) -> usize`

- <span id="skip-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="skip-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Skip<I>`

##### `impl<T> IntoParallelIterator for Skip<I>`

- <span id="skip-type-iter"></span>`type Iter = T`

- <span id="skip-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skip-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Skip<I>`

- <span id="skip-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skip-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="skip-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Skip<I>`

- <span id="skip-const-align"></span>`const ALIGN: usize`

- <span id="skip-type-init"></span>`type Init = T`

- <span id="skip-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skip-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skip-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skip-drop"></span>`unsafe fn drop(ptr: usize)`

### `SkipAny<I>`

```rust
struct SkipAny<I> {
    base: I,
    count: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any.rs:11-14`](../../../.source_1765210505/rayon-1.11.0/src/iter/skip_any.rs#L11-L14)*

`SkipAny` is an iterator that skips over `n` elements from anywhere in `I`.
This struct is created by the `skip_any()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="skipany-new"></span>`fn new(base: I, count: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for SkipAny<I>`

- <span id="skipany-clone"></span>`fn clone(&self) -> SkipAny<I>` — [`SkipAny`](skip_any/index.md)

##### `impl<I: fmt::Debug> Debug for SkipAny<I>`

- <span id="skipany-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SkipAny<I>`

##### `impl<T> IntoParallelIterator for SkipAny<I>`

- <span id="skipany-type-iter"></span>`type Iter = T`

- <span id="skipany-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skipany-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for SkipAny<I>`

- <span id="skipany-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skipany-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for SkipAny<I>`

- <span id="skipany-const-align"></span>`const ALIGN: usize`

- <span id="skipany-type-init"></span>`type Init = T`

- <span id="skipany-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipany-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipany-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipany-drop"></span>`unsafe fn drop(ptr: usize)`

### `SkipAnyWhile<I, P>`

```rust
struct SkipAnyWhile<I, P> {
    base: I,
    predicate: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any_while.rs:13-16`](../../../.source_1765210505/rayon-1.11.0/src/iter/skip_any_while.rs#L13-L16)*

`SkipAnyWhile` is an iterator that skips over elements from anywhere in `I`
until the callback returns `false`.
This struct is created by the `skip_any_while()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="skipanywhile-new"></span>`fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for SkipAnyWhile<I, P>`

- <span id="skipanywhile-clone"></span>`fn clone(&self) -> SkipAnyWhile<I, P>` — [`SkipAnyWhile`](skip_any_while/index.md)

##### `impl<I: fmt::Debug, P> Debug for SkipAnyWhile<I, P>`

- <span id="skipanywhile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SkipAnyWhile<I, P>`

##### `impl<T> IntoParallelIterator for SkipAnyWhile<I, P>`

- <span id="skipanywhile-type-iter"></span>`type Iter = T`

- <span id="skipanywhile-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skipanywhile-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for SkipAnyWhile<I, P>`

- <span id="skipanywhile-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skipanywhile-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for SkipAnyWhile<I, P>`

- <span id="skipanywhile-const-align"></span>`const ALIGN: usize`

- <span id="skipanywhile-type-init"></span>`type Init = T`

- <span id="skipanywhile-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanywhile-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanywhile-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanywhile-drop"></span>`unsafe fn drop(ptr: usize)`

### `Split<D, S>`

```rust
struct Split<D, S> {
    data: D,
    splitter: S,
}
```

*Defined in [`rayon-1.11.0/src/iter/splitter.rs:117-120`](../../../.source_1765210505/rayon-1.11.0/src/iter/splitter.rs#L117-L120)*

`Split` is a parallel iterator using arbitrary data and a splitting function.
This struct is created by the [`split()`](splitter/index.md) function.

#### Trait Implementations

##### `impl<D: clone::Clone, S: clone::Clone> Clone for Split<D, S>`

- <span id="split-clone"></span>`fn clone(&self) -> Split<D, S>` — [`Split`](splitter/index.md)

##### `impl<D: Debug, S> Debug for Split<D, S>`

- <span id="split-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Split<D, S>`

##### `impl<T> IntoParallelIterator for Split<D, S>`

- <span id="split-type-iter"></span>`type Iter = T`

- <span id="split-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="split-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<D, S> ParallelIterator for Split<D, S>`

- <span id="split-type-item"></span>`type Item = D`

- <span id="split-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Split<D, S>`

- <span id="split-const-align"></span>`const ALIGN: usize`

- <span id="split-type-init"></span>`type Init = T`

- <span id="split-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="split-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="split-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="split-drop"></span>`unsafe fn drop(ptr: usize)`

### `StepBy<I>`

```rust
struct StepBy<I> {
    base: I,
    step: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/step_by.rs:11-14`](../../../.source_1765210505/rayon-1.11.0/src/iter/step_by.rs#L11-L14)*

`StepBy` is an iterator that skips `n` elements between each yield, where `n` is the given step.
This struct is created by the `step_by()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="stepby-new"></span>`fn new(base: I, step: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for StepBy<I>`

- <span id="stepby-clone"></span>`fn clone(&self) -> StepBy<I>` — [`StepBy`](step_by/index.md)

##### `impl<I: fmt::Debug> Debug for StepBy<I>`

- <span id="stepby-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for StepBy<I>`

- <span id="stepby-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="stepby-len"></span>`fn len(&self) -> usize`

- <span id="stepby-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for StepBy<I>`

##### `impl<T> IntoParallelIterator for StepBy<I>`

- <span id="stepby-type-iter"></span>`type Iter = T`

- <span id="stepby-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="stepby-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for StepBy<I>`

- <span id="stepby-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="stepby-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="stepby-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for StepBy<I>`

- <span id="stepby-const-align"></span>`const ALIGN: usize`

- <span id="stepby-type-init"></span>`type Init = T`

- <span id="stepby-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stepby-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stepby-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stepby-drop"></span>`unsafe fn drop(ptr: usize)`

### `Take<I>`

```rust
struct Take<I> {
    base: I,
    n: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/take.rs:10-13`](../../../.source_1765210505/rayon-1.11.0/src/iter/take.rs#L10-L13)*

`Take` is an iterator that iterates over the first `n` elements.
This struct is created by the `take()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="take-new"></span>`fn new(base: I, n: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Take<I>`

- <span id="take-clone"></span>`fn clone(&self) -> Take<I>` — [`Take`](take/index.md)

##### `impl<I: fmt::Debug> Debug for Take<I>`

- <span id="take-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Take<I>`

- <span id="take-len"></span>`fn len(&self) -> usize`

- <span id="take-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="take-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Take<I>`

##### `impl<T> IntoParallelIterator for Take<I>`

- <span id="take-type-iter"></span>`type Iter = T`

- <span id="take-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="take-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Take<I>`

- <span id="take-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="take-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="take-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Take<I>`

- <span id="take-const-align"></span>`const ALIGN: usize`

- <span id="take-type-init"></span>`type Init = T`

- <span id="take-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="take-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="take-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="take-drop"></span>`unsafe fn drop(ptr: usize)`

### `TakeAny<I>`

```rust
struct TakeAny<I> {
    base: I,
    count: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any.rs:11-14`](../../../.source_1765210505/rayon-1.11.0/src/iter/take_any.rs#L11-L14)*

`TakeAny` is an iterator that iterates over `n` elements from anywhere in `I`.
This struct is created by the `take_any()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="takeany-new"></span>`fn new(base: I, count: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for TakeAny<I>`

- <span id="takeany-clone"></span>`fn clone(&self) -> TakeAny<I>` — [`TakeAny`](take_any/index.md)

##### `impl<I: fmt::Debug> Debug for TakeAny<I>`

- <span id="takeany-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TakeAny<I>`

##### `impl<T> IntoParallelIterator for TakeAny<I>`

- <span id="takeany-type-iter"></span>`type Iter = T`

- <span id="takeany-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="takeany-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for TakeAny<I>`

- <span id="takeany-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="takeany-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for TakeAny<I>`

- <span id="takeany-const-align"></span>`const ALIGN: usize`

- <span id="takeany-type-init"></span>`type Init = T`

- <span id="takeany-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeany-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeany-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeany-drop"></span>`unsafe fn drop(ptr: usize)`

### `TakeAnyWhile<I, P>`

```rust
struct TakeAnyWhile<I, P> {
    base: I,
    predicate: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any_while.rs:13-16`](../../../.source_1765210505/rayon-1.11.0/src/iter/take_any_while.rs#L13-L16)*

`TakeAnyWhile` is an iterator that iterates over elements from anywhere in `I`
until the callback returns `false`.
This struct is created by the `take_any_while()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="takeanywhile-new"></span>`fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for TakeAnyWhile<I, P>`

- <span id="takeanywhile-clone"></span>`fn clone(&self) -> TakeAnyWhile<I, P>` — [`TakeAnyWhile`](take_any_while/index.md)

##### `impl<I: fmt::Debug, P> Debug for TakeAnyWhile<I, P>`

- <span id="takeanywhile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TakeAnyWhile<I, P>`

##### `impl<T> IntoParallelIterator for TakeAnyWhile<I, P>`

- <span id="takeanywhile-type-iter"></span>`type Iter = T`

- <span id="takeanywhile-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="takeanywhile-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for TakeAnyWhile<I, P>`

- <span id="takeanywhile-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="takeanywhile-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for TakeAnyWhile<I, P>`

- <span id="takeanywhile-const-align"></span>`const ALIGN: usize`

- <span id="takeanywhile-type-init"></span>`type Init = T`

- <span id="takeanywhile-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanywhile-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanywhile-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanywhile-drop"></span>`unsafe fn drop(ptr: usize)`

### `TryFold<I, U, ID, F>`

```rust
struct TryFold<I, U, ID, F> {
    base: I,
    identity: ID,
    fold_op: F,
    marker: std::marker::PhantomData<U>,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_fold.rs:26-31`](../../../.source_1765210505/rayon-1.11.0/src/iter/try_fold.rs#L26-L31)*

`TryFold` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `try_fold()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="tryfold-new"></span>`fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, U: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for TryFold<I, U, ID, F>`

- <span id="tryfold-clone"></span>`fn clone(&self) -> TryFold<I, U, ID, F>` — [`TryFold`](try_fold/index.md)

##### `impl<U, I: ParallelIterator + Debug, ID, F> Debug for TryFold<I, U, ID, F>`

- <span id="tryfold-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TryFold<I, U, ID, F>`

##### `impl<T> IntoParallelIterator for TryFold<I, U, ID, F>`

- <span id="tryfold-type-iter"></span>`type Iter = T`

- <span id="tryfold-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="tryfold-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, ID, F> ParallelIterator for TryFold<I, U, ID, F>`

- <span id="tryfold-type-item"></span>`type Item = U`

- <span id="tryfold-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for TryFold<I, U, ID, F>`

- <span id="tryfold-const-align"></span>`const ALIGN: usize`

- <span id="tryfold-type-init"></span>`type Init = T`

- <span id="tryfold-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfold-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfold-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfold-drop"></span>`unsafe fn drop(ptr: usize)`

### `TryFoldWith<I, U: Try, F>`

```rust
struct TryFoldWith<I, U: Try, F> {
    base: I,
    item: <U as >::Output,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_fold.rs:180-184`](../../../.source_1765210505/rayon-1.11.0/src/iter/try_fold.rs#L180-L184)*

`TryFoldWith` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `try_fold_with()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="tryfoldwith-new"></span>`fn new(base: I, item: <U as >::Output, fold_op: F) -> Self` — [`Try`](private/index.md)

#### Trait Implementations

##### `impl<I: clone::Clone, U: clone::Clone + Try, F: clone::Clone> Clone for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-clone"></span>`fn clone(&self) -> TryFoldWith<I, U, F>` — [`TryFoldWith`](try_fold/index.md)

##### `impl<I, U, F> Debug for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TryFoldWith<I, U, F>`

##### `impl<T> IntoParallelIterator for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-type-iter"></span>`type Iter = T`

- <span id="tryfoldwith-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="tryfoldwith-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, F> ParallelIterator for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-type-item"></span>`type Item = U`

- <span id="tryfoldwith-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-const-align"></span>`const ALIGN: usize`

- <span id="tryfoldwith-type-init"></span>`type Init = T`

- <span id="tryfoldwith-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfoldwith-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfoldwith-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfoldwith-drop"></span>`unsafe fn drop(ptr: usize)`

### `Update<I, F>`

```rust
struct Update<I, F> {
    base: I,
    update_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:14-17`](../../../.source_1765210505/rayon-1.11.0/src/iter/update.rs#L14-L17)*

`Update` is an iterator that mutates the elements of an
underlying iterator before they are yielded.

This struct is created by the `update()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="update-new"></span>`fn new(base: I, update_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Update<I, F>`

- <span id="update-clone"></span>`fn clone(&self) -> Update<I, F>` — [`Update`](update/index.md)

##### `impl<I: Debug, F> Debug for Update<I, F>`

- <span id="update-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> IndexedParallelIterator for Update<I, F>`

- <span id="update-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="update-len"></span>`fn len(&self) -> usize`

- <span id="update-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Update<I, F>`

##### `impl<T> IntoParallelIterator for Update<I, F>`

- <span id="update-type-iter"></span>`type Iter = T`

- <span id="update-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="update-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for Update<I, F>`

- <span id="update-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="update-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="update-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Update<I, F>`

- <span id="update-const-align"></span>`const ALIGN: usize`

- <span id="update-type-init"></span>`type Init = T`

- <span id="update-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="update-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="update-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="update-drop"></span>`unsafe fn drop(ptr: usize)`

### `WalkTree<S, B>`

```rust
struct WalkTree<S, B>(WalkTreePostfix<S, B>);
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:459`](../../../.source_1765210505/rayon-1.11.0/src/iter/walk_tree.rs#L459)*

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree()`](walk_tree/index.md) function.

#### Trait Implementations

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTree<S, B>`

- <span id="walktree-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for WalkTree<S, B>`

##### `impl<T> IntoParallelIterator for WalkTree<S, B>`

- <span id="walktree-type-iter"></span>`type Iter = T`

- <span id="walktree-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktree-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTree<S, B>`

- <span id="walktree-type-item"></span>`type Item = S`

- <span id="walktree-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for WalkTree<S, B>`

- <span id="walktree-const-align"></span>`const ALIGN: usize`

- <span id="walktree-type-init"></span>`type Init = T`

- <span id="walktree-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktree-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktree-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktree-drop"></span>`unsafe fn drop(ptr: usize)`

### `WalkTreePostfix<S, B>`

```rust
struct WalkTreePostfix<S, B> {
    initial_state: S,
    children_of: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:302-305`](../../../.source_1765210505/rayon-1.11.0/src/iter/walk_tree.rs#L302-L305)*

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_postfix()`](walk_tree/index.md) function.

#### Trait Implementations

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for WalkTreePostfix<S, B>`

##### `impl<T> IntoParallelIterator for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-type-iter"></span>`type Iter = T`

- <span id="walktreepostfix-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktreepostfix-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-type-item"></span>`type Item = S`

- <span id="walktreepostfix-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-const-align"></span>`const ALIGN: usize`

- <span id="walktreepostfix-type-init"></span>`type Init = T`

- <span id="walktreepostfix-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreepostfix-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreepostfix-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreepostfix-drop"></span>`unsafe fn drop(ptr: usize)`

### `WalkTreePrefix<S, B>`

```rust
struct WalkTreePrefix<S, B> {
    initial_state: S,
    children_of: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:76-79`](../../../.source_1765210505/rayon-1.11.0/src/iter/walk_tree.rs#L76-L79)*

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_prefix()`](walk_tree/index.md) function.

#### Trait Implementations

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for WalkTreePrefix<S, B>`

##### `impl<T> IntoParallelIterator for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-type-iter"></span>`type Iter = T`

- <span id="walktreeprefix-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktreeprefix-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-type-item"></span>`type Item = S`

- <span id="walktreeprefix-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-const-align"></span>`const ALIGN: usize`

- <span id="walktreeprefix-type-init"></span>`type Init = T`

- <span id="walktreeprefix-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreeprefix-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreeprefix-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreeprefix-drop"></span>`unsafe fn drop(ptr: usize)`

### `WhileSome<I>`

```rust
struct WhileSome<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/while_some.rs:13-15`](../../../.source_1765210505/rayon-1.11.0/src/iter/while_some.rs#L13-L15)*

`WhileSome` is an iterator that yields the `Some` elements of an iterator,
halting as soon as any `None` is produced.

This struct is created by the `while_some()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="whilesome-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for WhileSome<I>`

- <span id="whilesome-clone"></span>`fn clone(&self) -> WhileSome<I>` — [`WhileSome`](while_some/index.md)

##### `impl<I: fmt::Debug> Debug for WhileSome<I>`

- <span id="whilesome-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for WhileSome<I>`

##### `impl<T> IntoParallelIterator for WhileSome<I>`

- <span id="whilesome-type-iter"></span>`type Iter = T`

- <span id="whilesome-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="whilesome-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, T> ParallelIterator for WhileSome<I>`

- <span id="whilesome-type-item"></span>`type Item = T`

- <span id="whilesome-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for WhileSome<I>`

- <span id="whilesome-const-align"></span>`const ALIGN: usize`

- <span id="whilesome-type-init"></span>`type Init = T`

- <span id="whilesome-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="whilesome-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="whilesome-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="whilesome-drop"></span>`unsafe fn drop(ptr: usize)`

### `Zip<A, B>`

```rust
struct Zip<A, B> {
    a: A,
    b: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/zip.rs:12-15`](../../../.source_1765210505/rayon-1.11.0/src/iter/zip.rs#L12-L15)*

`Zip` is an iterator that zips up `a` and `b` into a single iterator
of pairs. This struct is created by the `zip()` method on
[`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="zip-new"></span>`fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: clone::Clone, B: clone::Clone> Clone for Zip<A, B>`

- <span id="zip-clone"></span>`fn clone(&self) -> Zip<A, B>` — [`Zip`](zip/index.md)

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Zip<A, B>`

- <span id="zip-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> IndexedParallelIterator for Zip<A, B>`

- <span id="zip-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="zip-len"></span>`fn len(&self) -> usize`

- <span id="zip-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Zip<A, B>`

##### `impl<T> IntoParallelIterator for Zip<A, B>`

- <span id="zip-type-iter"></span>`type Iter = T`

- <span id="zip-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="zip-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for Zip<A, B>`

- <span id="zip-type-item"></span>`type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- <span id="zip-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="zip-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Zip<A, B>`

- <span id="zip-const-align"></span>`const ALIGN: usize`

- <span id="zip-type-init"></span>`type Init = T`

- <span id="zip-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="zip-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="zip-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="zip-drop"></span>`unsafe fn drop(ptr: usize)`

### `ZipEq<A, B>`

```rust
struct ZipEq<A, B> {
    zip: Zip<A, B>,
}
```

*Defined in [`rayon-1.11.0/src/iter/zip_eq.rs:13-15`](../../../.source_1765210505/rayon-1.11.0/src/iter/zip_eq.rs#L13-L15)*

An [`IndexedParallelIterator`](#indexedparalleliterator) that iterates over two parallel iterators of equal
length simultaneously.

This struct is created by the [`zip_eq`](zip_eq/index.md) method on [`IndexedParallelIterator`](#indexedparalleliterator),
see its documentation for more information.


#### Implementations

- <span id="zipeq-new"></span>`fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: clone::Clone, B: clone::Clone> Clone for ZipEq<A, B>`

- <span id="zipeq-clone"></span>`fn clone(&self) -> ZipEq<A, B>` — [`ZipEq`](zip_eq/index.md)

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for ZipEq<A, B>`

- <span id="zipeq-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> IndexedParallelIterator for ZipEq<A, B>`

- <span id="zipeq-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="zipeq-len"></span>`fn len(&self) -> usize`

- <span id="zipeq-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for ZipEq<A, B>`

##### `impl<T> IntoParallelIterator for ZipEq<A, B>`

- <span id="zipeq-type-iter"></span>`type Iter = T`

- <span id="zipeq-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="zipeq-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for ZipEq<A, B>`

- <span id="zipeq-type-item"></span>`type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- <span id="zipeq-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- <span id="zipeq-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ZipEq<A, B>`

- <span id="zipeq-const-align"></span>`const ALIGN: usize`

- <span id="zipeq-type-init"></span>`type Init = T`

- <span id="zipeq-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="zipeq-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="zipeq-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="zipeq-drop"></span>`unsafe fn drop(ptr: usize)`

## Traits

### `ParallelBridge`

```rust
trait ParallelBridge: Sized { ... }
```

*Defined in [`rayon-1.11.0/src/iter/par_bridge.rs:53-56`](../../../.source_1765210505/rayon-1.11.0/src/iter/par_bridge.rs#L53-L56)*

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

### `IntoParallelIterator`

```rust
trait IntoParallelIterator { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:219-249`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L219-L249)*

`IntoParallelIterator` implements the conversion to a [`ParallelIterator`](#paralleliterator).

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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:261-285`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L261-L285)*

`IntoParallelRefIterator` implements the conversion to a
[`ParallelIterator`](#paralleliterator), providing shared references to the data.

This is a parallel version of the `iter()` method
defined by various collections.

This trait is automatically implemented
`for I where &I: IntoParallelIterator`. In most cases, users
will want to implement [`IntoParallelIterator`](#intoparalleliterator) rather than implement
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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:309-329`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L309-L329)*

`IntoParallelRefMutIterator` implements the conversion to a
[`ParallelIterator`](#paralleliterator), providing mutable references to the data.

This is a parallel version of the `iter_mut()` method
defined by various collections.

This trait is automatically implemented
`for I where &mut I: IntoParallelIterator`. In most cases, users
will want to implement [`IntoParallelIterator`](#intoparalleliterator) rather than implement
this trait directly.

#### Associated Types

- `type Iter: 1`

- `type Item: 2`

#### Required Methods

- `fn par_iter_mut(self: &'data mut Self) -> <Self as >::Iter`

  Creates the parallel iterator from `self`.

#### Implementors

- `I`

### `ParallelIterator`

```rust
trait ParallelIterator: Sized + Send { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:356-2421`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L356-L2421)*

Parallel version of the standard iterator trait.

The combinators on this trait are available on **all** parallel
iterators.  Additional methods can be found on the
[`IndexedParallelIterator`](#indexedparalleliterator) trait: those methods are only
available for parallel iterators where the number of items is
known in advance (so, e.g., after invoking `filter`, those methods
become unavailable).

For examples of using parallel iterators, see [the docs on the
`iter` module][iter](#iter).


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

- [`Bytes`](../str/index.md)
- [`Chain`](chain/index.md)
- [`CharIndices`](../str/index.md)
- [`Chars`](../str/index.md)
- [`ChunkByMut`](../slice/chunk_by/index.md)
- [`ChunkBy`](../slice/chunk_by/index.md)
- [`ChunksExactMut`](../slice/chunks/index.md)
- [`ChunksExact`](../slice/chunks/index.md)
- [`ChunksMut`](../slice/chunks/index.md)
- [`Chunks`](../slice/chunks/index.md)
- [`Chunks`](chunks/index.md)
- [`Cloned`](cloned/index.md)
- [`Copied`](copied/index.md)
- [`Drain`](../collections/binary_heap/index.md)
- [`Drain`](../collections/hash_map/index.md)
- [`Drain`](../collections/hash_set/index.md)
- [`Drain`](../collections/vec_deque/index.md)
- [`Drain`](../string/index.md)
- [`Drain`](../vec/index.md)
- [`Either`](#either)
- [`Empty`](empty/index.md)
- [`EncodeUtf16`](../str/index.md)
- [`Enumerate`](enumerate/index.md)
- [`ExponentialBlocks`](blocks/index.md)
- [`FilterMap`](filter_map/index.md)
- [`Filter`](filter/index.md)
- [`FlatMapIter`](flat_map_iter/index.md)
- [`FlatMap`](flat_map/index.md)
- [`FlattenIter`](flatten_iter/index.md)
- [`Flatten`](flatten/index.md)
- [`FoldChunksWith`](fold_chunks_with/index.md)
- [`FoldChunks`](fold_chunks/index.md)
- [`FoldWith`](fold/index.md)
- [`Fold`](fold/index.md)
- [`Inspect`](inspect/index.md)
- [`InterleaveShortest`](interleave_shortest/index.md)
- [`Interleave`](interleave/index.md)
- [`Intersperse`](intersperse/index.md)
- [`IntoIter`](../array/index.md)
- [`IntoIter`](../collections/binary_heap/index.md)
- [`IntoIter`](../collections/btree_map/index.md)
- [`IntoIter`](../collections/btree_set/index.md)
- [`IntoIter`](../collections/hash_map/index.md)
- [`IntoIter`](../collections/hash_set/index.md)
- [`IntoIter`](../collections/linked_list/index.md)
- [`IntoIter`](../collections/vec_deque/index.md)
- [`IntoIter`](../option/index.md)
- [`IntoIter`](../result/index.md)
- [`IntoIter`](../vec/index.md)
- [`IterBridge`](par_bridge/index.md)
- [`IterMut`](../collections/btree_map/index.md)
- [`IterMut`](../collections/hash_map/index.md)
- [`IterMut`](../collections/linked_list/index.md)
- [`IterMut`](../collections/vec_deque/index.md)
- [`IterMut`](../option/index.md)
- [`IterMut`](../result/index.md)
- [`IterMut`](../slice/index.md)
- [`Iter`](../collections/binary_heap/index.md)
- [`Iter`](../collections/btree_map/index.md)
- [`Iter`](../collections/btree_set/index.md)
- [`Iter`](../collections/hash_map/index.md)
- [`Iter`](../collections/hash_set/index.md)
- [`Iter`](../collections/linked_list/index.md)
- [`Iter`](../collections/vec_deque/index.md)
- [`Iter`](../option/index.md)
- [`Iter`](../range/index.md)
- [`Iter`](../range_inclusive/index.md)
- [`Iter`](../result/index.md)
- [`Iter`](../slice/index.md)
- [`Lines`](../str/index.md)
- [`MapInit`](map_with/index.md)
- [`MapWith`](map_with/index.md)
- [`Map`](map/index.md)
- [`MatchIndices`](../str/index.md)
- [`Matches`](../str/index.md)
- [`MaxLen`](len/index.md)
- [`MinLen`](len/index.md)
- [`MultiZip`](multizip/index.md)
- [`Once`](once/index.md)
- [`PanicFuse`](panic_fuse/index.md)
- [`Positions`](positions/index.md)
- [`RChunksExactMut`](../slice/rchunks/index.md)
- [`RChunksExact`](../slice/rchunks/index.md)
- [`RChunksMut`](../slice/rchunks/index.md)
- [`RChunks`](../slice/rchunks/index.md)
- [`RepeatN`](repeat/index.md)
- [`Repeat`](repeat/index.md)
- [`Rev`](rev/index.md)
- [`SkipAnyWhile`](skip_any_while/index.md)
- [`SkipAny`](skip_any/index.md)
- [`Skip`](skip/index.md)
- [`SplitAsciiWhitespace`](../str/index.md)
- [`SplitInclusiveMut`](../slice/index.md)
- [`SplitInclusive`](../slice/index.md)
- [`SplitInclusive`](../str/index.md)
- [`SplitMut`](../slice/index.md)
- [`SplitTerminator`](../str/index.md)
- [`SplitWhitespace`](../str/index.md)
- [`Split`](../slice/index.md)
- [`Split`](../str/index.md)
- [`Split`](splitter/index.md)
- [`StepBy`](step_by/index.md)
- [`TakeAnyWhile`](take_any_while/index.md)
- [`TakeAny`](take_any/index.md)
- [`Take`](take/index.md)
- [`TryFoldWith`](try_fold/index.md)
- [`TryFold`](try_fold/index.md)
- [`UniformBlocks`](blocks/index.md)
- [`UnzipA`](unzip/index.md)
- [`UnzipB`](unzip/index.md)
- [`Update`](update/index.md)
- [`WalkTreePostfix`](walk_tree/index.md)
- [`WalkTreePrefix`](walk_tree/index.md)
- [`WalkTree`](walk_tree/index.md)
- [`WhileSome`](while_some/index.md)
- [`Windows`](../slice/index.md)
- [`ZipEq`](zip_eq/index.md)
- [`Zip`](zip/index.md)

### `IndexedParallelIterator`

```rust
trait IndexedParallelIterator: ParallelIterator { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:2439-3244`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L2439-L3244)*

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

- [`Chain`](chain/index.md)
- [`ChunksExactMut`](../slice/chunks/index.md)
- [`ChunksExact`](../slice/chunks/index.md)
- [`ChunksMut`](../slice/chunks/index.md)
- [`Chunks`](../slice/chunks/index.md)
- [`Chunks`](chunks/index.md)
- [`Cloned`](cloned/index.md)
- [`Copied`](copied/index.md)
- [`Drain`](../collections/binary_heap/index.md)
- [`Drain`](../collections/vec_deque/index.md)
- [`Drain`](../vec/index.md)
- [`Either`](#either)
- [`Empty`](empty/index.md)
- [`Enumerate`](enumerate/index.md)
- [`FoldChunksWith`](fold_chunks_with/index.md)
- [`FoldChunks`](fold_chunks/index.md)
- [`Inspect`](inspect/index.md)
- [`InterleaveShortest`](interleave_shortest/index.md)
- [`Interleave`](interleave/index.md)
- [`Intersperse`](intersperse/index.md)
- [`IntoIter`](../array/index.md)
- [`IntoIter`](../collections/binary_heap/index.md)
- [`IntoIter`](../collections/vec_deque/index.md)
- [`IntoIter`](../option/index.md)
- [`IntoIter`](../result/index.md)
- [`IntoIter`](../vec/index.md)
- [`IterMut`](../collections/vec_deque/index.md)
- [`IterMut`](../option/index.md)
- [`IterMut`](../result/index.md)
- [`IterMut`](../slice/index.md)
- [`Iter`](../collections/binary_heap/index.md)
- [`Iter`](../collections/vec_deque/index.md)
- [`Iter`](../option/index.md)
- [`Iter`](../range/index.md)
- [`Iter`](../range_inclusive/index.md)
- [`Iter`](../result/index.md)
- [`Iter`](../slice/index.md)
- [`MapInit`](map_with/index.md)
- [`MapWith`](map_with/index.md)
- [`Map`](map/index.md)
- [`MaxLen`](len/index.md)
- [`MinLen`](len/index.md)
- [`MultiZip`](multizip/index.md)
- [`Once`](once/index.md)
- [`PanicFuse`](panic_fuse/index.md)
- [`RChunksExactMut`](../slice/rchunks/index.md)
- [`RChunksExact`](../slice/rchunks/index.md)
- [`RChunksMut`](../slice/rchunks/index.md)
- [`RChunks`](../slice/rchunks/index.md)
- [`RepeatN`](repeat/index.md)
- [`Rev`](rev/index.md)
- [`Skip`](skip/index.md)
- [`StepBy`](step_by/index.md)
- [`Take`](take/index.md)
- [`Update`](update/index.md)
- [`Windows`](../slice/index.md)
- [`ZipEq`](zip_eq/index.md)
- [`Zip`](zip/index.md)

### `FromParallelIterator<T>`

```rust
trait FromParallelIterator<T>
where
    T: Send { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3280-3303`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L3280-L3303)*

`FromParallelIterator` implements the creation of a collection
from a [`ParallelIterator`](#paralleliterator). By implementing
`FromParallelIterator` for a given type, you define how it will be
created from an iterator.

`FromParallelIterator` is used through [`ParallelIterator`](#paralleliterator)'s `collect()` method.

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

### `ParallelExtend<T>`

```rust
trait ParallelExtend<T>
where
    T: Send { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3333-3353`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L3333-L3353)*

`ParallelExtend` extends an existing collection with items from a [`ParallelIterator`](#paralleliterator).

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

- [`Collector`](unzip/index.md)
- [`Either`](#either)
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

### `ParallelDrainFull`

```rust
trait ParallelDrainFull { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3360-3394`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L3360-L3394)*

`ParallelDrainFull` creates a parallel iterator that moves all items
from a collection while retaining the original capacity.

Types which are indexable typically implement [`ParallelDrainRange`](#paralleldrainrange)
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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3400-3467`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L3400-L3467)*

`ParallelDrainRange` creates a parallel iterator that moves a range of items
from a collection while retaining the original capacity.

Types which are not indexable may implement [`ParallelDrainFull`](#paralleldrainfull) instead.

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

## Functions

*Defined in [`rayon-1.11.0/src/iter/mod.rs:168`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L168)*

*Defined in [`rayon-1.11.0/src/iter/mod.rs:187`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L187)*

*Defined in [`rayon-1.11.0/src/iter/mod.rs:191`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L191)*

*Defined in [`rayon-1.11.0/src/iter/mod.rs:191`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L191)*

*Defined in [`rayon-1.11.0/src/iter/mod.rs:196`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L196)*

*Defined in [`rayon-1.11.0/src/iter/mod.rs:204`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L204)*

*Defined in [`rayon-1.11.0/src/iter/mod.rs:204`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L204)*

*Defined in [`rayon-1.11.0/src/iter/mod.rs:204`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L204)*

*Defined in [`rayon-1.11.0/src/iter/mod.rs:212`](../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L212)*

