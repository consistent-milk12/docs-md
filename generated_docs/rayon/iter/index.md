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
  - [`filter_map`](#filter-map)
  - [`find`](#find)
  - [`find_first_last`](#find-first-last)
  - [`flat_map`](#flat-map)
  - [`flat_map_iter`](#flat-map-iter)
  - [`flatten`](#flatten)
  - [`flatten_iter`](#flatten-iter)
  - [`fold`](#fold)
  - [`fold_chunks`](#fold-chunks)
  - [`fold_chunks_with`](#fold-chunks-with)
  - [`for_each`](#for-each)
  - [`from_par_iter`](#from-par-iter)
  - [`inspect`](#inspect)
  - [`interleave`](#interleave)
  - [`interleave_shortest`](#interleave-shortest)
  - [`intersperse`](#intersperse)
  - [`len`](#len)
  - [`map`](#map)
  - [`map_with`](#map-with)
  - [`multizip`](#multizip)
  - [`noop`](#noop)
  - [`once`](#once)
  - [`panic_fuse`](#panic-fuse)
  - [`par_bridge`](#par-bridge)
  - [`positions`](#positions)
  - [`product`](#product)
  - [`reduce`](#reduce)
  - [`repeat`](#repeat)
  - [`rev`](#rev)
  - [`skip`](#skip)
  - [`skip_any`](#skip-any)
  - [`skip_any_while`](#skip-any-while)
  - [`splitter`](#splitter)
  - [`step_by`](#step-by)
  - [`sum`](#sum)
  - [`take`](#take)
  - [`take_any`](#take-any)
  - [`take_any_while`](#take-any-while)
  - [`try_fold`](#try-fold)
  - [`try_reduce`](#try-reduce)
  - [`try_reduce_with`](#try-reduce-with)
  - [`unzip`](#unzip)
  - [`update`](#update)
  - [`walk_tree`](#walk-tree)
  - [`while_some`](#while-some)
  - [`zip`](#zip)
  - [`zip_eq`](#zip-eq)
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
  - [`repeat_n`](#repeat-n)
  - [`split`](#split)
  - [`walk_tree`](#walk-tree)
  - [`walk_tree_postfix`](#walk-tree-postfix)
  - [`walk_tree_prefix`](#walk-tree-prefix)
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
| [`filter_map`](#filter-map) | mod |  |
| [`find`](#find) | mod |  |
| [`find_first_last`](#find-first-last) | mod |  |
| [`flat_map`](#flat-map) | mod |  |
| [`flat_map_iter`](#flat-map-iter) | mod |  |
| [`flatten`](#flatten) | mod |  |
| [`flatten_iter`](#flatten-iter) | mod |  |
| [`fold`](#fold) | mod |  |
| [`fold_chunks`](#fold-chunks) | mod |  |
| [`fold_chunks_with`](#fold-chunks-with) | mod |  |
| [`for_each`](#for-each) | mod |  |
| [`from_par_iter`](#from-par-iter) | mod |  |
| [`inspect`](#inspect) | mod |  |
| [`interleave`](#interleave) | mod |  |
| [`interleave_shortest`](#interleave-shortest) | mod |  |
| [`intersperse`](#intersperse) | mod |  |
| [`len`](#len) | mod |  |
| [`map`](#map) | mod |  |
| [`map_with`](#map-with) | mod |  |
| [`multizip`](#multizip) | mod |  |
| [`noop`](#noop) | mod |  |
| [`once`](#once) | mod |  |
| [`panic_fuse`](#panic-fuse) | mod |  |
| [`par_bridge`](#par-bridge) | mod |  |
| [`positions`](#positions) | mod |  |
| [`product`](#product) | mod |  |
| [`reduce`](#reduce) | mod |  |
| [`repeat`](#repeat) | mod |  |
| [`rev`](#rev) | mod |  |
| [`skip`](#skip) | mod |  |
| [`skip_any`](#skip-any) | mod |  |
| [`skip_any_while`](#skip-any-while) | mod |  |
| [`splitter`](#splitter) | mod |  |
| [`step_by`](#step-by) | mod |  |
| [`sum`](#sum) | mod |  |
| [`take`](#take) | mod |  |
| [`take_any`](#take-any) | mod |  |
| [`take_any_while`](#take-any-while) | mod |  |
| [`try_fold`](#try-fold) | mod |  |
| [`try_reduce`](#try-reduce) | mod |  |
| [`try_reduce_with`](#try-reduce-with) | mod |  |
| [`unzip`](#unzip) | mod |  |
| [`update`](#update) | mod |  |
| [`walk_tree`](#walk-tree) | mod |  |
| [`while_some`](#while-some) | mod |  |
| [`zip`](#zip) | mod |  |
| [`zip_eq`](#zip-eq) | mod |  |
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
| [`repeat_n`](#repeat-n) | fn |  |
| [`split`](#split) | fn |  |
| [`walk_tree`](#walk-tree) | fn |  |
| [`walk_tree_postfix`](#walk-tree-postfix) | fn |  |
| [`walk_tree_prefix`](#walk-tree-prefix) | fn |  |
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

*Defined in [`rayon-1.11.0/src/iter/blocks.rs:59-61`](../../../.source_1765521767/rayon-1.11.0/src/iter/blocks.rs#L59-L61)*

`ExponentialBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of increasing sizes (exponentially).

This struct is created by the `by_exponential_blocks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="exponentialblocks-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl Any for ExponentialBlocks<I>`

- <span id="exponentialblocks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExponentialBlocks<I>`

- <span id="exponentialblocks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExponentialBlocks<I>`

- <span id="exponentialblocks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for ExponentialBlocks<I>`

- <span id="exponentialblocks-clone"></span>`fn clone(&self) -> ExponentialBlocks<I>` — [`ExponentialBlocks`](blocks/index.md#exponentialblocks)

##### `impl CloneToUninit for ExponentialBlocks<I>`

- <span id="exponentialblocks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for ExponentialBlocks<I>`

- <span id="exponentialblocks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ExponentialBlocks<I>`

- <span id="exponentialblocks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ExponentialBlocks<I>`

- <span id="exponentialblocks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ExponentialBlocks<I>`

##### `impl IntoParallelIterator for ExponentialBlocks<I>`

- <span id="exponentialblocks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="exponentialblocks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="exponentialblocks-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for ExponentialBlocks<I>`

- <span id="exponentialblocks-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="exponentialblocks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for ExponentialBlocks<I>`

- <span id="exponentialblocks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="exponentialblocks-pointable-type-init"></span>`type Init = T`

- <span id="exponentialblocks-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="exponentialblocks-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="exponentialblocks-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="exponentialblocks-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for ExponentialBlocks<I>`

- <span id="exponentialblocks-toowned-type-owned"></span>`type Owned = T`

- <span id="exponentialblocks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exponentialblocks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ExponentialBlocks<I>`

- <span id="exponentialblocks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exponentialblocks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExponentialBlocks<I>`

- <span id="exponentialblocks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exponentialblocks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UniformBlocks<I>`

```rust
struct UniformBlocks<I> {
    base: I,
    block_size: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/blocks.rs:101-104`](../../../.source_1765521767/rayon-1.11.0/src/iter/blocks.rs#L101-L104)*

`UniformBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of constant sizes.

This struct is created by the `by_uniform_blocks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="uniformblocks-new"></span>`fn new(base: I, block_size: usize) -> Self`

#### Trait Implementations

##### `impl Any for UniformBlocks<I>`

- <span id="uniformblocks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UniformBlocks<I>`

- <span id="uniformblocks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UniformBlocks<I>`

- <span id="uniformblocks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for UniformBlocks<I>`

- <span id="uniformblocks-clone"></span>`fn clone(&self) -> UniformBlocks<I>` — [`UniformBlocks`](blocks/index.md#uniformblocks)

##### `impl CloneToUninit for UniformBlocks<I>`

- <span id="uniformblocks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for UniformBlocks<I>`

- <span id="uniformblocks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UniformBlocks<I>`

- <span id="uniformblocks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UniformBlocks<I>`

- <span id="uniformblocks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UniformBlocks<I>`

##### `impl IntoParallelIterator for UniformBlocks<I>`

- <span id="uniformblocks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="uniformblocks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="uniformblocks-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for UniformBlocks<I>`

- <span id="uniformblocks-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="uniformblocks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for UniformBlocks<I>`

- <span id="uniformblocks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="uniformblocks-pointable-type-init"></span>`type Init = T`

- <span id="uniformblocks-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="uniformblocks-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="uniformblocks-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="uniformblocks-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for UniformBlocks<I>`

- <span id="uniformblocks-toowned-type-owned"></span>`type Owned = T`

- <span id="uniformblocks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="uniformblocks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UniformBlocks<I>`

- <span id="uniformblocks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uniformblocks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UniformBlocks<I>`

- <span id="uniformblocks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uniformblocks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Chain<A, B>`

```rust
struct Chain<A, B> {
    a: A,
    b: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/chain.rs:12-15`](../../../.source_1765521767/rayon-1.11.0/src/iter/chain.rs#L12-L15)*

`Chain` is an iterator that joins `b` after `a` in one continuous iterator.
This struct is created by the `chain()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="chain-new"></span>`fn new(a: A, b: B) -> Self`

  Creates a new `Chain` iterator.

#### Trait Implementations

##### `impl Any for Chain<A, B>`

- <span id="chain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Chain<A, B>`

- <span id="chain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Chain<A, B>`

- <span id="chain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: clone::Clone, B: clone::Clone> Clone for Chain<A, B>`

- <span id="chain-clone"></span>`fn clone(&self) -> Chain<A, B>` — [`Chain`](chain/index.md#chain)

##### `impl CloneToUninit for Chain<A, B>`

- <span id="chain-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Chain<A, B>`

- <span id="chain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Chain<A, B>`

- <span id="chain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A, B> IndexedParallelIterator for Chain<A, B>`

- <span id="chain-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="chain-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chain-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Chain<A, B>`

- <span id="chain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Chain<A, B>`

##### `impl IntoParallelIterator for Chain<A, B>`

- <span id="chain-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chain-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chain-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for Chain<A, B>`

- <span id="chain-paralleliterator-type-item"></span>`type Item = <A as ParallelIterator>::Item`

- <span id="chain-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="chain-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Chain<A, B>`

- <span id="chain-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chain-pointable-type-init"></span>`type Init = T`

- <span id="chain-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chain-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chain-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chain-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Chain<A, B>`

- <span id="chain-toowned-type-owned"></span>`type Owned = T`

- <span id="chain-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chain-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Chain<A, B>`

- <span id="chain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Chain<A, B>`

- <span id="chain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Chunks<I>`

```rust
struct Chunks<I> {
    size: usize,
    i: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/chunks.rs:11-14`](../../../.source_1765521767/rayon-1.11.0/src/iter/chunks.rs#L11-L14)*

`Chunks` is an iterator that groups elements of an underlying iterator.

This struct is created by the `chunks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="chunks-new"></span>`fn new(i: I, size: usize) -> Self`

  Creates a new `Chunks` iterator

#### Trait Implementations

##### `impl Any for Chunks<I>`

- <span id="chunks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Chunks<I>`

- <span id="chunks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Chunks<I>`

- <span id="chunks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Chunks<I>`

- <span id="chunks-clone"></span>`fn clone(&self) -> Chunks<I>` — [`Chunks`](chunks/index.md#chunks)

##### `impl CloneToUninit for Chunks<I>`

- <span id="chunks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Chunks<I>`

- <span id="chunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Chunks<I>`

- <span id="chunks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Chunks<I>`

- <span id="chunks-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="chunks-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chunks-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Chunks<I>`

- <span id="chunks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Chunks<I>`

##### `impl IntoParallelIterator for Chunks<I>`

- <span id="chunks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunks-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Chunks<I>`

- <span id="chunks-paralleliterator-type-item"></span>`type Item = Vec<<I as ParallelIterator>::Item>`

- <span id="chunks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="chunks-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Chunks<I>`

- <span id="chunks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunks-pointable-type-init"></span>`type Init = T`

- <span id="chunks-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunks-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunks-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunks-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Chunks<I>`

- <span id="chunks-toowned-type-owned"></span>`type Owned = T`

- <span id="chunks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chunks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Chunks<I>`

- <span id="chunks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Chunks<I>`

- <span id="chunks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Cloned<I>`

```rust
struct Cloned<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/cloned.rs:13-15`](../../../.source_1765521767/rayon-1.11.0/src/iter/cloned.rs#L13-L15)*

`Cloned` is an iterator that clones the elements of an underlying iterator.

This struct is created by the `cloned()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="cloned-new"></span>`fn new(base: I) -> Self`

  Creates a new `Cloned` iterator.

#### Trait Implementations

##### `impl Any for Cloned<I>`

- <span id="cloned-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cloned<I>`

- <span id="cloned-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cloned<I>`

- <span id="cloned-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Cloned<I>`

- <span id="cloned-clone"></span>`fn clone(&self) -> Cloned<I>` — [`Cloned`](cloned/index.md#cloned)

##### `impl CloneToUninit for Cloned<I>`

- <span id="cloned-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Cloned<I>`

- <span id="cloned-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Cloned<I>`

- <span id="cloned-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Cloned<I>`

- <span id="cloned-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="cloned-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="cloned-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Cloned<I>`

- <span id="cloned-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Cloned<I>`

##### `impl IntoParallelIterator for Cloned<I>`

- <span id="cloned-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="cloned-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="cloned-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Cloned<I>`

- <span id="cloned-paralleliterator-type-item"></span>`type Item = T`

- <span id="cloned-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="cloned-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Cloned<I>`

- <span id="cloned-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cloned-pointable-type-init"></span>`type Init = T`

- <span id="cloned-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cloned-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cloned-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cloned-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Cloned<I>`

- <span id="cloned-toowned-type-owned"></span>`type Owned = T`

- <span id="cloned-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cloned-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Cloned<I>`

- <span id="cloned-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cloned-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cloned<I>`

- <span id="cloned-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cloned-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Copied<I>`

```rust
struct Copied<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/copied.rs:13-15`](../../../.source_1765521767/rayon-1.11.0/src/iter/copied.rs#L13-L15)*

`Copied` is an iterator that copies the elements of an underlying iterator.

This struct is created by the `copied()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="copied-new"></span>`fn new(base: I) -> Self`

  Creates a new `Copied` iterator.

#### Trait Implementations

##### `impl Any for Copied<I>`

- <span id="copied-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Copied<I>`

- <span id="copied-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Copied<I>`

- <span id="copied-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Copied<I>`

- <span id="copied-clone"></span>`fn clone(&self) -> Copied<I>` — [`Copied`](copied/index.md#copied)

##### `impl CloneToUninit for Copied<I>`

- <span id="copied-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Copied<I>`

- <span id="copied-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Copied<I>`

- <span id="copied-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Copied<I>`

- <span id="copied-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="copied-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="copied-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Copied<I>`

- <span id="copied-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Copied<I>`

##### `impl IntoParallelIterator for Copied<I>`

- <span id="copied-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="copied-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="copied-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Copied<I>`

- <span id="copied-paralleliterator-type-item"></span>`type Item = T`

- <span id="copied-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="copied-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Copied<I>`

- <span id="copied-pointable-const-align"></span>`const ALIGN: usize`

- <span id="copied-pointable-type-init"></span>`type Init = T`

- <span id="copied-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="copied-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="copied-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="copied-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Copied<I>`

- <span id="copied-toowned-type-owned"></span>`type Owned = T`

- <span id="copied-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="copied-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Copied<I>`

- <span id="copied-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="copied-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Copied<I>`

- <span id="copied-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="copied-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Empty<T>`

```rust
struct Empty<T> {
    marker: std::marker::PhantomData<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/empty.rs:33-35`](../../../.source_1765521767/rayon-1.11.0/src/iter/empty.rs#L33-L35)*

Iterator adaptor for [the `empty()` function].


#### Trait Implementations

##### `impl<T> Any for Empty<T>`

- <span id="empty-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Empty<T>`

- <span id="empty-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Empty<T>`

- <span id="empty-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Empty<T>`

- <span id="empty-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Empty<T>`

- <span id="empty-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: Send> Debug for Empty<T>`

- <span id="empty-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Empty<T>`

- <span id="empty-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for Empty<T>`

- <span id="empty-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="empty-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="empty-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<T, U> Into for Empty<T>`

- <span id="empty-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Empty<T>`

##### `impl<T> IntoParallelIterator for Empty<T>`

- <span id="empty-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="empty-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="empty-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Empty<T>`

- <span id="empty-paralleliterator-type-item"></span>`type Item = T`

- <span id="empty-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="empty-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Empty<T>`

- <span id="empty-pointable-const-align"></span>`const ALIGN: usize`

- <span id="empty-pointable-type-init"></span>`type Init = T`

- <span id="empty-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="empty-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="empty-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="empty-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Empty<T>`

- <span id="empty-toowned-type-owned"></span>`type Owned = T`

- <span id="empty-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="empty-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Empty<T>`

- <span id="empty-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="empty-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Empty<T>`

- <span id="empty-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="empty-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Enumerate<I>`

```rust
struct Enumerate<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/enumerate.rs:12-14`](../../../.source_1765521767/rayon-1.11.0/src/iter/enumerate.rs#L12-L14)*

`Enumerate` is an iterator that returns the current count along with the element.
This struct is created by the `enumerate()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="enumerate-new"></span>`fn new(base: I) -> Self`

  Creates a new `Enumerate` iterator.

#### Trait Implementations

##### `impl Any for Enumerate<I>`

- <span id="enumerate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Enumerate<I>`

- <span id="enumerate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Enumerate<I>`

- <span id="enumerate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Enumerate<I>`

- <span id="enumerate-clone"></span>`fn clone(&self) -> Enumerate<I>` — [`Enumerate`](enumerate/index.md#enumerate)

##### `impl CloneToUninit for Enumerate<I>`

- <span id="enumerate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Enumerate<I>`

- <span id="enumerate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Enumerate<I>`

- <span id="enumerate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Enumerate<I>`

- <span id="enumerate-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="enumerate-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="enumerate-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Enumerate<I>`

- <span id="enumerate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Enumerate<I>`

##### `impl IntoParallelIterator for Enumerate<I>`

- <span id="enumerate-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="enumerate-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="enumerate-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Enumerate<I>`

- <span id="enumerate-paralleliterator-type-item"></span>`type Item = (usize, <I as ParallelIterator>::Item)`

- <span id="enumerate-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="enumerate-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Enumerate<I>`

- <span id="enumerate-pointable-const-align"></span>`const ALIGN: usize`

- <span id="enumerate-pointable-type-init"></span>`type Init = T`

- <span id="enumerate-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="enumerate-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="enumerate-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="enumerate-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Enumerate<I>`

- <span id="enumerate-toowned-type-owned"></span>`type Owned = T`

- <span id="enumerate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="enumerate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Enumerate<I>`

- <span id="enumerate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="enumerate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Enumerate<I>`

- <span id="enumerate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="enumerate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Filter<I, P>`

```rust
struct Filter<I, P> {
    base: I,
    filter_op: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter.rs:12-15`](../../../.source_1765521767/rayon-1.11.0/src/iter/filter.rs#L12-L15)*

`Filter` takes a predicate `filter_op` and filters out elements that match.
This struct is created by the `filter()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="filter-new"></span>`fn new(base: I, filter_op: P) -> Self`

  Creates a new `Filter` iterator.

#### Trait Implementations

##### `impl Any for Filter<I, P>`

- <span id="filter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Filter<I, P>`

- <span id="filter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Filter<I, P>`

- <span id="filter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, P: clone::Clone> Clone for Filter<I, P>`

- <span id="filter-clone"></span>`fn clone(&self) -> Filter<I, P>` — [`Filter`](filter/index.md#filter)

##### `impl CloneToUninit for Filter<I, P>`

- <span id="filter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, P> Debug for Filter<I, P>`

- <span id="filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Filter<I, P>`

- <span id="filter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Filter<I, P>`

- <span id="filter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Filter<I, P>`

##### `impl IntoParallelIterator for Filter<I, P>`

- <span id="filter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="filter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="filter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for Filter<I, P>`

- <span id="filter-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="filter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for Filter<I, P>`

- <span id="filter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filter-pointable-type-init"></span>`type Init = T`

- <span id="filter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Filter<I, P>`

- <span id="filter-toowned-type-owned"></span>`type Owned = T`

- <span id="filter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="filter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Filter<I, P>`

- <span id="filter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Filter<I, P>`

- <span id="filter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FilterMap<I, P>`

```rust
struct FilterMap<I, P> {
    base: I,
    filter_op: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter_map.rs:12-15`](../../../.source_1765521767/rayon-1.11.0/src/iter/filter_map.rs#L12-L15)*

`FilterMap` creates an iterator that uses `filter_op` to both filter and map elements.
This struct is created by the `filter_map()` method on [`ParallelIterator`](#paralleliterator).


#### Implementations

- <span id="filtermap-new"></span>`fn new(base: I, filter_op: P) -> Self`

  Creates a new `FilterMap` iterator.

#### Trait Implementations

##### `impl Any for FilterMap<I, P>`

- <span id="filtermap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FilterMap<I, P>`

- <span id="filtermap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FilterMap<I, P>`

- <span id="filtermap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, P: clone::Clone> Clone for FilterMap<I, P>`

- <span id="filtermap-clone"></span>`fn clone(&self) -> FilterMap<I, P>` — [`FilterMap`](filter_map/index.md#filtermap)

##### `impl CloneToUninit for FilterMap<I, P>`

- <span id="filtermap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, P> Debug for FilterMap<I, P>`

- <span id="filtermap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FilterMap<I, P>`

- <span id="filtermap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FilterMap<I, P>`

- <span id="filtermap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FilterMap<I, P>`

##### `impl IntoParallelIterator for FilterMap<I, P>`

- <span id="filtermap-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="filtermap-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="filtermap-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for FilterMap<I, P>`

- <span id="filtermap-paralleliterator-type-item"></span>`type Item = R`

- <span id="filtermap-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for FilterMap<I, P>`

- <span id="filtermap-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filtermap-pointable-type-init"></span>`type Init = T`

- <span id="filtermap-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filtermap-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filtermap-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filtermap-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FilterMap<I, P>`

- <span id="filtermap-toowned-type-owned"></span>`type Owned = T`

- <span id="filtermap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="filtermap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FilterMap<I, P>`

- <span id="filtermap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filtermap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FilterMap<I, P>`

- <span id="filtermap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filtermap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FlatMap<I, F>`

```rust
struct FlatMap<I, F> {
    base: I,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map.rs:12-15`](../../../.source_1765521767/rayon-1.11.0/src/iter/flat_map.rs#L12-L15)*

`FlatMap` maps each element to a parallel iterator, then flattens these iterators together.
This struct is created by the `flat_map()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="flatmap-new"></span>`fn new(base: I, map_op: F) -> Self`

  Creates a new `FlatMap` iterator.

#### Trait Implementations

##### `impl Any for FlatMap<I, F>`

- <span id="flatmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlatMap<I, F>`

- <span id="flatmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlatMap<I, F>`

- <span id="flatmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for FlatMap<I, F>`

- <span id="flatmap-clone"></span>`fn clone(&self) -> FlatMap<I, F>` — [`FlatMap`](flat_map/index.md#flatmap)

##### `impl CloneToUninit for FlatMap<I, F>`

- <span id="flatmap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, F> Debug for FlatMap<I, F>`

- <span id="flatmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FlatMap<I, F>`

- <span id="flatmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlatMap<I, F>`

- <span id="flatmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlatMap<I, F>`

##### `impl IntoParallelIterator for FlatMap<I, F>`

- <span id="flatmap-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="flatmap-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatmap-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for FlatMap<I, F>`

- <span id="flatmap-paralleliterator-type-item"></span>`type Item = <PI as IntoParallelIterator>::Item`

- <span id="flatmap-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for FlatMap<I, F>`

- <span id="flatmap-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatmap-pointable-type-init"></span>`type Init = T`

- <span id="flatmap-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmap-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmap-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmap-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FlatMap<I, F>`

- <span id="flatmap-toowned-type-owned"></span>`type Owned = T`

- <span id="flatmap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flatmap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FlatMap<I, F>`

- <span id="flatmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlatMap<I, F>`

- <span id="flatmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FlatMapIter<I, F>`

```rust
struct FlatMapIter<I, F> {
    base: I,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map_iter.rs:12-15`](../../../.source_1765521767/rayon-1.11.0/src/iter/flat_map_iter.rs#L12-L15)*

`FlatMapIter` maps each element to a serial iterator, then flattens these iterators together.
This struct is created by the `flat_map_iter()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="flatmapiter-new"></span>`fn new(base: I, map_op: F) -> Self`

  Creates a new `FlatMapIter` iterator.

#### Trait Implementations

##### `impl Any for FlatMapIter<I, F>`

- <span id="flatmapiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlatMapIter<I, F>`

- <span id="flatmapiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlatMapIter<I, F>`

- <span id="flatmapiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for FlatMapIter<I, F>`

- <span id="flatmapiter-clone"></span>`fn clone(&self) -> FlatMapIter<I, F>` — [`FlatMapIter`](flat_map_iter/index.md#flatmapiter)

##### `impl CloneToUninit for FlatMapIter<I, F>`

- <span id="flatmapiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, F> Debug for FlatMapIter<I, F>`

- <span id="flatmapiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FlatMapIter<I, F>`

- <span id="flatmapiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlatMapIter<I, F>`

- <span id="flatmapiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlatMapIter<I, F>`

##### `impl IntoParallelIterator for FlatMapIter<I, F>`

- <span id="flatmapiter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="flatmapiter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatmapiter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for FlatMapIter<I, F>`

- <span id="flatmapiter-paralleliterator-type-item"></span>`type Item = <SI as IntoIterator>::Item`

- <span id="flatmapiter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for FlatMapIter<I, F>`

- <span id="flatmapiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatmapiter-pointable-type-init"></span>`type Init = T`

- <span id="flatmapiter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapiter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapiter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapiter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FlatMapIter<I, F>`

- <span id="flatmapiter-toowned-type-owned"></span>`type Owned = T`

- <span id="flatmapiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flatmapiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FlatMapIter<I, F>`

- <span id="flatmapiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatmapiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlatMapIter<I, F>`

- <span id="flatmapiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatmapiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Flatten<I>`

```rust
struct Flatten<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten.rs:10-12`](../../../.source_1765521767/rayon-1.11.0/src/iter/flatten.rs#L10-L12)*

`Flatten` turns each element to a parallel iterator, then flattens these iterators
together. This struct is created by the `flatten()` method on [`ParallelIterator`](#paralleliterator).


#### Implementations

- <span id="flatten-new"></span>`fn new(base: I) -> Self`

  Creates a new `Flatten` iterator.

#### Trait Implementations

##### `impl Any for Flatten<I>`

- <span id="flatten-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Flatten<I>`

- <span id="flatten-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Flatten<I>`

- <span id="flatten-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Flatten<I>`

- <span id="flatten-clone"></span>`fn clone(&self) -> Flatten<I>` — [`Flatten`](flatten/index.md#flatten)

##### `impl CloneToUninit for Flatten<I>`

- <span id="flatten-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Flatten<I>`

- <span id="flatten-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Flatten<I>`

- <span id="flatten-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Flatten<I>`

- <span id="flatten-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Flatten<I>`

##### `impl IntoParallelIterator for Flatten<I>`

- <span id="flatten-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="flatten-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatten-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Flatten<I>`

- <span id="flatten-paralleliterator-type-item"></span>`type Item = <<I as ParallelIterator>::Item as IntoParallelIterator>::Item`

- <span id="flatten-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for Flatten<I>`

- <span id="flatten-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatten-pointable-type-init"></span>`type Init = T`

- <span id="flatten-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatten-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatten-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatten-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Flatten<I>`

- <span id="flatten-toowned-type-owned"></span>`type Owned = T`

- <span id="flatten-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flatten-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Flatten<I>`

- <span id="flatten-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatten-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Flatten<I>`

- <span id="flatten-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatten-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FlattenIter<I>`

```rust
struct FlattenIter<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten_iter.rs:10-12`](../../../.source_1765521767/rayon-1.11.0/src/iter/flatten_iter.rs#L10-L12)*

`FlattenIter` turns each element to a serial iterator, then flattens these iterators
together. This struct is created by the `flatten_iter()` method on [`ParallelIterator`](#paralleliterator).


#### Implementations

- <span id="flatteniter-new"></span>`fn new(base: I) -> Self`

  Creates a new `FlattenIter` iterator.

#### Trait Implementations

##### `impl Any for FlattenIter<I>`

- <span id="flatteniter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlattenIter<I>`

- <span id="flatteniter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlattenIter<I>`

- <span id="flatteniter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for FlattenIter<I>`

- <span id="flatteniter-clone"></span>`fn clone(&self) -> FlattenIter<I>` — [`FlattenIter`](flatten_iter/index.md#flatteniter)

##### `impl CloneToUninit for FlattenIter<I>`

- <span id="flatteniter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for FlattenIter<I>`

- <span id="flatteniter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FlattenIter<I>`

- <span id="flatteniter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlattenIter<I>`

- <span id="flatteniter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlattenIter<I>`

##### `impl IntoParallelIterator for FlattenIter<I>`

- <span id="flatteniter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="flatteniter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatteniter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for FlattenIter<I>`

- <span id="flatteniter-paralleliterator-type-item"></span>`type Item = <<I as ParallelIterator>::Item as IntoIterator>::Item`

- <span id="flatteniter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for FlattenIter<I>`

- <span id="flatteniter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatteniter-pointable-type-init"></span>`type Init = T`

- <span id="flatteniter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatteniter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatteniter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatteniter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FlattenIter<I>`

- <span id="flatteniter-toowned-type-owned"></span>`type Owned = T`

- <span id="flatteniter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flatteniter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FlattenIter<I>`

- <span id="flatteniter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatteniter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlattenIter<I>`

- <span id="flatteniter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatteniter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Fold<I, ID, F>`

```rust
struct Fold<I, ID, F> {
    base: I,
    identity: ID,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold.rs:22-26`](../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L22-L26)*

`Fold` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="fold-new"></span>`fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl Any for Fold<I, ID, F>`

- <span id="fold-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Fold<I, ID, F>`

- <span id="fold-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fold<I, ID, F>`

- <span id="fold-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for Fold<I, ID, F>`

- <span id="fold-clone"></span>`fn clone(&self) -> Fold<I, ID, F>` — [`Fold`](fold/index.md#fold)

##### `impl CloneToUninit for Fold<I, ID, F>`

- <span id="fold-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, ID, F> Debug for Fold<I, ID, F>`

- <span id="fold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Fold<I, ID, F>`

- <span id="fold-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Fold<I, ID, F>`

- <span id="fold-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Fold<I, ID, F>`

##### `impl IntoParallelIterator for Fold<I, ID, F>`

- <span id="fold-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="fold-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="fold-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, ID, F> ParallelIterator for Fold<I, ID, F>`

- <span id="fold-paralleliterator-type-item"></span>`type Item = U`

- <span id="fold-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for Fold<I, ID, F>`

- <span id="fold-pointable-const-align"></span>`const ALIGN: usize`

- <span id="fold-pointable-type-init"></span>`type Init = T`

- <span id="fold-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fold-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fold-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fold-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Fold<I, ID, F>`

- <span id="fold-toowned-type-owned"></span>`type Owned = T`

- <span id="fold-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fold-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Fold<I, ID, F>`

- <span id="fold-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fold-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fold<I, ID, F>`

- <span id="fold-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fold-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FoldWith<I, U, F>`

```rust
struct FoldWith<I, U, F> {
    base: I,
    item: U,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold.rs:191-195`](../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L191-L195)*

`FoldWith` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold_with()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="foldwith-new"></span>`fn new(base: I, item: U, fold_op: F) -> Self`

#### Trait Implementations

##### `impl Any for FoldWith<I, U, F>`

- <span id="foldwith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FoldWith<I, U, F>`

- <span id="foldwith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FoldWith<I, U, F>`

- <span id="foldwith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, U: clone::Clone, F: clone::Clone> Clone for FoldWith<I, U, F>`

- <span id="foldwith-clone"></span>`fn clone(&self) -> FoldWith<I, U, F>` — [`FoldWith`](fold/index.md#foldwith)

##### `impl CloneToUninit for FoldWith<I, U, F>`

- <span id="foldwith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, U: Debug, F> Debug for FoldWith<I, U, F>`

- <span id="foldwith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FoldWith<I, U, F>`

- <span id="foldwith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FoldWith<I, U, F>`

- <span id="foldwith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FoldWith<I, U, F>`

##### `impl IntoParallelIterator for FoldWith<I, U, F>`

- <span id="foldwith-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="foldwith-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldwith-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, F> ParallelIterator for FoldWith<I, U, F>`

- <span id="foldwith-paralleliterator-type-item"></span>`type Item = U`

- <span id="foldwith-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for FoldWith<I, U, F>`

- <span id="foldwith-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldwith-pointable-type-init"></span>`type Init = T`

- <span id="foldwith-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldwith-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldwith-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldwith-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FoldWith<I, U, F>`

- <span id="foldwith-toowned-type-owned"></span>`type Owned = T`

- <span id="foldwith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foldwith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FoldWith<I, U, F>`

- <span id="foldwith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foldwith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FoldWith<I, U, F>`

- <span id="foldwith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foldwith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FoldChunks<I, ID, F>`

```rust
struct FoldChunks<I, ID, F> {
    base: I,
    chunk_size: usize,
    fold_op: F,
    identity: ID,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold_chunks.rs:15-20`](../../../.source_1765521767/rayon-1.11.0/src/iter/fold_chunks.rs#L15-L20)*

`FoldChunks` is an iterator that groups elements of an underlying iterator and applies a
function over them, producing a single value for each group.

This struct is created by the `fold_chunks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="foldchunks-new"></span>`fn new(base: I, chunk_size: usize, identity: ID, fold_op: F) -> Self`

  Creates a new `FoldChunks` iterator

#### Trait Implementations

##### `impl Any for FoldChunks<I, ID, F>`

- <span id="foldchunks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FoldChunks<I, ID, F>`

- <span id="foldchunks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FoldChunks<I, ID, F>`

- <span id="foldchunks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for FoldChunks<I, ID, F>`

- <span id="foldchunks-clone"></span>`fn clone(&self) -> FoldChunks<I, ID, F>` — [`FoldChunks`](fold_chunks/index.md#foldchunks)

##### `impl CloneToUninit for FoldChunks<I, ID, F>`

- <span id="foldchunks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, ID, F> Debug for FoldChunks<I, ID, F>`

- <span id="foldchunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FoldChunks<I, ID, F>`

- <span id="foldchunks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, ID, F> IndexedParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="foldchunks-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="foldchunks-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for FoldChunks<I, ID, F>`

- <span id="foldchunks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FoldChunks<I, ID, F>`

##### `impl IntoParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="foldchunks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldchunks-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, ID, F> ParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-paralleliterator-type-item"></span>`type Item = U`

- <span id="foldchunks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="foldchunks-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for FoldChunks<I, ID, F>`

- <span id="foldchunks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldchunks-pointable-type-init"></span>`type Init = T`

- <span id="foldchunks-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldchunks-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldchunks-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldchunks-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FoldChunks<I, ID, F>`

- <span id="foldchunks-toowned-type-owned"></span>`type Owned = T`

- <span id="foldchunks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foldchunks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FoldChunks<I, ID, F>`

- <span id="foldchunks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foldchunks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FoldChunks<I, ID, F>`

- <span id="foldchunks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foldchunks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FoldChunksWith<I, U, F>`

```rust
struct FoldChunksWith<I, U, F> {
    base: I,
    chunk_size: usize,
    item: U,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold_chunks_with.rs:15-20`](../../../.source_1765521767/rayon-1.11.0/src/iter/fold_chunks_with.rs#L15-L20)*

`FoldChunksWith` is an iterator that groups elements of an underlying iterator and applies a
function over them, producing a single value for each group.

This struct is created by the `fold_chunks_with()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="foldchunkswith-new"></span>`fn new(base: I, chunk_size: usize, item: U, fold_op: F) -> Self`

  Creates a new `FoldChunksWith` iterator

#### Trait Implementations

##### `impl Any for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, U: clone::Clone, F: clone::Clone> Clone for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-clone"></span>`fn clone(&self) -> FoldChunksWith<I, U, F>` — [`FoldChunksWith`](fold_chunks_with/index.md#foldchunkswith)

##### `impl CloneToUninit for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, U: Debug, F> Debug for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, U, F> IndexedParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="foldchunkswith-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="foldchunkswith-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FoldChunksWith<I, U, F>`

##### `impl IntoParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="foldchunkswith-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldchunkswith-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, U, F> ParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-paralleliterator-type-item"></span>`type Item = U`

- <span id="foldchunkswith-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="foldchunkswith-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldchunkswith-pointable-type-init"></span>`type Init = T`

- <span id="foldchunkswith-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldchunkswith-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldchunkswith-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldchunkswith-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-toowned-type-owned"></span>`type Owned = T`

- <span id="foldchunkswith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foldchunkswith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foldchunkswith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foldchunkswith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Inspect<I, F>`

```rust
struct Inspect<I, F> {
    base: I,
    inspect_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/inspect.rs:15-18`](../../../.source_1765521767/rayon-1.11.0/src/iter/inspect.rs#L15-L18)*

`Inspect` is an iterator that calls a function with a reference to each
element before yielding it.

This struct is created by the `inspect()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="inspect-new"></span>`fn new(base: I, inspect_op: F) -> Self`

  Creates a new `Inspect` iterator.

#### Trait Implementations

##### `impl Any for Inspect<I, F>`

- <span id="inspect-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Inspect<I, F>`

- <span id="inspect-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Inspect<I, F>`

- <span id="inspect-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Inspect<I, F>`

- <span id="inspect-clone"></span>`fn clone(&self) -> Inspect<I, F>` — [`Inspect`](inspect/index.md#inspect)

##### `impl CloneToUninit for Inspect<I, F>`

- <span id="inspect-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, F> Debug for Inspect<I, F>`

- <span id="inspect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Inspect<I, F>`

- <span id="inspect-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> IndexedParallelIterator for Inspect<I, F>`

- <span id="inspect-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="inspect-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="inspect-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Inspect<I, F>`

- <span id="inspect-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Inspect<I, F>`

##### `impl IntoParallelIterator for Inspect<I, F>`

- <span id="inspect-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="inspect-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="inspect-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for Inspect<I, F>`

- <span id="inspect-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="inspect-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="inspect-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Inspect<I, F>`

- <span id="inspect-pointable-const-align"></span>`const ALIGN: usize`

- <span id="inspect-pointable-type-init"></span>`type Init = T`

- <span id="inspect-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inspect-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inspect-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inspect-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Inspect<I, F>`

- <span id="inspect-toowned-type-owned"></span>`type Owned = T`

- <span id="inspect-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="inspect-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Inspect<I, F>`

- <span id="inspect-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inspect-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Inspect<I, F>`

- <span id="inspect-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inspect-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Interleave<I, J>`

```rust
struct Interleave<I, J> {
    i: I,
    j: J,
}
```

*Defined in [`rayon-1.11.0/src/iter/interleave.rs:12-15`](../../../.source_1765521767/rayon-1.11.0/src/iter/interleave.rs#L12-L15)*

`Interleave` is an iterator that interleaves elements of iterators
`i` and `j` in one continuous iterator. This struct is created by
the `interleave()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="interleave-new"></span>`fn new(i: I, j: J) -> Self`

  Creates a new `Interleave` iterator

#### Trait Implementations

##### `impl Any for Interleave<I, J>`

- <span id="interleave-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Interleave<I, J>`

- <span id="interleave-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Interleave<I, J>`

- <span id="interleave-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, J: clone::Clone> Clone for Interleave<I, J>`

- <span id="interleave-clone"></span>`fn clone(&self) -> Interleave<I, J>` — [`Interleave`](interleave/index.md#interleave)

##### `impl CloneToUninit for Interleave<I, J>`

- <span id="interleave-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, J: fmt::Debug> Debug for Interleave<I, J>`

- <span id="interleave-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Interleave<I, J>`

- <span id="interleave-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, J> IndexedParallelIterator for Interleave<I, J>`

- <span id="interleave-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="interleave-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="interleave-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Interleave<I, J>`

- <span id="interleave-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Interleave<I, J>`

##### `impl IntoParallelIterator for Interleave<I, J>`

- <span id="interleave-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="interleave-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="interleave-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, J> ParallelIterator for Interleave<I, J>`

- <span id="interleave-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="interleave-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="interleave-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Interleave<I, J>`

- <span id="interleave-pointable-const-align"></span>`const ALIGN: usize`

- <span id="interleave-pointable-type-init"></span>`type Init = T`

- <span id="interleave-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interleave-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interleave-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interleave-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Interleave<I, J>`

- <span id="interleave-toowned-type-owned"></span>`type Owned = T`

- <span id="interleave-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="interleave-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Interleave<I, J>`

- <span id="interleave-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interleave-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Interleave<I, J>`

- <span id="interleave-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interleave-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `InterleaveShortest<I, J>`

```rust
struct InterleaveShortest<I, J> {
    interleave: Interleave<Take<I>, Take<J>>,
}
```

*Defined in [`rayon-1.11.0/src/iter/interleave_shortest.rs:14-16`](../../../.source_1765521767/rayon-1.11.0/src/iter/interleave_shortest.rs#L14-L16)*

`InterleaveShortest` is an iterator that works similarly to
`Interleave`, but this version stops returning elements once one
of the iterators run out.

This struct is created by the `interleave_shortest()` method on
[`IndexedParallelIterator`](#indexedparalleliterator).


#### Implementations

- <span id="interleaveshortest-new"></span>`fn new(i: I, j: J) -> Self`

  Creates a new `InterleaveShortest` iterator

#### Trait Implementations

##### `impl Any for InterleaveShortest<I, J>`

- <span id="interleaveshortest-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InterleaveShortest<I, J>`

- <span id="interleaveshortest-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InterleaveShortest<I, J>`

- <span id="interleaveshortest-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, J: clone::Clone> Clone for InterleaveShortest<I, J>`

- <span id="interleaveshortest-clone"></span>`fn clone(&self) -> InterleaveShortest<I, J>` — [`InterleaveShortest`](interleave_shortest/index.md#interleaveshortest)

##### `impl CloneToUninit for InterleaveShortest<I, J>`

- <span id="interleaveshortest-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, J: fmt::Debug> Debug for InterleaveShortest<I, J>`

- <span id="interleaveshortest-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for InterleaveShortest<I, J>`

- <span id="interleaveshortest-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, J> IndexedParallelIterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="interleaveshortest-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="interleaveshortest-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for InterleaveShortest<I, J>`

- <span id="interleaveshortest-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for InterleaveShortest<I, J>`

##### `impl IntoParallelIterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="interleaveshortest-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="interleaveshortest-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, J> ParallelIterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="interleaveshortest-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="interleaveshortest-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for InterleaveShortest<I, J>`

- <span id="interleaveshortest-pointable-const-align"></span>`const ALIGN: usize`

- <span id="interleaveshortest-pointable-type-init"></span>`type Init = T`

- <span id="interleaveshortest-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interleaveshortest-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interleaveshortest-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interleaveshortest-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for InterleaveShortest<I, J>`

- <span id="interleaveshortest-toowned-type-owned"></span>`type Owned = T`

- <span id="interleaveshortest-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="interleaveshortest-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for InterleaveShortest<I, J>`

- <span id="interleaveshortest-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interleaveshortest-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InterleaveShortest<I, J>`

- <span id="interleaveshortest-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interleaveshortest-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Intersperse<I>`

```rust
struct Intersperse<I>
where
    I: ParallelIterator {
    base: I,
    item: <I as >::Item,
}
```

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:13-19`](../../../.source_1765521767/rayon-1.11.0/src/iter/intersperse.rs#L13-L19)*

`Intersperse` is an iterator that inserts a particular item between each
item of the adapted iterator.  This struct is created by the
`intersperse()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="intersperse-new"></span>`fn new(base: I, item: <I as >::Item) -> Self` — [`ParallelIterator`](#paralleliterator)

  Creates a new `Intersperse` iterator

#### Trait Implementations

##### `impl Any for Intersperse<I>`

- <span id="intersperse-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Intersperse<I>`

- <span id="intersperse-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Intersperse<I>`

- <span id="intersperse-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Intersperse<I>`

- <span id="intersperse-clone"></span>`fn clone(&self) -> Intersperse<I>` — [`Intersperse`](intersperse/index.md#intersperse)

##### `impl CloneToUninit for Intersperse<I>`

- <span id="intersperse-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for Intersperse<I>`

- <span id="intersperse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Intersperse<I>`

- <span id="intersperse-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Intersperse<I>`

- <span id="intersperse-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="intersperse-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="intersperse-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Intersperse<I>`

- <span id="intersperse-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Intersperse<I>`

##### `impl IntoParallelIterator for Intersperse<I>`

- <span id="intersperse-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="intersperse-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intersperse-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Intersperse<I>`

- <span id="intersperse-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="intersperse-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="intersperse-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Intersperse<I>`

- <span id="intersperse-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intersperse-pointable-type-init"></span>`type Init = T`

- <span id="intersperse-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intersperse-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intersperse-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intersperse-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Intersperse<I>`

- <span id="intersperse-toowned-type-owned"></span>`type Owned = T`

- <span id="intersperse-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intersperse-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Intersperse<I>`

- <span id="intersperse-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intersperse-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Intersperse<I>`

- <span id="intersperse-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intersperse-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MaxLen<I>`

```rust
struct MaxLen<I> {
    base: I,
    max: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/len.rs:140-143`](../../../.source_1765521767/rayon-1.11.0/src/iter/len.rs#L140-L143)*

`MaxLen` is an iterator that imposes a maximum length on iterator splits.
This struct is created by the `with_max_len()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="maxlen-new"></span>`fn new(base: I, max: usize) -> Self`

  Creates a new `MaxLen` iterator.

#### Trait Implementations

##### `impl Any for MaxLen<I>`

- <span id="maxlen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MaxLen<I>`

- <span id="maxlen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MaxLen<I>`

- <span id="maxlen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for MaxLen<I>`

- <span id="maxlen-clone"></span>`fn clone(&self) -> MaxLen<I>` — [`MaxLen`](len/index.md#maxlen)

##### `impl CloneToUninit for MaxLen<I>`

- <span id="maxlen-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for MaxLen<I>`

- <span id="maxlen-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MaxLen<I>`

- <span id="maxlen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for MaxLen<I>`

- <span id="maxlen-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="maxlen-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="maxlen-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for MaxLen<I>`

- <span id="maxlen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MaxLen<I>`

##### `impl IntoParallelIterator for MaxLen<I>`

- <span id="maxlen-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="maxlen-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="maxlen-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for MaxLen<I>`

- <span id="maxlen-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="maxlen-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="maxlen-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for MaxLen<I>`

- <span id="maxlen-pointable-const-align"></span>`const ALIGN: usize`

- <span id="maxlen-pointable-type-init"></span>`type Init = T`

- <span id="maxlen-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="maxlen-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="maxlen-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="maxlen-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for MaxLen<I>`

- <span id="maxlen-toowned-type-owned"></span>`type Owned = T`

- <span id="maxlen-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="maxlen-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MaxLen<I>`

- <span id="maxlen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="maxlen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MaxLen<I>`

- <span id="maxlen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="maxlen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MinLen<I>`

```rust
struct MinLen<I> {
    base: I,
    min: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/len.rs:10-13`](../../../.source_1765521767/rayon-1.11.0/src/iter/len.rs#L10-L13)*

`MinLen` is an iterator that imposes a minimum length on iterator splits.
This struct is created by the `with_min_len()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="minlen-new"></span>`fn new(base: I, min: usize) -> Self`

  Creates a new `MinLen` iterator.

#### Trait Implementations

##### `impl Any for MinLen<I>`

- <span id="minlen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MinLen<I>`

- <span id="minlen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MinLen<I>`

- <span id="minlen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for MinLen<I>`

- <span id="minlen-clone"></span>`fn clone(&self) -> MinLen<I>` — [`MinLen`](len/index.md#minlen)

##### `impl CloneToUninit for MinLen<I>`

- <span id="minlen-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for MinLen<I>`

- <span id="minlen-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MinLen<I>`

- <span id="minlen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for MinLen<I>`

- <span id="minlen-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="minlen-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="minlen-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for MinLen<I>`

- <span id="minlen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MinLen<I>`

##### `impl IntoParallelIterator for MinLen<I>`

- <span id="minlen-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="minlen-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="minlen-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for MinLen<I>`

- <span id="minlen-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="minlen-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="minlen-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for MinLen<I>`

- <span id="minlen-pointable-const-align"></span>`const ALIGN: usize`

- <span id="minlen-pointable-type-init"></span>`type Init = T`

- <span id="minlen-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="minlen-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="minlen-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="minlen-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for MinLen<I>`

- <span id="minlen-toowned-type-owned"></span>`type Owned = T`

- <span id="minlen-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="minlen-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MinLen<I>`

- <span id="minlen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="minlen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MinLen<I>`

- <span id="minlen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="minlen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Map<I, F>`

```rust
struct Map<I, F> {
    base: I,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map.rs:14-17`](../../../.source_1765521767/rayon-1.11.0/src/iter/map.rs#L14-L17)*

`Map` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="map-new"></span>`fn new(base: I, map_op: F) -> Self`

  Creates a new `Map` iterator.

#### Trait Implementations

##### `impl Any for Map<I, F>`

- <span id="map-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Map<I, F>`

- <span id="map-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Map<I, F>`

- <span id="map-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Map<I, F>`

- <span id="map-clone"></span>`fn clone(&self) -> Map<I, F>` — [`Map`](map/index.md#map)

##### `impl CloneToUninit for Map<I, F>`

- <span id="map-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, F> Debug for Map<I, F>`

- <span id="map-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Map<I, F>`

- <span id="map-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> IndexedParallelIterator for Map<I, F>`

- <span id="map-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="map-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="map-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Map<I, F>`

- <span id="map-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Map<I, F>`

##### `impl IntoParallelIterator for Map<I, F>`

- <span id="map-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="map-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="map-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for Map<I, F>`

- <span id="map-paralleliterator-type-item"></span>`type Item = <F as FnOnce>::Output`

- <span id="map-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="map-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Map<I, F>`

- <span id="map-pointable-const-align"></span>`const ALIGN: usize`

- <span id="map-pointable-type-init"></span>`type Init = T`

- <span id="map-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="map-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="map-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="map-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Map<I, F>`

- <span id="map-toowned-type-owned"></span>`type Owned = T`

- <span id="map-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="map-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Map<I, F>`

- <span id="map-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="map-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Map<I, F>`

- <span id="map-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="map-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapInit<I, INIT, F>`

```rust
struct MapInit<I, INIT, F> {
    base: I,
    init: INIT,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:341-345`](../../../.source_1765521767/rayon-1.11.0/src/iter/map_with.rs#L341-L345)*

`MapInit` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_init()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="mapinit-new"></span>`fn new(base: I, init: INIT, map_op: F) -> Self`

  Creates a new `MapInit` iterator.

#### Trait Implementations

##### `impl Any for MapInit<I, INIT, F>`

- <span id="mapinit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapInit<I, INIT, F>`

- <span id="mapinit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapInit<I, INIT, F>`

- <span id="mapinit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, INIT: clone::Clone, F: clone::Clone> Clone for MapInit<I, INIT, F>`

- <span id="mapinit-clone"></span>`fn clone(&self) -> MapInit<I, INIT, F>` — [`MapInit`](map_with/index.md#mapinit)

##### `impl CloneToUninit for MapInit<I, INIT, F>`

- <span id="mapinit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, INIT, F> Debug for MapInit<I, INIT, F>`

- <span id="mapinit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MapInit<I, INIT, F>`

- <span id="mapinit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, INIT, F> IndexedParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="mapinit-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="mapinit-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for MapInit<I, INIT, F>`

- <span id="mapinit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapInit<I, INIT, F>`

##### `impl IntoParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="mapinit-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="mapinit-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, INIT, F> ParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-paralleliterator-type-item"></span>`type Item = R`

- <span id="mapinit-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="mapinit-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for MapInit<I, INIT, F>`

- <span id="mapinit-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapinit-pointable-type-init"></span>`type Init = T`

- <span id="mapinit-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapinit-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapinit-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapinit-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for MapInit<I, INIT, F>`

- <span id="mapinit-toowned-type-owned"></span>`type Owned = T`

- <span id="mapinit-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mapinit-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MapInit<I, INIT, F>`

- <span id="mapinit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapinit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapInit<I, INIT, F>`

- <span id="mapinit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapinit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapWith<I, T, F>`

```rust
struct MapWith<I, T, F> {
    base: I,
    item: T,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:13-17`](../../../.source_1765521767/rayon-1.11.0/src/iter/map_with.rs#L13-L17)*

`MapWith` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_with()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="mapwith-new"></span>`fn new(base: I, item: T, map_op: F) -> Self`

  Creates a new `MapWith` iterator.

#### Trait Implementations

##### `impl<T> Any for MapWith<I, T, F>`

- <span id="mapwith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapWith<I, T, F>`

- <span id="mapwith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapWith<I, T, F>`

- <span id="mapwith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, T: clone::Clone, F: clone::Clone> Clone for MapWith<I, T, F>`

- <span id="mapwith-clone"></span>`fn clone(&self) -> MapWith<I, T, F>` — [`MapWith`](map_with/index.md#mapwith)

##### `impl<T> CloneToUninit for MapWith<I, T, F>`

- <span id="mapwith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, T: Debug, F> Debug for MapWith<I, T, F>`

- <span id="mapwith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MapWith<I, T, F>`

- <span id="mapwith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, T, F> IndexedParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="mapwith-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="mapwith-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<T, U> Into for MapWith<I, T, F>`

- <span id="mapwith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for MapWith<I, T, F>`

##### `impl<T> IntoParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="mapwith-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="mapwith-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, T, F> ParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-paralleliterator-type-item"></span>`type Item = R`

- <span id="mapwith-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="mapwith-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MapWith<I, T, F>`

- <span id="mapwith-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapwith-pointable-type-init"></span>`type Init = T`

- <span id="mapwith-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwith-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwith-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwith-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for MapWith<I, T, F>`

- <span id="mapwith-toowned-type-owned"></span>`type Owned = T`

- <span id="mapwith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mapwith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for MapWith<I, T, F>`

- <span id="mapwith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapwith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for MapWith<I, T, F>`

- <span id="mapwith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapwith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MultiZip<T>`

```rust
struct MultiZip<T> {
    tuple: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/multizip.rs:79-81`](../../../.source_1765521767/rayon-1.11.0/src/iter/multizip.rs#L79-L81)*

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

##### `impl<T> Any for MultiZip<T>`

- <span id="multizip-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiZip<T>`

- <span id="multizip-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiZip<T>`

- <span id="multizip-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for MultiZip<T>`

- <span id="multizip-clone"></span>`fn clone(&self) -> MultiZip<T>` — [`MultiZip`](multizip/index.md#multizip)

##### `impl<T> CloneToUninit for MultiZip<T>`

- <span id="multizip-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for MultiZip<T>`

- <span id="multizip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MultiZip<T>`

- <span id="multizip-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A> IndexedParallelIterator for MultiZip<(A)>`

- <span id="multizip-indexedparalleliterator-drive"></span>`fn drive<CONSUMER>(self, consumer: CONSUMER) -> <CONSUMER as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="multizip-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="multizip-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<T, U> Into for MultiZip<T>`

- <span id="multizip-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for MultiZip<T>`

##### `impl<T> IntoParallelIterator for MultiZip<T>`

- <span id="multizip-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="multizip-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="multizip-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A> ParallelIterator for MultiZip<(A)>`

- <span id="multizip-paralleliterator-type-item"></span>`type Item = (<A as ParallelIterator>::Item)`

- <span id="multizip-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<CONSUMER>(self, consumer: CONSUMER) -> <CONSUMER as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="multizip-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MultiZip<T>`

- <span id="multizip-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multizip-pointable-type-init"></span>`type Init = T`

- <span id="multizip-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multizip-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multizip-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multizip-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for MultiZip<T>`

- <span id="multizip-toowned-type-owned"></span>`type Owned = T`

- <span id="multizip-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multizip-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for MultiZip<T>`

- <span id="multizip-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multizip-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for MultiZip<T>`

- <span id="multizip-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multizip-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Once<T>`

```rust
struct Once<T> {
    item: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/once.rs:32-34`](../../../.source_1765521767/rayon-1.11.0/src/iter/once.rs#L32-L34)*

Iterator adaptor for [the `once()` function].


#### Trait Implementations

##### `impl<T> Any for Once<T>`

- <span id="once-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Once<T>`

- <span id="once-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Once<T>`

- <span id="once-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Once<T>`

- <span id="once-clone"></span>`fn clone(&self) -> Once<T>` — [`Once`](once/index.md#once)

##### `impl<T> CloneToUninit for Once<T>`

- <span id="once-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Once<T>`

- <span id="once-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Once<T>`

- <span id="once-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for Once<T>`

- <span id="once-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="once-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="once-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<T, U> Into for Once<T>`

- <span id="once-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Once<T>`

##### `impl<T> IntoParallelIterator for Once<T>`

- <span id="once-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="once-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="once-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Once<T>`

- <span id="once-paralleliterator-type-item"></span>`type Item = T`

- <span id="once-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="once-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Once<T>`

- <span id="once-pointable-const-align"></span>`const ALIGN: usize`

- <span id="once-pointable-type-init"></span>`type Init = T`

- <span id="once-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="once-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="once-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="once-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Once<T>`

- <span id="once-toowned-type-owned"></span>`type Owned = T`

- <span id="once-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="once-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Once<T>`

- <span id="once-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="once-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Once<T>`

- <span id="once-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="once-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PanicFuse<I>`

```rust
struct PanicFuse<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:14-16`](../../../.source_1765521767/rayon-1.11.0/src/iter/panic_fuse.rs#L14-L16)*

`PanicFuse` is an adaptor that wraps an iterator with a fuse in case
of panics, to halt all threads as soon as possible.

This struct is created by the `panic_fuse()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="panicfuse-new"></span>`fn new(base: I) -> PanicFuse<I>` — [`PanicFuse`](panic_fuse/index.md#panicfuse)

  Creates a new `PanicFuse` iterator.

#### Trait Implementations

##### `impl Any for PanicFuse<I>`

- <span id="panicfuse-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PanicFuse<I>`

- <span id="panicfuse-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PanicFuse<I>`

- <span id="panicfuse-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for PanicFuse<I>`

- <span id="panicfuse-clone"></span>`fn clone(&self) -> PanicFuse<I>` — [`PanicFuse`](panic_fuse/index.md#panicfuse)

##### `impl CloneToUninit for PanicFuse<I>`

- <span id="panicfuse-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for PanicFuse<I>`

- <span id="panicfuse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PanicFuse<I>`

- <span id="panicfuse-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for PanicFuse<I>`

- <span id="panicfuse-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="panicfuse-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="panicfuse-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for PanicFuse<I>`

- <span id="panicfuse-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PanicFuse<I>`

##### `impl IntoParallelIterator for PanicFuse<I>`

- <span id="panicfuse-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="panicfuse-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="panicfuse-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for PanicFuse<I>`

- <span id="panicfuse-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="panicfuse-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="panicfuse-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for PanicFuse<I>`

- <span id="panicfuse-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfuse-pointable-type-init"></span>`type Init = T`

- <span id="panicfuse-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuse-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuse-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuse-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for PanicFuse<I>`

- <span id="panicfuse-toowned-type-owned"></span>`type Owned = T`

- <span id="panicfuse-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="panicfuse-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PanicFuse<I>`

- <span id="panicfuse-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="panicfuse-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PanicFuse<I>`

- <span id="panicfuse-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="panicfuse-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterBridge<Iter>`

```rust
struct IterBridge<Iter> {
    iter: Iter,
}
```

*Defined in [`rayon-1.11.0/src/iter/par_bridge.rs:72-74`](../../../.source_1765521767/rayon-1.11.0/src/iter/par_bridge.rs#L72-L74)*

`IterBridge` is a parallel iterator that wraps a sequential iterator.

This type is created when using the `par_bridge` method on `ParallelBridge`. See the
[`ParallelBridge`](par_bridge/index.md) documentation for details.

#### Trait Implementations

##### `impl Any for IterBridge<Iter>`

- <span id="iterbridge-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterBridge<Iter>`

- <span id="iterbridge-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterBridge<Iter>`

- <span id="iterbridge-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Iter: clone::Clone> Clone for IterBridge<Iter>`

- <span id="iterbridge-clone"></span>`fn clone(&self) -> IterBridge<Iter>` — [`IterBridge`](par_bridge/index.md#iterbridge)

##### `impl CloneToUninit for IterBridge<Iter>`

- <span id="iterbridge-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Iter: fmt::Debug> Debug for IterBridge<Iter>`

- <span id="iterbridge-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IterBridge<Iter>`

- <span id="iterbridge-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IterBridge<Iter>`

- <span id="iterbridge-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IterBridge<Iter>`

##### `impl IntoParallelIterator for IterBridge<Iter>`

- <span id="iterbridge-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="iterbridge-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iterbridge-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<Iter> ParallelIterator for IterBridge<Iter>`

- <span id="iterbridge-paralleliterator-type-item"></span>`type Item = <Iter as Iterator>::Item`

- <span id="iterbridge-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for IterBridge<Iter>`

- <span id="iterbridge-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iterbridge-pointable-type-init"></span>`type Init = T`

- <span id="iterbridge-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iterbridge-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iterbridge-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iterbridge-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for IterBridge<Iter>`

- <span id="iterbridge-toowned-type-owned"></span>`type Owned = T`

- <span id="iterbridge-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iterbridge-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IterBridge<Iter>`

- <span id="iterbridge-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterbridge-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterBridge<Iter>`

- <span id="iterbridge-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterbridge-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Positions<I, P>`

```rust
struct Positions<I, P> {
    base: I,
    predicate: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/positions.rs:14-17`](../../../.source_1765521767/rayon-1.11.0/src/iter/positions.rs#L14-L17)*

`Positions` takes a predicate `predicate` and filters out elements that match,
yielding their indices.

This struct is created by the `positions()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="positions-new"></span>`fn new(base: I, predicate: P) -> Self`

  Create a new `Positions` iterator.

#### Trait Implementations

##### `impl Any for Positions<I, P>`

- <span id="positions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Positions<I, P>`

- <span id="positions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Positions<I, P>`

- <span id="positions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, P: clone::Clone> Clone for Positions<I, P>`

- <span id="positions-clone"></span>`fn clone(&self) -> Positions<I, P>` — [`Positions`](positions/index.md#positions)

##### `impl CloneToUninit for Positions<I, P>`

- <span id="positions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, P> Debug for Positions<I, P>`

- <span id="positions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Positions<I, P>`

- <span id="positions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Positions<I, P>`

- <span id="positions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Positions<I, P>`

##### `impl IntoParallelIterator for Positions<I, P>`

- <span id="positions-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="positions-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="positions-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for Positions<I, P>`

- <span id="positions-paralleliterator-type-item"></span>`type Item = usize`

- <span id="positions-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for Positions<I, P>`

- <span id="positions-pointable-const-align"></span>`const ALIGN: usize`

- <span id="positions-pointable-type-init"></span>`type Init = T`

- <span id="positions-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="positions-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="positions-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="positions-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Positions<I, P>`

- <span id="positions-toowned-type-owned"></span>`type Owned = T`

- <span id="positions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="positions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Positions<I, P>`

- <span id="positions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="positions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Positions<I, P>`

- <span id="positions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="positions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Repeat<T>`

```rust
struct Repeat<T> {
    element: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:10-12`](../../../.source_1765521767/rayon-1.11.0/src/iter/repeat.rs#L10-L12)*

Iterator adaptor for [the `repeat()` function].


#### Implementations

- <span id="repeat-take"></span>`fn take(self, n: usize) -> RepeatN<T>` — [`RepeatN`](repeat/index.md#repeatn)

  Takes only `n` repeats of the element, similar to the general

  `take()`.

  

  The resulting `RepeatN` is an `IndexedParallelIterator`, allowing

  more functionality than `Repeat` alone.

- <span id="repeat-zip"></span>`fn zip<Z>(self, zip_op: Z) -> Zip<RepeatN<T>, <Z as >::Iter>` — [`Zip`](zip/index.md#zip), [`RepeatN`](repeat/index.md#repeatn), [`IntoParallelIterator`](#intoparalleliterator)

  Iterates tuples, repeating the element with items from another

  iterator, similar to the general `zip()`.

#### Trait Implementations

##### `impl<T> Any for Repeat<T>`

- <span id="repeat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Repeat<T>`

- <span id="repeat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Repeat<T>`

- <span id="repeat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Repeat<T>`

- <span id="repeat-clone"></span>`fn clone(&self) -> Repeat<T>` — [`Repeat`](repeat/index.md#repeat)

##### `impl<T> CloneToUninit for Repeat<T>`

- <span id="repeat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Repeat<T>`

- <span id="repeat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Repeat<T>`

- <span id="repeat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Repeat<T>`

- <span id="repeat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Repeat<T>`

##### `impl<T> IntoParallelIterator for Repeat<T>`

- <span id="repeat-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="repeat-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="repeat-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T> ParallelIterator for Repeat<T>`

- <span id="repeat-paralleliterator-type-item"></span>`type Item = T`

- <span id="repeat-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl<T> Pointable for Repeat<T>`

- <span id="repeat-pointable-const-align"></span>`const ALIGN: usize`

- <span id="repeat-pointable-type-init"></span>`type Init = T`

- <span id="repeat-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeat-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeat-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeat-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Repeat<T>`

- <span id="repeat-toowned-type-owned"></span>`type Owned = T`

- <span id="repeat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repeat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Repeat<T>`

- <span id="repeat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repeat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Repeat<T>`

- <span id="repeat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repeat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RepeatN<T>`

```rust
struct RepeatN<T> {
    inner: RepeatNProducer<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:111-113`](../../../.source_1765521767/rayon-1.11.0/src/iter/repeat.rs#L111-L113)*

Iterator adaptor for [the `repeat_n()` function].


#### Trait Implementations

##### `impl<T> Any for RepeatN<T>`

- <span id="repeatn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RepeatN<T>`

- <span id="repeatn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RepeatN<T>`

- <span id="repeatn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for RepeatN<T>`

- <span id="repeatn-clone"></span>`fn clone(&self) -> RepeatN<T>` — [`RepeatN`](repeat/index.md#repeatn)

##### `impl<T> CloneToUninit for RepeatN<T>`

- <span id="repeatn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for RepeatN<T>`

- <span id="repeatn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RepeatN<T>`

- <span id="repeatn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T> IndexedParallelIterator for RepeatN<T>`

- <span id="repeatn-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="repeatn-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

- <span id="repeatn-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

##### `impl<T, U> Into for RepeatN<T>`

- <span id="repeatn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RepeatN<T>`

##### `impl<T> IntoParallelIterator for RepeatN<T>`

- <span id="repeatn-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="repeatn-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="repeatn-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T> ParallelIterator for RepeatN<T>`

- <span id="repeatn-paralleliterator-type-item"></span>`type Item = T`

- <span id="repeatn-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="repeatn-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RepeatN<T>`

- <span id="repeatn-pointable-const-align"></span>`const ALIGN: usize`

- <span id="repeatn-pointable-type-init"></span>`type Init = T`

- <span id="repeatn-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeatn-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeatn-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeatn-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for RepeatN<T>`

- <span id="repeatn-toowned-type-owned"></span>`type Owned = T`

- <span id="repeatn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repeatn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RepeatN<T>`

- <span id="repeatn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repeatn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RepeatN<T>`

- <span id="repeatn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repeatn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Rev<I>`

```rust
struct Rev<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/rev.rs:11-13`](../../../.source_1765521767/rayon-1.11.0/src/iter/rev.rs#L11-L13)*

`Rev` is an iterator that produces elements in reverse order. This struct
is created by the `rev()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="rev-new"></span>`fn new(base: I) -> Self`

  Creates a new `Rev` iterator.

#### Trait Implementations

##### `impl Any for Rev<I>`

- <span id="rev-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Rev<I>`

- <span id="rev-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Rev<I>`

- <span id="rev-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Rev<I>`

- <span id="rev-clone"></span>`fn clone(&self) -> Rev<I>` — [`Rev`](rev/index.md#rev)

##### `impl CloneToUninit for Rev<I>`

- <span id="rev-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Rev<I>`

- <span id="rev-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Rev<I>`

- <span id="rev-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Rev<I>`

- <span id="rev-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="rev-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="rev-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Rev<I>`

- <span id="rev-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Rev<I>`

##### `impl IntoParallelIterator for Rev<I>`

- <span id="rev-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rev-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rev-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Rev<I>`

- <span id="rev-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="rev-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="rev-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Rev<I>`

- <span id="rev-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rev-pointable-type-init"></span>`type Init = T`

- <span id="rev-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rev-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rev-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rev-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Rev<I>`

- <span id="rev-toowned-type-owned"></span>`type Owned = T`

- <span id="rev-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rev-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Rev<I>`

- <span id="rev-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rev-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Rev<I>`

- <span id="rev-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rev-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Skip<I>`

```rust
struct Skip<I> {
    base: I,
    n: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip.rs:11-14`](../../../.source_1765521767/rayon-1.11.0/src/iter/skip.rs#L11-L14)*

`Skip` is an iterator that skips over the first `n` elements.
This struct is created by the `skip()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="skip-new"></span>`fn new(base: I, n: usize) -> Self`

  Creates a new `Skip` iterator.

#### Trait Implementations

##### `impl Any for Skip<I>`

- <span id="skip-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Skip<I>`

- <span id="skip-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Skip<I>`

- <span id="skip-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Skip<I>`

- <span id="skip-clone"></span>`fn clone(&self) -> Skip<I>` — [`Skip`](skip/index.md#skip)

##### `impl CloneToUninit for Skip<I>`

- <span id="skip-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Skip<I>`

- <span id="skip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Skip<I>`

- <span id="skip-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Skip<I>`

- <span id="skip-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="skip-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="skip-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Skip<I>`

- <span id="skip-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Skip<I>`

##### `impl IntoParallelIterator for Skip<I>`

- <span id="skip-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="skip-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skip-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Skip<I>`

- <span id="skip-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skip-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="skip-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Skip<I>`

- <span id="skip-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skip-pointable-type-init"></span>`type Init = T`

- <span id="skip-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skip-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skip-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skip-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Skip<I>`

- <span id="skip-toowned-type-owned"></span>`type Owned = T`

- <span id="skip-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="skip-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Skip<I>`

- <span id="skip-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skip-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Skip<I>`

- <span id="skip-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skip-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SkipAny<I>`

```rust
struct SkipAny<I> {
    base: I,
    count: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any.rs:11-14`](../../../.source_1765521767/rayon-1.11.0/src/iter/skip_any.rs#L11-L14)*

`SkipAny` is an iterator that skips over `n` elements from anywhere in `I`.
This struct is created by the `skip_any()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="skipany-new"></span>`fn new(base: I, count: usize) -> Self`

  Creates a new `SkipAny` iterator.

#### Trait Implementations

##### `impl Any for SkipAny<I>`

- <span id="skipany-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SkipAny<I>`

- <span id="skipany-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SkipAny<I>`

- <span id="skipany-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for SkipAny<I>`

- <span id="skipany-clone"></span>`fn clone(&self) -> SkipAny<I>` — [`SkipAny`](skip_any/index.md#skipany)

##### `impl CloneToUninit for SkipAny<I>`

- <span id="skipany-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for SkipAny<I>`

- <span id="skipany-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SkipAny<I>`

- <span id="skipany-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SkipAny<I>`

- <span id="skipany-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SkipAny<I>`

##### `impl IntoParallelIterator for SkipAny<I>`

- <span id="skipany-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="skipany-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skipany-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for SkipAny<I>`

- <span id="skipany-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skipany-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for SkipAny<I>`

- <span id="skipany-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skipany-pointable-type-init"></span>`type Init = T`

- <span id="skipany-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipany-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipany-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipany-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for SkipAny<I>`

- <span id="skipany-toowned-type-owned"></span>`type Owned = T`

- <span id="skipany-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="skipany-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SkipAny<I>`

- <span id="skipany-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skipany-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SkipAny<I>`

- <span id="skipany-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skipany-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SkipAnyWhile<I, P>`

```rust
struct SkipAnyWhile<I, P> {
    base: I,
    predicate: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any_while.rs:13-16`](../../../.source_1765521767/rayon-1.11.0/src/iter/skip_any_while.rs#L13-L16)*

`SkipAnyWhile` is an iterator that skips over elements from anywhere in `I`
until the callback returns `false`.
This struct is created by the `skip_any_while()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="skipanywhile-new"></span>`fn new(base: I, predicate: P) -> Self`

  Creates a new `SkipAnyWhile` iterator.

#### Trait Implementations

##### `impl Any for SkipAnyWhile<I, P>`

- <span id="skipanywhile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SkipAnyWhile<I, P>`

- <span id="skipanywhile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SkipAnyWhile<I, P>`

- <span id="skipanywhile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, P: clone::Clone> Clone for SkipAnyWhile<I, P>`

- <span id="skipanywhile-clone"></span>`fn clone(&self) -> SkipAnyWhile<I, P>` — [`SkipAnyWhile`](skip_any_while/index.md#skipanywhile)

##### `impl CloneToUninit for SkipAnyWhile<I, P>`

- <span id="skipanywhile-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, P> Debug for SkipAnyWhile<I, P>`

- <span id="skipanywhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SkipAnyWhile<I, P>`

- <span id="skipanywhile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SkipAnyWhile<I, P>`

- <span id="skipanywhile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SkipAnyWhile<I, P>`

##### `impl IntoParallelIterator for SkipAnyWhile<I, P>`

- <span id="skipanywhile-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="skipanywhile-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skipanywhile-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for SkipAnyWhile<I, P>`

- <span id="skipanywhile-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skipanywhile-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for SkipAnyWhile<I, P>`

- <span id="skipanywhile-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skipanywhile-pointable-type-init"></span>`type Init = T`

- <span id="skipanywhile-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanywhile-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanywhile-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanywhile-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for SkipAnyWhile<I, P>`

- <span id="skipanywhile-toowned-type-owned"></span>`type Owned = T`

- <span id="skipanywhile-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="skipanywhile-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SkipAnyWhile<I, P>`

- <span id="skipanywhile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skipanywhile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SkipAnyWhile<I, P>`

- <span id="skipanywhile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skipanywhile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Split<D, S>`

```rust
struct Split<D, S> {
    data: D,
    splitter: S,
}
```

*Defined in [`rayon-1.11.0/src/iter/splitter.rs:117-120`](../../../.source_1765521767/rayon-1.11.0/src/iter/splitter.rs#L117-L120)*

`Split` is a parallel iterator using arbitrary data and a splitting function.
This struct is created by the [`split()`](splitter/index.md) function.

#### Trait Implementations

##### `impl Any for Split<D, S>`

- <span id="split-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Split<D, S>`

- <span id="split-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Split<D, S>`

- <span id="split-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<D: clone::Clone, S: clone::Clone> Clone for Split<D, S>`

- <span id="split-clone"></span>`fn clone(&self) -> Split<D, S>` — [`Split`](splitter/index.md#split)

##### `impl CloneToUninit for Split<D, S>`

- <span id="split-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<D: Debug, S> Debug for Split<D, S>`

- <span id="split-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Split<D, S>`

- <span id="split-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Split<D, S>`

- <span id="split-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Split<D, S>`

##### `impl IntoParallelIterator for Split<D, S>`

- <span id="split-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="split-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="split-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<D, S> ParallelIterator for Split<D, S>`

- <span id="split-paralleliterator-type-item"></span>`type Item = D`

- <span id="split-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for Split<D, S>`

- <span id="split-pointable-const-align"></span>`const ALIGN: usize`

- <span id="split-pointable-type-init"></span>`type Init = T`

- <span id="split-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="split-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="split-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="split-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Split<D, S>`

- <span id="split-toowned-type-owned"></span>`type Owned = T`

- <span id="split-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="split-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Split<D, S>`

- <span id="split-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="split-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Split<D, S>`

- <span id="split-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="split-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StepBy<I>`

```rust
struct StepBy<I> {
    base: I,
    step: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/step_by.rs:11-14`](../../../.source_1765521767/rayon-1.11.0/src/iter/step_by.rs#L11-L14)*

`StepBy` is an iterator that skips `n` elements between each yield, where `n` is the given step.
This struct is created by the `step_by()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="stepby-new"></span>`fn new(base: I, step: usize) -> Self`

  Creates a new `StepBy` iterator.

#### Trait Implementations

##### `impl Any for StepBy<I>`

- <span id="stepby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StepBy<I>`

- <span id="stepby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StepBy<I>`

- <span id="stepby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for StepBy<I>`

- <span id="stepby-clone"></span>`fn clone(&self) -> StepBy<I>` — [`StepBy`](step_by/index.md#stepby)

##### `impl CloneToUninit for StepBy<I>`

- <span id="stepby-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for StepBy<I>`

- <span id="stepby-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StepBy<I>`

- <span id="stepby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for StepBy<I>`

- <span id="stepby-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="stepby-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="stepby-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for StepBy<I>`

- <span id="stepby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for StepBy<I>`

##### `impl IntoParallelIterator for StepBy<I>`

- <span id="stepby-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="stepby-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="stepby-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for StepBy<I>`

- <span id="stepby-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="stepby-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="stepby-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for StepBy<I>`

- <span id="stepby-pointable-const-align"></span>`const ALIGN: usize`

- <span id="stepby-pointable-type-init"></span>`type Init = T`

- <span id="stepby-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stepby-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stepby-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stepby-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for StepBy<I>`

- <span id="stepby-toowned-type-owned"></span>`type Owned = T`

- <span id="stepby-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stepby-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StepBy<I>`

- <span id="stepby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stepby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StepBy<I>`

- <span id="stepby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stepby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Take<I>`

```rust
struct Take<I> {
    base: I,
    n: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/take.rs:10-13`](../../../.source_1765521767/rayon-1.11.0/src/iter/take.rs#L10-L13)*

`Take` is an iterator that iterates over the first `n` elements.
This struct is created by the `take()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="take-new"></span>`fn new(base: I, n: usize) -> Self`

  Creates a new `Take` iterator.

#### Trait Implementations

##### `impl Any for Take<I>`

- <span id="take-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Take<I>`

- <span id="take-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Take<I>`

- <span id="take-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Take<I>`

- <span id="take-clone"></span>`fn clone(&self) -> Take<I>` — [`Take`](take/index.md#take)

##### `impl CloneToUninit for Take<I>`

- <span id="take-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Take<I>`

- <span id="take-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Take<I>`

- <span id="take-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Take<I>`

- <span id="take-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="take-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="take-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Take<I>`

- <span id="take-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Take<I>`

##### `impl IntoParallelIterator for Take<I>`

- <span id="take-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="take-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="take-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Take<I>`

- <span id="take-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="take-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="take-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Take<I>`

- <span id="take-pointable-const-align"></span>`const ALIGN: usize`

- <span id="take-pointable-type-init"></span>`type Init = T`

- <span id="take-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="take-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="take-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="take-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Take<I>`

- <span id="take-toowned-type-owned"></span>`type Owned = T`

- <span id="take-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="take-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Take<I>`

- <span id="take-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="take-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Take<I>`

- <span id="take-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="take-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TakeAny<I>`

```rust
struct TakeAny<I> {
    base: I,
    count: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any.rs:11-14`](../../../.source_1765521767/rayon-1.11.0/src/iter/take_any.rs#L11-L14)*

`TakeAny` is an iterator that iterates over `n` elements from anywhere in `I`.
This struct is created by the `take_any()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="takeany-new"></span>`fn new(base: I, count: usize) -> Self`

  Creates a new `TakeAny` iterator.

#### Trait Implementations

##### `impl Any for TakeAny<I>`

- <span id="takeany-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TakeAny<I>`

- <span id="takeany-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TakeAny<I>`

- <span id="takeany-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for TakeAny<I>`

- <span id="takeany-clone"></span>`fn clone(&self) -> TakeAny<I>` — [`TakeAny`](take_any/index.md#takeany)

##### `impl CloneToUninit for TakeAny<I>`

- <span id="takeany-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for TakeAny<I>`

- <span id="takeany-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TakeAny<I>`

- <span id="takeany-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TakeAny<I>`

- <span id="takeany-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TakeAny<I>`

##### `impl IntoParallelIterator for TakeAny<I>`

- <span id="takeany-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="takeany-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="takeany-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for TakeAny<I>`

- <span id="takeany-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="takeany-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for TakeAny<I>`

- <span id="takeany-pointable-const-align"></span>`const ALIGN: usize`

- <span id="takeany-pointable-type-init"></span>`type Init = T`

- <span id="takeany-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeany-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeany-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeany-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TakeAny<I>`

- <span id="takeany-toowned-type-owned"></span>`type Owned = T`

- <span id="takeany-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="takeany-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TakeAny<I>`

- <span id="takeany-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="takeany-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TakeAny<I>`

- <span id="takeany-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="takeany-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TakeAnyWhile<I, P>`

```rust
struct TakeAnyWhile<I, P> {
    base: I,
    predicate: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any_while.rs:13-16`](../../../.source_1765521767/rayon-1.11.0/src/iter/take_any_while.rs#L13-L16)*

`TakeAnyWhile` is an iterator that iterates over elements from anywhere in `I`
until the callback returns `false`.
This struct is created by the `take_any_while()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="takeanywhile-new"></span>`fn new(base: I, predicate: P) -> Self`

  Creates a new `TakeAnyWhile` iterator.

#### Trait Implementations

##### `impl Any for TakeAnyWhile<I, P>`

- <span id="takeanywhile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TakeAnyWhile<I, P>`

- <span id="takeanywhile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TakeAnyWhile<I, P>`

- <span id="takeanywhile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, P: clone::Clone> Clone for TakeAnyWhile<I, P>`

- <span id="takeanywhile-clone"></span>`fn clone(&self) -> TakeAnyWhile<I, P>` — [`TakeAnyWhile`](take_any_while/index.md#takeanywhile)

##### `impl CloneToUninit for TakeAnyWhile<I, P>`

- <span id="takeanywhile-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, P> Debug for TakeAnyWhile<I, P>`

- <span id="takeanywhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TakeAnyWhile<I, P>`

- <span id="takeanywhile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TakeAnyWhile<I, P>`

- <span id="takeanywhile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TakeAnyWhile<I, P>`

##### `impl IntoParallelIterator for TakeAnyWhile<I, P>`

- <span id="takeanywhile-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="takeanywhile-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="takeanywhile-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for TakeAnyWhile<I, P>`

- <span id="takeanywhile-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="takeanywhile-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for TakeAnyWhile<I, P>`

- <span id="takeanywhile-pointable-const-align"></span>`const ALIGN: usize`

- <span id="takeanywhile-pointable-type-init"></span>`type Init = T`

- <span id="takeanywhile-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanywhile-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanywhile-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanywhile-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TakeAnyWhile<I, P>`

- <span id="takeanywhile-toowned-type-owned"></span>`type Owned = T`

- <span id="takeanywhile-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="takeanywhile-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TakeAnyWhile<I, P>`

- <span id="takeanywhile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="takeanywhile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TakeAnyWhile<I, P>`

- <span id="takeanywhile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="takeanywhile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TryFold<I, U, ID, F>`

```rust
struct TryFold<I, U, ID, F> {
    base: I,
    identity: ID,
    fold_op: F,
    marker: std::marker::PhantomData<U>,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_fold.rs:26-31`](../../../.source_1765521767/rayon-1.11.0/src/iter/try_fold.rs#L26-L31)*

`TryFold` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `try_fold()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="tryfold-new"></span>`fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl Any for TryFold<I, U, ID, F>`

- <span id="tryfold-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryFold<I, U, ID, F>`

- <span id="tryfold-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryFold<I, U, ID, F>`

- <span id="tryfold-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, U: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for TryFold<I, U, ID, F>`

- <span id="tryfold-clone"></span>`fn clone(&self) -> TryFold<I, U, ID, F>` — [`TryFold`](try_fold/index.md#tryfold)

##### `impl CloneToUninit for TryFold<I, U, ID, F>`

- <span id="tryfold-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<U, I: ParallelIterator + Debug, ID, F> Debug for TryFold<I, U, ID, F>`

- <span id="tryfold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TryFold<I, U, ID, F>`

- <span id="tryfold-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryFold<I, U, ID, F>`

- <span id="tryfold-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TryFold<I, U, ID, F>`

##### `impl IntoParallelIterator for TryFold<I, U, ID, F>`

- <span id="tryfold-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="tryfold-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="tryfold-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, ID, F> ParallelIterator for TryFold<I, U, ID, F>`

- <span id="tryfold-paralleliterator-type-item"></span>`type Item = U`

- <span id="tryfold-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for TryFold<I, U, ID, F>`

- <span id="tryfold-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tryfold-pointable-type-init"></span>`type Init = T`

- <span id="tryfold-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfold-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfold-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfold-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TryFold<I, U, ID, F>`

- <span id="tryfold-toowned-type-owned"></span>`type Owned = T`

- <span id="tryfold-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tryfold-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TryFold<I, U, ID, F>`

- <span id="tryfold-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryfold-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryFold<I, U, ID, F>`

- <span id="tryfold-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryfold-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TryFoldWith<I, U: Try, F>`

```rust
struct TryFoldWith<I, U: Try, F> {
    base: I,
    item: <U as >::Output,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_fold.rs:180-184`](../../../.source_1765521767/rayon-1.11.0/src/iter/try_fold.rs#L180-L184)*

`TryFoldWith` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `try_fold_with()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="tryfoldwith-new"></span>`fn new(base: I, item: <U as >::Output, fold_op: F) -> Self` — [`Try`](private/index.md#try)

#### Trait Implementations

##### `impl Any for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, U: clone::Clone + Try, F: clone::Clone> Clone for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-clone"></span>`fn clone(&self) -> TryFoldWith<I, U, F>` — [`TryFoldWith`](try_fold/index.md#tryfoldwith)

##### `impl CloneToUninit for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, U, F> Debug for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TryFoldWith<I, U, F>`

##### `impl IntoParallelIterator for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="tryfoldwith-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="tryfoldwith-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, F> ParallelIterator for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-paralleliterator-type-item"></span>`type Item = U`

- <span id="tryfoldwith-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tryfoldwith-pointable-type-init"></span>`type Init = T`

- <span id="tryfoldwith-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfoldwith-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfoldwith-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfoldwith-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-toowned-type-owned"></span>`type Owned = T`

- <span id="tryfoldwith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tryfoldwith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryfoldwith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryfoldwith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Update<I, F>`

```rust
struct Update<I, F> {
    base: I,
    update_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:14-17`](../../../.source_1765521767/rayon-1.11.0/src/iter/update.rs#L14-L17)*

`Update` is an iterator that mutates the elements of an
underlying iterator before they are yielded.

This struct is created by the `update()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="update-new"></span>`fn new(base: I, update_op: F) -> Self`

  Creates a new `Update` iterator.

#### Trait Implementations

##### `impl Any for Update<I, F>`

- <span id="update-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Update<I, F>`

- <span id="update-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Update<I, F>`

- <span id="update-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Update<I, F>`

- <span id="update-clone"></span>`fn clone(&self) -> Update<I, F>` — [`Update`](update/index.md#update)

##### `impl CloneToUninit for Update<I, F>`

- <span id="update-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, F> Debug for Update<I, F>`

- <span id="update-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Update<I, F>`

- <span id="update-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> IndexedParallelIterator for Update<I, F>`

- <span id="update-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="update-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="update-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Update<I, F>`

- <span id="update-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Update<I, F>`

##### `impl IntoParallelIterator for Update<I, F>`

- <span id="update-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="update-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="update-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for Update<I, F>`

- <span id="update-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="update-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="update-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Update<I, F>`

- <span id="update-pointable-const-align"></span>`const ALIGN: usize`

- <span id="update-pointable-type-init"></span>`type Init = T`

- <span id="update-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="update-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="update-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="update-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Update<I, F>`

- <span id="update-toowned-type-owned"></span>`type Owned = T`

- <span id="update-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="update-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Update<I, F>`

- <span id="update-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="update-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Update<I, F>`

- <span id="update-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="update-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WalkTree<S, B>`

```rust
struct WalkTree<S, B>(WalkTreePostfix<S, B>);
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:459`](../../../.source_1765521767/rayon-1.11.0/src/iter/walk_tree.rs#L459)*

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree()`](walk_tree/index.md) function.

#### Trait Implementations

##### `impl Any for WalkTree<S, B>`

- <span id="walktree-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WalkTree<S, B>`

- <span id="walktree-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WalkTree<S, B>`

- <span id="walktree-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTree<S, B>`

- <span id="walktree-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WalkTree<S, B>`

- <span id="walktree-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WalkTree<S, B>`

- <span id="walktree-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WalkTree<S, B>`

##### `impl IntoParallelIterator for WalkTree<S, B>`

- <span id="walktree-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="walktree-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktree-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B> ParallelIterator for WalkTree<S, B>`

- <span id="walktree-paralleliterator-type-item"></span>`type Item = S`

- <span id="walktree-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for WalkTree<S, B>`

- <span id="walktree-pointable-const-align"></span>`const ALIGN: usize`

- <span id="walktree-pointable-type-init"></span>`type Init = T`

- <span id="walktree-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktree-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktree-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktree-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WalkTree<S, B>`

- <span id="walktree-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="walktree-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WalkTree<S, B>`

- <span id="walktree-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="walktree-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WalkTreePostfix<S, B>`

```rust
struct WalkTreePostfix<S, B> {
    initial_state: S,
    children_of: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:302-305`](../../../.source_1765521767/rayon-1.11.0/src/iter/walk_tree.rs#L302-L305)*

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_postfix()`](walk_tree/index.md) function.

#### Trait Implementations

##### `impl Any for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WalkTreePostfix<S, B>`

##### `impl IntoParallelIterator for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="walktreepostfix-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktreepostfix-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B> ParallelIterator for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-paralleliterator-type-item"></span>`type Item = S`

- <span id="walktreepostfix-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-pointable-const-align"></span>`const ALIGN: usize`

- <span id="walktreepostfix-pointable-type-init"></span>`type Init = T`

- <span id="walktreepostfix-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreepostfix-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreepostfix-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreepostfix-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="walktreepostfix-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="walktreepostfix-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WalkTreePrefix<S, B>`

```rust
struct WalkTreePrefix<S, B> {
    initial_state: S,
    children_of: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:76-79`](../../../.source_1765521767/rayon-1.11.0/src/iter/walk_tree.rs#L76-L79)*

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_prefix()`](walk_tree/index.md) function.

#### Trait Implementations

##### `impl Any for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WalkTreePrefix<S, B>`

##### `impl IntoParallelIterator for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="walktreeprefix-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktreeprefix-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B> ParallelIterator for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-paralleliterator-type-item"></span>`type Item = S`

- <span id="walktreeprefix-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-pointable-const-align"></span>`const ALIGN: usize`

- <span id="walktreeprefix-pointable-type-init"></span>`type Init = T`

- <span id="walktreeprefix-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreeprefix-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreeprefix-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreeprefix-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="walktreeprefix-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="walktreeprefix-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WhileSome<I>`

```rust
struct WhileSome<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/while_some.rs:13-15`](../../../.source_1765521767/rayon-1.11.0/src/iter/while_some.rs#L13-L15)*

`WhileSome` is an iterator that yields the `Some` elements of an iterator,
halting as soon as any `None` is produced.

This struct is created by the `while_some()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- <span id="whilesome-new"></span>`fn new(base: I) -> Self`

  Creates a new `WhileSome` iterator.

#### Trait Implementations

##### `impl Any for WhileSome<I>`

- <span id="whilesome-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WhileSome<I>`

- <span id="whilesome-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WhileSome<I>`

- <span id="whilesome-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for WhileSome<I>`

- <span id="whilesome-clone"></span>`fn clone(&self) -> WhileSome<I>` — [`WhileSome`](while_some/index.md#whilesome)

##### `impl CloneToUninit for WhileSome<I>`

- <span id="whilesome-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for WhileSome<I>`

- <span id="whilesome-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WhileSome<I>`

- <span id="whilesome-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WhileSome<I>`

- <span id="whilesome-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WhileSome<I>`

##### `impl IntoParallelIterator for WhileSome<I>`

- <span id="whilesome-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="whilesome-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="whilesome-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for WhileSome<I>`

- <span id="whilesome-paralleliterator-type-item"></span>`type Item = T`

- <span id="whilesome-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

##### `impl Pointable for WhileSome<I>`

- <span id="whilesome-pointable-const-align"></span>`const ALIGN: usize`

- <span id="whilesome-pointable-type-init"></span>`type Init = T`

- <span id="whilesome-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="whilesome-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="whilesome-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="whilesome-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for WhileSome<I>`

- <span id="whilesome-toowned-type-owned"></span>`type Owned = T`

- <span id="whilesome-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="whilesome-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WhileSome<I>`

- <span id="whilesome-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="whilesome-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WhileSome<I>`

- <span id="whilesome-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="whilesome-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Zip<A, B>`

```rust
struct Zip<A, B> {
    a: A,
    b: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/zip.rs:12-15`](../../../.source_1765521767/rayon-1.11.0/src/iter/zip.rs#L12-L15)*

`Zip` is an iterator that zips up `a` and `b` into a single iterator
of pairs. This struct is created by the `zip()` method on
[`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- <span id="zip-new"></span>`fn new(a: A, b: B) -> Self`

  Creates a new `Zip` iterator.

#### Trait Implementations

##### `impl Any for Zip<A, B>`

- <span id="zip-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Zip<A, B>`

- <span id="zip-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Zip<A, B>`

- <span id="zip-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: clone::Clone, B: clone::Clone> Clone for Zip<A, B>`

- <span id="zip-clone"></span>`fn clone(&self) -> Zip<A, B>` — [`Zip`](zip/index.md#zip)

##### `impl CloneToUninit for Zip<A, B>`

- <span id="zip-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Zip<A, B>`

- <span id="zip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Zip<A, B>`

- <span id="zip-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A, B> IndexedParallelIterator for Zip<A, B>`

- <span id="zip-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="zip-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="zip-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for Zip<A, B>`

- <span id="zip-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Zip<A, B>`

##### `impl IntoParallelIterator for Zip<A, B>`

- <span id="zip-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="zip-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="zip-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for Zip<A, B>`

- <span id="zip-paralleliterator-type-item"></span>`type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- <span id="zip-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="zip-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Zip<A, B>`

- <span id="zip-pointable-const-align"></span>`const ALIGN: usize`

- <span id="zip-pointable-type-init"></span>`type Init = T`

- <span id="zip-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="zip-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="zip-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="zip-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Zip<A, B>`

- <span id="zip-toowned-type-owned"></span>`type Owned = T`

- <span id="zip-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="zip-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Zip<A, B>`

- <span id="zip-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="zip-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Zip<A, B>`

- <span id="zip-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="zip-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ZipEq<A, B>`

```rust
struct ZipEq<A, B> {
    zip: Zip<A, B>,
}
```

*Defined in [`rayon-1.11.0/src/iter/zip_eq.rs:13-15`](../../../.source_1765521767/rayon-1.11.0/src/iter/zip_eq.rs#L13-L15)*

An [`IndexedParallelIterator`](#indexedparalleliterator) that iterates over two parallel iterators of equal
length simultaneously.

This struct is created by the [`zip_eq`](zip_eq/index.md) method on [`IndexedParallelIterator`](#indexedparalleliterator),
see its documentation for more information.


#### Implementations

- <span id="zipeq-new"></span>`fn new(a: A, b: B) -> Self`

  Creates a new `ZipEq` iterator.

#### Trait Implementations

##### `impl Any for ZipEq<A, B>`

- <span id="zipeq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ZipEq<A, B>`

- <span id="zipeq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ZipEq<A, B>`

- <span id="zipeq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: clone::Clone, B: clone::Clone> Clone for ZipEq<A, B>`

- <span id="zipeq-clone"></span>`fn clone(&self) -> ZipEq<A, B>` — [`ZipEq`](zip_eq/index.md#zipeq)

##### `impl CloneToUninit for ZipEq<A, B>`

- <span id="zipeq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for ZipEq<A, B>`

- <span id="zipeq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ZipEq<A, B>`

- <span id="zipeq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A, B> IndexedParallelIterator for ZipEq<A, B>`

- <span id="zipeq-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="zipeq-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="zipeq-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md#producercallback)

##### `impl<U> Into for ZipEq<A, B>`

- <span id="zipeq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ZipEq<A, B>`

##### `impl IntoParallelIterator for ZipEq<A, B>`

- <span id="zipeq-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="zipeq-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="zipeq-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for ZipEq<A, B>`

- <span id="zipeq-paralleliterator-type-item"></span>`type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- <span id="zipeq-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md#consumer)

- <span id="zipeq-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for ZipEq<A, B>`

- <span id="zipeq-pointable-const-align"></span>`const ALIGN: usize`

- <span id="zipeq-pointable-type-init"></span>`type Init = T`

- <span id="zipeq-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="zipeq-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="zipeq-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="zipeq-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for ZipEq<A, B>`

- <span id="zipeq-toowned-type-owned"></span>`type Owned = T`

- <span id="zipeq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="zipeq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ZipEq<A, B>`

- <span id="zipeq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="zipeq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ZipEq<A, B>`

- <span id="zipeq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="zipeq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

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

### `IntoParallelIterator`

```rust
trait IntoParallelIterator { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:219-249`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L219-L249)*

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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:261-285`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L261-L285)*

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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:309-329`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L309-L329)*

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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:356-2421`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L356-L2421)*

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

- [`Bytes`](../str/index.md#bytes)
- [`Chain`](chain/index.md#chain)
- [`CharIndices`](../str/index.md#charindices)
- [`Chars`](../str/index.md#chars)
- [`ChunkByMut`](../slice/chunk_by/index.md#chunkbymut)
- [`ChunkBy`](../slice/chunk_by/index.md#chunkby)
- [`ChunksExactMut`](../slice/chunks/index.md#chunksexactmut)
- [`ChunksExact`](../slice/chunks/index.md#chunksexact)
- [`ChunksMut`](../slice/chunks/index.md#chunksmut)
- [`Chunks`](../slice/chunks/index.md#chunks)
- [`Chunks`](chunks/index.md#chunks)
- [`Cloned`](cloned/index.md#cloned)
- [`Copied`](copied/index.md#copied)
- [`Drain`](../collections/binary_heap/index.md#drain)
- [`Drain`](../collections/hash_map/index.md#drain)
- [`Drain`](../collections/hash_set/index.md#drain)
- [`Drain`](../collections/vec_deque/index.md#drain)
- [`Drain`](../string/index.md#drain)
- [`Drain`](../vec/index.md#drain)
- [`Either`](#either)
- [`Empty`](empty/index.md#empty)
- [`EncodeUtf16`](../str/index.md#encodeutf16)
- [`Enumerate`](enumerate/index.md#enumerate)
- [`ExponentialBlocks`](blocks/index.md#exponentialblocks)
- [`FilterMap`](filter_map/index.md#filtermap)
- [`Filter`](filter/index.md#filter)
- [`FlatMapIter`](flat_map_iter/index.md#flatmapiter)
- [`FlatMap`](flat_map/index.md#flatmap)
- [`FlattenIter`](flatten_iter/index.md#flatteniter)
- [`Flatten`](flatten/index.md#flatten)
- [`FoldChunksWith`](fold_chunks_with/index.md#foldchunkswith)
- [`FoldChunks`](fold_chunks/index.md#foldchunks)
- [`FoldWith`](fold/index.md#foldwith)
- [`Fold`](fold/index.md#fold)
- [`Inspect`](inspect/index.md#inspect)
- [`InterleaveShortest`](interleave_shortest/index.md#interleaveshortest)
- [`Interleave`](interleave/index.md#interleave)
- [`Intersperse`](intersperse/index.md#intersperse)
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
- [`IterBridge`](par_bridge/index.md#iterbridge)
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
- [`MapInit`](map_with/index.md#mapinit)
- [`MapWith`](map_with/index.md#mapwith)
- [`Map`](map/index.md#map)
- [`MatchIndices`](../str/index.md#matchindices)
- [`Matches`](../str/index.md#matches)
- [`MaxLen`](len/index.md#maxlen)
- [`MinLen`](len/index.md#minlen)
- [`MultiZip`](multizip/index.md#multizip)
- [`Once`](once/index.md#once)
- [`PanicFuse`](panic_fuse/index.md#panicfuse)
- [`Positions`](positions/index.md#positions)
- [`RChunksExactMut`](../slice/rchunks/index.md#rchunksexactmut)
- [`RChunksExact`](../slice/rchunks/index.md#rchunksexact)
- [`RChunksMut`](../slice/rchunks/index.md#rchunksmut)
- [`RChunks`](../slice/rchunks/index.md#rchunks)
- [`RepeatN`](repeat/index.md#repeatn)
- [`Repeat`](repeat/index.md#repeat)
- [`Rev`](rev/index.md#rev)
- [`SkipAnyWhile`](skip_any_while/index.md#skipanywhile)
- [`SkipAny`](skip_any/index.md#skipany)
- [`Skip`](skip/index.md#skip)
- [`SplitAsciiWhitespace`](../str/index.md#splitasciiwhitespace)
- [`SplitInclusiveMut`](../slice/index.md#splitinclusivemut)
- [`SplitInclusive`](../slice/index.md#splitinclusive)
- [`SplitInclusive`](../str/index.md#splitinclusive)
- [`SplitMut`](../slice/index.md#splitmut)
- [`SplitTerminator`](../str/index.md#splitterminator)
- [`SplitWhitespace`](../str/index.md#splitwhitespace)
- [`Split`](../slice/index.md#split)
- [`Split`](../str/index.md#split)
- [`Split`](splitter/index.md#split)
- [`StepBy`](step_by/index.md#stepby)
- [`TakeAnyWhile`](take_any_while/index.md#takeanywhile)
- [`TakeAny`](take_any/index.md#takeany)
- [`Take`](take/index.md#take)
- [`TryFoldWith`](try_fold/index.md#tryfoldwith)
- [`TryFold`](try_fold/index.md#tryfold)
- [`UniformBlocks`](blocks/index.md#uniformblocks)
- [`UnzipA`](unzip/index.md#unzipa)
- [`UnzipB`](unzip/index.md#unzipb)
- [`Update`](update/index.md#update)
- [`WalkTreePostfix`](walk_tree/index.md#walktreepostfix)
- [`WalkTreePrefix`](walk_tree/index.md#walktreeprefix)
- [`WalkTree`](walk_tree/index.md#walktree)
- [`WhileSome`](while_some/index.md#whilesome)
- [`Windows`](../slice/index.md#windows)
- [`ZipEq`](zip_eq/index.md#zipeq)
- [`Zip`](zip/index.md#zip)

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

- [`Chain`](chain/index.md#chain)
- [`ChunksExactMut`](../slice/chunks/index.md#chunksexactmut)
- [`ChunksExact`](../slice/chunks/index.md#chunksexact)
- [`ChunksMut`](../slice/chunks/index.md#chunksmut)
- [`Chunks`](../slice/chunks/index.md#chunks)
- [`Chunks`](chunks/index.md#chunks)
- [`Cloned`](cloned/index.md#cloned)
- [`Copied`](copied/index.md#copied)
- [`Drain`](../collections/binary_heap/index.md#drain)
- [`Drain`](../collections/vec_deque/index.md#drain)
- [`Drain`](../vec/index.md#drain)
- [`Either`](#either)
- [`Empty`](empty/index.md#empty)
- [`Enumerate`](enumerate/index.md#enumerate)
- [`FoldChunksWith`](fold_chunks_with/index.md#foldchunkswith)
- [`FoldChunks`](fold_chunks/index.md#foldchunks)
- [`Inspect`](inspect/index.md#inspect)
- [`InterleaveShortest`](interleave_shortest/index.md#interleaveshortest)
- [`Interleave`](interleave/index.md#interleave)
- [`Intersperse`](intersperse/index.md#intersperse)
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
- [`MapInit`](map_with/index.md#mapinit)
- [`MapWith`](map_with/index.md#mapwith)
- [`Map`](map/index.md#map)
- [`MaxLen`](len/index.md#maxlen)
- [`MinLen`](len/index.md#minlen)
- [`MultiZip`](multizip/index.md#multizip)
- [`Once`](once/index.md#once)
- [`PanicFuse`](panic_fuse/index.md#panicfuse)
- [`RChunksExactMut`](../slice/rchunks/index.md#rchunksexactmut)
- [`RChunksExact`](../slice/rchunks/index.md#rchunksexact)
- [`RChunksMut`](../slice/rchunks/index.md#rchunksmut)
- [`RChunks`](../slice/rchunks/index.md#rchunks)
- [`RepeatN`](repeat/index.md#repeatn)
- [`Rev`](rev/index.md#rev)
- [`Skip`](skip/index.md#skip)
- [`StepBy`](step_by/index.md#stepby)
- [`Take`](take/index.md#take)
- [`Update`](update/index.md#update)
- [`Windows`](../slice/index.md#windows)
- [`ZipEq`](zip_eq/index.md#zipeq)
- [`Zip`](zip/index.md#zip)

### `FromParallelIterator<T>`

```rust
trait FromParallelIterator<T>
where
    T: Send { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3280-3303`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L3280-L3303)*

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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3333-3353`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L3333-L3353)*

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

- [`Collector`](unzip/index.md#collector)
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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3360-3394`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L3360-L3394)*

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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3400-3467`](../../../.source_1765521767/rayon-1.11.0/src/iter/mod.rs#L3400-L3467)*

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

### `empty`

```rust
fn empty<T: Send>() -> Empty<T>
```

*Defined in [`rayon-1.11.0/src/iter/empty.rs:24-28`](../../../.source_1765521767/rayon-1.11.0/src/iter/empty.rs#L24-L28)*

Creates a parallel iterator that produces nothing.

This admits no parallelism on its own, but it could be used for code that
deals with generic parallel iterators.

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::empty;

let pi = (0..1234).into_par_iter()
    .chain(empty())
    .chain(1234..10_000);

assert_eq!(pi.count(), 10_000);
```

### `once`

```rust
fn once<T: Send>(item: T) -> Once<T>
```

*Defined in [`rayon-1.11.0/src/iter/once.rs:24-26`](../../../.source_1765521767/rayon-1.11.0/src/iter/once.rs#L24-L26)*

Creates a parallel iterator that produces an element exactly once.

This admits no parallelism on its own, but it could be chained to existing
parallel iterators to extend their contents, or otherwise used for any code
that deals with generic parallel iterators.

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::once;

let pi = (0..1234).into_par_iter()
    .chain(once(-1))
    .chain(1234..10_000);

assert_eq!(pi.clone().count(), 10_001);
assert_eq!(pi.clone().filter(|&x| x < 0).count(), 1);
assert_eq!(pi.position_any(|x| x < 0), Some(1234));
```

### `repeat`

```rust
fn repeat<T: Clone + Send>(element: T) -> Repeat<T>
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:30-32`](../../../.source_1765521767/rayon-1.11.0/src/iter/repeat.rs#L30-L32)*

Creates a parallel iterator that endlessly repeats `element` (by
cloning it). Note that this iterator has "infinite" length, so
typically you would want to use `zip` or `take` or some other
means to shorten it, or consider using
[the `repeat_n()` function] instead.

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::repeat;
let x: Vec<(i32, i32)> = repeat(22).zip(0..3).collect();
assert_eq!(x, vec![(22, 0), (22, 1), (22, 2)]);
```

### `repeat_n`

```rust
fn repeat_n<T: Clone + Send>(element: T, n: usize) -> RepeatN<T>
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:126-132`](../../../.source_1765521767/rayon-1.11.0/src/iter/repeat.rs#L126-L132)*

Creates a parallel iterator that produces `n` repeats of `element`
(by cloning it).

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::repeat_n;
let x: Vec<(i32, i32)> = repeat_n(22, 3).zip(0..3).collect();
assert_eq!(x, vec![(22, 0), (22, 1), (22, 2)]);
```

### `split`

```rust
fn split<D, S>(data: D, splitter: S) -> Split<D, S>
where
    D: Send,
    S: Fn(D) -> (D, Option<D>) + Sync
```

*Defined in [`rayon-1.11.0/src/iter/splitter.rs:106-112`](../../../.source_1765521767/rayon-1.11.0/src/iter/splitter.rs#L106-L112)*

The `split` function takes arbitrary data and a closure that knows how to
split it, and turns this into a `ParallelIterator`.

# Examples

As a simple example, Rayon can recursively split ranges of indices

```rust
use rayon::iter;
use rayon::prelude::*;
use std::ops::Range;


// We define a range of indices as follows
type Range1D = Range<usize>;

// Splitting it in two can be done like this
fn split_range1(r: Range1D) -> (Range1D, Option<Range1D>) {
    // We are mathematically unable to split the range if there is only
    // one point inside of it, but we could stop splitting before that.
    if r.end - r.start <= 1 { return (r, None); }

    // Here, our range is considered large enough to be splittable
    let midpoint = r.start + (r.end - r.start) / 2;
    (r.start..midpoint, Some(midpoint..r.end))
}

// By using iter::split, Rayon will split the range until it has enough work
// to feed the CPU cores, then give us the resulting sub-ranges
iter::split(0..4096, split_range1).for_each(|sub_range| {
    // As our initial range had a power-of-two size, the final sub-ranges
    // should have power-of-two sizes too
    assert!((sub_range.end - sub_range.start).is_power_of_two());
});
```

This recursive splitting can be extended to two or three dimensions,
to reproduce a classic "block-wise" parallelization scheme of graphics and
numerical simulations:

```rust
use rayon::iter;
use rayon::prelude::*;
use std::ops::Range;
type Range1D = Range<usize>;
fn split_range1(r: Range1D) -> (Range1D, Option<Range1D>) {
    if r.end - r.start <= 1 { return (r, None); }
    let midpoint = r.start + (r.end - r.start) / 2;
    (r.start..midpoint, Some(midpoint..r.end))
}

// A two-dimensional range of indices can be built out of two 1D ones
struct Range2D {
    // Range of horizontal indices
    pub rx: Range1D,

    // Range of vertical indices
    pub ry: Range1D,
}

// We want to recursively split them by the largest dimension until we have
// enough sub-ranges to feed our mighty multi-core CPU. This function
// carries out one such split.
fn split_range2(r2: Range2D) -> (Range2D, Option<Range2D>) {
    // Decide on which axis (horizontal/vertical) the range should be split
    let width = r2.rx.end - r2.rx.start;
    let height = r2.ry.end - r2.ry.start;
    if width >= height {
        // This is a wide range, split it on the horizontal axis
        let (split_rx, ry) = (split_range1(r2.rx), r2.ry);
        let out1 = Range2D {
            rx: split_rx.0,
            ry: ry.clone(),
        };
        let out2 = split_rx.1.map(|rx| Range2D { rx, ry });
        (out1, out2)
    } else {
        // This is a tall range, split it on the vertical axis
        let (rx, split_ry) = (r2.rx, split_range1(r2.ry));
        let out1 = Range2D {
            rx: rx.clone(),
            ry: split_ry.0,
        };
        let out2 = split_ry.1.map(|ry| Range2D { rx, ry, });
        (out1, out2)
    }
}

// Again, rayon can handle the recursive splitting for us
let range = Range2D { rx: 0..800, ry: 0..600 };
iter::split(range, split_range2).for_each(|sub_range| {
    // If the sub-ranges were indeed split by the largest dimension, then
    // if no dimension was twice larger than the other initially, this
    // property will remain true in the final sub-ranges.
    let width = sub_range.rx.end - sub_range.rx.start;
    let height = sub_range.ry.end - sub_range.ry.start;
    assert!((width / 2 <= height) && (height / 2 <= width));
});
```


### `walk_tree`

```rust
fn walk_tree<S, B, I>(root: S, children_of: B) -> WalkTree<S, B>
where
    S: Send,
    B: Fn(&S) -> I + Send + Sync,
    I: IntoIterator<Item = S, IntoIter: DoubleEndedIterator>
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:497-508`](../../../.source_1765521767/rayon-1.11.0/src/iter/walk_tree.rs#L497-L508)*

Create a tree like parallel iterator from an initial root node.
The `children_of` function should take a node and iterate on all of its child nodes.
The best parallelization is obtained when the tree is balanced
but we should also be able to handle harder cases.

# Ordering

This function does not guarantee any ordering but will
use whatever algorithm is thought to achieve the fastest traversal.
See also [`walk_tree_prefix`](walk_tree/index.md) which guarantees a
prefix order and [`walk_tree_postfix`](walk_tree/index.md) which guarantees a postfix order.

# Example

```text
     4
    / \
   /   \
  2     3
       / \
      1   2
```

```rust
use rayon::iter::walk_tree;
use rayon::prelude::*;

let par_iter = walk_tree(4, |&e| {
    if e <= 2 {
        Vec::new()
    } else {
        vec![e / 2, e / 2 + 1]
    }
});
assert_eq!(par_iter.sum::<u32>(), 12);
```

### `walk_tree_postfix`

```rust
fn walk_tree_postfix<S, B, I>(root: S, children_of: B) -> WalkTreePostfix<S, B>
where
    S: Send,
    B: Fn(&S) -> I + Send + Sync,
    I: IntoIterator<Item = S>
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:444-454`](../../../.source_1765521767/rayon-1.11.0/src/iter/walk_tree.rs#L444-L454)*

Create a tree like postfix parallel iterator from an initial root node.
The `children_of` function should take a node and iterate on all of its child nodes.
The best parallelization is obtained when the tree is balanced
but we should also be able to handle harder cases.

# Ordering

This function guarantees a postfix ordering. See also [`walk_tree_prefix`](walk_tree/index.md) which guarantees a
prefix order. If you don't care about ordering, you should use [`walk_tree`](walk_tree/index.md), which will use
whatever is believed to be fastest.

Between siblings, children are reduced in order -- that is first children are reduced first.

For example a perfect binary tree of 7 nodes will reduced in the following order:

```text
     a
    / \
   /   \
  b     c
 / \   / \
d   e f   g

reduced as d,e,b,f,g,c,a

```

# Example

```text
     4
    / \
   /   \
  2     3
       / \
      1   2
```

```rust
use rayon::iter::walk_tree_postfix;
use rayon::prelude::*;

let par_iter = walk_tree_postfix(4, |&e| {
    if e <= 2 {
        Vec::new()
    } else {
        vec![e / 2, e / 2 + 1]
    }
});
assert_eq!(par_iter.sum::<u32>(), 12);
```

# Example

```rust
use rayon::prelude::*;
use rayon::iter::walk_tree_postfix;

struct Node {
    content: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Here we loop on the following tree:
//
//       10
//      /  \
//     /    \
//    3     14
//            \
//             \
//              18

let root = Node {
    content: 10,
    left: Some(Box::new(Node {
        content: 3,
        left: None,
        right: None,
    })),
    right: Some(Box::new(Node {
        content: 14,
        left: None,
        right: Some(Box::new(Node {
            content: 18,
            left: None,
            right: None,
        })),
    })),
};

let mut v: Vec<u32> = walk_tree_postfix(&root, |r| {
    r.left
        .as_ref()
        .into_iter()
        .chain(r.right.as_ref())
        .map(|n| &**n)
})
.map(|node| node.content)
.collect();
assert_eq!(v, vec![3, 18, 14, 10]);
```


### `walk_tree_prefix`

```rust
fn walk_tree_prefix<S, B, I>(root: S, children_of: B) -> WalkTreePrefix<S, B>
where
    S: Send,
    B: Fn(&S) -> I + Send + Sync,
    I: IntoIterator<Item = S, IntoIter: DoubleEndedIterator>
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:204-214`](../../../.source_1765521767/rayon-1.11.0/src/iter/walk_tree.rs#L204-L214)*

Create a tree-like prefix parallel iterator from an initial root node.
The `children_of` function should take a node and return an iterator over its child nodes.
The best parallelization is obtained when the tree is balanced
but we should also be able to handle harder cases.

# Ordering

This function guarantees a prefix ordering. See also [`walk_tree_postfix`](walk_tree/index.md),
which guarantees a postfix order.
If you don't care about ordering, you should use [`walk_tree`](walk_tree/index.md),
which will use whatever is believed to be fastest.
For example a perfect binary tree of 7 nodes will reduced in the following order:

```text
     a
    / \
   /   \
  b     c
 / \   / \
d   e f   g

reduced as a,b,d,e,c,f,g

```

# Example

```text
     4
    / \
   /   \
  2     3
       / \
      1   2
```

```rust
use rayon::iter::walk_tree_prefix;
use rayon::prelude::*;

let par_iter = walk_tree_prefix(4, |&e| {
    if e <= 2 {
        Vec::new()
    } else {
        vec![e / 2, e / 2 + 1]
    }
});
assert_eq!(par_iter.sum::<u32>(), 12);
```

# Example

```rust
use rayon::prelude::*;
use rayon::iter::walk_tree_prefix;

struct Node {
    content: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Here we loop on the following tree:
//
//       10
//      /  \
//     /    \
//    3     14
//            \
//             \
//              18

let root = Node {
    content: 10,
    left: Some(Box::new(Node {
        content: 3,
        left: None,
        right: None,
    })),
    right: Some(Box::new(Node {
        content: 14,
        left: None,
        right: Some(Box::new(Node {
            content: 18,
            left: None,
            right: None,
        })),
    })),
};

let mut v: Vec<u32> = walk_tree_prefix(&root, |r| {
    r.left
        .as_ref()
        .into_iter()
        .chain(r.right.as_ref())
        .map(|n| &**n)
})
.map(|node| node.content)
.collect();
assert_eq!(v, vec![10, 3, 14, 18]);
```


### `repeatn`

```rust
fn repeatn<T: Clone + Send>(element: T, n: usize) -> RepeatN<T>
```

*Defined in [`rayon-1.11.0/src/iter/repeat.rs:139-141`](../../../.source_1765521767/rayon-1.11.0/src/iter/repeat.rs#L139-L141)*

Creates a parallel iterator that produces `n` repeats of `element`
(by cloning it).

Deprecated in favor of [`repeat_n`](repeat/index.md) for consistency with the standard library.

