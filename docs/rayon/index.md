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
    - The [`ParallelIterator`](prelude/index.md) trait defines general methods for all parallel iterators.
    - The [`IndexedParallelIterator`](prelude/index.md) trait adds methods for iterators that support random
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


## Modules

- [`delegate`](delegate/index.md) - Macros for delegating newtype iterators to inner types.
- [`private`](private/index.md) - The public parts of this private module are used to create traits
- [`split_producer`](split_producer/index.md) - Common splitter for strings and slices
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
- [`math`](math/index.md) - 
- [`par_either`](par_either/index.md) - 
- [`compile_fail`](compile_fail/index.md) - 

## Structs

### `SendPtr<T>`

```rust
struct SendPtr<T>(*mut T);
```

We need to transmit raw pointers across threads. It is possible to do this
without any unsafe code by converting pointers to usize or to AtomicPtr<T>
then back to a raw pointer for use. We prefer this approach because code
that uses this type is more explicit.

Unsafe code is still required to dereference the pointer, so this type is
not unsound on its own, although it does partly lift the unconditional
!Send and !Sync on raw pointers. As always, dereference with care.

#### Implementations

- `fn get(self: Self) -> *mut T`

#### Trait Implementations

##### `impl<T> Clone for SendPtr<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T> Copy for SendPtr<T>`

##### `impl<T> IntoEither for SendPtr<T>`

##### `impl<T> Pointable for SendPtr<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for SendPtr<T>`

##### `impl<T: Send> Sync for SendPtr<T>`

## Functions

