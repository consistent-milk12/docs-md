*[rayon](../../index.md) / [slice](../index.md) / [sort](index.md)*

---

# Module `sort`

**Parallel** Slice sorting

This implementation is mostly copied from the `core::slice::sort` module, with minimal changes
to support stable Rust and parallel `is_less` (e.g. `Fn` rather than `FnMut`).

---

This module contains a sorting algorithm based on Orson Peters' pattern-defeating quicksort,
published at: <https://github.com/orlp/pdqsort>

Unstable sorting is compatible with core because it doesn't allocate memory, unlike our
stable sorting implementation.

In addition it also contains the core logic of the stable sort used by `slice::sort` based on
TimSort.

## Contents

- [Structs](#structs)
  - [`InsertionHole`](#insertionhole)
  - [`MergeHole`](#mergehole)
  - [`TimSortRun`](#timsortrun)
- [Enums](#enums)
  - [`MergeSortResult`](#mergesortresult)
- [Functions](#functions)
  - [`insert_tail`](#insert_tail)
  - [`insert_head`](#insert_head)
  - [`insertion_sort_shift_left`](#insertion_sort_shift_left)
  - [`insertion_sort_shift_right`](#insertion_sort_shift_right)
  - [`partial_insertion_sort`](#partial_insertion_sort)
  - [`heapsort`](#heapsort)
  - [`partition_in_blocks`](#partition_in_blocks)
  - [`partition`](#partition)
  - [`partition_equal`](#partition_equal)
  - [`break_patterns`](#break_patterns)
  - [`choose_pivot`](#choose_pivot)
  - [`recurse`](#recurse)
  - [`par_quicksort`](#par_quicksort)
  - [`merge`](#merge)
  - [`merge_sort`](#merge_sort)
  - [`provide_sorted_batch`](#provide_sorted_batch)
  - [`find_streak`](#find_streak)
  - [`split_for_merge`](#split_for_merge)
  - [`par_merge`](#par_merge)
  - [`merge_recurse`](#merge_recurse)
  - [`par_mergesort`](#par_mergesort)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`InsertionHole`](#insertionhole) | struct |  |
| [`MergeHole`](#mergehole) | struct |  |
| [`TimSortRun`](#timsortrun) | struct | Internal type used by merge_sort. |
| [`MergeSortResult`](#mergesortresult) | enum | The result of merge sort. |
| [`insert_tail`](#insert_tail) | fn | Inserts `v[v.len() - 1]` into pre-sorted sequence `v[..v.len() - 1]` so that whole `v[..]` |
| [`insert_head`](#insert_head) | fn | Inserts `v[0]` into pre-sorted sequence `v[1..]` so that whole `v[..]` becomes sorted. |
| [`insertion_sort_shift_left`](#insertion_sort_shift_left) | fn | Sort `v` assuming `v[..offset]` is already sorted. |
| [`insertion_sort_shift_right`](#insertion_sort_shift_right) | fn | Sort `v` assuming `v[offset..]` is already sorted. |
| [`partial_insertion_sort`](#partial_insertion_sort) | fn | Partially sorts a slice by shifting several out-of-order elements around. |
| [`heapsort`](#heapsort) | fn | Sorts `v` using heapsort, which guarantees *O*(*n* \* log(*n*)) worst-case. |
| [`partition_in_blocks`](#partition_in_blocks) | fn | Partitions `v` into elements smaller than `pivot`, followed by elements greater than or equal |
| [`partition`](#partition) | fn | Partitions `v` into elements smaller than `v[pivot]`, followed by elements greater than or |
| [`partition_equal`](#partition_equal) | fn | Partitions `v` into elements equal to `v[pivot]` followed by elements greater than `v[pivot]`. |
| [`break_patterns`](#break_patterns) | fn | Scatters some elements around in an attempt to break patterns that might cause imbalanced |
| [`choose_pivot`](#choose_pivot) | fn | Chooses a pivot in `v` and returns the index and `true` if the slice is likely already sorted. |
| [`recurse`](#recurse) | fn | Sorts `v` recursively. |
| [`par_quicksort`](#par_quicksort) | fn | Sorts `v` using pattern-defeating quicksort in parallel. |
| [`merge`](#merge) | fn | Merges non-decreasing runs `v[..mid]` and `v[mid..]` using `buf` as temporary storage, and |
| [`merge_sort`](#merge_sort) | fn | This merge sort borrows some (but not all) ideas from TimSort, which used to be described in |
| [`provide_sorted_batch`](#provide_sorted_batch) | fn | Takes a range as denoted by start and end, that is already sorted and extends it to the right if |
| [`find_streak`](#find_streak) | fn | Finds a streak of presorted elements starting at the beginning of the slice. |
| [`split_for_merge`](#split_for_merge) | fn | Splits two sorted slices so that they can be merged in parallel. |
| [`par_merge`](#par_merge) | fn | Merges slices `left` and `right` in parallel and stores the result into `dest`. |
| [`merge_recurse`](#merge_recurse) | fn | Recursively merges pre-sorted chunks inside `v`. |
| [`par_mergesort`](#par_mergesort) | fn | Sorts `v` using merge sort in parallel. |

## Structs

### `InsertionHole<T>`

```rust
struct InsertionHole<T> {
    src: *const T,
    dest: *mut T,
}
```

#### Trait Implementations

##### `impl<T> Drop for InsertionHole<T>`

- <span id="insertionhole-drop"></span>`fn drop(&mut self)`

##### `impl<T> IntoEither for InsertionHole<T>`

##### `impl<T> Pointable for InsertionHole<T>`

- <span id="insertionhole-align"></span>`const ALIGN: usize`

- <span id="insertionhole-init"></span>`type Init = T`

- <span id="insertionhole-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="insertionhole-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="insertionhole-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="insertionhole-drop"></span>`unsafe fn drop(ptr: usize)`

### `MergeHole<T>`

```rust
struct MergeHole<T> {
    start: *mut T,
    end: *mut T,
    dest: *mut T,
}
```

#### Trait Implementations

##### `impl<T> Drop for MergeHole<T>`

- <span id="mergehole-drop"></span>`fn drop(&mut self)`

##### `impl<T> IntoEither for MergeHole<T>`

##### `impl<T> Pointable for MergeHole<T>`

- <span id="mergehole-align"></span>`const ALIGN: usize`

- <span id="mergehole-init"></span>`type Init = T`

- <span id="mergehole-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mergehole-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mergehole-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mergehole-drop"></span>`unsafe fn drop(ptr: usize)`

### `TimSortRun`

```rust
struct TimSortRun {
    len: usize,
    start: usize,
}
```

Internal type used by merge_sort.

#### Trait Implementations

##### `impl Clone for TimSortRun`

- <span id="timsortrun-clone"></span>`fn clone(&self) -> TimSortRun` — [`TimSortRun`](#timsortrun)

##### `impl Copy for TimSortRun`

##### `impl Debug for TimSortRun`

- <span id="timsortrun-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TimSortRun`

##### `impl<T> Pointable for TimSortRun`

- <span id="timsortrun-align"></span>`const ALIGN: usize`

- <span id="timsortrun-init"></span>`type Init = T`

- <span id="timsortrun-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="timsortrun-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="timsortrun-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="timsortrun-drop"></span>`unsafe fn drop(ptr: usize)`

## Enums

### `MergeSortResult`

```rust
enum MergeSortResult {
    NonDescending,
    Descending,
    Sorted,
}
```

The result of merge sort.

#### Variants

- **`NonDescending`**

  The slice has already been sorted.

- **`Descending`**

  The slice has been descending and therefore it was left intact.

- **`Sorted`**

  The slice was sorted.

#### Trait Implementations

##### `impl Clone for MergeSortResult`

- <span id="mergesortresult-clone"></span>`fn clone(&self) -> MergeSortResult` — [`MergeSortResult`](#mergesortresult)

##### `impl Copy for MergeSortResult`

##### `impl Eq for MergeSortResult`

##### `impl<T> IntoEither for MergeSortResult`

##### `impl PartialEq for MergeSortResult`

- <span id="mergesortresult-eq"></span>`fn eq(&self, other: &MergeSortResult) -> bool` — [`MergeSortResult`](#mergesortresult)

##### `impl<T> Pointable for MergeSortResult`

- <span id="mergesortresult-align"></span>`const ALIGN: usize`

- <span id="mergesortresult-init"></span>`type Init = T`

- <span id="mergesortresult-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mergesortresult-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mergesortresult-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mergesortresult-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for MergeSortResult`

## Functions

### `insert_tail`

```rust
unsafe fn insert_tail<T, F>(v: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool
```

Inserts `v[v.len() - 1]` into pre-sorted sequence `v[..v.len() - 1]` so that whole `v[..]`
becomes sorted.

### `insert_head`

```rust
unsafe fn insert_head<T, F>(v: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool
```

Inserts `v[0]` into pre-sorted sequence `v[1..]` so that whole `v[..]` becomes sorted.

This is the integral subroutine of insertion sort.

### `insertion_sort_shift_left`

```rust
fn insertion_sort_shift_left<T, F>(v: &mut [T], offset: usize, is_less: &F)
where
    F: Fn(&T, &T) -> bool
```

Sort `v` assuming `v[..offset]` is already sorted.

Never inline this function to avoid code bloat. It still optimizes nicely and has practically no
performance impact. Even improving performance in some cases.

### `insertion_sort_shift_right`

```rust
fn insertion_sort_shift_right<T, F>(v: &mut [T], offset: usize, is_less: &F)
where
    F: Fn(&T, &T) -> bool
```

Sort `v` assuming `v[offset..]` is already sorted.

Never inline this function to avoid code bloat. It still optimizes nicely and has practically no
performance impact. Even improving performance in some cases.

### `partial_insertion_sort`

```rust
fn partial_insertion_sort<T, F>(v: &mut [T], is_less: &F) -> bool
where
    F: Fn(&T, &T) -> bool
```

Partially sorts a slice by shifting several out-of-order elements around.

Returns `true` if the slice is sorted at the end. This function is *O*(*n*) worst-case.

### `heapsort`

```rust
fn heapsort<T, F>(v: &mut [T], is_less: F)
where
    F: Fn(&T, &T) -> bool
```

Sorts `v` using heapsort, which guarantees *O*(*n* \* log(*n*)) worst-case.

### `partition_in_blocks`

```rust
fn partition_in_blocks<T, F>(v: &mut [T], pivot: &T, is_less: &F) -> usize
where
    F: Fn(&T, &T) -> bool
```

Partitions `v` into elements smaller than `pivot`, followed by elements greater than or equal
to `pivot`.

Returns the number of elements smaller than `pivot`.

Partitioning is performed block-by-block in order to minimize the cost of branching operations.
This idea is presented in the [BlockQuicksort][pdf] paper.


### `partition`

```rust
fn partition<T, F>(v: &mut [T], pivot: usize, is_less: &F) -> (usize, bool)
where
    F: Fn(&T, &T) -> bool
```

Partitions `v` into elements smaller than `v[pivot]`, followed by elements greater than or
equal to `v[pivot]`.

Returns a tuple of:

1. Number of elements smaller than `v[pivot]`.
2. True if `v` was already partitioned.

### `partition_equal`

```rust
fn partition_equal<T, F>(v: &mut [T], pivot: usize, is_less: &F) -> usize
where
    F: Fn(&T, &T) -> bool
```

Partitions `v` into elements equal to `v[pivot]` followed by elements greater than `v[pivot]`.

Returns the number of elements equal to the pivot. It is assumed that `v` does not contain
elements smaller than the pivot.

### `break_patterns`

```rust
fn break_patterns<T>(v: &mut [T])
```

Scatters some elements around in an attempt to break patterns that might cause imbalanced
partitions in quicksort.

### `choose_pivot`

```rust
fn choose_pivot<T, F>(v: &mut [T], is_less: &F) -> (usize, bool)
where
    F: Fn(&T, &T) -> bool
```

Chooses a pivot in `v` and returns the index and `true` if the slice is likely already sorted.

Elements in `v` might be reordered in the process.

### `recurse`

```rust
fn recurse<'a, T, F>(v: &'a mut [T], is_less: &F, pred: Option<&'a mut T>, limit: u32)
where
    T: Send,
    F: Fn(&T, &T) -> bool + Sync
```

Sorts `v` recursively.

If the slice had a predecessor in the original array, it is specified as `pred`.

`limit` is the number of allowed imbalanced partitions before switching to `heapsort`. If zero,
this function will immediately switch to heapsort.

### `par_quicksort`

```rust
fn par_quicksort<T, F>(v: &mut [T], is_less: F)
where
    T: Send,
    F: Fn(&T, &T) -> bool + Sync
```

Sorts `v` using pattern-defeating quicksort in parallel.

The algorithm is unstable, in-place, and *O*(*n* \* log(*n*)) worst-case.

### `merge`

```rust
unsafe fn merge<T, F>(v: &mut [T], mid: usize, buf: *mut T, is_less: &F)
where
    F: Fn(&T, &T) -> bool
```

Merges non-decreasing runs `v[..mid]` and `v[mid..]` using `buf` as temporary storage, and
stores the result into `v[..]`.

# Safety

The two slices must be non-empty and `mid` must be in bounds. Buffer `buf` must be long enough
to hold a copy of the shorter slice. Also, `T` must not be a zero-sized type.

### `merge_sort`

```rust
unsafe fn merge_sort<T, CmpF>(v: &mut [T], buf_ptr: *mut T, is_less: &CmpF) -> MergeSortResult
where
    CmpF: Fn(&T, &T) -> bool
```

This merge sort borrows some (but not all) ideas from TimSort, which used to be described in
detail [here](https://github.com/python/cpython/blob/main/Objects/listsort.txt). However Python
has switched to a Powersort based implementation.

The algorithm identifies strictly descending and non-descending subsequences, which are called
natural runs. There is a stack of pending runs yet to be merged. Each newly found run is pushed
onto the stack, and then some pairs of adjacent runs are merged until these two invariants are
satisfied:

1. for every `i` in `1..runs.len()`: `runs[i - 1].len > runs[i].len`
2. for every `i` in `2..runs.len()`: `runs[i - 2].len > runs[i - 1].len + runs[i].len`

The invariants ensure that the total running time is *O*(*n* \* log(*n*)) worst-case.

# Safety

The argument `buf` is used as a temporary buffer and must hold at least `v.len() / 2`.

### `provide_sorted_batch`

```rust
fn provide_sorted_batch<T, F>(v: &mut [T], start: usize, end: usize, is_less: &F) -> usize
where
    F: Fn(&T, &T) -> bool
```

Takes a range as denoted by start and end, that is already sorted and extends it to the right if
necessary with sorts optimized for smaller ranges such as insertion sort.

### `find_streak`

```rust
fn find_streak<T, F>(v: &[T], is_less: &F) -> (usize, bool)
where
    F: Fn(&T, &T) -> bool
```

Finds a streak of presorted elements starting at the beginning of the slice. Returns the first
value that is not part of said streak, and a bool denoting whether the streak was reversed.
Streaks can be increasing or decreasing.

### `split_for_merge`

```rust
fn split_for_merge<T, F>(left: &[T], right: &[T], is_less: &F) -> (usize, usize)
where
    F: Fn(&T, &T) -> bool
```

Splits two sorted slices so that they can be merged in parallel.

Returns two indices `(a, b)` so that slices `left[..a]` and `right[..b]` come before
`left[a..]` and `right[b..]`.

### `par_merge`

```rust
unsafe fn par_merge<T, F>(left: &mut [T], right: &mut [T], dest: *mut T, is_less: &F)
where
    T: Send,
    F: Fn(&T, &T) -> bool + Sync
```

Merges slices `left` and `right` in parallel and stores the result into `dest`.

# Safety

The `dest` pointer must have enough space to store the result.

Even if `is_less` panics at any point during the merge process, this function will fully copy
all elements from `left` and `right` into `dest` (not necessarily in sorted order).

### `merge_recurse`

```rust
unsafe fn merge_recurse<T, F>(v: *mut T, buf: *mut T, chunks: &[(usize, usize)], into_buf: bool, is_less: &F)
where
    T: Send,
    F: Fn(&T, &T) -> bool + Sync
```

Recursively merges pre-sorted chunks inside `v`.

Chunks of `v` are stored in `chunks` as intervals (inclusive left and exclusive right bound).
Argument `buf` is an auxiliary buffer that will be used during the procedure.
If `into_buf` is true, the result will be stored into `buf`, otherwise it will be in `v`.

# Safety

The number of chunks must be positive and they must be adjacent: the right bound of each chunk
must equal the left bound of the following chunk.

The buffer must be at least as long as `v`.

### `par_mergesort`

```rust
fn par_mergesort<T, F>(v: &mut [T], is_less: F)
where
    T: Send,
    F: Fn(&T, &T) -> bool + Sync
```

Sorts `v` using merge sort in parallel.

The algorithm is stable, allocates memory, and `O(n log n)` worst-case.
The allocated temporary buffer is of the same length as is `v`.

