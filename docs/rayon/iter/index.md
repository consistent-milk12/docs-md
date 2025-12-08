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
combinator, then check out the [`split`](../str/index.md) function and the [`plumbing`](plumbing/index.md) module.



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

## Modules

- [`plumbing`](plumbing/index.md) - Traits and functions used to implement parallel iteration.  These are
- [`blocks`](blocks/index.md) - 
- [`chain`](chain/index.md) - 
- [`chunks`](chunks/index.md) - 
- [`cloned`](cloned/index.md) - 
- [`collect`](collect/index.md) - 
- [`copied`](copied/index.md) - 
- [`empty`](empty/index.md) - 
- [`enumerate`](enumerate/index.md) - 
- [`extend`](extend/index.md) - 
- [`filter`](filter/index.md) - 
- [`filter_map`](filter_map/index.md) - 
- [`find`](find/index.md) - 
- [`find_first_last`](find_first_last/index.md) - 
- [`flat_map`](flat_map/index.md) - 
- [`flat_map_iter`](flat_map_iter/index.md) - 
- [`flatten`](flatten/index.md) - 
- [`flatten_iter`](flatten_iter/index.md) - 
- [`fold`](fold/index.md) - 
- [`fold_chunks`](fold_chunks/index.md) - 
- [`fold_chunks_with`](fold_chunks_with/index.md) - 
- [`for_each`](for_each/index.md) - 
- [`from_par_iter`](from_par_iter/index.md) - 
- [`inspect`](inspect/index.md) - 
- [`interleave`](interleave/index.md) - 
- [`interleave_shortest`](interleave_shortest/index.md) - 
- [`intersperse`](intersperse/index.md) - 
- [`len`](len/index.md) - 
- [`map`](map/index.md) - 
- [`map_with`](map_with/index.md) - 
- [`multizip`](multizip/index.md) - 
- [`noop`](noop/index.md) - 
- [`once`](once/index.md) - 
- [`panic_fuse`](panic_fuse/index.md) - 
- [`par_bridge`](par_bridge/index.md) - 
- [`positions`](positions/index.md) - 
- [`product`](product/index.md) - 
- [`reduce`](reduce/index.md) - 
- [`repeat`](repeat/index.md) - 
- [`rev`](rev/index.md) - 
- [`skip`](skip/index.md) - 
- [`skip_any`](skip_any/index.md) - 
- [`skip_any_while`](skip_any_while/index.md) - 
- [`splitter`](splitter/index.md) - 
- [`step_by`](step_by/index.md) - 
- [`sum`](sum/index.md) - 
- [`take`](take/index.md) - 
- [`take_any`](take_any/index.md) - 
- [`take_any_while`](take_any_while/index.md) - 
- [`try_fold`](try_fold/index.md) - 
- [`try_reduce`](try_reduce/index.md) - 
- [`try_reduce_with`](try_reduce_with/index.md) - 
- [`unzip`](unzip/index.md) - 
- [`update`](update/index.md) - 
- [`walk_tree`](walk_tree/index.md) - 
- [`while_some`](while_some/index.md) - 
- [`zip`](zip/index.md) - 
- [`zip_eq`](zip_eq/index.md) - 
- [`private`](private/index.md) - We hide the `Try` trait in a private module, as it's only meant to be a

## Structs

### `ExponentialBlocks<I>`

```rust
struct ExponentialBlocks<I> {
    base: I,
}
```

`ExponentialBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of increasing sizes (exponentially).

This struct is created by the `by_exponential_blocks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for ExponentialBlocks<I>`

- `fn clone(self: &Self) -> ExponentialBlocks<I>` — [`ExponentialBlocks`](blocks/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for ExponentialBlocks<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for ExponentialBlocks<I>`

##### `impl<T> IntoParallelIterator for ExponentialBlocks<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for ExponentialBlocks<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for ExponentialBlocks<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `UniformBlocks<I>`

```rust
struct UniformBlocks<I> {
    base: I,
    block_size: usize,
}
```

`UniformBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of constant sizes.

This struct is created by the `by_uniform_blocks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I, block_size: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for UniformBlocks<I>`

- `fn clone(self: &Self) -> UniformBlocks<I>` — [`UniformBlocks`](blocks/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for UniformBlocks<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for UniformBlocks<I>`

##### `impl<T> IntoParallelIterator for UniformBlocks<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for UniformBlocks<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for UniformBlocks<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Chain<A, B>`

```rust
struct Chain<A, B> {
    a: A,
    b: B,
}
```

`Chain` is an iterator that joins `b` after `a` in one continuous iterator.
This struct is created by the `chain()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone, B: $crate::clone::Clone> Clone for Chain<A, B>`

- `fn clone(self: &Self) -> Chain<A, B>` — [`Chain`](chain/index.md)

##### `impl<A: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for Chain<A, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A, B> IndexedParallelIterator for Chain<A, B>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Chain<A, B>`

##### `impl<T> IntoParallelIterator for Chain<A, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<A, B> ParallelIterator for Chain<A, B>`

- `type Item = <A as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Chain<A, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Chunks<I>`

```rust
struct Chunks<I> {
    size: usize,
    i: I,
}
```

`Chunks` is an iterator that groups elements of an underlying iterator.

This struct is created by the `chunks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(i: I, size: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Chunks<I>`

