# Crate `rayon`

Rayon is a data-parallelism library that makes it easy to convert sequential
computations into parallel.

It is lightweight and convenient for introducing parallelism into existing
code. It guarantees data-race free executions and takes advantage of
parallelism when sensible, based on work-load at runtime.

# How to use Rayon

There are two ways to use Rayon:

- **High-level parallel constructs** are the simplest way to use Rayon and also
  typically the most efficient.
  - [Parallel iterators] make it easy to convert a sequential iterator to
    execute in parallel.
    - The [`ParallelIterator`](iter/index.md) trait defines general methods for all parallel iterators.
    - The [`IndexedParallelIterator`](iter/index.md) trait adds methods for iterators that support random
      access.
  - The `par_sort` method sorts `&mut [T]` slices (or vectors) in parallel.
  - `par_extend` can be used to efficiently grow collections with items produced
    by a parallel iterator.
- **Custom tasks** let you divide your work into parallel tasks yourself.
  - [`join`](#join) is used to subdivide a task into two pieces.
  - [`scope`](#scope) creates a scope within which you can create any number of parallel tasks.
  - [`ThreadPoolBuilder`](#threadpoolbuilder) can be used to create your own thread pools or customize
    the global one.





# Basic usage and the Rayon prelude

First, you will need to add `rayon` to your `Cargo.toml`.

Next, to use parallel iterators or the other high-level methods,
you need to import several traits. Those traits are bundled into
the module `rayon::prelude`. It is recommended that you import
all of these traits at once by adding `use rayon::prelude::*` at
the top of each module that uses Rayon methods.

These traits give you access to the `par_iter` method which provides
parallel implementations of many iterative functions such as [`map`](iter/map/index.md),
[`for_each`](iter/for_each/index.md), [`filter`](iter/filter/index.md), [`fold`](iter/fold/index.md), and [more].






# Crate Layout

Rayon extends many of the types found in the standard library with
parallel iterator implementations. The modules in the `rayon`
crate mirror `std` itself: so, e.g., the `option` module in
Rayon contains parallel iterators for the `Option` type, which is
found in [the `option` module of `std`]. Similarly, the
`collections` module in Rayon offers parallel iterator types for
[the `collections` from `std`]. You will rarely need to access
these submodules unless you need to name iterator types
explicitly.


# Targets without threading

Rayon has limited support for targets without `std` threading implementations.
See the [`rayon_core`](../rayon_core/index.md) documentation for more information about its global fallback.

# Other questions?

See [the Rayon FAQ][faq].


## Contents

- [Modules](#modules)
  - [`delegate`](#delegate)
  - [`private`](#private)
  - [`split_producer`](#split-producer)
  - [`array`](#array)
  - [`collections`](#collections)
  - [`iter`](#iter)
  - [`option`](#option)
  - [`prelude`](#prelude)
  - [`range`](#range)
  - [`range_inclusive`](#range-inclusive)
  - [`result`](#result)
  - [`slice`](#slice)
  - [`str`](#str)
  - [`string`](#string)
  - [`vec`](#vec)
  - [`math`](#math)
  - [`par_either`](#par-either)
  - [`compile_fail`](#compile-fail)
- [Structs](#structs)
  - [`SendPtr`](#sendptr)
- [Functions](#functions)
  - [`FnContext`](#fncontext)
  - [`ThreadBuilder`](#threadbuilder)
  - [`ThreadPool`](#threadpool)
  - [`ThreadPoolBuildError`](#threadpoolbuilderror)
  - [`ThreadPoolBuilder`](#threadpoolbuilder)
  - [`broadcast`](#broadcast)
  - [`in_place_scope_fifo`](#in-place-scope-fifo)
  - [`scope_fifo`](#scope-fifo)
  - [`spawn`](#spawn)
  - [`Yield`](#yield)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`delegate`](#delegate) | mod | Macros for delegating newtype iterators to inner types. |
| [`private`](#private) | mod | The public parts of this private module are used to create traits that cannot be implemented outside of our own crate. |
| [`split_producer`](#split-producer) | mod | Common splitter for strings and slices |
| [`array`](#array) | mod | Parallel iterator types for [arrays] (`[T; N]`) |
| [`collections`](#collections) | mod | Parallel iterator types for [standard collections] |
| [`iter`](#iter) | mod | Traits for writing parallel programs using an iterator-style interface |
| [`option`](#option) | mod | Parallel iterator types for [options] |
| [`prelude`](#prelude) | mod | The rayon prelude imports the various `ParallelIterator` traits. |
| [`range`](#range) | mod | Parallel iterator types for [ranges], the type for values created by `a..b` expressions |
| [`range_inclusive`](#range-inclusive) | mod | Parallel iterator types for [inclusive ranges], the type for values created by `a..=b` expressions |
| [`result`](#result) | mod | Parallel iterator types for [results] |
| [`slice`](#slice) | mod | Parallel iterator types for [slices] |
| [`str`](#str) | mod | Parallel iterator types for [strings] |
| [`string`](#string) | mod | This module contains the parallel iterator types for owned strings (`String`). |
| [`vec`](#vec) | mod | Parallel iterator types for [vectors] (`Vec<T>`) |
| [`math`](#math) | mod |  |
| [`par_either`](#par-either) | mod |  |
| [`compile_fail`](#compile-fail) | mod |  |
| [`SendPtr`](#sendptr) | struct | We need to transmit raw pointers across threads. |
| [`FnContext`](#fncontext) | fn |  |
| [`ThreadBuilder`](#threadbuilder) | fn |  |
| [`ThreadPool`](#threadpool) | fn |  |
| [`ThreadPoolBuildError`](#threadpoolbuilderror) | fn |  |
| [`ThreadPoolBuilder`](#threadpoolbuilder) | fn |  |
| [`broadcast`](#broadcast) | fn |  |
| [`in_place_scope_fifo`](#in-place-scope-fifo) | fn |  |
| [`scope_fifo`](#scope-fifo) | fn |  |
| [`spawn`](#spawn) | fn |  |
| [`Yield`](#yield) | fn |  |

## Modules

- [`delegate`](delegate/index.md) — Macros for delegating newtype iterators to inner types.
- [`private`](private/index.md) — The public parts of this private module are used to create traits
- [`split_producer`](split_producer/index.md) — Common splitter for strings and slices
- [`array`](array/index.md) — Parallel iterator types for [arrays] (`[T; N]`)
- [`collections`](collections/index.md) — Parallel iterator types for [standard collections]
- [`iter`](iter/index.md) — Traits for writing parallel programs using an iterator-style interface
- [`option`](option/index.md) — Parallel iterator types for [options]
- [`prelude`](prelude/index.md) — The rayon prelude imports the various `ParallelIterator` traits.
- [`range`](range/index.md) — Parallel iterator types for [ranges],
- [`range_inclusive`](range_inclusive/index.md) — Parallel iterator types for [inclusive ranges],
- [`result`](result/index.md) — Parallel iterator types for [results]
- [`slice`](slice/index.md) — Parallel iterator types for [slices]
- [`str`](str/index.md) — Parallel iterator types for [strings]
- [`string`](string/index.md) — This module contains the parallel iterator types for owned strings
- [`vec`](vec/index.md) — Parallel iterator types for [vectors] (`Vec<T>`)
- [`math`](math/index.md)
- [`par_either`](par_either/index.md)
- [`compile_fail`](compile_fail/index.md)

## Structs

### `SendPtr<T>`

```rust
struct SendPtr<T>(*mut T);
```

*Defined in [`rayon-1.11.0/src/lib.rs:133`](../../.source_1765521767/rayon-1.11.0/src/lib.rs#L133)*

We need to transmit raw pointers across threads. It is possible to do this
without any unsafe code by converting pointers to usize or to AtomicPtr<T>
then back to a raw pointer for use. We prefer this approach because code
that uses this type is more explicit.

Unsafe code is still required to dereference the pointer, so this type is
not unsound on its own, although it does partly lift the unconditional
!Send and !Sync on raw pointers. As always, dereference with care.

#### Implementations

- <span id="sendptr-get"></span>`fn get(self) -> *mut T`

#### Trait Implementations

##### `impl<T> Clone for SendPtr<T>`

- <span id="sendptr-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Copy for SendPtr<T>`

##### `impl<T> IntoEither for SendPtr<T>`

##### `impl<T> Pointable for SendPtr<T>`

- <span id="sendptr-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sendptr-pointable-type-init"></span>`type Init = T`

- <span id="sendptr-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sendptr-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sendptr-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sendptr-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for SendPtr<T>`

##### `impl<T: Send> Sync for SendPtr<T>`

## Functions

### `FnContext`

```rust
fn FnContext(&self) -> Format
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:418-420`](../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L418-L420)*

Whether this compilation unit is encoded in 64- or 32-bit DWARF.

### `ThreadBuilder`

```rust
fn ThreadBuilder(&self, offset: UnitOffset<Offset>) -> bool
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:427-435`](../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L427-L435)*

### `ThreadPool`

```rust
fn ThreadPool(&self, idx: RangeFrom<UnitOffset<Offset>>) -> Result<R>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:456-464`](../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L456-L464)*

Get the underlying bytes for the supplied range.

### `ThreadPoolBuildError`

```rust
fn ThreadPoolBuildError<'me, 'abbrev>(self: &'me Self, abbreviations: &'abbrev Abbreviations, offset: UnitOffset<Offset>) -> Result<DebuggingInformationEntry<'abbrev, 'me, R>>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:478-486`](../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L478-L486)*

Read the `DebuggingInformationEntry` at the given offset.

### `ThreadPoolBuilder`

```rust
fn ThreadPoolBuilder<'me, 'abbrev>(self: &'me Self, abbreviations: &'abbrev Abbreviations, offset: UnitOffset<Offset>) -> Result<EntriesCursor<'abbrev, 'me, R>>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:504-517`](../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L504-L517)*

Navigate this compilation unit's `DebuggingInformationEntry`s
starting at the given offset.

### `broadcast`

```rust
fn broadcast<'me, 'abbrev>(self: &'me Self, abbreviations: &'abbrev Abbreviations, offset: Option<UnitOffset<Offset>>) -> Result<EntriesRaw<'abbrev, 'me, R>>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:534-549`](../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L534-L549)*

Read the raw data that defines the Debugging Information Entries.

### `in_place_scope_fifo`

```rust
fn in_place_scope_fifo(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:302`](../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L302)*

### `scope_fifo`

```rust
fn scope_fifo(&self) -> UnitHeader<R, Offset>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:302`](../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L302)*

### `spawn`

```rust
fn spawn<R, Offset>(input: &mut R, unit_offset: crate::common::UnitSectionOffset<Offset>) -> crate::read::Result<UnitHeader<R>>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:558-636`](../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L558-L636)*

Parse a unit header.

### `Yield`

```rust
fn Yield(offset: UnitOffset<Offset>, attrs_slice: R, abbrev: &'abbrev Abbreviation, unit: &'unit UnitHeader<R, Offset>) -> Self
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:665-678`](../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L665-L678)*

Construct a new `DebuggingInformationEntry`.

