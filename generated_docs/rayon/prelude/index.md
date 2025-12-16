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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3280-3303`](../../../.source_1765900590/rayon-1.11.0/src/iter/mod.rs#L3280-L3303)*

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

- `fn FromParallelIterator::from_par_iter<I>(par_iter: I) -> Self`

  Creates an instance of the collection from the parallel iterator `par_iter`.
  
  If your collection is not naturally parallel, the easiest (and
  fastest) way to do this is often to collect `par_iter` into a
  `LinkedList` (via `collect_vec_list`) or another intermediate
  data structure and then sequentially extend your collection. However,
  a more 'native' technique is to use the `par_iter.fold` or
  `par_iter.fold_with` methods to create the collection.
  Alternatively, if your collection is 'natively' parallel, you
  can use `par_iter.for_each` to process each element in turn.
  
  
  
  

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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:2439-3244`](../../../.source_1765900590/rayon-1.11.0/src/iter/mod.rs#L2439-L3244)*

An iterator that supports "random access" to its data, meaning
that you can split it at arbitrary indices and draw data from
those points.

**Note:** Not implemented for `u64`, `i64`, `u128`, or `i128` ranges

<details>
<summary><strong>Methods (33)</strong> - click to expand</summary>

**Required:**
- [`IndexedParallelIterator::len`](#fn-indexedparalleliteratorlen)
- [`IndexedParallelIterator::drive`](#fn-indexedparalleliteratordrive)
- [`IndexedParallelIterator::with_producer`](#fn-indexedparalleliteratorwith-producer)

**Provided:**
- [`IndexedParallelIterator::by_exponential_blocks`](#fn-indexedparalleliteratorby-exponential-blocks)
- [`IndexedParallelIterator::by_uniform_blocks`](#fn-indexedparalleliteratorby-uniform-blocks)
- [`IndexedParallelIterator::collect_into_vec`](#fn-indexedparalleliteratorcollect-into-vec)
- [`IndexedParallelIterator::unzip_into_vecs`](#fn-indexedparalleliteratorunzip-into-vecs)
- [`IndexedParallelIterator::zip`](#fn-indexedparalleliteratorzip)
- [`IndexedParallelIterator::zip_eq`](#fn-indexedparalleliteratorzip-eq)
- [`IndexedParallelIterator::interleave`](#fn-indexedparalleliteratorinterleave)
- [`IndexedParallelIterator::interleave_shortest`](#fn-indexedparalleliteratorinterleave-shortest)
- [`IndexedParallelIterator::chunks`](#fn-indexedparalleliteratorchunks)
- [`IndexedParallelIterator::fold_chunks`](#fn-indexedparalleliteratorfold-chunks)
- [`IndexedParallelIterator::fold_chunks_with`](#fn-indexedparalleliteratorfold-chunks-with)
- [`IndexedParallelIterator::cmp`](#fn-indexedparalleliteratorcmp)
- [`IndexedParallelIterator::partial_cmp`](#fn-indexedparalleliteratorpartial-cmp)
- [`IndexedParallelIterator::eq`](#fn-indexedparalleliteratoreq)
- [`IndexedParallelIterator::ne`](#fn-indexedparalleliteratorne)
- [`IndexedParallelIterator::lt`](#fn-indexedparalleliteratorlt)
- [`IndexedParallelIterator::le`](#fn-indexedparalleliteratorle)
- [`IndexedParallelIterator::gt`](#fn-indexedparalleliteratorgt)
- [`IndexedParallelIterator::ge`](#fn-indexedparalleliteratorge)
- [`IndexedParallelIterator::enumerate`](#fn-indexedparalleliteratorenumerate)
- [`IndexedParallelIterator::step_by`](#fn-indexedparalleliteratorstep-by)
- [`IndexedParallelIterator::skip`](#fn-indexedparalleliteratorskip)
- [`IndexedParallelIterator::take`](#fn-indexedparalleliteratortake)
- [`IndexedParallelIterator::position_any`](#fn-indexedparalleliteratorposition-any)
- [`IndexedParallelIterator::position_first`](#fn-indexedparalleliteratorposition-first)
- [`IndexedParallelIterator::position_last`](#fn-indexedparalleliteratorposition-last)
- [`IndexedParallelIterator::positions`](#fn-indexedparalleliteratorpositions)
- [`IndexedParallelIterator::rev`](#fn-indexedparalleliteratorrev)
- [`IndexedParallelIterator::with_min_len`](#fn-indexedparalleliteratorwith-min-len)
- [`IndexedParallelIterator::with_max_len`](#fn-indexedparalleliteratorwith-max-len)

</details>

#### Required Methods

- `fn IndexedParallelIterator::len(&self) -> usize`

  Produces an exact count of how many items this iterator will
  produce, presuming no panic occurs.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let par_iter = (0..100).into_par_iter().zip(vec![0; 10]);
  assert_eq!(par_iter.len(), 10);
  
  let vec: Vec<_> = par_iter.collect();
  assert_eq!(vec.len(), 10);
  ```

- `fn IndexedParallelIterator::drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result`

  Internal method used to define the behavior of this parallel
  iterator. You should not need to call this directly.
  
  This method causes the iterator `self` to start producing
  items and to feed them to the consumer `consumer` one by one.
  It may split the consumer before doing so to create the
  opportunity to produce in parallel. If a split does happen, it
  will inform the consumer of the index where the split should
  occur (unlike `ParallelIterator::drive_unindexed()`).
  
  See the [README] for more details on the internals of parallel
  iterators.

- `fn IndexedParallelIterator::with_producer<CB: ProducerCallback<<Self as >::Item>>(self, callback: CB) -> <CB as >::Output`

  Internal method used to define the behavior of this parallel
  iterator. You should not need to call this directly.
  
  This method converts the iterator into a producer P and then
  invokes `callback.callback()` with P. Note that the type of
  this producer is not defined as part of the API, since
  `callback` must be defined generically for all producers. This
  allows the producer type to contain references; it also means
  that parallel iterators can adjust that type without causing a
  breaking change.
  
  See the [README] for more details on the internals of parallel
  iterators.

#### Provided Methods

- `fn IndexedParallelIterator::by_exponential_blocks(self) -> ExponentialBlocks<Self>`

  Divides an iterator into sequential blocks of exponentially-increasing size.
  
  Normally, parallel iterators are recursively divided into tasks in parallel.
  This adaptor changes the default behavior by splitting the iterator into a **sequence**
  of parallel iterators of increasing sizes.
  Sizes grow exponentially in order to avoid creating
  too many blocks. This also allows to balance the current block with all previous ones.
  
  This can have many applications but the most notable ones are:
  - better performance with `find_first()`
  - more predictable performance with `find_any()`
    or any interruptible computation
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  assert_eq!((0..10_000).into_par_iter()
                        .by_exponential_blocks()
                        .find_first(|&e| e==4_999), Some(4_999))
  ```
  
  In this example, without blocks, rayon will split the initial range into two but all work
  on the right hand side (from 5,000 onwards) is **useless** since the sequential algorithm
  never goes there. This means that if two threads are used there will be **no** speedup **at
  all**.
  
  `by_exponential_blocks` on the other hand will start with the leftmost range from 0
  to `p` (threads number), continue with p to 3p, the 3p to 7p...
  
  Each subrange is treated in parallel, while all subranges are treated sequentially.
  We therefore ensure a logarithmic number of blocks (and overhead) while guaranteeing
  we stop at the first block containing the searched data.

- `fn IndexedParallelIterator::by_uniform_blocks(self, block_size: usize) -> UniformBlocks<Self>`

  Divides an iterator into sequential blocks of the given size.
  
  Normally, parallel iterators are recursively divided into tasks in parallel.
  This adaptor changes the default behavior by splitting the iterator into a **sequence**
  of parallel iterators of given `block_size`.
  The main application is to obtain better
  memory locality (especially if the reduce operation re-use folded data).
  
  **Panics** if `block_size` is 0.
  
  ##### Example
  ```rust
  use rayon::prelude::*;
  // during most reductions v1 and v2 fit the cache
  let v = (0u32..10_000_000)
      .into_par_iter()
      .by_uniform_blocks(1_000_000)
      .fold(Vec::new, |mut v, e| { v.push(e); v})
      .reduce(Vec::new, |mut v1, mut v2| { v1.append(&mut v2); v1});
  assert_eq!(v, (0u32..10_000_000).collect::<Vec<u32>>());
  ```

- `fn IndexedParallelIterator::collect_into_vec(self, target: &mut Vec<<Self as >::Item>)`

  Collects the results of the iterator into the specified
  vector. The vector is always cleared before execution
  begins. If possible, reusing the vector across calls can lead
  to better performance since it reuses the same backing buffer.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  // any prior data will be cleared
  let mut vec = vec![-1, -2, -3];
  
  (0..5).into_par_iter()
      .collect_into_vec(&mut vec);
  
  assert_eq!(vec, [0, 1, 2, 3, 4]);
  ```

- `fn IndexedParallelIterator::unzip_into_vecs<A, B>(self, left: &mut Vec<A>, right: &mut Vec<B>)`

  Unzips the results of the iterator into the specified
  vectors. The vectors are always cleared before execution
  begins. If possible, reusing the vectors across calls can lead
  to better performance since they reuse the same backing buffer.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  // any prior data will be cleared
  let mut left = vec![42; 10];
  let mut right = vec![-1; 10];
  
  (10..15).into_par_iter()
      .enumerate()
      .unzip_into_vecs(&mut left, &mut right);
  
  assert_eq!(left, [0, 1, 2, 3, 4]);
  assert_eq!(right, [10, 11, 12, 13, 14]);
  ```

- `fn IndexedParallelIterator::zip<Z>(self, zip_op: Z) -> Zip<Self, <Z as >::Iter>`

  Iterates over tuples `(A, B)`, where the items `A` are from
  this iterator and `B` are from the iterator given as argument.
  Like the `zip` method on ordinary iterators, if the two
  iterators are of unequal length, you only get the items they
  have in common.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let result: Vec<_> = (1..4)
      .into_par_iter()
      .zip(vec!['a', 'b', 'c'])
      .collect();
  
  assert_eq!(result, [(1, 'a'), (2, 'b'), (3, 'c')]);
  ```

- `fn IndexedParallelIterator::zip_eq<Z>(self, zip_op: Z) -> ZipEq<Self, <Z as >::Iter>`

  The same as `Zip`, but requires that both iterators have the same length.
  
  ##### Panics
  Will panic if `self` and `zip_op` are not the same length.
  
  ```should_panic
  use rayon::prelude::*;
  
  let one = [1u8];
  let two = [2u8, 2];
  let one_iter = one.par_iter();
  let two_iter = two.par_iter();
  
  // this will panic
  let zipped: Vec<(&u8, &u8)> = one_iter.zip_eq(two_iter).collect();
  
  // we should never get here
  assert_eq!(1, zipped.len());
  ```

- `fn IndexedParallelIterator::interleave<I>(self, other: I) -> Interleave<Self, <I as >::Iter>`

  Interleaves elements of this iterator and the other given
  iterator. Alternately yields elements from this iterator and
  the given iterator, until both are exhausted. If one iterator
  is exhausted before the other, the last elements are provided
  from the other.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let (x, y) = (vec![1, 2], vec![3, 4, 5, 6]);
  let r: Vec<i32> = x.into_par_iter().interleave(y).collect();
  assert_eq!(r, vec![1, 3, 2, 4, 5, 6]);
  ```

- `fn IndexedParallelIterator::interleave_shortest<I>(self, other: I) -> InterleaveShortest<Self, <I as >::Iter>`

  Interleaves elements of this iterator and the other given
  iterator, until one is exhausted.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let (x, y) = (vec![1, 2, 3, 4], vec![5, 6]);
  let r: Vec<i32> = x.into_par_iter().interleave_shortest(y).collect();
  assert_eq!(r, vec![1, 5, 2, 6, 3]);
  ```

- `fn IndexedParallelIterator::chunks(self, chunk_size: usize) -> Chunks<Self>`

  Splits an iterator up into fixed-size chunks.
  
  Returns an iterator that returns `Vec`s of the given number of elements.
  If the number of elements in the iterator is not divisible by `chunk_size`,
  the last chunk may be shorter than `chunk_size`.
  
  See also `par_chunks()` and `par_chunks_mut()` for similar behavior on
  slices, without having to allocate intermediate `Vec`s for the chunks.
  
  
  **Panics** if `chunk_size` is 0.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let r: Vec<Vec<i32>> = a.into_par_iter().chunks(3).collect();
  assert_eq!(r, vec![vec![1,2,3], vec![4,5,6], vec![7,8,9], vec![10]]);
  ```

- `fn IndexedParallelIterator::fold_chunks<T, ID, F>(self, chunk_size: usize, identity: ID, fold_op: F) -> FoldChunks<Self, ID, F>`

  Splits an iterator into fixed-size chunks, performing a sequential `fold()` on
  each chunk.
  
  Returns an iterator that produces a folded result for each chunk of items
  produced by this iterator.
  
  This works essentially like:
  
  ```text
  iter.chunks(chunk_size)
      .map(|chunk|
          chunk.into_iter()
              .fold(identity, fold_op)
      )
  ```
  
  except there is no per-chunk allocation overhead.
  
  **Panics** if `chunk_size` is 0.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let chunk_sums = nums.into_par_iter().fold_chunks(2, || 0, |a, n| a + n).collect::<Vec<_>>();
  assert_eq!(chunk_sums, vec![3, 7, 11, 15, 19]);
  ```

- `fn IndexedParallelIterator::fold_chunks_with<T, F>(self, chunk_size: usize, init: T, fold_op: F) -> FoldChunksWith<Self, T, F>`

  Splits an iterator into fixed-size chunks, performing a sequential `fold()` on
  each chunk.
  
  Returns an iterator that produces a folded result for each chunk of items
  produced by this iterator.
  
  This works essentially like `fold_chunks(chunk_size, || init.clone(), fold_op)`,
  except it doesn't require the `init` type to be `Sync`, nor any other form of
  added synchronization.
  
  **Panics** if `chunk_size` is 0.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let chunk_sums = nums.into_par_iter().fold_chunks_with(2, 0, |a, n| a + n).collect::<Vec<_>>();
  assert_eq!(chunk_sums, vec![3, 7, 11, 15, 19]);
  ```

- `fn IndexedParallelIterator::cmp<I>(self, other: I) -> Ordering`

  Lexicographically compares the elements of this `ParallelIterator` with those of
  another.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  use std::cmp::Ordering::*;
  
  let x = vec![1, 2, 3];
  assert_eq!(x.par_iter().cmp(&vec![1, 3, 0]), Less);
  assert_eq!(x.par_iter().cmp(&vec![1, 2, 3]), Equal);
  assert_eq!(x.par_iter().cmp(&vec![1, 2]), Greater);
  ```

- `fn IndexedParallelIterator::partial_cmp<I>(self, other: I) -> Option<Ordering>`

  Lexicographically compares the elements of this `ParallelIterator` with those of
  another.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  use std::cmp::Ordering::*;
  
  let x = vec![1.0, 2.0, 3.0];
  assert_eq!(x.par_iter().partial_cmp(&vec![1.0, 3.0, 0.0]), Some(Less));
  assert_eq!(x.par_iter().partial_cmp(&vec![1.0, 2.0, 3.0]), Some(Equal));
  assert_eq!(x.par_iter().partial_cmp(&vec![1.0, 2.0]), Some(Greater));
  assert_eq!(x.par_iter().partial_cmp(&vec![1.0, f64::NAN]), None);
  ```

- `fn IndexedParallelIterator::eq<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`
  are equal to those of another

- `fn IndexedParallelIterator::ne<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`
  are unequal to those of another

- `fn IndexedParallelIterator::lt<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`
  are lexicographically less than those of another.

- `fn IndexedParallelIterator::le<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`
  are less than or equal to those of another.

- `fn IndexedParallelIterator::gt<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`
  are lexicographically greater than those of another.

- `fn IndexedParallelIterator::ge<I>(self, other: I) -> bool`

  Determines if the elements of this `ParallelIterator`
  are greater than or equal to those of another.

- `fn IndexedParallelIterator::enumerate(self) -> Enumerate<Self>`

  Yields an index along with each item.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let chars = vec!['a', 'b', 'c'];
  let result: Vec<_> = chars
      .into_par_iter()
      .enumerate()
      .collect();
  
  assert_eq!(result, [(0, 'a'), (1, 'b'), (2, 'c')]);
  ```

- `fn IndexedParallelIterator::step_by(self, step: usize) -> StepBy<Self>`

   Creates an iterator that steps by the given amount
  
   # Examples
  
   ```rust
  use rayon::prelude::*;
  
   let range = (3..10);
   let result: Vec<i32> = range
      .into_par_iter()
      .step_by(3)
      .collect();
  
   assert_eq!(result, [3, 6, 9])
   ```

- `fn IndexedParallelIterator::skip(self, n: usize) -> Skip<Self>`

  Creates an iterator that skips the first `n` elements.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let result: Vec<_> = (0..100)
      .into_par_iter()
      .skip(95)
      .collect();
  
  assert_eq!(result, [95, 96, 97, 98, 99]);
  ```

- `fn IndexedParallelIterator::take(self, n: usize) -> Take<Self>`

  Creates an iterator that yields the first `n` elements.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let result: Vec<_> = (0..100)
      .into_par_iter()
      .take(5)
      .collect();
  
  assert_eq!(result, [0, 1, 2, 3, 4]);
  ```

- `fn IndexedParallelIterator::position_any<P>(self, predicate: P) -> Option<usize>`

  Searches for **some** item in the parallel iterator that
  matches the given predicate, and returns its index.  Like
  `ParallelIterator::find_any`, the parallel search will not
  necessarily find the **first** match, and once a match is
  found we'll attempt to stop processing any more.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [1, 2, 3, 3];
  
  let i = a.par_iter().position_any(|&x| x == 3).expect("found");
  assert!(i == 2 || i == 3);
  
  assert_eq!(a.par_iter().position_any(|&x| x == 100), None);
  ```

- `fn IndexedParallelIterator::position_first<P>(self, predicate: P) -> Option<usize>`

  Searches for the sequentially **first** item in the parallel iterator
  that matches the given predicate, and returns its index.
  
  Like `ParallelIterator::find_first`, once a match is found,
  all attempts to the right of the match will be stopped, while
  attempts to the left must continue in case an earlier match
  is found.
  
  Note that not all parallel iterators have a useful order, much like
  sequential `HashMap` iteration, so "first" may be nebulous.  If you
  just want the first match that discovered anywhere in the iterator,
  `position_any` is a better choice.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [1, 2, 3, 3];
  
  assert_eq!(a.par_iter().position_first(|&x| x == 3), Some(2));
  
  assert_eq!(a.par_iter().position_first(|&x| x == 100), None);
  ```

- `fn IndexedParallelIterator::position_last<P>(self, predicate: P) -> Option<usize>`

  Searches for the sequentially **last** item in the parallel iterator
  that matches the given predicate, and returns its index.
  
  Like `ParallelIterator::find_last`, once a match is found,
  all attempts to the left of the match will be stopped, while
  attempts to the right must continue in case a later match
  is found.
  
  Note that not all parallel iterators have a useful order, much like
  sequential `HashMap` iteration, so "last" may be nebulous.  When the
  order doesn't actually matter to you, `position_any` is a better
  choice.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [1, 2, 3, 3];
  
  assert_eq!(a.par_iter().position_last(|&x| x == 3), Some(3));
  
  assert_eq!(a.par_iter().position_last(|&x| x == 100), None);
  ```

- `fn IndexedParallelIterator::positions<P>(self, predicate: P) -> Positions<Self, P>`

  Searches for items in the parallel iterator that match the given
  predicate, and returns their indices.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
  
  // Find the positions of primes congruent to 1 modulo 6
  let p1mod6: Vec<_> = primes.par_iter().positions(|&p| p % 6 == 1).collect();
  assert_eq!(p1mod6, [3, 5, 7]); // primes 7, 13, and 19
  
  // Find the positions of primes congruent to 5 modulo 6
  let p5mod6: Vec<_> = primes.par_iter().positions(|&p| p % 6 == 5).collect();
  assert_eq!(p5mod6, [2, 4, 6, 8, 9]); // primes 5, 11, 17, 23, and 29
  ```

- `fn IndexedParallelIterator::rev(self) -> Rev<Self>`

  Produces a new iterator with the elements of this iterator in
  reverse order.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let result: Vec<_> = (0..5)
      .into_par_iter()
      .rev()
      .collect();
  
  assert_eq!(result, [4, 3, 2, 1, 0]);
  ```

- `fn IndexedParallelIterator::with_min_len(self, min: usize) -> MinLen<Self>`

  Sets the minimum length of iterators desired to process in each
  rayon job.  Rayon will not split any smaller than this length, but
  of course an iterator could already be smaller to begin with.
  
  Producers like `zip` and `interleave` will use greater of the two
  minimums.
  Chained iterators and iterators inside `flat_map` may each use
  their own minimum length.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let min = (0..1_000_000)
      .into_par_iter()
      .with_min_len(1234)
      .fold(|| 0, |acc, _| acc + 1) // count how many are in this segment
      .min().unwrap();
  
  assert!(min >= 1234);
  ```

- `fn IndexedParallelIterator::with_max_len(self, max: usize) -> MaxLen<Self>`

  Sets the maximum length of iterators desired to process in each
  rayon job.  Rayon will try to split at least below this length,
  unless that would put it below the length from `with_min_len()`.
  For example, given min=10 and max=15, a length of 16 will not be
  split any further.
  
  Producers like `zip` and `interleave` will use lesser of the two
  maximums.
  Chained iterators and iterators inside `flat_map` may each use
  their own maximum length.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let max = (0..1_000_000)
      .into_par_iter()
      .with_max_len(1234)
      .fold(|| 0, |acc, _| acc + 1) // count how many are in this segment
      .max().unwrap();
  
  assert!(max <= 1234);
  ```

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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:219-249`](../../../.source_1765900590/rayon-1.11.0/src/iter/mod.rs#L219-L249)*

`IntoParallelIterator` implements the conversion to a [`ParallelIterator`](../iter/index.md).

By implementing `IntoParallelIterator` for a type, you define how it will
transformed into an iterator. This is a parallel version of the standard
library's `std::iter::IntoIterator` trait.

#### Associated Types

- `type Iter: 1`

- `type Item: 1`

#### Required Methods

- `fn IntoParallelIterator::into_par_iter(self) -> <Self as >::Iter`

  Converts `self` into a parallel iterator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  println!("counting in parallel:");
  (0..100).into_par_iter()
      .for_each(|i| println!("{}", i));
  ```
  
  This conversion is often implicit for arguments to methods like [`zip`](../iter/zip/index.md).
  
  ```rust
  use rayon::prelude::*;
  
  let v: Vec<_> = (0..5).into_par_iter().zip(5..10).collect();
  assert_eq!(v, [(0, 5), (1, 6), (2, 7), (3, 8), (4, 9)]);
  ```

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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:261-285`](../../../.source_1765900590/rayon-1.11.0/src/iter/mod.rs#L261-L285)*

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

- `fn IntoParallelRefIterator::par_iter(self: &'data Self) -> <Self as >::Iter`

  Converts `self` into a parallel iterator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let v: Vec<_> = (0..100).collect();
  assert_eq!(v.par_iter().sum::<i32>(), 100 * 99 / 2);
  
  // `v.par_iter()` is shorthand for `(&v).into_par_iter()`,
  // producing the exact same references.
  assert!(v.par_iter().zip(&v)
           .all(|(a, b)| std::ptr::eq(a, b)));
  ```

#### Implementors

- `I`

### `IntoParallelRefMutIterator<'data>`

```rust
trait IntoParallelRefMutIterator<'data> { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:309-329`](../../../.source_1765900590/rayon-1.11.0/src/iter/mod.rs#L309-L329)*

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

- `fn IntoParallelRefMutIterator::par_iter_mut(self: &'data mut Self) -> <Self as >::Iter`

  Creates the parallel iterator from `self`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut v = vec![0usize; 5];
  v.par_iter_mut().enumerate().for_each(|(i, x)| *x = i);
  assert_eq!(v, [0, 1, 2, 3, 4]);
  ```

#### Implementors

- `I`

### `ParallelBridge`

```rust
trait ParallelBridge: Sized { ... }
```

*Defined in [`rayon-1.11.0/src/iter/par_bridge.rs:53-56`](../../../.source_1765900590/rayon-1.11.0/src/iter/par_bridge.rs#L53-L56)*

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

- `fn ParallelBridge::par_bridge(self) -> IterBridge<Self>`

  Creates a bridge from this type to a `ParallelIterator`.

#### Implementors

- `T`

### `ParallelDrainFull`

```rust
trait ParallelDrainFull { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3360-3394`](../../../.source_1765900590/rayon-1.11.0/src/iter/mod.rs#L3360-L3394)*

`ParallelDrainFull` creates a parallel iterator that moves all items
from a collection while retaining the original capacity.

Types which are indexable typically implement [`ParallelDrainRange`](../iter/index.md)
instead, where you can drain fully with `par_drain(..)`.

#### Associated Types

- `type Iter: 1`

- `type Item: 1`

#### Required Methods

- `fn ParallelDrainFull::par_drain(self) -> <Self as >::Iter`

  Returns a draining parallel iterator over an entire collection.
  
  When the iterator is dropped, all items are removed, even if the
  iterator was not fully consumed. If the iterator is leaked, for example
  using `std::mem::forget`, it is unspecified how many items are removed.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  use std::collections::{BinaryHeap, HashSet};
  
  let squares: HashSet<i32> = (0..10).map(|x| x * x).collect();
  
  let mut heap: BinaryHeap<_> = squares.iter().copied().collect();
  assert_eq!(
      // heaps are drained in arbitrary order
      heap.par_drain()
          .inspect(|x| assert!(squares.contains(x)))
          .count(),
      squares.len(),
  );
  assert!(heap.is_empty());
  assert!(heap.capacity() >= squares.len());
  ```

#### Implementors

- `&'a mut std::collections::BinaryHeap<T>`
- `&'a mut std::collections::HashMap<K, V, S>`
- `&'a mut std::collections::HashSet<T, S>`

### `ParallelDrainRange<Idx>`

```rust
trait ParallelDrainRange<Idx> { ... }
```

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3400-3467`](../../../.source_1765900590/rayon-1.11.0/src/iter/mod.rs#L3400-L3467)*

`ParallelDrainRange` creates a parallel iterator that moves a range of items
from a collection while retaining the original capacity.

Types which are not indexable may implement [`ParallelDrainFull`](../iter/index.md) instead.

#### Associated Types

- `type Iter: 1`

- `type Item: 1`

#### Required Methods

- `fn ParallelDrainRange::par_drain<R: RangeBounds<Idx>>(self, range: R) -> <Self as >::Iter`

  Returns a draining parallel iterator over a range of the collection.
  
  When the iterator is dropped, all items in the range are removed, even
  if the iterator was not fully consumed. If the iterator is leaked, for
  example using `std::mem::forget`, it is unspecified how many items are
  removed.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let squares: Vec<i32> = (0..10).map(|x| x * x).collect();
  
  println!("RangeFull");
  let mut vec = squares.clone();
  assert!(vec.par_drain(..)
             .eq(squares.par_iter().copied()));
  assert!(vec.is_empty());
  assert!(vec.capacity() >= squares.len());
  
  println!("RangeFrom");
  let mut vec = squares.clone();
  assert!(vec.par_drain(5..)
             .eq(squares[5..].par_iter().copied()));
  assert_eq!(&vec[..], &squares[..5]);
  assert!(vec.capacity() >= squares.len());
  
  println!("RangeTo");
  let mut vec = squares.clone();
  assert!(vec.par_drain(..5)
             .eq(squares[..5].par_iter().copied()));
  assert_eq!(&vec[..], &squares[5..]);
  assert!(vec.capacity() >= squares.len());
  
  println!("RangeToInclusive");
  let mut vec = squares.clone();
  assert!(vec.par_drain(..=5)
             .eq(squares[..=5].par_iter().copied()));
  assert_eq!(&vec[..], &squares[6..]);
  assert!(vec.capacity() >= squares.len());
  
  println!("Range");
  let mut vec = squares.clone();
  assert!(vec.par_drain(3..7)
             .eq(squares[3..7].par_iter().copied()));
  assert_eq!(&vec[..3], &squares[..3]);
  assert_eq!(&vec[3..], &squares[7..]);
  assert!(vec.capacity() >= squares.len());
  
  println!("RangeInclusive");
  let mut vec = squares.clone();
  assert!(vec.par_drain(3..=7)
             .eq(squares[3..=7].par_iter().copied()));
  assert_eq!(&vec[..3], &squares[..3]);
  assert_eq!(&vec[3..], &squares[8..]);
  assert!(vec.capacity() >= squares.len());
  ```

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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3333-3353`](../../../.source_1765900590/rayon-1.11.0/src/iter/mod.rs#L3333-L3353)*

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

- `fn ParallelExtend::par_extend<I>(&mut self, par_iter: I)`

  Extends an instance of the collection with the elements drawn
  from the parallel iterator `par_iter`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut vec = vec![];
  vec.par_extend(0..5);
  vec.par_extend((0..5).into_par_iter().map(|i| i * i));
  assert_eq!(vec, [0, 1, 2, 3, 4, 0, 1, 4, 9, 16]);
  ```

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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:356-2421`](../../../.source_1765900590/rayon-1.11.0/src/iter/mod.rs#L356-L2421)*

Parallel version of the standard iterator trait.

The combinators on this trait are available on **all** parallel
iterators.  Additional methods can be found on the
[`IndexedParallelIterator`](../iter/index.md) trait: those methods are only
available for parallel iterators where the number of items is
known in advance (so, e.g., after invoking `filter`, those methods
become unavailable).

For examples of using parallel iterators, see [the docs on the
`iter` module][`iter`](../iter/index.md).


<details>
<summary><strong>Methods (59)</strong> - click to expand</summary>

**Required:**
- [`ParallelIterator::drive_unindexed`](#fn-paralleliteratordrive-unindexed)

**Provided:**
- [`ParallelIterator::for_each`](#fn-paralleliteratorfor-each)
- [`ParallelIterator::for_each_with`](#fn-paralleliteratorfor-each-with)
- [`ParallelIterator::for_each_init`](#fn-paralleliteratorfor-each-init)
- [`ParallelIterator::try_for_each`](#fn-paralleliteratortry-for-each)
- [`ParallelIterator::try_for_each_with`](#fn-paralleliteratortry-for-each-with)
- [`ParallelIterator::try_for_each_init`](#fn-paralleliteratortry-for-each-init)
- [`ParallelIterator::count`](#fn-paralleliteratorcount)
- [`ParallelIterator::map`](#fn-paralleliteratormap)
- [`ParallelIterator::map_with`](#fn-paralleliteratormap-with)
- [`ParallelIterator::map_init`](#fn-paralleliteratormap-init)
- [`ParallelIterator::cloned`](#fn-paralleliteratorcloned)
- [`ParallelIterator::copied`](#fn-paralleliteratorcopied)
- [`ParallelIterator::inspect`](#fn-paralleliteratorinspect)
- [`ParallelIterator::update`](#fn-paralleliteratorupdate)
- [`ParallelIterator::filter`](#fn-paralleliteratorfilter)
- [`ParallelIterator::filter_map`](#fn-paralleliteratorfilter-map)
- [`ParallelIterator::flat_map`](#fn-paralleliteratorflat-map)
- [`ParallelIterator::flat_map_iter`](#fn-paralleliteratorflat-map-iter)
- [`ParallelIterator::flatten`](#fn-paralleliteratorflatten)
- [`ParallelIterator::flatten_iter`](#fn-paralleliteratorflatten-iter)
- [`ParallelIterator::reduce`](#fn-paralleliteratorreduce)
- [`ParallelIterator::reduce_with`](#fn-paralleliteratorreduce-with)
- [`ParallelIterator::try_reduce`](#fn-paralleliteratortry-reduce)
- [`ParallelIterator::try_reduce_with`](#fn-paralleliteratortry-reduce-with)
- [`ParallelIterator::fold`](#fn-paralleliteratorfold)
- [`ParallelIterator::fold_with`](#fn-paralleliteratorfold-with)
- [`ParallelIterator::try_fold`](#fn-paralleliteratortry-fold)
- [`ParallelIterator::try_fold_with`](#fn-paralleliteratortry-fold-with)
- [`ParallelIterator::sum`](#fn-paralleliteratorsum)
- [`ParallelIterator::product`](#fn-paralleliteratorproduct)
- [`ParallelIterator::min`](#fn-paralleliteratormin)
- [`ParallelIterator::min_by`](#fn-paralleliteratormin-by)
- [`ParallelIterator::min_by_key`](#fn-paralleliteratormin-by-key)
- [`ParallelIterator::max`](#fn-paralleliteratormax)
- [`ParallelIterator::max_by`](#fn-paralleliteratormax-by)
- [`ParallelIterator::max_by_key`](#fn-paralleliteratormax-by-key)
- [`ParallelIterator::chain`](#fn-paralleliteratorchain)
- [`ParallelIterator::find_any`](#fn-paralleliteratorfind-any)
- [`ParallelIterator::find_first`](#fn-paralleliteratorfind-first)
- [`ParallelIterator::find_last`](#fn-paralleliteratorfind-last)
- [`ParallelIterator::find_map_any`](#fn-paralleliteratorfind-map-any)
- [`ParallelIterator::find_map_first`](#fn-paralleliteratorfind-map-first)
- [`ParallelIterator::find_map_last`](#fn-paralleliteratorfind-map-last)
- [`ParallelIterator::any`](#fn-paralleliteratorany)
- [`ParallelIterator::all`](#fn-paralleliteratorall)
- [`ParallelIterator::while_some`](#fn-paralleliteratorwhile-some)
- [`ParallelIterator::panic_fuse`](#fn-paralleliteratorpanic-fuse)
- [`ParallelIterator::collect`](#fn-paralleliteratorcollect)
- [`ParallelIterator::unzip`](#fn-paralleliteratorunzip)
- [`ParallelIterator::partition`](#fn-paralleliteratorpartition)
- [`ParallelIterator::partition_map`](#fn-paralleliteratorpartition-map)
- [`ParallelIterator::intersperse`](#fn-paralleliteratorintersperse)
- [`ParallelIterator::take_any`](#fn-paralleliteratortake-any)
- [`ParallelIterator::skip_any`](#fn-paralleliteratorskip-any)
- [`ParallelIterator::take_any_while`](#fn-paralleliteratortake-any-while)
- [`ParallelIterator::skip_any_while`](#fn-paralleliteratorskip-any-while)
- [`ParallelIterator::collect_vec_list`](#fn-paralleliteratorcollect-vec-list)
- [`ParallelIterator::opt_len`](#fn-paralleliteratoropt-len)

</details>

#### Associated Types

- `type Item: 1`

#### Required Methods

- `fn ParallelIterator::drive_unindexed<C>(self, consumer: C) -> <C as >::Result`

  Internal method used to define the behavior of this parallel
  iterator. You should not need to call this directly.
  
  This method causes the iterator `self` to start producing
  items and to feed them to the consumer `consumer` one by one.
  It may split the consumer before doing so to create the
  opportunity to produce in parallel.
  
  See the [README] for more details on the internals of parallel
  iterators.

#### Provided Methods

- `fn ParallelIterator::for_each<OP>(self, op: OP)`

  Executes `OP` on each item produced by the iterator, in parallel.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  (0..100).into_par_iter().for_each(|x| println!("{:?}", x));
  ```

- `fn ParallelIterator::for_each_with<OP, T>(self, init: T, op: OP)`

  Executes `OP` on the given `init` value with each item produced by
  the iterator, in parallel.
  
  The `init` value will be cloned only as needed to be paired with
  the group of items in each rayon job.  It does not require the type
  to be `Sync`.
  
  ##### Examples
  
  ```rust
  use std::sync::mpsc::channel;
  use rayon::prelude::*;
  
  let (sender, receiver) = channel();
  
  (0..5).into_par_iter().for_each_with(sender, |s, x| s.send(x).unwrap());
  
  let mut res: Vec<_> = receiver.iter().collect();
  
  res.sort();
  
  assert_eq!(&res[..], &[0, 1, 2, 3, 4])
  ```

- `fn ParallelIterator::for_each_init<OP, INIT, T>(self, init: INIT, op: OP)`

  Executes `OP` on a value returned by `init` with each item produced by
  the iterator, in parallel.
  
  The `init` function will be called only as needed for a value to be
  paired with the group of items in each rayon job.  There is no
  constraint on that returned type at all!
  
  ##### Examples
  
  ```rust
  use rand::Rng;
  use rayon::prelude::*;
  
  let mut v = vec![0u8; 1_000_000];
  
  v.par_chunks_mut(1000)
      .for_each_init(
          || rand::rng(),
          |rng, chunk| rng.fill(chunk),
      );
  
  // There's a remote chance that this will fail...
  for i in 0u8..=255 {
      assert!(v.contains(&i));
  }
  ```

- `fn ParallelIterator::try_for_each<OP, R>(self, op: OP) -> R`

  Executes a fallible `OP` on each item produced by the iterator, in parallel.
  
  If the `OP` returns `Result::Err` or `Option::None`, we will attempt to
  stop processing the rest of the items in the iterator as soon as
  possible, and we will return that terminating value.  Otherwise, we will
  return an empty `Result::Ok(())` or `Option::Some(())`.  If there are
  multiple errors in parallel, it is not specified which will be returned.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  use std::io::{self, Write};
  
  // This will stop iteration early if there's any write error, like
  // having piped output get closed on the other end.
  (0..100).into_par_iter()
      .try_for_each(|x| writeln!(io::stdout(), "{:?}", x))
      .expect("expected no write errors");
  ```

- `fn ParallelIterator::try_for_each_with<OP, T, R>(self, init: T, op: OP) -> R`

  Executes a fallible `OP` on the given `init` value with each item
  produced by the iterator, in parallel.
  
  This combines the `init` semantics of `for_each_with()` and the
  failure semantics of `try_for_each()`.
  
  
  ##### Examples
  
  ```rust
  use std::sync::mpsc::channel;
  use rayon::prelude::*;
  
  let (sender, receiver) = channel();
  
  (0..5).into_par_iter()
      .try_for_each_with(sender, |s, x| s.send(x))
      .expect("expected no send errors");
  
  let mut res: Vec<_> = receiver.iter().collect();
  
  res.sort();
  
  assert_eq!(&res[..], &[0, 1, 2, 3, 4])
  ```

- `fn ParallelIterator::try_for_each_init<OP, INIT, T, R>(self, init: INIT, op: OP) -> R`

  Executes a fallible `OP` on a value returned by `init` with each item
  produced by the iterator, in parallel.
  
  This combines the `init` semantics of `for_each_init()` and the
  failure semantics of `try_for_each()`.
  
  
  ##### Examples
  
  ```rust
  use rand::{Rng, TryRngCore};
  use rayon::prelude::*;
  
  let mut v = vec![0u8; 1_000_000];
  
  v.par_chunks_mut(1000)
      .try_for_each_init(
          || rand::rng(),
          |rng, chunk| rng.try_fill_bytes(chunk),
      )
      .expect("expected no rand errors");
  
  // There's a remote chance that this will fail...
  for i in 0u8..=255 {
      assert!(v.contains(&i));
  }
  ```

- `fn ParallelIterator::count(self) -> usize`

  Counts the number of items in this parallel iterator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let count = (0..100).into_par_iter().count();
  
  assert_eq!(count, 100);
  ```

- `fn ParallelIterator::map<F, R>(self, map_op: F) -> Map<Self, F>`

  Applies `map_op` to each item of this iterator, producing a new
  iterator with the results.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut par_iter = (0..5).into_par_iter().map(|x| x * 2);
  
  let doubles: Vec<_> = par_iter.collect();
  
  assert_eq!(&doubles[..], &[0, 2, 4, 6, 8]);
  ```

- `fn ParallelIterator::map_with<F, T, R>(self, init: T, map_op: F) -> MapWith<Self, T, F>`

  Applies `map_op` to the given `init` value with each item of this
  iterator, producing a new iterator with the results.
  
  The `init` value will be cloned only as needed to be paired with
  the group of items in each rayon job.  It does not require the type
  to be `Sync`.
  
  ##### Examples
  
  ```rust
  use std::sync::mpsc::channel;
  use rayon::prelude::*;
  
  let (sender, receiver) = channel();
  
  let a: Vec<_> = (0..5)
                  .into_par_iter()            // iterating over i32
                  .map_with(sender, |s, x| {
                      s.send(x).unwrap();     // sending i32 values through the channel
                      x                       // returning i32
                  })
                  .collect();                 // collecting the returned values into a vector
  
  let mut b: Vec<_> = receiver.iter()         // iterating over the values in the channel
                              .collect();     // and collecting them
  b.sort();
  
  assert_eq!(a, b);
  ```

- `fn ParallelIterator::map_init<F, INIT, T, R>(self, init: INIT, map_op: F) -> MapInit<Self, INIT, F>`

  Applies `map_op` to a value returned by `init` with each item of this
  iterator, producing a new iterator with the results.
  
  The `init` function will be called only as needed for a value to be
  paired with the group of items in each rayon job.  There is no
  constraint on that returned type at all!
  
  ##### Examples
  
  ```rust
  use rand::Rng;
  use rayon::prelude::*;
  
  let a: Vec<_> = (1i32..1_000_000)
      .into_par_iter()
      .map_init(
          || rand::rng(),  // get the thread-local RNG
          |rng, x| if rng.random() { // randomly negate items
              -x
          } else {
              x
          },
      ).collect();
  
  // There's a remote chance that this will fail...
  assert!(a.iter().any(|&x| x < 0));
  assert!(a.iter().any(|&x| x > 0));
  ```

- `fn ParallelIterator::cloned<'a, T>(self) -> Cloned<Self>`

  Creates an iterator which clones all of its elements.  This may be
  useful when you have an iterator over `&T`, but you need `T`, and
  that type implements `Clone`. See also `copied()`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [1, 2, 3];
  
  let v_cloned: Vec<_> = a.par_iter().cloned().collect();
  
  // cloned is the same as .map(|&x| x), for integers
  let v_map: Vec<_> = a.par_iter().map(|&x| x).collect();
  
  assert_eq!(v_cloned, vec![1, 2, 3]);
  assert_eq!(v_map, vec![1, 2, 3]);
  ```

- `fn ParallelIterator::copied<'a, T>(self) -> Copied<Self>`

  Creates an iterator which copies all of its elements.  This may be
  useful when you have an iterator over `&T`, but you need `T`, and
  that type implements `Copy`. See also `cloned()`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [1, 2, 3];
  
  let v_copied: Vec<_> = a.par_iter().copied().collect();
  
  // copied is the same as .map(|&x| x), for integers
  let v_map: Vec<_> = a.par_iter().map(|&x| x).collect();
  
  assert_eq!(v_copied, vec![1, 2, 3]);
  assert_eq!(v_map, vec![1, 2, 3]);
  ```

- `fn ParallelIterator::inspect<OP>(self, inspect_op: OP) -> Inspect<Self, OP>`

  Applies `inspect_op` to a reference to each item of this iterator,
  producing a new iterator passing through the original items.  This is
  often useful for debugging to see what's happening in iterator stages.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [1, 4, 2, 3];
  
  // this iterator sequence is complex.
  let sum = a.par_iter()
              .cloned()
              .filter(|&x| x % 2 == 0)
              .reduce(|| 0, |sum, i| sum + i);
  
  println!("{}", sum);
  
  // let's add some inspect() calls to investigate what's happening
  let sum = a.par_iter()
              .cloned()
              .inspect(|x| println!("about to filter: {}", x))
              .filter(|&x| x % 2 == 0)
              .inspect(|x| println!("made it through filter: {}", x))
              .reduce(|| 0, |sum, i| sum + i);
  
  println!("{}", sum);
  ```

- `fn ParallelIterator::update<F>(self, update_op: F) -> Update<Self, F>`

  Mutates each item of this iterator before yielding it.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let par_iter = (0..5).into_par_iter().update(|x| {*x *= 2;});
  
  let doubles: Vec<_> = par_iter.collect();
  
  assert_eq!(&doubles[..], &[0, 2, 4, 6, 8]);
  ```

- `fn ParallelIterator::filter<P>(self, filter_op: P) -> Filter<Self, P>`

  Applies `filter_op` to each item of this iterator, producing a new
  iterator with only the items that gave `true` results.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut par_iter = (0..10).into_par_iter().filter(|x| x % 2 == 0);
  
  let even_numbers: Vec<_> = par_iter.collect();
  
  assert_eq!(&even_numbers[..], &[0, 2, 4, 6, 8]);
  ```

- `fn ParallelIterator::filter_map<P, R>(self, filter_op: P) -> FilterMap<Self, P>`

  Applies `filter_op` to each item of this iterator to get an `Option`,
  producing a new iterator with only the items from `Some` results.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut par_iter = (0..10).into_par_iter()
                          .filter_map(|x| {
                              if x % 2 == 0 { Some(x * 3) }
                              else { None }
                          });
  
  let even_numbers: Vec<_> = par_iter.collect();
  
  assert_eq!(&even_numbers[..], &[0, 6, 12, 18, 24]);
  ```

- `fn ParallelIterator::flat_map<F, PI>(self, map_op: F) -> FlatMap<Self, F>`

  Applies `map_op` to each item of this iterator to get nested parallel iterators,
  producing a new parallel iterator that flattens these back into one.
  
  See also [`flat_map_iter`](#method.flat_map_iter).
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [[1, 2], [3, 4], [5, 6], [7, 8]];
  
  let par_iter = a.par_iter().cloned().flat_map(|a| a.to_vec());
  
  let vec: Vec<_> = par_iter.collect();
  
  assert_eq!(&vec[..], &[1, 2, 3, 4, 5, 6, 7, 8]);
  ```

- `fn ParallelIterator::flat_map_iter<F, SI>(self, map_op: F) -> FlatMapIter<Self, F>`

  Applies `map_op` to each item of this iterator to get nested serial iterators,
  producing a new parallel iterator that flattens these back into one.
  
  ##### `flat_map_iter` versus `flat_map`
  
  These two methods are similar but behave slightly differently. With [`flat_map`](../iter/flat_map/index.md),
  each of the nested iterators must be a parallel iterator, and they will be further
  split up with nested parallelism. With `flat_map_iter`, each nested iterator is a
  sequential `Iterator`, and we only parallelize _between_ them, while the items
  produced by each nested iterator are processed sequentially.
  
  When choosing between these methods, consider whether nested parallelism suits the
  potential iterators at hand. If there's little computation involved, or its length
  is much less than the outer parallel iterator, then it may perform better to avoid
  the overhead of parallelism, just flattening sequentially with `flat_map_iter`.
  If there is a lot of computation, potentially outweighing the outer parallel
  iterator, then the nested parallelism of `flat_map` may be worthwhile.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  use std::cell::RefCell;
  
  let a = [[1, 2], [3, 4], [5, 6], [7, 8]];
  
  let par_iter = a.par_iter().flat_map_iter(|a| {
      // The serial iterator doesn't have to be thread-safe, just its items.
      let cell_iter = RefCell::new(a.iter().cloned());
      std::iter::from_fn(move || cell_iter.borrow_mut().next())
  });
  
  let vec: Vec<_> = par_iter.collect();
  
  assert_eq!(&vec[..], &[1, 2, 3, 4, 5, 6, 7, 8]);
  ```

- `fn ParallelIterator::flatten(self) -> Flatten<Self>`

  An adaptor that flattens parallel-iterable `Item`s into one large iterator.
  
  See also [`flatten_iter`](#method.flatten_iter).
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let x: Vec<Vec<_>> = vec![vec![1, 2], vec![3, 4]];
  let y: Vec<_> = x.into_par_iter().flatten().collect();
  
  assert_eq!(y, vec![1, 2, 3, 4]);
  ```

- `fn ParallelIterator::flatten_iter(self) -> FlattenIter<Self>`

  An adaptor that flattens serial-iterable `Item`s into one large iterator.
  
  See also [`flatten`](#method.flatten) and the analogous comparison of
  [`flat_map_iter` versus `flat_map`](#flat_map_iter-versus-flat_map).
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let x: Vec<Vec<_>> = vec![vec![1, 2], vec![3, 4]];
  let iters: Vec<_> = x.into_iter().map(Vec::into_iter).collect();
  let y: Vec<_> = iters.into_par_iter().flatten_iter().collect();
  
  assert_eq!(y, vec![1, 2, 3, 4]);
  ```

- `fn ParallelIterator::reduce<OP, ID>(self, identity: ID, op: OP) -> <Self as >::Item`

  Reduces the items in the iterator into one item using `op`.
  The argument `identity` should be a closure that can produce
  "identity" value which may be inserted into the sequence as
  needed to create opportunities for parallel execution. So, for
  example, if you are doing a summation, then `identity()` ought
  to produce something that represents the zero for your type
  (but consider just calling `sum()` in that case).
  
  ##### Examples
  
  ```rust
  // Iterate over a sequence of pairs `(x0, y0), ..., (xN, yN)`
  // and use reduce to compute one pair `(x0 + ... + xN, y0 + ... + yN)`
  // where the first/second elements are summed separately.
  use rayon::prelude::*;
  let sums = [(0, 1), (5, 6), (16, 2), (8, 9)]
             .par_iter()        // iterating over &(i32, i32)
             .cloned()          // iterating over (i32, i32)
             .reduce(|| (0, 0), // the "identity" is 0 in both columns
                     |a, b| (a.0 + b.0, a.1 + b.1));
  assert_eq!(sums, (0 + 5 + 16 + 8, 1 + 6 + 2 + 9));
  ```
  
  **Note:** unlike a sequential `fold` operation, the order in
  which `op` will be applied to reduce the result is not fully
  specified. So `op` should be [associative] or else the results
  will be non-deterministic. And of course `identity()` should
  produce a true identity.

- `fn ParallelIterator::reduce_with<OP>(self, op: OP) -> Option<<Self as >::Item>`

  Reduces the items in the iterator into one item using `op`.
  If the iterator is empty, `None` is returned; otherwise,
  `Some` is returned.
  
  This version of `reduce` is simple but somewhat less
  efficient. If possible, it is better to call `reduce()`, which
  requires an identity element.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let sums = [(0, 1), (5, 6), (16, 2), (8, 9)]
             .par_iter()        // iterating over &(i32, i32)
             .cloned()          // iterating over (i32, i32)
             .reduce_with(|a, b| (a.0 + b.0, a.1 + b.1))
             .unwrap();
  assert_eq!(sums, (0 + 5 + 16 + 8, 1 + 6 + 2 + 9));
  ```
  
  **Note:** unlike a sequential `fold` operation, the order in
  which `op` will be applied to reduce the result is not fully
  specified. So `op` should be [associative] or else the results
  will be non-deterministic.

- `fn ParallelIterator::try_reduce<T, OP, ID>(self, identity: ID, op: OP) -> <Self as >::Item`

  Reduces the items in the iterator into one item using a fallible `op`.
  The `identity` argument is used the same way as in `reduce()`.
  
  If a `Result::Err` or `Option::None` item is found, or if `op` reduces
  to one, we will attempt to stop processing the rest of the items in the
  iterator as soon as possible, and we will return that terminating value.
  Otherwise, we will return the final reduced `Result::Ok(T)` or
  `Option::Some(T)`.  If there are multiple errors in parallel, it is not
  specified which will be returned.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  // Compute the sum of squares, being careful about overflow.
  fn sum_squares<I: IntoParallelIterator<Item = i32>>(iter: I) -> Option<i32> {
      iter.into_par_iter()
          .map(|i| i.checked_mul(i))            // square each item,
          .try_reduce(|| 0, i32::checked_add)   // and add them up!
  }
  assert_eq!(sum_squares(0..5), Some(0 + 1 + 4 + 9 + 16));
  
  // The sum might overflow
  assert_eq!(sum_squares(0..10_000), None);
  
  // Or the squares might overflow before it even reaches `try_reduce`
  assert_eq!(sum_squares(1_000_000..1_000_001), None);
  ```

- `fn ParallelIterator::try_reduce_with<T, OP>(self, op: OP) -> Option<<Self as >::Item>`

  Reduces the items in the iterator into one item using a fallible `op`.
  
  Like `reduce_with()`, if the iterator is empty, `None` is returned;
  otherwise, `Some` is returned.  Beyond that, it behaves like
  `try_reduce()` for handling `Err`/`None`.
  
  
  For instance, with `Option` items, the return value may be:
  - `None`, the iterator was empty
  - `Some(None)`, we stopped after encountering `None`.
  - `Some(Some(x))`, the entire iterator reduced to `x`.
  
  With `Result` items, the nesting is more obvious:
  - `None`, the iterator was empty
  - `Some(Err(e))`, we stopped after encountering an error `e`.
  - `Some(Ok(x))`, the entire iterator reduced to `x`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let files = ["/dev/null", "/does/not/exist"];
  
  // Find the biggest file
  files.into_par_iter()
      .map(|path| std::fs::metadata(path).map(|m| (path, m.len())))
      .try_reduce_with(|a, b| {
          Ok(if a.1 >= b.1 { a } else { b })
      })
      .expect("Some value, since the iterator is not empty")
      .expect_err("not found");
  ```

- `fn ParallelIterator::fold<T, ID, F>(self, identity: ID, fold_op: F) -> Fold<Self, ID, F>`

  Parallel fold is similar to sequential fold except that the
  sequence of items may be subdivided before it is
  folded. Consider a list of numbers like `22 3 77 89 46`. If
  you used sequential fold to add them (`fold(0, |a,b| a+b)`,
  you would wind up first adding 0 + 22, then 22 + 3, then 25 +
  77, and so forth. The **parallel fold** works similarly except
  that it first breaks up your list into sublists, and hence
  instead of yielding up a single sum at the end, it yields up
  multiple sums. The number of results is nondeterministic, as
  is the point where the breaks occur.
  
  So if we did the same parallel fold (`fold(0, |a,b| a+b)`) on
  our example list, we might wind up with a sequence of two numbers,
  like so:
  
  ```notrust
  22 3 77 89 46
        |     |
      102   135
  ```
  
  Or perhaps these three numbers:
  
  ```notrust
  22 3 77 89 46
        |  |  |
      102 89 46
  ```
  
  In general, Rayon will attempt to find good breaking points
  that keep all of your cores busy.
  
  ###### Fold versus reduce
  
  The `fold()` and `reduce()` methods each take an identity element
  and a combining function, but they operate rather differently.
  
  `reduce()` requires that the identity function has the same
  type as the things you are iterating over, and it fully
  reduces the list of items into a single item. So, for example,
  imagine we are iterating over a list of bytes `bytes: [128_u8,
  64_u8, 64_u8]`. If we used `bytes.reduce(|| 0_u8, |a: u8, b:
  u8| a + b)`, we would get an overflow. This is because `0`,
  `a`, and `b` here are all bytes, just like the numbers in the
  list (I wrote the types explicitly above, but those are the
  only types you can use). To avoid the overflow, we would need
  to do something like `bytes.map(|b| b as u32).reduce(|| 0, |a,
  b| a + b)`, in which case our result would be `256`.
  
  In contrast, with `fold()`, the identity function does not
  have to have the same type as the things you are iterating
  over, and you potentially get back many results. So, if we
  continue with the `bytes` example from the previous paragraph,
  we could do `bytes.fold(|| 0_u32, |a, b| a + (b as u32))` to
  convert our bytes into `u32`. And of course we might not get
  back a single sum.
  
  There is a more subtle distinction as well, though it's
  actually implied by the above points. When you use `reduce()`,
  your reduction function is sometimes called with values that
  were never part of your original parallel iterator (for
  example, both the left and right might be a partial sum). With
  `fold()`, in contrast, the left value in the fold function is
  always the accumulator, and the right value is always from
  your original sequence.
  
  ###### Fold vs Map/Reduce
  
  Fold makes sense if you have some operation where it is
  cheaper to create groups of elements at a time. For example,
  imagine collecting characters into a string. If you were going
  to use map/reduce, you might try this:
  
  ```rust
  use rayon::prelude::*;
  
  let s =
      ['a', 'b', 'c', 'd', 'e']
      .par_iter()
      .map(|c: &char| format!("{}", c))
      .reduce(|| String::new(),
              |mut a: String, b: String| { a.push_str(&b); a });
  
  assert_eq!(s, "abcde");
  ```
  
  Because reduce produces the same type of element as its input,
  you have to first map each character into a string, and then
  you can reduce them. This means we create one string per
  element in our iterator -- not so great. Using `fold`, we can
  do this instead:
  
  ```rust
  use rayon::prelude::*;
  
  let s =
      ['a', 'b', 'c', 'd', 'e']
      .par_iter()
      .fold(|| String::new(),
              |mut s: String, c: &char| { s.push(*c); s })
      .reduce(|| String::new(),
              |mut a: String, b: String| { a.push_str(&b); a });
  
  assert_eq!(s, "abcde");
  ```
  
  Now `fold` will process groups of our characters at a time,
  and we only make one string per group. We should wind up with
  some small-ish number of strings roughly proportional to the
  number of CPUs you have (it will ultimately depend on how busy
  your processors are). Note that we still need to do a reduce
  afterwards to combine those groups of strings into a single
  string.
  
  You could use a similar trick to save partial results (e.g., a
  cache) or something similar.
  
  ###### Combining fold with other operations
  
  You can combine `fold` with `reduce` if you want to produce a
  single value. This is then roughly equivalent to a map/reduce
  combination in effect:
  
  ```rust
  use rayon::prelude::*;
  
  let bytes = 0..22_u8;
  let sum = bytes.into_par_iter()
                 .fold(|| 0_u32, |a: u32, b: u8| a + (b as u32))
                 .sum::<u32>();
  
  assert_eq!(sum, (0..22).sum()); // compare to sequential
  ```

- `fn ParallelIterator::fold_with<F, T>(self, init: T, fold_op: F) -> FoldWith<Self, T, F>`

  Applies `fold_op` to the given `init` value with each item of this
  iterator, finally producing the value for further use.
  
  This works essentially like `fold(|| init.clone(), fold_op)`, except
  it doesn't require the `init` type to be `Sync`, nor any other form
  of added synchronization.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let bytes = 0..22_u8;
  let sum = bytes.into_par_iter()
                 .fold_with(0_u32, |a: u32, b: u8| a + (b as u32))
                 .sum::<u32>();
  
  assert_eq!(sum, (0..22).sum()); // compare to sequential
  ```

- `fn ParallelIterator::try_fold<T, R, ID, F>(self, identity: ID, fold_op: F) -> TryFold<Self, R, ID, F>`

  Performs a fallible parallel fold.
  
  This is a variation of `fold()` for operations which can fail with
  `Option::None` or `Result::Err`.  The first such failure stops
  processing the local set of items, without affecting other folds in the
  iterator's subdivisions.
  
  Often, `try_fold()` will be followed by `try_reduce()`
  for a final reduction and global short-circuiting effect.
  
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let bytes = 0..22_u8;
  let sum = bytes.into_par_iter()
                 .try_fold(|| 0_u32, |a: u32, b: u8| a.checked_add(b as u32))
                 .try_reduce(|| 0, u32::checked_add);
  
  assert_eq!(sum, Some((0..22).sum())); // compare to sequential
  ```

- `fn ParallelIterator::try_fold_with<F, T, R>(self, init: T, fold_op: F) -> TryFoldWith<Self, R, F>`

  Performs a fallible parallel fold with a cloneable `init` value.
  
  This combines the `init` semantics of `fold_with()` and the failure
  semantics of `try_fold()`.
  
  
  ```rust
  use rayon::prelude::*;
  
  let bytes = 0..22_u8;
  let sum = bytes.into_par_iter()
                 .try_fold_with(0_u32, |a: u32, b: u8| a.checked_add(b as u32))
                 .try_reduce(|| 0, u32::checked_add);
  
  assert_eq!(sum, Some((0..22).sum())); // compare to sequential
  ```

- `fn ParallelIterator::sum<S>(self) -> S`

  Sums up the items in the iterator.
  
  Note that the order in items will be reduced is not specified,
  so if the `+` operator is not truly [associative] \(as is the
  case for floating point numbers), then the results are not
  fully deterministic.
  
  Basically equivalent to `self.reduce(|| 0, |a, b| a + b)`,
  except that the type of `0` and the `+` operation may vary
  depending on the type of value being produced.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [1, 5, 7];
  
  let sum: i32 = a.par_iter().sum();
  
  assert_eq!(sum, 13);
  ```

- `fn ParallelIterator::product<P>(self) -> P`

  Multiplies all the items in the iterator.
  
  Note that the order in items will be reduced is not specified,
  so if the `*` operator is not truly [associative] \(as is the
  case for floating point numbers), then the results are not
  fully deterministic.
  
  Basically equivalent to `self.reduce(|| 1, |a, b| a * b)`,
  except that the type of `1` and the `*` operation may vary
  depending on the type of value being produced.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  fn factorial(n: u32) -> u32 {
     (1..n+1).into_par_iter().product()
  }
  
  assert_eq!(factorial(0), 1);
  assert_eq!(factorial(1), 1);
  assert_eq!(factorial(5), 120);
  ```

- `fn ParallelIterator::min(self) -> Option<<Self as >::Item>`

  Computes the minimum of all the items in the iterator. If the
  iterator is empty, `None` is returned; otherwise, `Some(min)`
  is returned.
  
  Note that the order in which the items will be reduced is not
  specified, so if the `Ord` impl is not truly associative, then
  the results are not deterministic.
  
  Basically equivalent to `self.reduce_with(|a, b| Ord::min(a, b))`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [45, 74, 32];
  
  assert_eq!(a.par_iter().min(), Some(&32));
  
  let b: [i32; 0] = [];
  
  assert_eq!(b.par_iter().min(), None);
  ```

- `fn ParallelIterator::min_by<F>(self, f: F) -> Option<<Self as >::Item>`

  Computes the minimum of all the items in the iterator with respect to
  the given comparison function. If the iterator is empty, `None` is
  returned; otherwise, `Some(min)` is returned.
  
  Note that the order in which the items will be reduced is not
  specified, so if the comparison function is not associative, then
  the results are not deterministic.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [-3_i32, 77, 53, 240, -1];
  
  assert_eq!(a.par_iter().min_by(|x, y| x.cmp(y)), Some(&-3));
  ```

- `fn ParallelIterator::min_by_key<K, F>(self, f: F) -> Option<<Self as >::Item>`

  Computes the item that yields the minimum value for the given
  function. If the iterator is empty, `None` is returned;
  otherwise, `Some(item)` is returned.
  
  Note that the order in which the items will be reduced is not
  specified, so if the `Ord` impl is not truly associative, then
  the results are not deterministic.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [-3_i32, 34, 2, 5, -10, -3, -23];
  
  assert_eq!(a.par_iter().min_by_key(|x| x.abs()), Some(&2));
  ```

- `fn ParallelIterator::max(self) -> Option<<Self as >::Item>`

  Computes the maximum of all the items in the iterator. If the
  iterator is empty, `None` is returned; otherwise, `Some(max)`
  is returned.
  
  Note that the order in which the items will be reduced is not
  specified, so if the `Ord` impl is not truly associative, then
  the results are not deterministic.
  
  Basically equivalent to `self.reduce_with(|a, b| Ord::max(a, b))`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [45, 74, 32];
  
  assert_eq!(a.par_iter().max(), Some(&74));
  
  let b: [i32; 0] = [];
  
  assert_eq!(b.par_iter().max(), None);
  ```

- `fn ParallelIterator::max_by<F>(self, f: F) -> Option<<Self as >::Item>`

  Computes the maximum of all the items in the iterator with respect to
  the given comparison function. If the iterator is empty, `None` is
  returned; otherwise, `Some(max)` is returned.
  
  Note that the order in which the items will be reduced is not
  specified, so if the comparison function is not associative, then
  the results are not deterministic.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [-3_i32, 77, 53, 240, -1];
  
  assert_eq!(a.par_iter().max_by(|x, y| x.abs().cmp(&y.abs())), Some(&240));
  ```

- `fn ParallelIterator::max_by_key<K, F>(self, f: F) -> Option<<Self as >::Item>`

  Computes the item that yields the maximum value for the given
  function. If the iterator is empty, `None` is returned;
  otherwise, `Some(item)` is returned.
  
  Note that the order in which the items will be reduced is not
  specified, so if the `Ord` impl is not truly associative, then
  the results are not deterministic.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [-3_i32, 34, 2, 5, -10, -3, -23];
  
  assert_eq!(a.par_iter().max_by_key(|x| x.abs()), Some(&34));
  ```

- `fn ParallelIterator::chain<C>(self, chain: C) -> Chain<Self, <C as >::Iter>`

  Takes two iterators and creates a new iterator over both.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [0, 1, 2];
  let b = [9, 8, 7];
  
  let par_iter = a.par_iter().chain(b.par_iter());
  
  let chained: Vec<_> = par_iter.cloned().collect();
  
  assert_eq!(&chained[..], &[0, 1, 2, 9, 8, 7]);
  ```

- `fn ParallelIterator::find_any<P>(self, predicate: P) -> Option<<Self as >::Item>`

  Searches for **some** item in the parallel iterator that
  matches the given predicate and returns it. This operation
  is similar to [`find` on sequential iterators][`find`](../iter/find/index.md) but
  the item returned may not be the **first** one in the parallel
  sequence which matches, since we search the entire sequence in parallel.
  
  Once a match is found, we will attempt to stop processing
  the rest of the items in the iterator as soon as possible
  (just as `find` stops iterating once a match is found).
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [1, 2, 3, 3];
  
  assert_eq!(a.par_iter().find_any(|&&x| x == 3), Some(&3));
  
  assert_eq!(a.par_iter().find_any(|&&x| x == 100), None);
  ```

- `fn ParallelIterator::find_first<P>(self, predicate: P) -> Option<<Self as >::Item>`

  Searches for the sequentially **first** item in the parallel iterator
  that matches the given predicate and returns it.
  
  Once a match is found, all attempts to the right of the match
  will be stopped, while attempts to the left must continue in case
  an earlier match is found.
  
  For added performance, you might consider using `find_first` in conjunction with
  `by_exponential_blocks()`.
  
  Note that not all parallel iterators have a useful order, much like
  sequential `HashMap` iteration, so "first" may be nebulous.  If you
  just want the first match that discovered anywhere in the iterator,
  `find_any` is a better choice.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [1, 2, 3, 3];
  
  assert_eq!(a.par_iter().find_first(|&&x| x == 3), Some(&3));
  
  assert_eq!(a.par_iter().find_first(|&&x| x == 100), None);
  ```

- `fn ParallelIterator::find_last<P>(self, predicate: P) -> Option<<Self as >::Item>`

  Searches for the sequentially **last** item in the parallel iterator
  that matches the given predicate and returns it.
  
  Once a match is found, all attempts to the left of the match
  will be stopped, while attempts to the right must continue in case
  a later match is found.
  
  Note that not all parallel iterators have a useful order, much like
  sequential `HashMap` iteration, so "last" may be nebulous.  When the
  order doesn't actually matter to you, `find_any` is a better choice.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [1, 2, 3, 3];
  
  assert_eq!(a.par_iter().find_last(|&&x| x == 3), Some(&3));
  
  assert_eq!(a.par_iter().find_last(|&&x| x == 100), None);
  ```

- `fn ParallelIterator::find_map_any<P, R>(self, predicate: P) -> Option<R>`

  Applies the given predicate to the items in the parallel iterator
  and returns **any** non-None result of the map operation.
  
  Once a non-None value is produced from the map operation, we will
  attempt to stop processing the rest of the items in the iterator
  as soon as possible.
  
  Note that this method only returns **some** item in the parallel
  iterator that is not None from the map predicate. The item returned
  may not be the **first** non-None value produced in the parallel
  sequence, since the entire sequence is mapped over in parallel.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let c = ["lol", "NaN", "5", "5"];
  
  let found_number = c.par_iter().find_map_any(|s| s.parse().ok());
  
  assert_eq!(found_number, Some(5));
  ```

- `fn ParallelIterator::find_map_first<P, R>(self, predicate: P) -> Option<R>`

  Applies the given predicate to the items in the parallel iterator and
  returns the sequentially **first** non-None result of the map operation.
  
  Once a non-None value is produced from the map operation, all attempts
  to the right of the match will be stopped, while attempts to the left
  must continue in case an earlier match is found.
  
  Note that not all parallel iterators have a useful order, much like
  sequential `HashMap` iteration, so "first" may be nebulous. If you
  just want the first non-None value discovered anywhere in the iterator,
  `find_map_any` is a better choice.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let c = ["lol", "NaN", "2", "5"];
  
  let first_number = c.par_iter().find_map_first(|s| s.parse().ok());
  
  assert_eq!(first_number, Some(2));
  ```

- `fn ParallelIterator::find_map_last<P, R>(self, predicate: P) -> Option<R>`

  Applies the given predicate to the items in the parallel iterator and
  returns the sequentially **last** non-None result of the map operation.
  
  Once a non-None value is produced from the map operation, all attempts
  to the left of the match will be stopped, while attempts to the right
  must continue in case a later match is found.
  
  Note that not all parallel iterators have a useful order, much like
  sequential `HashMap` iteration, so "first" may be nebulous. If you
  just want the first non-None value discovered anywhere in the iterator,
  `find_map_any` is a better choice.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let c = ["lol", "NaN", "2", "5"];
  
  let last_number = c.par_iter().find_map_last(|s| s.parse().ok());
  
  assert_eq!(last_number, Some(5));
  ```

- `fn ParallelIterator::any<P>(self, predicate: P) -> bool`

  Searches for **some** item in the parallel iterator that
  matches the given predicate, and if so returns true.  Once
  a match is found, we'll attempt to stop process the rest
  of the items.  Proving that there's no match, returning false,
  does require visiting every item.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [0, 12, 3, 4, 0, 23, 0];
  
  let is_valid = a.par_iter().any(|&x| x > 10);
  
  assert!(is_valid);
  ```

- `fn ParallelIterator::all<P>(self, predicate: P) -> bool`

  Tests that every item in the parallel iterator matches the given
  predicate, and if so returns true.  If a counter-example is found,
  we'll attempt to stop processing more items, then return false.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [0, 12, 3, 4, 0, 23, 0];
  
  let is_valid = a.par_iter().all(|&x| x > 10);
  
  assert!(!is_valid);
  ```

- `fn ParallelIterator::while_some<T>(self) -> WhileSome<Self>`

  Creates an iterator over the `Some` items of this iterator, halting
  as soon as any `None` is found.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  use std::sync::atomic::{AtomicUsize, Ordering};
  
  let counter = AtomicUsize::new(0);
  let value = (0_i32..2048)
      .into_par_iter()
      .map(|x| {
               counter.fetch_add(1, Ordering::SeqCst);
               if x < 1024 { Some(x) } else { None }
           })
      .while_some()
      .max();
  
  assert!(value < Some(1024));
  assert!(counter.load(Ordering::SeqCst) < 2048); // should not have visited every single one
  ```

- `fn ParallelIterator::panic_fuse(self) -> PanicFuse<Self>`

  Wraps an iterator with a fuse in case of panics, to halt all threads
  as soon as possible.
  
  Panics within parallel iterators are always propagated to the caller,
  but they don't always halt the rest of the iterator right away, due to
  the internal semantics of [`join`](../../rayon_core/join/index.md). This adaptor makes a greater effort
  to stop processing other items sooner, with the cost of additional
  synchronization overhead, which may also inhibit some optimizations.
  
  ##### Examples
  
  If this code didn't use `panic_fuse()`, it would continue processing
  many more items in other threads (with long sleep delays) before the
  panic is finally propagated.
  
  ```should_panic
  use rayon::prelude::*;
  use std::{thread, time};
  
  (0..1_000_000)
      .into_par_iter()
      .panic_fuse()
      .for_each(|i| {
          // simulate some work
          thread::sleep(time::Duration::from_secs(1));
          assert!(i > 0); // oops!
      });
  ```

- `fn ParallelIterator::collect<C>(self) -> C`

  Creates a fresh collection containing all the elements produced
  by this parallel iterator.
  
  You may prefer `collect_into_vec()` implemented on
  [`IndexedParallelIterator`](../iter/index.md), if your underlying iterator also implements
  it. `collect_into_vec()` allocates efficiently with precise knowledge
  of how many elements the iterator contains, and even allows you to reuse
  an existing vector's backing store rather than allocating a fresh vector.
  
  See also `collect_vec_list()` for collecting into a
  `LinkedList<Vec<T>>`.
  
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let sync_vec: Vec<_> = (0..100).into_iter().collect();
  
  let async_vec: Vec<_> = (0..100).into_par_iter().collect();
  
  assert_eq!(sync_vec, async_vec);
  ```
  
  You can collect a pair of collections like [`unzip`](#method.unzip)
  for paired items:
  
  ```rust
  use rayon::prelude::*;
  
  let a = [(0, 1), (1, 2), (2, 3), (3, 4)];
  let (first, second): (Vec<_>, Vec<_>) = a.into_par_iter().collect();
  
  assert_eq!(first, [0, 1, 2, 3]);
  assert_eq!(second, [1, 2, 3, 4]);
  ```
  
  Or like [`partition_map`](#method.partition_map) for `Either` items:
  
  ```rust
  use rayon::prelude::*;
  use rayon::iter::Either;
  
  let (left, right): (Vec<_>, Vec<_>) = (0..8).into_par_iter().map(|x| {
      if x % 2 == 0 {
          Either::Left(x * 4)
      } else {
          Either::Right(x * 3)
      }
  }).collect();
  
  assert_eq!(left, [0, 8, 16, 24]);
  assert_eq!(right, [3, 9, 15, 21]);
  ```
  
  You can even collect an arbitrarily-nested combination of pairs and `Either`:
  
  ```rust
  use rayon::prelude::*;
  use rayon::iter::Either;
  
  let (first, (left, right)): (Vec<_>, (Vec<_>, Vec<_>))
      = (0..8).into_par_iter().map(|x| {
          if x % 2 == 0 {
              (x, Either::Left(x * 4))
          } else {
              (-x, Either::Right(x * 3))
          }
      }).collect();
  
  assert_eq!(first, [0, -1, 2, -3, 4, -5, 6, -7]);
  assert_eq!(left, [0, 8, 16, 24]);
  assert_eq!(right, [3, 9, 15, 21]);
  ```
  
  All of that can _also_ be combined with short-circuiting collection of
  `Result` or `Option` types:
  
  ```rust
  use rayon::prelude::*;
  use rayon::iter::Either;
  
  let result: Result<(Vec<_>, (Vec<_>, Vec<_>)), _>
      = (0..8).into_par_iter().map(|x| {
          if x > 5 {
              Err(x)
          } else if x % 2 == 0 {
              Ok((x, Either::Left(x * 4)))
          } else {
              Ok((-x, Either::Right(x * 3)))
          }
      }).collect();
  
  let error = result.unwrap_err();
  assert!(error == 6 || error == 7);
  ```

- `fn ParallelIterator::unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)`

  Unzips the items of a parallel iterator into a pair of arbitrary
  `ParallelExtend` containers.
  
  You may prefer to use `unzip_into_vecs()`, which allocates more
  efficiently with precise knowledge of how many elements the
  iterator contains, and even allows you to reuse existing
  vectors' backing stores rather than allocating fresh vectors.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let a = [(0, 1), (1, 2), (2, 3), (3, 4)];
  
  let (left, right): (Vec<_>, Vec<_>) = a.par_iter().cloned().unzip();
  
  assert_eq!(left, [0, 1, 2, 3]);
  assert_eq!(right, [1, 2, 3, 4]);
  ```
  
  Nested pairs can be unzipped too.
  
  ```rust
  use rayon::prelude::*;
  
  let (values, (squares, cubes)): (Vec<_>, (Vec<_>, Vec<_>)) = (0..4).into_par_iter()
      .map(|i| (i, (i * i, i * i * i)))
      .unzip();
  
  assert_eq!(values, [0, 1, 2, 3]);
  assert_eq!(squares, [0, 1, 4, 9]);
  assert_eq!(cubes, [0, 1, 8, 27]);
  ```

- `fn ParallelIterator::partition<A, B, P>(self, predicate: P) -> (A, B)`

  Partitions the items of a parallel iterator into a pair of arbitrary
  `ParallelExtend` containers.  Items for which the `predicate` returns
  true go into the first container, and the rest go into the second.
  
  Note: unlike the standard `Iterator::partition`, this allows distinct
  collection types for the left and right items.  This is more flexible,
  but may require new type annotations when converting sequential code
  that used type inference assuming the two were the same.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let (left, right): (Vec<_>, Vec<_>) = (0..8).into_par_iter().partition(|x| x % 2 == 0);
  
  assert_eq!(left, [0, 2, 4, 6]);
  assert_eq!(right, [1, 3, 5, 7]);
  ```

- `fn ParallelIterator::partition_map<A, B, P, L, R>(self, predicate: P) -> (A, B)`

  Partitions and maps the items of a parallel iterator into a pair of
  arbitrary `ParallelExtend` containers.  `Either::Left` items go into
  the first container, and `Either::Right` items go into the second.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  use rayon::iter::Either;
  
  let (left, right): (Vec<_>, Vec<_>) = (0..8).into_par_iter()
      .partition_map(|x| {
          if x % 2 == 0 {
              Either::Left(x * 4)
          } else {
              Either::Right(x * 3)
          }
      });
  
  assert_eq!(left, [0, 8, 16, 24]);
  assert_eq!(right, [3, 9, 15, 21]);
  ```
  
  Nested `Either` enums can be split as well.
  
  ```rust
  use rayon::prelude::*;
  use rayon::iter::Either::*;
  
  let ((fizzbuzz, fizz), (buzz, other)): ((Vec<_>, Vec<_>), (Vec<_>, Vec<_>)) = (1..20)
      .into_par_iter()
      .partition_map(|x| match (x % 3, x % 5) {
          (0, 0) => Left(Left(x)),
          (0, _) => Left(Right(x)),
          (_, 0) => Right(Left(x)),
          (_, _) => Right(Right(x)),
      });
  
  assert_eq!(fizzbuzz, [15]);
  assert_eq!(fizz, [3, 6, 9, 12, 18]);
  assert_eq!(buzz, [5, 10]);
  assert_eq!(other, [1, 2, 4, 7, 8, 11, 13, 14, 16, 17, 19]);
  ```

- `fn ParallelIterator::intersperse(self, element: <Self as >::Item) -> Intersperse<Self>`

  Intersperses clones of an element between items of this iterator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let x = vec![1, 2, 3];
  let r: Vec<_> = x.into_par_iter().intersperse(-1).collect();
  
  assert_eq!(r, vec![1, -1, 2, -1, 3]);
  ```

- `fn ParallelIterator::take_any(self, n: usize) -> TakeAny<Self>`

  Creates an iterator that yields `n` elements from *anywhere* in the original iterator.
  
  This is similar to `IndexedParallelIterator::take` without being
  constrained to the "first" `n` of the original iterator order. The
  taken items will still maintain their relative order where that is
  visible in `collect`, `reduce`, and similar outputs.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let result: Vec<_> = (0..100)
      .into_par_iter()
      .filter(|&x| x % 2 == 0)
      .take_any(5)
      .collect();
  
  assert_eq!(result.len(), 5);
  assert!(result.windows(2).all(|w| w[0] < w[1]));
  ```

- `fn ParallelIterator::skip_any(self, n: usize) -> SkipAny<Self>`

  Creates an iterator that skips `n` elements from *anywhere* in the original iterator.
  
  This is similar to `IndexedParallelIterator::skip` without being
  constrained to the "first" `n` of the original iterator order. The
  remaining items will still maintain their relative order where that is
  visible in `collect`, `reduce`, and similar outputs.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let result: Vec<_> = (0..100)
      .into_par_iter()
      .filter(|&x| x % 2 == 0)
      .skip_any(5)
      .collect();
  
  assert_eq!(result.len(), 45);
  assert!(result.windows(2).all(|w| w[0] < w[1]));
  ```

- `fn ParallelIterator::take_any_while<P>(self, predicate: P) -> TakeAnyWhile<Self, P>`

  Creates an iterator that takes elements from *anywhere* in the original iterator
  until the given `predicate` returns `false`.
  
  The `predicate` may be anything -- e.g. it could be checking a fact about the item, a
  global condition unrelated to the item itself, or some combination thereof.
  
  If parallel calls to the `predicate` race and give different results, then the
  `true` results will still take those particular items, while respecting the `false`
  result from elsewhere to skip any further items.
  
  This is similar to `Iterator::take_while` without being constrained to the original
  iterator order. The taken items will still maintain their relative order where that is
  visible in `collect`, `reduce`, and similar outputs.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let result: Vec<_> = (0..100)
      .into_par_iter()
      .take_any_while(|x| *x < 50)
      .collect();
  
  assert!(result.len() <= 50);
  assert!(result.windows(2).all(|w| w[0] < w[1]));
  ```
  
  ```rust
  use rayon::prelude::*;
  use std::sync::atomic::AtomicUsize;
  use std::sync::atomic::Ordering::Relaxed;
  
  // Collect any group of items that sum <= 1000
  let quota = AtomicUsize::new(1000);
  let result: Vec<_> = (0_usize..100)
      .into_par_iter()
      .take_any_while(|&x| {
          quota.fetch_update(Relaxed, Relaxed, |q| q.checked_sub(x))
              .is_ok()
      })
      .collect();
  
  let sum = result.iter().sum::<usize>();
  assert!(matches!(sum, 902..=1000));
  ```

- `fn ParallelIterator::skip_any_while<P>(self, predicate: P) -> SkipAnyWhile<Self, P>`

  Creates an iterator that skips elements from *anywhere* in the original iterator
  until the given `predicate` returns `false`.
  
  The `predicate` may be anything -- e.g. it could be checking a fact about the item, a
  global condition unrelated to the item itself, or some combination thereof.
  
  If parallel calls to the `predicate` race and give different results, then the
  `true` results will still skip those particular items, while respecting the `false`
  result from elsewhere to skip any further items.
  
  This is similar to `Iterator::skip_while` without being constrained to the original
  iterator order. The remaining items will still maintain their relative order where that is
  visible in `collect`, `reduce`, and similar outputs.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let result: Vec<_> = (0..100)
      .into_par_iter()
      .skip_any_while(|x| *x < 50)
      .collect();
  
  assert!(result.len() >= 50);
  assert!(result.windows(2).all(|w| w[0] < w[1]));
  ```

- `fn ParallelIterator::collect_vec_list(self) -> LinkedList<Vec<<Self as >::Item>>`

  Collects this iterator into a linked list of vectors.
  
  This is useful when you need to condense a parallel iterator into a collection,
  but have no specific requirements for what that collection should be. If you
  plan to store the collection longer-term, `Vec<T>` is, as always, likely the
  best default choice, despite the overhead that comes from concatenating each
  vector. Or, if this is an `IndexedParallelIterator`, you should also prefer to
  just collect to a `Vec<T>`.
  
  Internally, most [`FromParallelIterator`](../iter/index.md)/[`ParallelExtend`](../iter/index.md) implementations
  use this strategy; each job collecting their chunk of the iterator to a `Vec<T>`
  and those chunks getting merged into a `LinkedList`, before then extending the
  collection with each vector. This is a very efficient way to collect an
  unindexed parallel iterator, without much intermediate data movement.
  
  ##### Examples
  
  ```rust
  use std::collections::LinkedList;
  use rayon::prelude::*;
  
  let result: LinkedList<Vec<_>> = (0..=100)
      .into_par_iter()
      .filter(|x| x % 2 == 0)
      .flat_map(|x| 0..x)
      .collect_vec_list();
  
  // `par_iter.collect_vec_list().into_iter().flatten()` turns
  // a parallel iterator into a serial one
  let total_len = result.into_iter().flatten().count();
  assert_eq!(total_len, 2550);
  ```

- `fn ParallelIterator::opt_len(&self) -> Option<usize>`

  Internal method used to define the behavior of this parallel
  iterator. You should not need to call this directly.
  
  Returns the number of items produced by this iterator, if known
  statically. This can be used by consumers to trigger special fast
  paths. Therefore, if `Some(_)` is returned, this iterator must only
  use the (indexed) `Consumer` methods when driving a consumer, such
  as `split_at()`. Calling `UnindexedConsumer::split_off_left()` or
  other `UnindexedConsumer` methods -- or returning an inaccurate
  value -- may result in panics.
  
  This method is currently used to optimize `collect` for want
  of true Rust specialization; it may be removed when
  specialization is stable.

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

*Defined in [`rayon-1.11.0/src/slice/mod.rs:29-199`](../../../.source_1765900590/rayon-1.11.0/src/slice/mod.rs#L29-L199)*

Parallel extensions for slices.

#### Required Methods

- `fn ParallelSlice::as_parallel_slice(&self) -> &[T]`

  Returns a plain slice, which is used to implement the rest of the
  parallel methods.

#### Provided Methods

- `fn ParallelSlice::par_split<P>(&self, separator: P) -> Split<'_, T, P>`

  Returns a parallel iterator over subslices separated by elements that
  match the separator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let products: Vec<_> = [1, 2, 3, 0, 2, 4, 8, 0, 3, 6, 9]
      .par_split(|i| *i == 0)
      .map(|numbers| numbers.iter().product::<i32>())
      .collect();
  assert_eq!(products, [6, 64, 162]);
  ```

- `fn ParallelSlice::par_split_inclusive<P>(&self, separator: P) -> SplitInclusive<'_, T, P>`

  Returns a parallel iterator over subslices separated by elements that
  match the separator, including the matched part as a terminator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let lengths: Vec<_> = [1, 2, 3, 0, 2, 4, 8, 0, 3, 6, 9]
      .par_split_inclusive(|i| *i == 0)
      .map(|numbers| numbers.len())
      .collect();
  assert_eq!(lengths, [4, 4, 3]);
  ```

- `fn ParallelSlice::par_windows(&self, window_size: usize) -> Windows<'_, T>`

  Returns a parallel iterator over all contiguous windows of length
  `window_size`. The windows overlap.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let windows: Vec<_> = [1, 2, 3].par_windows(2).collect();
  assert_eq!(vec![[1, 2], [2, 3]], windows);
  ```

- `fn ParallelSlice::par_chunks(&self, chunk_size: usize) -> Chunks<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of
  `self` at a time. The chunks do not overlap.
  
  If the number of elements in the iterator is not divisible by
  `chunk_size`, the last chunk may be shorter than `chunk_size`.  All
  other chunks will have that exact length.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let chunks: Vec<_> = [1, 2, 3, 4, 5].par_chunks(2).collect();
  assert_eq!(chunks, vec![&[1, 2][..], &[3, 4], &[5]]);
  ```

- `fn ParallelSlice::par_chunks_exact(&self, chunk_size: usize) -> ChunksExact<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of
  `self` at a time. The chunks do not overlap.
  
  If `chunk_size` does not divide the length of the slice, then the
  last up to `chunk_size-1` elements will be omitted and can be
  retrieved from the remainder function of the iterator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let chunks: Vec<_> = [1, 2, 3, 4, 5].par_chunks_exact(2).collect();
  assert_eq!(chunks, vec![&[1, 2][..], &[3, 4]]);
  ```

- `fn ParallelSlice::par_rchunks(&self, chunk_size: usize) -> RChunks<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of `self` at a time,
  starting at the end. The chunks do not overlap.
  
  If the number of elements in the iterator is not divisible by
  `chunk_size`, the last chunk may be shorter than `chunk_size`.  All
  other chunks will have that exact length.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let chunks: Vec<_> = [1, 2, 3, 4, 5].par_rchunks(2).collect();
  assert_eq!(chunks, vec![&[4, 5][..], &[2, 3], &[1]]);
  ```

- `fn ParallelSlice::par_rchunks_exact(&self, chunk_size: usize) -> RChunksExact<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of `self` at a time,
  starting at the end. The chunks do not overlap.
  
  If `chunk_size` does not divide the length of the slice, then the
  last up to `chunk_size-1` elements will be omitted and can be
  retrieved from the remainder function of the iterator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let chunks: Vec<_> = [1, 2, 3, 4, 5].par_rchunks_exact(2).collect();
  assert_eq!(chunks, vec![&[4, 5][..], &[2, 3]]);
  ```

- `fn ParallelSlice::par_chunk_by<F>(&self, pred: F) -> ChunkBy<'_, T, F>`

  Returns a parallel iterator over the slice producing non-overlapping runs
  of elements using the predicate to separate them.
  
  The predicate is called on two elements following themselves,
  it means the predicate is called on `slice[0]` and `slice[1]`
  then on `slice[1]` and `slice[2]` and so on.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let chunks: Vec<_> = [1, 2, 2, 3, 3, 3].par_chunk_by(|&x, &y| x == y).collect();
  assert_eq!(chunks[0], &[1]);
  assert_eq!(chunks[1], &[2, 2]);
  assert_eq!(chunks[2], &[3, 3, 3]);
  ```

#### Implementors

- `[T]`

### `ParallelSliceMut<T: Send>`

```rust
trait ParallelSliceMut<T: Send> { ... }
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:209-754`](../../../.source_1765900590/rayon-1.11.0/src/slice/mod.rs#L209-L754)*

Parallel extensions for mutable slices.

<details>
<summary><strong>Methods (15)</strong> - click to expand</summary>

**Required:**
- [`ParallelSliceMut::as_parallel_slice_mut`](#fn-parallelslicemutas-parallel-slice-mut)

**Provided:**
- [`ParallelSliceMut::par_split_mut`](#fn-parallelslicemutpar-split-mut)
- [`ParallelSliceMut::par_split_inclusive_mut`](#fn-parallelslicemutpar-split-inclusive-mut)
- [`ParallelSliceMut::par_chunks_mut`](#fn-parallelslicemutpar-chunks-mut)
- [`ParallelSliceMut::par_chunks_exact_mut`](#fn-parallelslicemutpar-chunks-exact-mut)
- [`ParallelSliceMut::par_rchunks_mut`](#fn-parallelslicemutpar-rchunks-mut)
- [`ParallelSliceMut::par_rchunks_exact_mut`](#fn-parallelslicemutpar-rchunks-exact-mut)
- [`ParallelSliceMut::par_sort`](#fn-parallelslicemutpar-sort)
- [`ParallelSliceMut::par_sort_by`](#fn-parallelslicemutpar-sort-by)
- [`ParallelSliceMut::par_sort_by_key`](#fn-parallelslicemutpar-sort-by-key)
- [`ParallelSliceMut::par_sort_by_cached_key`](#fn-parallelslicemutpar-sort-by-cached-key)
- [`ParallelSliceMut::par_sort_unstable`](#fn-parallelslicemutpar-sort-unstable)
- [`ParallelSliceMut::par_sort_unstable_by`](#fn-parallelslicemutpar-sort-unstable-by)
- [`ParallelSliceMut::par_sort_unstable_by_key`](#fn-parallelslicemutpar-sort-unstable-by-key)
- [`ParallelSliceMut::par_chunk_by_mut`](#fn-parallelslicemutpar-chunk-by-mut)

</details>

#### Required Methods

- `fn ParallelSliceMut::as_parallel_slice_mut(&mut self) -> &mut [T]`

  Returns a plain mutable slice, which is used to implement the rest of
  the parallel methods.

#### Provided Methods

- `fn ParallelSliceMut::par_split_mut<P>(&mut self, separator: P) -> SplitMut<'_, T, P>`

  Returns a parallel iterator over mutable subslices separated by
  elements that match the separator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let mut array = [1, 2, 3, 0, 2, 4, 8, 0, 3, 6, 9];
  array.par_split_mut(|i| *i == 0)
       .for_each(|slice| slice.reverse());
  assert_eq!(array, [3, 2, 1, 0, 8, 4, 2, 0, 9, 6, 3]);
  ```

- `fn ParallelSliceMut::par_split_inclusive_mut<P>(&mut self, separator: P) -> SplitInclusiveMut<'_, T, P>`

  Returns a parallel iterator over mutable subslices separated by elements
  that match the separator, including the matched part as a terminator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let mut array = [1, 2, 3, 0, 2, 4, 8, 0, 3, 6, 9];
  array.par_split_inclusive_mut(|i| *i == 0)
       .for_each(|slice| slice.reverse());
  assert_eq!(array, [0, 3, 2, 1, 0, 8, 4, 2, 9, 6, 3]);
  ```

- `fn ParallelSliceMut::par_chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of
  `self` at a time. The chunks are mutable and do not overlap.
  
  If the number of elements in the iterator is not divisible by
  `chunk_size`, the last chunk may be shorter than `chunk_size`.  All
  other chunks will have that exact length.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let mut array = [1, 2, 3, 4, 5];
  array.par_chunks_mut(2)
       .for_each(|slice| slice.reverse());
  assert_eq!(array, [2, 1, 4, 3, 5]);
  ```

- `fn ParallelSliceMut::par_chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of
  `self` at a time. The chunks are mutable and do not overlap.
  
  If `chunk_size` does not divide the length of the slice, then the
  last up to `chunk_size-1` elements will be omitted and can be
  retrieved from the remainder function of the iterator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let mut array = [1, 2, 3, 4, 5];
  array.par_chunks_exact_mut(3)
       .for_each(|slice| slice.reverse());
  assert_eq!(array, [3, 2, 1, 4, 5]);
  ```

- `fn ParallelSliceMut::par_rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of `self` at a time,
  starting at the end. The chunks are mutable and do not overlap.
  
  If the number of elements in the iterator is not divisible by
  `chunk_size`, the last chunk may be shorter than `chunk_size`.  All
  other chunks will have that exact length.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let mut array = [1, 2, 3, 4, 5];
  array.par_rchunks_mut(2)
       .for_each(|slice| slice.reverse());
  assert_eq!(array, [1, 3, 2, 5, 4]);
  ```

- `fn ParallelSliceMut::par_rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of `self` at a time,
  starting at the end. The chunks are mutable and do not overlap.
  
  If `chunk_size` does not divide the length of the slice, then the
  last up to `chunk_size-1` elements will be omitted and can be
  retrieved from the remainder function of the iterator.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let mut array = [1, 2, 3, 4, 5];
  array.par_rchunks_exact_mut(3)
       .for_each(|slice| slice.reverse());
  assert_eq!(array, [1, 2, 5, 4, 3]);
  ```

- `fn ParallelSliceMut::par_sort(&mut self)`

  Sorts the slice in parallel.
  
  This sort is stable (i.e., does not reorder equal elements) and *O*(*n* \* log(*n*)) worst-case.
  
  When applicable, unstable sorting is preferred because it is generally faster than stable
  sorting and it doesn't allocate auxiliary memory.
  See [`par_sort_unstable`](#method.par_sort_unstable).
  
  ##### Current implementation
  
  The current algorithm is an adaptive merge sort inspired by
  [timsort](https://en.wikipedia.org/wiki/Timsort).
  It is designed to be very fast in cases where the slice is nearly sorted, or consists of
  two or more sorted sequences concatenated one after another.
  
  Also, it allocates temporary storage the same size as `self`, but for very short slices a
  non-allocating insertion sort is used instead.
  
  In order to sort the slice in parallel, the slice is first divided into smaller chunks and
  all chunks are sorted in parallel. Then, adjacent chunks that together form non-descending
  or descending runs are concatenated. Finally, the remaining chunks are merged together using
  parallel subdivision of chunks and parallel merge operation.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut v = [-5, 4, 1, -3, 2];
  
  v.par_sort();
  assert_eq!(v, [-5, -3, 1, 2, 4]);
  ```

- `fn ParallelSliceMut::par_sort_by<F>(&mut self, compare: F)`

  Sorts the slice in parallel with a comparator function.
  
  This sort is stable (i.e., does not reorder equal elements) and *O*(*n* \* log(*n*)) worst-case.
  
  The comparator function must define a total ordering for the elements in the slice. If
  the ordering is not total, the order of the elements is unspecified. An order is a
  total order if it is (for all `a`, `b` and `c`):
  
  * total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true, and
  * transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
  
  For example, while `f64` doesn't implement `Ord` because `NaN != NaN`, we can use
  `partial_cmp` as our sort function when we know the slice doesn't contain a `NaN`.
  
  ```rust
  use rayon::prelude::*;
  
  let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
  floats.par_sort_by(|a, b| a.partial_cmp(b).unwrap());
  assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
  ```
  
  When applicable, unstable sorting is preferred because it is generally faster than stable
  sorting and it doesn't allocate auxiliary memory.
  See [`par_sort_unstable_by`](#method.par_sort_unstable_by).
  
  ##### Current implementation
  
  The current algorithm is an adaptive merge sort inspired by
  [timsort](https://en.wikipedia.org/wiki/Timsort).
  It is designed to be very fast in cases where the slice is nearly sorted, or consists of
  two or more sorted sequences concatenated one after another.
  
  Also, it allocates temporary storage the same size as `self`, but for very short slices a
  non-allocating insertion sort is used instead.
  
  In order to sort the slice in parallel, the slice is first divided into smaller chunks and
  all chunks are sorted in parallel. Then, adjacent chunks that together form non-descending
  or descending runs are concatenated. Finally, the remaining chunks are merged together using
  parallel subdivision of chunks and parallel merge operation.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut v = [5, 4, 1, 3, 2];
  v.par_sort_by(|a, b| a.cmp(b));
  assert_eq!(v, [1, 2, 3, 4, 5]);
  
  // reverse sorting
  v.par_sort_by(|a, b| b.cmp(a));
  assert_eq!(v, [5, 4, 3, 2, 1]);
  ```

- `fn ParallelSliceMut::par_sort_by_key<K, F>(&mut self, f: F)`

  Sorts the slice in parallel with a key extraction function.
  
  This sort is stable (i.e., does not reorder equal elements) and *O*(*m* \* *n* \* log(*n*))
  worst-case, where the key function is *O*(*m*).
  
  For expensive key functions (e.g. functions that are not simple property accesses or
  basic operations), [`par_sort_by_cached_key`](#method.par_sort_by_cached_key) is likely to
  be significantly faster, as it does not recompute element keys.
  
  When applicable, unstable sorting is preferred because it is generally faster than stable
  sorting and it doesn't allocate auxiliary memory.
  See [`par_sort_unstable_by_key`](#method.par_sort_unstable_by_key).
  
  ##### Current implementation
  
  The current algorithm is an adaptive merge sort inspired by
  [timsort](https://en.wikipedia.org/wiki/Timsort).
  It is designed to be very fast in cases where the slice is nearly sorted, or consists of
  two or more sorted sequences concatenated one after another.
  
  Also, it allocates temporary storage the same size as `self`, but for very short slices a
  non-allocating insertion sort is used instead.
  
  In order to sort the slice in parallel, the slice is first divided into smaller chunks and
  all chunks are sorted in parallel. Then, adjacent chunks that together form non-descending
  or descending runs are concatenated. Finally, the remaining chunks are merged together using
  parallel subdivision of chunks and parallel merge operation.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut v = [-5i32, 4, 1, -3, 2];
  
  v.par_sort_by_key(|k| k.abs());
  assert_eq!(v, [1, 2, -3, 4, -5]);
  ```

- `fn ParallelSliceMut::par_sort_by_cached_key<K, F>(&mut self, f: F)`

  Sorts the slice in parallel with a key extraction function.
  
  During sorting, the key function is called at most once per element, by using
  temporary storage to remember the results of key evaluation.
  The key function is called in parallel, so the order of calls is completely unspecified.
  
  This sort is stable (i.e., does not reorder equal elements) and *O*(*m* \* *n* + *n* \* log(*n*))
  worst-case, where the key function is *O*(*m*).
  
  For simple key functions (e.g., functions that are property accesses or
  basic operations), [`par_sort_by_key`](#method.par_sort_by_key) is likely to be
  faster.
  
  ##### Current implementation
  
  The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  which combines the fast average case of randomized quicksort with the fast worst case of
  heapsort, while achieving linear time on slices with certain patterns. It uses some
  randomization to avoid degenerate cases, but with a fixed seed to always provide
  deterministic behavior.
  
  In the worst case, the algorithm allocates temporary storage in a `Vec<(K, usize)>` the
  length of the slice.
  
  All quicksorts work in two stages: partitioning into two halves followed by recursive
  calls. The partitioning phase is sequential, but the two recursive calls are performed in
  parallel. Finally, after sorting the cached keys, the item positions are updated sequentially.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut v = [-5i32, 4, 32, -3, 2];
  
  v.par_sort_by_cached_key(|k| k.to_string());
  assert!(v == [-3, -5, 2, 32, 4]);
  ```

- `fn ParallelSliceMut::par_sort_unstable(&mut self)`

  Sorts the slice in parallel, but might not preserve the order of equal elements.
  
  This sort is unstable (i.e., may reorder equal elements), in-place
  (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.
  
  ##### Current implementation
  
  The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  which combines the fast average case of randomized quicksort with the fast worst case of
  heapsort, while achieving linear time on slices with certain patterns. It uses some
  randomization to avoid degenerate cases, but with a fixed seed to always provide
  deterministic behavior.
  
  It is typically faster than stable sorting, except in a few special cases, e.g., when the
  slice consists of several concatenated sorted sequences.
  
  All quicksorts work in two stages: partitioning into two halves followed by recursive
  calls. The partitioning phase is sequential, but the two recursive calls are performed in
  parallel.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut v = [-5, 4, 1, -3, 2];
  
  v.par_sort_unstable();
  assert_eq!(v, [-5, -3, 1, 2, 4]);
  ```

- `fn ParallelSliceMut::par_sort_unstable_by<F>(&mut self, compare: F)`

  Sorts the slice in parallel with a comparator function, but might not preserve the order of
  equal elements.
  
  This sort is unstable (i.e., may reorder equal elements), in-place
  (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.
  
  The comparator function must define a total ordering for the elements in the slice. If
  the ordering is not total, the order of the elements is unspecified. An order is a
  total order if it is (for all `a`, `b` and `c`):
  
  * total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true, and
  * transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
  
  For example, while `f64` doesn't implement `Ord` because `NaN != NaN`, we can use
  `partial_cmp` as our sort function when we know the slice doesn't contain a `NaN`.
  
  ```rust
  use rayon::prelude::*;
  
  let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
  floats.par_sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
  assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
  ```
  
  ##### Current implementation
  
  The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  which combines the fast average case of randomized quicksort with the fast worst case of
  heapsort, while achieving linear time on slices with certain patterns. It uses some
  randomization to avoid degenerate cases, but with a fixed seed to always provide
  deterministic behavior.
  
  It is typically faster than stable sorting, except in a few special cases, e.g., when the
  slice consists of several concatenated sorted sequences.
  
  All quicksorts work in two stages: partitioning into two halves followed by recursive
  calls. The partitioning phase is sequential, but the two recursive calls are performed in
  parallel.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut v = [5, 4, 1, 3, 2];
  v.par_sort_unstable_by(|a, b| a.cmp(b));
  assert_eq!(v, [1, 2, 3, 4, 5]);
  
  // reverse sorting
  v.par_sort_unstable_by(|a, b| b.cmp(a));
  assert_eq!(v, [5, 4, 3, 2, 1]);
  ```

- `fn ParallelSliceMut::par_sort_unstable_by_key<K, F>(&mut self, f: F)`

  Sorts the slice in parallel with a key extraction function, but might not preserve the order
  of equal elements.
  
  This sort is unstable (i.e., may reorder equal elements), in-place
  (i.e., does not allocate), and *O*(m \* *n* \* log(*n*)) worst-case,
  where the key function is *O*(*m*).
  
  ##### Current implementation
  
  The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  which combines the fast average case of randomized quicksort with the fast worst case of
  heapsort, while achieving linear time on slices with certain patterns. It uses some
  randomization to avoid degenerate cases, but with a fixed seed to always provide
  deterministic behavior.
  
  Due to its key calling strategy, `par_sort_unstable_by_key` is likely to be slower than
  [`par_sort_by_cached_key`](#method.par_sort_by_cached_key) in cases where the key function
  is expensive.
  
  All quicksorts work in two stages: partitioning into two halves followed by recursive
  calls. The partitioning phase is sequential, but the two recursive calls are performed in
  parallel.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let mut v = [-5i32, 4, 1, -3, 2];
  
  v.par_sort_unstable_by_key(|k| k.abs());
  assert_eq!(v, [1, 2, -3, 4, -5]);
  ```

- `fn ParallelSliceMut::par_chunk_by_mut<F>(&mut self, pred: F) -> ChunkByMut<'_, T, F>`

  Returns a parallel iterator over the slice producing non-overlapping mutable
  runs of elements using the predicate to separate them.
  
  The predicate is called on two elements following themselves,
  it means the predicate is called on `slice[0]` and `slice[1]`
  then on `slice[1]` and `slice[2]` and so on.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let mut xs = [1, 2, 2, 3, 3, 3];
  let chunks: Vec<_> = xs.par_chunk_by_mut(|&x, &y| x == y).collect();
  assert_eq!(chunks[0], &mut [1]);
  assert_eq!(chunks[1], &mut [2, 2]);
  assert_eq!(chunks[2], &mut [3, 3, 3]);
  ```

#### Implementors

- `[T]`

### `ParallelString`

```rust
trait ParallelString { ... }
```

*Defined in [`rayon-1.11.0/src/str.rs:58-342`](../../../.source_1765900590/rayon-1.11.0/src/str.rs#L58-L342)*

Parallel extensions for strings.

<details>
<summary><strong>Methods (13)</strong> - click to expand</summary>

**Required:**
- [`ParallelString::as_parallel_string`](#fn-parallelstringas-parallel-string)

**Provided:**
- [`ParallelString::par_chars`](#fn-parallelstringpar-chars)
- [`ParallelString::par_char_indices`](#fn-parallelstringpar-char-indices)
- [`ParallelString::par_bytes`](#fn-parallelstringpar-bytes)
- [`ParallelString::par_encode_utf16`](#fn-parallelstringpar-encode-utf16)
- [`ParallelString::par_split`](#fn-parallelstringpar-split)
- [`ParallelString::par_split_inclusive`](#fn-parallelstringpar-split-inclusive)
- [`ParallelString::par_split_terminator`](#fn-parallelstringpar-split-terminator)
- [`ParallelString::par_lines`](#fn-parallelstringpar-lines)
- [`ParallelString::par_split_whitespace`](#fn-parallelstringpar-split-whitespace)
- [`ParallelString::par_split_ascii_whitespace`](#fn-parallelstringpar-split-ascii-whitespace)
- [`ParallelString::par_matches`](#fn-parallelstringpar-matches)
- [`ParallelString::par_match_indices`](#fn-parallelstringpar-match-indices)

</details>

#### Required Methods

- `fn ParallelString::as_parallel_string(&self) -> &str`

  Returns a plain string slice, which is used to implement the rest of
  the parallel methods.

#### Provided Methods

- `fn ParallelString::par_chars(&self) -> Chars<'_>`

  Returns a parallel iterator over the characters of a string.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let max = "hello".par_chars().max_by_key(|c| *c as i32);
  assert_eq!(Some('o'), max);
  ```

- `fn ParallelString::par_char_indices(&self) -> CharIndices<'_>`

  Returns a parallel iterator over the characters of a string, with their positions.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let min = "hello".par_char_indices().min_by_key(|&(_i, c)| c as i32);
  assert_eq!(Some((1, 'e')), min);
  ```

- `fn ParallelString::par_bytes(&self) -> Bytes<'_>`

  Returns a parallel iterator over the bytes of a string.
  
  Note that multi-byte sequences (for code points greater than `U+007F`)
  are produced as separate items, but will not be split across threads.
  If you would prefer an indexed iterator without that guarantee, consider
  `string.as_bytes().par_iter().copied()` instead.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let max = "hello".par_bytes().max();
  assert_eq!(Some(b'o'), max);
  ```

- `fn ParallelString::par_encode_utf16(&self) -> EncodeUtf16<'_>`

  Returns a parallel iterator over a string encoded as UTF-16.
  
  Note that surrogate pairs (for code points greater than `U+FFFF`) are
  produced as separate items, but will not be split across threads.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  
  let max = "hello".par_encode_utf16().max();
  assert_eq!(Some(b'o' as u16), max);
  
  let text = "Zażółć gęślą jaźń";
  let utf8_len = text.len();
  let utf16_len = text.par_encode_utf16().count();
  assert!(utf16_len <= utf8_len);
  ```

- `fn ParallelString::par_split<P: Pattern>(&self, separator: P) -> Split<'_, P>`

  Returns a parallel iterator over substrings separated by a
  given character or predicate, similar to `str::split`.
  
  Note: the `Pattern` trait is private, for use only by Rayon itself.
  It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
  and any function or closure `F: Fn(char) -> bool + Sync + Send`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let total = "1, 2, buckle, 3, 4, door"
     .par_split(',')
     .filter_map(|s| s.trim().parse::<i32>().ok())
     .sum();
  assert_eq!(10, total);
  ```

- `fn ParallelString::par_split_inclusive<P: Pattern>(&self, separator: P) -> SplitInclusive<'_, P>`

  Returns a parallel iterator over substrings separated by a
  given character or predicate, keeping the matched part as a terminator
  of the substring similar to `str::split_inclusive`.
  
  Note: the `Pattern` trait is private, for use only by Rayon itself.
  It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
  and any function or closure `F: Fn(char) -> bool + Sync + Send`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let lines: Vec<_> = "Mary had a little lamb\nlittle lamb\nlittle lamb."
     .par_split_inclusive('\n')
     .collect();
  assert_eq!(lines, ["Mary had a little lamb\n", "little lamb\n", "little lamb."]);
  ```

- `fn ParallelString::par_split_terminator<P: Pattern>(&self, terminator: P) -> SplitTerminator<'_, P>`

  Returns a parallel iterator over substrings terminated by a
  given character or predicate, similar to `str::split_terminator`.
  It's equivalent to `par_split`, except it doesn't produce an empty
  substring after a trailing terminator.
  
  Note: the `Pattern` trait is private, for use only by Rayon itself.
  It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
  and any function or closure `F: Fn(char) -> bool + Sync + Send`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let parts: Vec<_> = "((1 + 3) * 2)"
      .par_split_terminator(|c| c == '(' || c == ')')
      .collect();
  assert_eq!(vec!["", "", "1 + 3", " * 2"], parts);
  ```

- `fn ParallelString::par_lines(&self) -> Lines<'_>`

  Returns a parallel iterator over the lines of a string, ending with an
  optional carriage return and with a newline (`\r\n` or just `\n`).
  The final line ending is optional, and line endings are not included in
  the output strings.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let lengths: Vec<_> = "hello world\nfizbuzz"
      .par_lines()
      .map(|l| l.len())
      .collect();
  assert_eq!(vec![11, 7], lengths);
  ```

- `fn ParallelString::par_split_whitespace(&self) -> SplitWhitespace<'_>`

  Returns a parallel iterator over the sub-slices of a string that are
  separated by any amount of whitespace.
  
  As with `str::split_whitespace`, 'whitespace' is defined according to
  the terms of the Unicode Derived Core Property `White_Space`.
  If you only want to split on ASCII whitespace instead, use
  [`par_split_ascii_whitespace`]`ParallelString::par_split_ascii_whitespace`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let longest = "which is the longest word?"
      .par_split_whitespace()
      .max_by_key(|word| word.len());
  assert_eq!(Some("longest"), longest);
  ```
  
  All kinds of whitespace are considered:
  
  ```rust
  use rayon::prelude::*;
  let words: Vec<&str> = " Mary   had\ta\u{2009}little  \n\t lamb"
      .par_split_whitespace()
      .collect();
  assert_eq!(words, ["Mary", "had", "a", "little", "lamb"]);
  ```
  
  If the string is empty or all whitespace, the iterator yields no string slices:
  
  ```rust
  use rayon::prelude::*;
  assert_eq!("".par_split_whitespace().count(), 0);
  assert_eq!("   ".par_split_whitespace().count(), 0);
  ```

- `fn ParallelString::par_split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_>`

  Returns a parallel iterator over the sub-slices of a string that are
  separated by any amount of ASCII whitespace.
  
  To split by Unicode `White_Space` instead, use
  [`par_split_whitespace`]`ParallelString::par_split_whitespace`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let longest = "which is the longest word?"
      .par_split_ascii_whitespace()
      .max_by_key(|word| word.len());
  assert_eq!(Some("longest"), longest);
  ```
  
  All kinds of ASCII whitespace are considered, but not Unicode `White_Space`:
  
  ```rust
  use rayon::prelude::*;
  let words: Vec<&str> = " Mary   had\ta\u{2009}little  \n\t lamb"
      .par_split_ascii_whitespace()
      .collect();
  assert_eq!(words, ["Mary", "had", "a\u{2009}little", "lamb"]);
  ```
  
  If the string is empty or all ASCII whitespace, the iterator yields no string slices:
  
  ```rust
  use rayon::prelude::*;
  assert_eq!("".par_split_whitespace().count(), 0);
  assert_eq!("   ".par_split_whitespace().count(), 0);
  ```

- `fn ParallelString::par_matches<P: Pattern>(&self, pattern: P) -> Matches<'_, P>`

  Returns a parallel iterator over substrings that match a
  given character or predicate, similar to `str::matches`.
  
  Note: the `Pattern` trait is private, for use only by Rayon itself.
  It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
  and any function or closure `F: Fn(char) -> bool + Sync + Send`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let total = "1, 2, buckle, 3, 4, door"
     .par_matches(char::is_numeric)
     .map(|s| s.parse::<i32>().expect("digit"))
     .sum();
  assert_eq!(10, total);
  ```

- `fn ParallelString::par_match_indices<P: Pattern>(&self, pattern: P) -> MatchIndices<'_, P>`

  Returns a parallel iterator over substrings that match a given character
  or predicate, with their positions, similar to `str::match_indices`.
  
  Note: the `Pattern` trait is private, for use only by Rayon itself.
  It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
  and any function or closure `F: Fn(char) -> bool + Sync + Send`.
  
  ##### Examples
  
  ```rust
  use rayon::prelude::*;
  let digits: Vec<_> = "1, 2, buckle, 3, 4, door"
     .par_match_indices(char::is_numeric)
     .collect();
  assert_eq!(digits, vec![(0, "1"), (3, "2"), (14, "3"), (17, "4")]);
  ```

#### Implementors

- `str`