- `fn clone(self: &Self) -> Chunks<I>` — [`Chunks`](chunks/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for Chunks<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for Chunks<I>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Chunks<I>`

##### `impl<T> IntoParallelIterator for Chunks<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Chunks<I>`

- `type Item = Vec<<I as ParallelIterator>::Item>`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Chunks<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Cloned<I>`

```rust
struct Cloned<I> {
    base: I,
}
```

`Cloned` is an iterator that clones the elements of an underlying iterator.

This struct is created by the `cloned()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Cloned<I>`

- `fn clone(self: &Self) -> Cloned<I>` — [`Cloned`](cloned/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for Cloned<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, T, I> IndexedParallelIterator for Cloned<I>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Cloned<I>`

##### `impl<T> IntoParallelIterator for Cloned<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'a, T, I> ParallelIterator for Cloned<I>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Cloned<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Copied<I>`

```rust
struct Copied<I> {
    base: I,
}
```

`Copied` is an iterator that copies the elements of an underlying iterator.

This struct is created by the `copied()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Copied<I>`

- `fn clone(self: &Self) -> Copied<I>` — [`Copied`](copied/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for Copied<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, T, I> IndexedParallelIterator for Copied<I>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Copied<I>`

##### `impl<T> IntoParallelIterator for Copied<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'a, T, I> ParallelIterator for Copied<I>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Copied<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Empty<T>`

```rust
struct Empty<T> {
    marker: std::marker::PhantomData<T>,
}
```

Iterator adaptor for [the `empty()` function].


#### Trait Implementations

##### `impl<T> Clone for Empty<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T: Send> Debug for Empty<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for Empty<T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Empty<T>`

##### `impl<T> IntoParallelIterator for Empty<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<T: Send> ParallelIterator for Empty<T>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Empty<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Enumerate<I>`

```rust
struct Enumerate<I> {
    base: I,
}
```

`Enumerate` is an iterator that returns the current count along with the element.
This struct is created by the `enumerate()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Enumerate<I>`

- `fn clone(self: &Self) -> Enumerate<I>` — [`Enumerate`](enumerate/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for Enumerate<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for Enumerate<I>`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Enumerate<I>`

##### `impl<T> IntoParallelIterator for Enumerate<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Enumerate<I>`

- `type Item = (usize, <I as ParallelIterator>::Item)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Enumerate<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Filter<I, P>`

```rust
struct Filter<I, P> {
    base: I,
    filter_op: P,
}
```

`Filter` takes a predicate `filter_op` and filters out elements that match.
This struct is created by the `filter()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, filter_op: P) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, P: $crate::clone::Clone> Clone for Filter<I, P>`

- `fn clone(self: &Self) -> Filter<I, P>` — [`Filter`](filter/index.md)

##### `impl<I: Debug, P> Debug for Filter<I, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Filter<I, P>`

##### `impl<T> IntoParallelIterator for Filter<I, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, P> ParallelIterator for Filter<I, P>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Filter<I, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FilterMap<I, P>`

```rust
struct FilterMap<I, P> {
    base: I,
    filter_op: P,
}
```

`FilterMap` creates an iterator that uses `filter_op` to both filter and map elements.
This struct is created by the `filter_map()` method on [`ParallelIterator`](#paralleliterator).


#### Implementations

- `fn new(base: I, filter_op: P) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, P: $crate::clone::Clone> Clone for FilterMap<I, P>`

- `fn clone(self: &Self) -> FilterMap<I, P>` — [`FilterMap`](filter_map/index.md)

##### `impl<I: Debug, P> Debug for FilterMap<I, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FilterMap<I, P>`

##### `impl<T> IntoParallelIterator for FilterMap<I, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, P, R> ParallelIterator for FilterMap<I, P>`

- `type Item = R`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for FilterMap<I, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FlatMap<I, F>`

```rust
struct FlatMap<I, F> {
    base: I,
    map_op: F,
}
```

`FlatMap` maps each element to a parallel iterator, then flattens these iterators together.
This struct is created by the `flat_map()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, F: $crate::clone::Clone> Clone for FlatMap<I, F>`

- `fn clone(self: &Self) -> FlatMap<I, F>` — [`FlatMap`](flat_map/index.md)

##### `impl<I: Debug, F> Debug for FlatMap<I, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FlatMap<I, F>`

##### `impl<T> IntoParallelIterator for FlatMap<I, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, F, PI> ParallelIterator for FlatMap<I, F>`

- `type Item = <PI as IntoParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for FlatMap<I, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FlatMapIter<I, F>`

```rust
struct FlatMapIter<I, F> {
    base: I,
    map_op: F,
}
```

`FlatMapIter` maps each element to a serial iterator, then flattens these iterators together.
This struct is created by the `flat_map_iter()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, F: $crate::clone::Clone> Clone for FlatMapIter<I, F>`

- `fn clone(self: &Self) -> FlatMapIter<I, F>` — [`FlatMapIter`](flat_map_iter/index.md)

##### `impl<I: Debug, F> Debug for FlatMapIter<I, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FlatMapIter<I, F>`

##### `impl<T> IntoParallelIterator for FlatMapIter<I, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, F, SI> ParallelIterator for FlatMapIter<I, F>`

- `type Item = <SI as IntoIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for FlatMapIter<I, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Flatten<I>`

```rust
struct Flatten<I> {
    base: I,
}
```

`Flatten` turns each element to a parallel iterator, then flattens these iterators
together. This struct is created by the `flatten()` method on [`ParallelIterator`](#paralleliterator).


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Flatten<I>`

- `fn clone(self: &Self) -> Flatten<I>` — [`Flatten`](flatten/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for Flatten<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for Flatten<I>`

##### `impl<T> IntoParallelIterator for Flatten<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Flatten<I>`

- `type Item = <<I as ParallelIterator>::Item as IntoParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Flatten<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FlattenIter<I>`

```rust
struct FlattenIter<I> {
    base: I,
}
```

`FlattenIter` turns each element to a serial iterator, then flattens these iterators
together. This struct is created by the `flatten_iter()` method on [`ParallelIterator`](#paralleliterator).


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for FlattenIter<I>`

- `fn clone(self: &Self) -> FlattenIter<I>` — [`FlattenIter`](flatten_iter/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for FlattenIter<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for FlattenIter<I>`

##### `impl<T> IntoParallelIterator for FlattenIter<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for FlattenIter<I>`

- `type Item = <<I as ParallelIterator>::Item as IntoIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for FlattenIter<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Fold<I, ID, F>`

```rust
struct Fold<I, ID, F> {
    base: I,
    identity: ID,
    fold_op: F,
}
```

`Fold` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, ID: $crate::clone::Clone, F: $crate::clone::Clone> Clone for Fold<I, ID, F>`

- `fn clone(self: &Self) -> Fold<I, ID, F>` — [`Fold`](fold/index.md)

##### `impl<I: Debug, ID, F> Debug for Fold<I, ID, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Fold<I, ID, F>`

##### `impl<T> IntoParallelIterator for Fold<I, ID, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<U, I, ID, F> ParallelIterator for Fold<I, ID, F>`

- `type Item = U`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Fold<I, ID, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FoldWith<I, U, F>`

```rust
struct FoldWith<I, U, F> {
    base: I,
    item: U,
    fold_op: F,
}
```

`FoldWith` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold_with()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, item: U, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, U: $crate::clone::Clone, F: $crate::clone::Clone> Clone for FoldWith<I, U, F>`

- `fn clone(self: &Self) -> FoldWith<I, U, F>` — [`FoldWith`](fold/index.md)

##### `impl<I: Debug, U: Debug, F> Debug for FoldWith<I, U, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FoldWith<I, U, F>`

##### `impl<T> IntoParallelIterator for FoldWith<I, U, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<U, I, F> ParallelIterator for FoldWith<I, U, F>`

- `type Item = U`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for FoldWith<I, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FoldChunks<I, ID, F>`

```rust
struct FoldChunks<I, ID, F> {
    base: I,
    chunk_size: usize,
    fold_op: F,
    identity: ID,
}
```

`FoldChunks` is an iterator that groups elements of an underlying iterator and applies a
function over them, producing a single value for each group.

This struct is created by the `fold_chunks()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I, chunk_size: usize, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, ID: $crate::clone::Clone, F: $crate::clone::Clone> Clone for FoldChunks<I, ID, F>`

- `fn clone(self: &Self) -> FoldChunks<I, ID, F>` — [`FoldChunks`](fold_chunks/index.md)

##### `impl<I: Debug, ID, F> Debug for FoldChunks<I, ID, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, ID, U, F> IndexedParallelIterator for FoldChunks<I, ID, F>`

- `fn len(self: &Self) -> usize`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for FoldChunks<I, ID, F>`

##### `impl<T> IntoParallelIterator for FoldChunks<I, ID, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, ID, U, F> ParallelIterator for FoldChunks<I, ID, F>`

- `type Item = U`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for FoldChunks<I, ID, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FoldChunksWith<I, U, F>`

```rust
struct FoldChunksWith<I, U, F> {
    base: I,
    chunk_size: usize,
    item: U,
    fold_op: F,
}
```

`FoldChunksWith` is an iterator that groups elements of an underlying iterator and applies a
function over them, producing a single value for each group.

This struct is created by the `fold_chunks_with()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I, chunk_size: usize, item: U, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, U: $crate::clone::Clone, F: $crate::clone::Clone> Clone for FoldChunksWith<I, U, F>`

- `fn clone(self: &Self) -> FoldChunksWith<I, U, F>` — [`FoldChunksWith`](fold_chunks_with/index.md)

##### `impl<I: Debug, U: Debug, F> Debug for FoldChunksWith<I, U, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, U, F> IndexedParallelIterator for FoldChunksWith<I, U, F>`

- `fn len(self: &Self) -> usize`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for FoldChunksWith<I, U, F>`

##### `impl<T> IntoParallelIterator for FoldChunksWith<I, U, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, U, F> ParallelIterator for FoldChunksWith<I, U, F>`

- `type Item = U`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for FoldChunksWith<I, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Inspect<I, F>`

```rust
struct Inspect<I, F> {
    base: I,
    inspect_op: F,
}
```

`Inspect` is an iterator that calls a function with a reference to each
element before yielding it.

This struct is created by the `inspect()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, inspect_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, F: $crate::clone::Clone> Clone for Inspect<I, F>`

- `fn clone(self: &Self) -> Inspect<I, F>` — [`Inspect`](inspect/index.md)

##### `impl<I: Debug, F> Debug for Inspect<I, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> IndexedParallelIterator for Inspect<I, F>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Inspect<I, F>`

##### `impl<T> IntoParallelIterator for Inspect<I, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, F> ParallelIterator for Inspect<I, F>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Inspect<I, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Interleave<I, J>`

```rust
struct Interleave<I, J> {
    i: I,
    j: J,
}
```

`Interleave` is an iterator that interleaves elements of iterators
`i` and `j` in one continuous iterator. This struct is created by
the `interleave()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(i: I, j: J) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, J: $crate::clone::Clone> Clone for Interleave<I, J>`

- `fn clone(self: &Self) -> Interleave<I, J>` — [`Interleave`](interleave/index.md)

##### `impl<I: $crate::fmt::Debug, J: $crate::fmt::Debug> Debug for Interleave<I, J>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I, J> IndexedParallelIterator for Interleave<I, J>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Interleave<I, J>`

##### `impl<T> IntoParallelIterator for Interleave<I, J>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, J> ParallelIterator for Interleave<I, J>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Interleave<I, J>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `InterleaveShortest<I, J>`

```rust
struct InterleaveShortest<I, J> {
    interleave: Interleave<Take<I>, Take<J>>,
}
```

`InterleaveShortest` is an iterator that works similarly to
`Interleave`, but this version stops returning elements once one
of the iterators run out.

This struct is created by the `interleave_shortest()` method on
[`IndexedParallelIterator`](#indexedparalleliterator).


#### Implementations

- `fn new(i: I, j: J) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, J: $crate::clone::Clone> Clone for InterleaveShortest<I, J>`

- `fn clone(self: &Self) -> InterleaveShortest<I, J>` — [`InterleaveShortest`](interleave_shortest/index.md)

##### `impl<I: $crate::fmt::Debug, J: $crate::fmt::Debug> Debug for InterleaveShortest<I, J>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I, J> IndexedParallelIterator for InterleaveShortest<I, J>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for InterleaveShortest<I, J>`

##### `impl<T> IntoParallelIterator for InterleaveShortest<I, J>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, J> ParallelIterator for InterleaveShortest<I, J>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for InterleaveShortest<I, J>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Intersperse<I>`

```rust
struct Intersperse<I>
where
    I: ParallelIterator {
    base: I,
    item: <I as >::Item,
}
```

`Intersperse` is an iterator that inserts a particular item between each
item of the adapted iterator.  This struct is created by the
`intersperse()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, item: <I as >::Item) -> Self` — [`ParallelIterator`](#paralleliterator)

#### Trait Implementations

##### `impl<I> Clone for Intersperse<I>`

- `fn clone(self: &Self) -> Intersperse<I>` — [`Intersperse`](intersperse/index.md)

##### `impl<I> Debug for Intersperse<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for Intersperse<I>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Intersperse<I>`

##### `impl<T> IntoParallelIterator for Intersperse<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Intersperse<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Intersperse<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MaxLen<I>`

```rust
struct MaxLen<I> {
    base: I,
    max: usize,
}
```

`MaxLen` is an iterator that imposes a maximum length on iterator splits.
This struct is created by the `with_max_len()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I, max: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for MaxLen<I>`

- `fn clone(self: &Self) -> MaxLen<I>` — [`MaxLen`](len/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for MaxLen<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for MaxLen<I>`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for MaxLen<I>`

##### `impl<T> IntoParallelIterator for MaxLen<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for MaxLen<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for MaxLen<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MinLen<I>`

```rust
struct MinLen<I> {
    base: I,
    min: usize,
}
```

`MinLen` is an iterator that imposes a minimum length on iterator splits.
This struct is created by the `with_min_len()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I, min: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for MinLen<I>`

- `fn clone(self: &Self) -> MinLen<I>` — [`MinLen`](len/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for MinLen<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for MinLen<I>`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for MinLen<I>`

##### `impl<T> IntoParallelIterator for MinLen<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for MinLen<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for MinLen<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Map<I, F>`

```rust
struct Map<I, F> {
    base: I,
    map_op: F,
}
```

`Map` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, F: $crate::clone::Clone> Clone for Map<I, F>`

- `fn clone(self: &Self) -> Map<I, F>` — [`Map`](map/index.md)

##### `impl<I: Debug, F> Debug for Map<I, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F, R> IndexedParallelIterator for Map<I, F>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Map<I, F>`

##### `impl<T> IntoParallelIterator for Map<I, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, F, R> ParallelIterator for Map<I, F>`

- `type Item = <F as FnOnce>::Output`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Map<I, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MapInit<I, INIT, F>`

```rust
struct MapInit<I, INIT, F> {
    base: I,
    init: INIT,
    map_op: F,
}
```

`MapInit` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_init()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, init: INIT, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, INIT: $crate::clone::Clone, F: $crate::clone::Clone> Clone for MapInit<I, INIT, F>`

- `fn clone(self: &Self) -> MapInit<I, INIT, F>` — [`MapInit`](map_with/index.md)

##### `impl<I: Debug, INIT, F> Debug for MapInit<I, INIT, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, INIT, T, F, R> IndexedParallelIterator for MapInit<I, INIT, F>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for MapInit<I, INIT, F>`

##### `impl<T> IntoParallelIterator for MapInit<I, INIT, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, INIT, T, F, R> ParallelIterator for MapInit<I, INIT, F>`

- `type Item = R`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for MapInit<I, INIT, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MapWith<I, T, F>`

```rust
struct MapWith<I, T, F> {
    base: I,
    item: T,
    map_op: F,
}
```

`MapWith` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_with()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, item: T, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, T: $crate::clone::Clone, F: $crate::clone::Clone> Clone for MapWith<I, T, F>`

- `fn clone(self: &Self) -> MapWith<I, T, F>` — [`MapWith`](map_with/index.md)

##### `impl<I: Debug, T: Debug, F> Debug for MapWith<I, T, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, T, F, R> IndexedParallelIterator for MapWith<I, T, F>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for MapWith<I, T, F>`

##### `impl<T> IntoParallelIterator for MapWith<I, T, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, T, F, R> ParallelIterator for MapWith<I, T, F>`

- `type Item = R`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for MapWith<I, T, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MultiZip<T>`

```rust
struct MultiZip<T> {
    tuple: T,
}
```

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

##### `impl<T: $crate::clone::Clone> Clone for MultiZip<T>`

- `fn clone(self: &Self) -> MultiZip<T>` — [`MultiZip`](multizip/index.md)

##### `impl<T: $crate::fmt::Debug> Debug for MultiZip<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A, B, C, D, E, F> IndexedParallelIterator for MultiZip<(A, B, C, D, E, F)>`

- `fn drive<CONSUMER>(self: Self, consumer: CONSUMER) -> <CONSUMER as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for MultiZip<T>`

##### `impl<T> IntoParallelIterator for MultiZip<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<A, B, C, D, E, F, G, H, I, J> ParallelIterator for MultiZip<(A, B, C, D, E, F, G, H, I, J)>`

- `type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item, <C as ParallelIterator>::Item, <D as ParallelIterator>::Item, <E as ParallelIterator>::Item, <F as ParallelIterator>::Item, <G as ParallelIterator>::Item, <H as ParallelIterator>::Item, <I as ParallelIterator>::Item, <J as ParallelIterator>::Item)`

- `fn drive_unindexed<CONSUMER>(self: Self, consumer: CONSUMER) -> <CONSUMER as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for MultiZip<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Once<T>`

```rust
struct Once<T> {
    item: T,
}
```

Iterator adaptor for [the `once()` function].


#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Once<T>`

- `fn clone(self: &Self) -> Once<T>` — [`Once`](once/index.md)

##### `impl<T: $crate::fmt::Debug> Debug for Once<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for Once<T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Once<T>`

##### `impl<T> IntoParallelIterator for Once<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<T: Send> ParallelIterator for Once<T>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Once<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `PanicFuse<I>`

```rust
struct PanicFuse<I> {
    base: I,
}
```

`PanicFuse` is an adaptor that wraps an iterator with a fuse in case
of panics, to halt all threads as soon as possible.

This struct is created by the `panic_fuse()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I) -> PanicFuse<I>` — [`PanicFuse`](panic_fuse/index.md)

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for PanicFuse<I>`

- `fn clone(self: &Self) -> PanicFuse<I>` — [`PanicFuse`](panic_fuse/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for PanicFuse<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for PanicFuse<I>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for PanicFuse<I>`

##### `impl<T> IntoParallelIterator for PanicFuse<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for PanicFuse<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for PanicFuse<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `IterBridge<Iter>`

```rust
struct IterBridge<Iter> {
    iter: Iter,
}
```

`IterBridge` is a parallel iterator that wraps a sequential iterator.

This type is created when using the `par_bridge` method on `ParallelBridge`. See the
[`ParallelBridge`](par_bridge/index.md) documentation for details.

#### Trait Implementations

##### `impl<Iter: $crate::clone::Clone> Clone for IterBridge<Iter>`

- `fn clone(self: &Self) -> IterBridge<Iter>` — [`IterBridge`](par_bridge/index.md)

##### `impl<Iter: $crate::fmt::Debug> Debug for IterBridge<Iter>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for IterBridge<Iter>`

##### `impl<T> IntoParallelIterator for IterBridge<Iter>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<Iter> ParallelIterator for IterBridge<Iter>`

- `type Item = <Iter as Iterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for IterBridge<Iter>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Positions<I, P>`

```rust
struct Positions<I, P> {
    base: I,
    predicate: P,
}
```

`Positions` takes a predicate `predicate` and filters out elements that match,
yielding their indices.

This struct is created by the `positions()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, P: $crate::clone::Clone> Clone for Positions<I, P>`

- `fn clone(self: &Self) -> Positions<I, P>` — [`Positions`](positions/index.md)

##### `impl<I: Debug, P> Debug for Positions<I, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Positions<I, P>`

##### `impl<T> IntoParallelIterator for Positions<I, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, P> ParallelIterator for Positions<I, P>`

- `type Item = usize`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Positions<I, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Repeat<T>`

```rust
struct Repeat<T> {
    element: T,
}
```

Iterator adaptor for [the `repeat()` function].


#### Implementations

- `fn take(self: Self, n: usize) -> RepeatN<T>` — [`RepeatN`](repeat/index.md)

- `fn zip<Z>(self: Self, zip_op: Z) -> Zip<RepeatN<T>, <Z as >::Iter>` — [`Zip`](zip/index.md), [`RepeatN`](repeat/index.md), [`IntoParallelIterator`](#intoparalleliterator)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Repeat<T>`

- `fn clone(self: &Self) -> Repeat<T>` — [`Repeat`](repeat/index.md)

##### `impl<T: $crate::fmt::Debug> Debug for Repeat<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for Repeat<T>`

##### `impl<T> IntoParallelIterator for Repeat<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<T> ParallelIterator for Repeat<T>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Repeat<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `RepeatN<T>`

```rust
struct RepeatN<T> {
    inner: RepeatNProducer<T>,
}
```

Iterator adaptor for [the `repeat_n()` function].


#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for RepeatN<T>`

- `fn clone(self: &Self) -> RepeatN<T>` — [`RepeatN`](repeat/index.md)

##### `impl<T: fmt::Debug> Debug for RepeatN<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IndexedParallelIterator for RepeatN<T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

##### `impl<T> IntoEither for RepeatN<T>`

##### `impl<T> IntoParallelIterator for RepeatN<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<T> ParallelIterator for RepeatN<T>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for RepeatN<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Rev<I>`

```rust
struct Rev<I> {
    base: I,
}
```

`Rev` is an iterator that produces elements in reverse order. This struct
is created by the `rev()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Rev<I>`

- `fn clone(self: &Self) -> Rev<I>` — [`Rev`](rev/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for Rev<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for Rev<I>`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Rev<I>`

##### `impl<T> IntoParallelIterator for Rev<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Rev<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Rev<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Skip<I>`

```rust
struct Skip<I> {
    base: I,
    n: usize,
}
```

`Skip` is an iterator that skips over the first `n` elements.
This struct is created by the `skip()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I, n: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Skip<I>`

- `fn clone(self: &Self) -> Skip<I>` — [`Skip`](skip/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for Skip<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for Skip<I>`

- `fn len(self: &Self) -> usize`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Skip<I>`

##### `impl<T> IntoParallelIterator for Skip<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Skip<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Skip<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `SkipAny<I>`

```rust
struct SkipAny<I> {
    base: I,
    count: usize,
}
```

`SkipAny` is an iterator that skips over `n` elements from anywhere in `I`.
This struct is created by the `skip_any()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, count: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for SkipAny<I>`

- `fn clone(self: &Self) -> SkipAny<I>` — [`SkipAny`](skip_any/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for SkipAny<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for SkipAny<I>`

##### `impl<T> IntoParallelIterator for SkipAny<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for SkipAny<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for SkipAny<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `SkipAnyWhile<I, P>`

```rust
struct SkipAnyWhile<I, P> {
    base: I,
    predicate: P,
}
```

`SkipAnyWhile` is an iterator that skips over elements from anywhere in `I`
until the callback returns `false`.
This struct is created by the `skip_any_while()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, P: $crate::clone::Clone> Clone for SkipAnyWhile<I, P>`

- `fn clone(self: &Self) -> SkipAnyWhile<I, P>` — [`SkipAnyWhile`](skip_any_while/index.md)

##### `impl<I: fmt::Debug, P> Debug for SkipAnyWhile<I, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SkipAnyWhile<I, P>`

##### `impl<T> IntoParallelIterator for SkipAnyWhile<I, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, P> ParallelIterator for SkipAnyWhile<I, P>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for SkipAnyWhile<I, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Split<D, S>`

```rust
struct Split<D, S> {
    data: D,
    splitter: S,
}
```

`Split` is a parallel iterator using arbitrary data and a splitting function.
This struct is created by the [`split()`](splitter/index.md) function.

#### Trait Implementations

##### `impl<D: $crate::clone::Clone, S: $crate::clone::Clone> Clone for Split<D, S>`

- `fn clone(self: &Self) -> Split<D, S>` — [`Split`](splitter/index.md)

##### `impl<D: Debug, S> Debug for Split<D, S>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Split<D, S>`

##### `impl<T> IntoParallelIterator for Split<D, S>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<D, S> ParallelIterator for Split<D, S>`

- `type Item = D`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for Split<D, S>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `StepBy<I>`

```rust
struct StepBy<I> {
    base: I,
    step: usize,
}
```

`StepBy` is an iterator that skips `n` elements between each yield, where `n` is the given step.
This struct is created by the `step_by()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I, step: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for StepBy<I>`

- `fn clone(self: &Self) -> StepBy<I>` — [`StepBy`](step_by/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for StepBy<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for StepBy<I>`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for StepBy<I>`

##### `impl<T> IntoParallelIterator for StepBy<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for StepBy<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for StepBy<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Take<I>`

```rust
struct Take<I> {
    base: I,
    n: usize,
}
```

`Take` is an iterator that iterates over the first `n` elements.
This struct is created by the `take()` method on [`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(base: I, n: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Take<I>`

- `fn clone(self: &Self) -> Take<I>` — [`Take`](take/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for Take<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for Take<I>`

- `fn len(self: &Self) -> usize`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Take<I>`

##### `impl<T> IntoParallelIterator for Take<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Take<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Take<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `TakeAny<I>`

```rust
struct TakeAny<I> {
    base: I,
    count: usize,
}
```

`TakeAny` is an iterator that iterates over `n` elements from anywhere in `I`.
This struct is created by the `take_any()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, count: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for TakeAny<I>`

- `fn clone(self: &Self) -> TakeAny<I>` — [`TakeAny`](take_any/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for TakeAny<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for TakeAny<I>`

##### `impl<T> IntoParallelIterator for TakeAny<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for TakeAny<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for TakeAny<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `TakeAnyWhile<I, P>`

```rust
struct TakeAnyWhile<I, P> {
    base: I,
    predicate: P,
}
```

`TakeAnyWhile` is an iterator that iterates over elements from anywhere in `I`
until the callback returns `false`.
This struct is created by the `take_any_while()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, P: $crate::clone::Clone> Clone for TakeAnyWhile<I, P>`

- `fn clone(self: &Self) -> TakeAnyWhile<I, P>` — [`TakeAnyWhile`](take_any_while/index.md)

##### `impl<I: fmt::Debug, P> Debug for TakeAnyWhile<I, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TakeAnyWhile<I, P>`

##### `impl<T> IntoParallelIterator for TakeAnyWhile<I, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, P> ParallelIterator for TakeAnyWhile<I, P>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for TakeAnyWhile<I, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `TryFold<I, U, ID, F>`

```rust
struct TryFold<I, U, ID, F> {
    base: I,
    identity: ID,
    fold_op: F,
    marker: std::marker::PhantomData<U>,
}
```

`TryFold` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `try_fold()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, U: $crate::clone::Clone, ID: $crate::clone::Clone, F: $crate::clone::Clone> Clone for TryFold<I, U, ID, F>`

- `fn clone(self: &Self) -> TryFold<I, U, ID, F>` — [`TryFold`](try_fold/index.md)

##### `impl<U, I: ParallelIterator + Debug, ID, F> Debug for TryFold<I, U, ID, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TryFold<I, U, ID, F>`

##### `impl<T> IntoParallelIterator for TryFold<I, U, ID, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<U, I, ID, F> ParallelIterator for TryFold<I, U, ID, F>`

- `type Item = U`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for TryFold<I, U, ID, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `TryFoldWith<I, U: Try, F>`

```rust
struct TryFoldWith<I, U: Try, F> {
    base: I,
    item: <U as >::Output,
    fold_op: F,
}
```

`TryFoldWith` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `try_fold_with()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, item: <U as >::Output, fold_op: F) -> Self` — [`Try`](private/index.md)

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, U: $crate::clone::Clone + Try, F: $crate::clone::Clone> Clone for TryFoldWith<I, U, F>`

- `fn clone(self: &Self) -> TryFoldWith<I, U, F>` — [`TryFoldWith`](try_fold/index.md)

##### `impl<I, U, F> Debug for TryFoldWith<I, U, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TryFoldWith<I, U, F>`

##### `impl<T> IntoParallelIterator for TryFoldWith<I, U, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<U, I, F> ParallelIterator for TryFoldWith<I, U, F>`

- `type Item = U`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for TryFoldWith<I, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Update<I, F>`

```rust
struct Update<I, F> {
    base: I,
    update_op: F,
}
```

`Update` is an iterator that mutates the elements of an
underlying iterator before they are yielded.

This struct is created by the `update()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I, update_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, F: $crate::clone::Clone> Clone for Update<I, F>`

- `fn clone(self: &Self) -> Update<I, F>` — [`Update`](update/index.md)

##### `impl<I: Debug, F> Debug for Update<I, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> IndexedParallelIterator for Update<I, F>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Update<I, F>`

##### `impl<T> IntoParallelIterator for Update<I, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, F> ParallelIterator for Update<I, F>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Update<I, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `WalkTree<S, B>`

```rust
struct WalkTree<S, B>(WalkTreePostfix<S, B>);
```

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree()`](walk_tree/index.md) function.

#### Trait Implementations

##### `impl<S: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for WalkTree<S, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for WalkTree<S, B>`

##### `impl<T> IntoParallelIterator for WalkTree<S, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTree<S, B>`

- `type Item = S`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for WalkTree<S, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `WalkTreePostfix<S, B>`

```rust
struct WalkTreePostfix<S, B> {
    initial_state: S,
    children_of: B,
}
```

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_postfix()`](walk_tree/index.md) function.

#### Trait Implementations

##### `impl<S: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for WalkTreePostfix<S, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for WalkTreePostfix<S, B>`

##### `impl<T> IntoParallelIterator for WalkTreePostfix<S, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTreePostfix<S, B>`

- `type Item = S`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for WalkTreePostfix<S, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `WalkTreePrefix<S, B>`

```rust
struct WalkTreePrefix<S, B> {
    initial_state: S,
    children_of: B,
}
```

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_prefix()`](walk_tree/index.md) function.

#### Trait Implementations

##### `impl<S: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for WalkTreePrefix<S, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for WalkTreePrefix<S, B>`

##### `impl<T> IntoParallelIterator for WalkTreePrefix<S, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTreePrefix<S, B>`

- `type Item = S`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for WalkTreePrefix<S, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `WhileSome<I>`

```rust
struct WhileSome<I> {
    base: I,
}
```

`WhileSome` is an iterator that yields the `Some` elements of an iterator,
halting as soon as any `None` is produced.

This struct is created by the `while_some()` method on [`ParallelIterator`](#paralleliterator)


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for WhileSome<I>`

- `fn clone(self: &Self) -> WhileSome<I>` — [`WhileSome`](while_some/index.md)

##### `impl<I: $crate::fmt::Debug> Debug for WhileSome<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for WhileSome<I>`

##### `impl<T> IntoParallelIterator for WhileSome<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, T> ParallelIterator for WhileSome<I>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

##### `impl<T> Pointable for WhileSome<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Zip<A, B>`

```rust
struct Zip<A, B> {
    a: A,
    b: B,
}
```

`Zip` is an iterator that zips up `a` and `b` into a single iterator
of pairs. This struct is created by the `zip()` method on
[`IndexedParallelIterator`](#indexedparalleliterator)


#### Implementations

- `fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone, B: $crate::clone::Clone> Clone for Zip<A, B>`

- `fn clone(self: &Self) -> Zip<A, B>` — [`Zip`](zip/index.md)

##### `impl<A: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for Zip<A, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A, B> IndexedParallelIterator for Zip<A, B>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for Zip<A, B>`

##### `impl<T> IntoParallelIterator for Zip<A, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<A, B> ParallelIterator for Zip<A, B>`

- `type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Zip<A, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ZipEq<A, B>`

```rust
struct ZipEq<A, B> {
    zip: Zip<A, B>,
}
```

An [`IndexedParallelIterator`](#indexedparalleliterator) that iterates over two parallel iterators of equal
length simultaneously.

This struct is created by the [`zip_eq`](zip_eq/index.md) method on [`IndexedParallelIterator`](#indexedparalleliterator),
see its documentation for more information.


#### Implementations

- `fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone, B: $crate::clone::Clone> Clone for ZipEq<A, B>`

- `fn clone(self: &Self) -> ZipEq<A, B>` — [`ZipEq`](zip_eq/index.md)

##### `impl<A: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for ZipEq<A, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A, B> IndexedParallelIterator for ZipEq<A, B>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](plumbing/index.md)

##### `impl<T> IntoEither for ZipEq<A, B>`

##### `impl<T> IntoParallelIterator for ZipEq<A, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<A, B> ParallelIterator for ZipEq<A, B>`

- `type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for ZipEq<A, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Traits

### `IntoParallelIterator`

```rust
trait IntoParallelIterator { ... }
```

`IntoParallelIterator` implements the conversion to a [`ParallelIterator`](#paralleliterator).

By implementing `IntoParallelIterator` for a type, you define how it will
transformed into an iterator. This is a parallel version of the standard
library's `std::iter::IntoIterator` trait.

#### Required Methods

- `type Iter: 1`

- `type Item: 1`

- `fn into_par_iter(self: Self) -> <Self as >::Iter`

  Converts `self` into a parallel iterator.

### `IntoParallelRefIterator<'data>`

```rust
trait IntoParallelRefIterator<'data> { ... }
```

`IntoParallelRefIterator` implements the conversion to a
[`ParallelIterator`](#paralleliterator), providing shared references to the data.

This is a parallel version of the `iter()` method
defined by various collections.

This trait is automatically implemented
`for I where &I: IntoParallelIterator`. In most cases, users
will want to implement [`IntoParallelIterator`](#intoparalleliterator) rather than implement
this trait directly.

#### Required Methods

- `type Iter: 1`

- `type Item: 2`

- `fn par_iter(self: &'data Self) -> <Self as >::Iter`

  Converts `self` into a parallel iterator.

### `IntoParallelRefMutIterator<'data>`

```rust
trait IntoParallelRefMutIterator<'data> { ... }
```

`IntoParallelRefMutIterator` implements the conversion to a
[`ParallelIterator`](#paralleliterator), providing mutable references to the data.

This is a parallel version of the `iter_mut()` method
defined by various collections.

This trait is automatically implemented
`for I where &mut I: IntoParallelIterator`. In most cases, users
will want to implement [`IntoParallelIterator`](#intoparalleliterator) rather than implement
this trait directly.

#### Required Methods

- `type Iter: 1`

- `type Item: 2`

- `fn par_iter_mut(self: &'data mut Self) -> <Self as >::Iter`

  Creates the parallel iterator from `self`.

### `ParallelIterator`

```rust
trait ParallelIterator: Sized + Send { ... }
```

Parallel version of the standard iterator trait.

The combinators on this trait are available on **all** parallel
iterators.  Additional methods can be found on the
[`IndexedParallelIterator`](#indexedparalleliterator) trait: those methods are only
available for parallel iterators where the number of items is
known in advance (so, e.g., after invoking `filter`, those methods
become unavailable).

For examples of using parallel iterators, see [the docs on the
`iter` module][iter](#iter).


#### Required Methods

- `type Item: 1`

- `fn for_each<OP>(self: Self, op: OP)`

  Executes `OP` on each item produced by the iterator, in parallel.

- `fn for_each_with<OP, T>(self: Self, init: T, op: OP)`

  Executes `OP` on the given `init` value with each item produced by

- `fn for_each_init<OP, INIT, T>(self: Self, init: INIT, op: OP)`

  Executes `OP` on a value returned by `init` with each item produced by

- `fn try_for_each<OP, R>(self: Self, op: OP) -> R`

  Executes a fallible `OP` on each item produced by the iterator, in parallel.

- `fn try_for_each_with<OP, T, R>(self: Self, init: T, op: OP) -> R`

  Executes a fallible `OP` on the given `init` value with each item

- `fn try_for_each_init<OP, INIT, T, R>(self: Self, init: INIT, op: OP) -> R`

  Executes a fallible `OP` on a value returned by `init` with each item

- `fn count(self: Self) -> usize`

  Counts the number of items in this parallel iterator.

- `fn map<F, R>(self: Self, map_op: F) -> Map<Self, F>`

  Applies `map_op` to each item of this iterator, producing a new

- `fn map_with<F, T, R>(self: Self, init: T, map_op: F) -> MapWith<Self, T, F>`

  Applies `map_op` to the given `init` value with each item of this

- `fn map_init<F, INIT, T, R>(self: Self, init: INIT, map_op: F) -> MapInit<Self, INIT, F>`

  Applies `map_op` to a value returned by `init` with each item of this

- `fn cloned<'a, T>(self: Self) -> Cloned<Self>`

  Creates an iterator which clones all of its elements.  This may be

- `fn copied<'a, T>(self: Self) -> Copied<Self>`

  Creates an iterator which copies all of its elements.  This may be

- `fn inspect<OP>(self: Self, inspect_op: OP) -> Inspect<Self, OP>`

  Applies `inspect_op` to a reference to each item of this iterator,

- `fn update<F>(self: Self, update_op: F) -> Update<Self, F>`

  Mutates each item of this iterator before yielding it.

- `fn filter<P>(self: Self, filter_op: P) -> Filter<Self, P>`

  Applies `filter_op` to each item of this iterator, producing a new

- `fn filter_map<P, R>(self: Self, filter_op: P) -> FilterMap<Self, P>`

  Applies `filter_op` to each item of this iterator to get an `Option`,

- `fn flat_map<F, PI>(self: Self, map_op: F) -> FlatMap<Self, F>`

  Applies `map_op` to each item of this iterator to get nested parallel iterators,

- `fn flat_map_iter<F, SI>(self: Self, map_op: F) -> FlatMapIter<Self, F>`

  Applies `map_op` to each item of this iterator to get nested serial iterators,

- `fn flatten(self: Self) -> Flatten<Self>`

  An adaptor that flattens parallel-iterable `Item`s into one large iterator.

- `fn flatten_iter(self: Self) -> FlattenIter<Self>`

  An adaptor that flattens serial-iterable `Item`s into one large iterator.

- `fn reduce<OP, ID>(self: Self, identity: ID, op: OP) -> <Self as >::Item`

  Reduces the items in the iterator into one item using `op`.

- `fn reduce_with<OP>(self: Self, op: OP) -> Option<<Self as >::Item>`

  Reduces the items in the iterator into one item using `op`.

- `fn try_reduce<T, OP, ID>(self: Self, identity: ID, op: OP) -> <Self as >::Item`

  Reduces the items in the iterator into one item using a fallible `op`.

- `fn try_reduce_with<T, OP>(self: Self, op: OP) -> Option<<Self as >::Item>`

  Reduces the items in the iterator into one item using a fallible `op`.

- `fn fold<T, ID, F>(self: Self, identity: ID, fold_op: F) -> Fold<Self, ID, F>`

  Parallel fold is similar to sequential fold except that the

- `fn fold_with<F, T>(self: Self, init: T, fold_op: F) -> FoldWith<Self, T, F>`

  Applies `fold_op` to the given `init` value with each item of this

- `fn try_fold<T, R, ID, F>(self: Self, identity: ID, fold_op: F) -> TryFold<Self, R, ID, F>`

  Performs a fallible parallel fold.

- `fn try_fold_with<F, T, R>(self: Self, init: T, fold_op: F) -> TryFoldWith<Self, R, F>`

  Performs a fallible parallel fold with a cloneable `init` value.

- `fn sum<S>(self: Self) -> S`

  Sums up the items in the iterator.

- `fn product<P>(self: Self) -> P`

  Multiplies all the items in the iterator.

- `fn min(self: Self) -> Option<<Self as >::Item>`

  Computes the minimum of all the items in the iterator. If the

- `fn min_by<F>(self: Self, f: F) -> Option<<Self as >::Item>`

  Computes the minimum of all the items in the iterator with respect to

- `fn min_by_key<K, F>(self: Self, f: F) -> Option<<Self as >::Item>`

  Computes the item that yields the minimum value for the given

- `fn max(self: Self) -> Option<<Self as >::Item>`

  Computes the maximum of all the items in the iterator. If the

- `fn max_by<F>(self: Self, f: F) -> Option<<Self as >::Item>`

  Computes the maximum of all the items in the iterator with respect to

- `fn max_by_key<K, F>(self: Self, f: F) -> Option<<Self as >::Item>`

  Computes the item that yields the maximum value for the given

- `fn chain<C>(self: Self, chain: C) -> Chain<Self, <C as >::Iter>`

  Takes two iterators and creates a new iterator over both.

- `fn find_any<P>(self: Self, predicate: P) -> Option<<Self as >::Item>`

  Searches for **some** item in the parallel iterator that

- `fn find_first<P>(self: Self, predicate: P) -> Option<<Self as >::Item>`

  Searches for the sequentially **first** item in the parallel iterator

- `fn find_last<P>(self: Self, predicate: P) -> Option<<Self as >::Item>`

  Searches for the sequentially **last** item in the parallel iterator

- `fn find_map_any<P, R>(self: Self, predicate: P) -> Option<R>`

  Applies the given predicate to the items in the parallel iterator

- `fn find_map_first<P, R>(self: Self, predicate: P) -> Option<R>`

  Applies the given predicate to the items in the parallel iterator and

- `fn find_map_last<P, R>(self: Self, predicate: P) -> Option<R>`

  Applies the given predicate to the items in the parallel iterator and

- `fn any<P>(self: Self, predicate: P) -> bool`

  Searches for **some** item in the parallel iterator that

- `fn all<P>(self: Self, predicate: P) -> bool`

  Tests that every item in the parallel iterator matches the given

- `fn while_some<T>(self: Self) -> WhileSome<Self>`

  Creates an iterator over the `Some` items of this iterator, halting

- `fn panic_fuse(self: Self) -> PanicFuse<Self>`

  Wraps an iterator with a fuse in case of panics, to halt all threads

- `fn collect<C>(self: Self) -> C`

  Creates a fresh collection containing all the elements produced

- `fn unzip<A, B, FromA, FromB>(self: Self) -> (FromA, FromB)`

  Unzips the items of a parallel iterator into a pair of arbitrary

- `fn partition<A, B, P>(self: Self, predicate: P) -> (A, B)`

  Partitions the items of a parallel iterator into a pair of arbitrary

- `fn partition_map<A, B, P, L, R>(self: Self, predicate: P) -> (A, B)`

  Partitions and maps the items of a parallel iterator into a pair of

- `fn intersperse(self: Self, element: <Self as >::Item) -> Intersperse<Self>`

  Intersperses clones of an element between items of this iterator.

- `fn take_any(self: Self, n: usize) -> TakeAny<Self>`

  Creates an iterator that yields `n` elements from *anywhere* in the original iterator.

- `fn skip_any(self: Self, n: usize) -> SkipAny<Self>`

  Creates an iterator that skips `n` elements from *anywhere* in the original iterator.

- `fn take_any_while<P>(self: Self, predicate: P) -> TakeAnyWhile<Self, P>`

  Creates an iterator that takes elements from *anywhere* in the original iterator

- `fn skip_any_while<P>(self: Self, predicate: P) -> SkipAnyWhile<Self, P>`

  Creates an iterator that skips elements from *anywhere* in the original iterator

- `fn collect_vec_list(self: Self) -> LinkedList<Vec<<Self as >::Item>>`

  Collects this iterator into a linked list of vectors.

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result`

  Internal method used to define the behavior of this parallel

- `fn opt_len(self: &Self) -> Option<usize>`

  Internal method used to define the behavior of this parallel

### `IndexedParallelIterator`

```rust
trait IndexedParallelIterator: ParallelIterator { ... }
```

An iterator that supports "random access" to its data, meaning
that you can split it at arbitrary indices and draw data from
those points.

**Note:** Not implemented for `u64`, `i64`, `u128`, or `i128` ranges

#### Required Methods

- `fn by_exponential_blocks(self: Self) -> ExponentialBlocks<Self>`

  Divides an iterator into sequential blocks of exponentially-increasing size.

- `fn by_uniform_blocks(self: Self, block_size: usize) -> UniformBlocks<Self>`

  Divides an iterator into sequential blocks of the given size.

- `fn collect_into_vec(self: Self, target: &mut Vec<<Self as >::Item>)`

  Collects the results of the iterator into the specified

- `fn unzip_into_vecs<A, B>(self: Self, left: &mut Vec<A>, right: &mut Vec<B>)`

  Unzips the results of the iterator into the specified

- `fn zip<Z>(self: Self, zip_op: Z) -> Zip<Self, <Z as >::Iter>`

  Iterates over tuples `(A, B)`, where the items `A` are from

- `fn zip_eq<Z>(self: Self, zip_op: Z) -> ZipEq<Self, <Z as >::Iter>`

  The same as `Zip`, but requires that both iterators have the same length.

- `fn interleave<I>(self: Self, other: I) -> Interleave<Self, <I as >::Iter>`

  Interleaves elements of this iterator and the other given

- `fn interleave_shortest<I>(self: Self, other: I) -> InterleaveShortest<Self, <I as >::Iter>`

  Interleaves elements of this iterator and the other given

- `fn chunks(self: Self, chunk_size: usize) -> Chunks<Self>`

  Splits an iterator up into fixed-size chunks.

- `fn fold_chunks<T, ID, F>(self: Self, chunk_size: usize, identity: ID, fold_op: F) -> FoldChunks<Self, ID, F>`

  Splits an iterator into fixed-size chunks, performing a sequential `fold()` on

- `fn fold_chunks_with<T, F>(self: Self, chunk_size: usize, init: T, fold_op: F) -> FoldChunksWith<Self, T, F>`

  Splits an iterator into fixed-size chunks, performing a sequential `fold()` on

- `fn cmp<I>(self: Self, other: I) -> Ordering`

  Lexicographically compares the elements of this `ParallelIterator` with those of

- `fn partial_cmp<I>(self: Self, other: I) -> Option<Ordering>`

  Lexicographically compares the elements of this `ParallelIterator` with those of

- `fn eq<I>(self: Self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn ne<I>(self: Self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn lt<I>(self: Self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn le<I>(self: Self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn gt<I>(self: Self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn ge<I>(self: Self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`

- `fn enumerate(self: Self) -> Enumerate<Self>`

  Yields an index along with each item.

- `fn step_by(self: Self, step: usize) -> StepBy<Self>`

   Creates an iterator that steps by the given amount

- `fn skip(self: Self, n: usize) -> Skip<Self>`

  Creates an iterator that skips the first `n` elements.

- `fn take(self: Self, n: usize) -> Take<Self>`

  Creates an iterator that yields the first `n` elements.

- `fn position_any<P>(self: Self, predicate: P) -> Option<usize>`

  Searches for **some** item in the parallel iterator that

- `fn position_first<P>(self: Self, predicate: P) -> Option<usize>`

  Searches for the sequentially **first** item in the parallel iterator

- `fn position_last<P>(self: Self, predicate: P) -> Option<usize>`

  Searches for the sequentially **last** item in the parallel iterator

- `fn positions<P>(self: Self, predicate: P) -> Positions<Self, P>`

  Searches for items in the parallel iterator that match the given

- `fn rev(self: Self) -> Rev<Self>`

  Produces a new iterator with the elements of this iterator in

- `fn with_min_len(self: Self, min: usize) -> MinLen<Self>`

  Sets the minimum length of iterators desired to process in each

- `fn with_max_len(self: Self, max: usize) -> MaxLen<Self>`

  Sets the maximum length of iterators desired to process in each

- `fn len(self: &Self) -> usize`

  Produces an exact count of how many items this iterator will

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result`

  Internal method used to define the behavior of this parallel

- `fn with_producer<CB: ProducerCallback<<Self as >::Item>>(self: Self, callback: CB) -> <CB as >::Output`

  Internal method used to define the behavior of this parallel

### `FromParallelIterator<T>`

```rust
trait FromParallelIterator<T>
where
    T: Send { ... }
```

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

### `ParallelExtend<T>`

```rust
trait ParallelExtend<T>
where
    T: Send { ... }
```

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

- `fn par_extend<I>(self: &mut Self, par_iter: I)`

  Extends an instance of the collection with the elements drawn

### `ParallelDrainFull`

```rust
trait ParallelDrainFull { ... }
```

`ParallelDrainFull` creates a parallel iterator that moves all items
from a collection while retaining the original capacity.

Types which are indexable typically implement [`ParallelDrainRange`](#paralleldrainrange)
instead, where you can drain fully with `par_drain(..)`.

#### Required Methods

- `type Iter: 1`

- `type Item: 1`

- `fn par_drain(self: Self) -> <Self as >::Iter`

  Returns a draining parallel iterator over an entire collection.

### `ParallelDrainRange<Idx>`

```rust
trait ParallelDrainRange<Idx> { ... }
```

`ParallelDrainRange` creates a parallel iterator that moves a range of items
from a collection while retaining the original capacity.

Types which are not indexable may implement [`ParallelDrainFull`](#paralleldrainfull) instead.

#### Required Methods

- `type Iter: 1`

- `type Item: 1`

- `fn par_drain<R: RangeBounds<Idx>>(self: Self, range: R) -> <Self as >::Iter`

  Returns a draining parallel iterator over a range of the collection.

## Functions

