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
  - The [`par_sort`](#par-sort) method sorts `&mut [T]` slices (or vectors) in parallel.
  - [`par_extend`](#par-extend) can be used to efficiently grow collections with items produced
    by a parallel iterator.
- **Custom tasks** let you divide your work into parallel tasks yourself.
  - [`join`](#join) is used to subdivide a task into two pieces.
  - [`scope`](#scope) creates a scope within which you can create any number of parallel tasks.
  - [`ThreadPoolBuilder`](#threadpoolbuilder) can be used to create your own thread pools or customize
    the global one.

[Parallel iterators]: iter




# Basic usage and the Rayon prelude

First, you will need to add `rayon` to your `Cargo.toml`.

Next, to use parallel iterators or the other high-level methods,
you need to import several traits. Those traits are bundled into
the module `rayon::prelude`. It is recommended that you import
all of these traits at once by adding `use rayon::prelude::*` at
the top of each module that uses Rayon methods.

These traits give you access to the `par_iter` method which provides
parallel implementations of many iterative functions such as [`map`](iter/map/index.md),
[`for_each`](iter/for_each/index.md), [`filter`](iter/filter/index.md), [`fold`](iter/fold/index.md), and [more](#more).





[more](#more): iter::ParallelIterator#provided-methods

# Crate Layout

Rayon extends many of the types found in the standard library with
parallel iterator implementations. The modules in the `rayon`
crate mirror [`std`](#std) itself: so, e.g., the `option` module in
Rayon contains parallel iterators for the `Option` type, which is
found in [the `option` module of `std`]. Similarly, the
`collections` module in Rayon offers parallel iterator types for
[the `collections` from `std`]. You will rarely need to access
these submodules unless you need to name iterator types
explicitly.

[the `option` module of `std`]: std::option
[the `collections` from `std`]: std::collections

# Targets without threading

Rayon has limited support for targets without `std` threading implementations.
See the [`rayon_core`](#rayon-core) documentation for more information about its global fallback.

# Other questions?

See [the Rayon FAQ][faq](#faq).

[faq](#faq): https://github.com/rayon-rs/rayon/blob/main/FAQ.md

## Modules

- [`array`](array/index.md) - Parallel iterator types for [arrays] (`[T; N]`)
- [`collections`](collections/index.md) - Parallel iterator types for [standard collections]
- [`iter`](iter/index.md) - Traits for writing parallel programs using an iterator-style interface
- [`option`](option/index.md) - Parallel iterator types for [options]
- [`prelude`](prelude/index.md) - The rayon prelude imports the various `ParallelIterator` traits.
- [`range`](range/index.md) - Parallel iterator types for [ranges],
- [`range_inclusive`](range_inclusive/index.md) - Parallel iterator types for [inclusive ranges],
- [`result`](result/index.md) - Parallel iterator types for [results]
- [`slice`](slice/index.md) - Parallel iterator types for [slices]
- [`str`](str/index.md) - Parallel iterator types for [strings]
- [`string`](string/index.md) - This module contains the parallel iterator types for owned strings
- [`vec`](vec/index.md) - Parallel iterator types for [vectors] (`Vec<T>`)

## Functions

