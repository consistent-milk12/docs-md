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
  - [`insert_tail`](#insert-tail)
  - [`insert_head`](#insert-head)
  - [`insertion_sort_shift_left`](#insertion-sort-shift-left)
  - [`insertion_sort_shift_right`](#insertion-sort-shift-right)
  - [`partial_insertion_sort`](#partial-insertion-sort)
  - [`heapsort`](#heapsort)
  - [`partition_in_blocks`](#partition-in-blocks)
  - [`partition`](#partition)
  - [`partition_equal`](#partition-equal)
  - [`break_patterns`](#break-patterns)
  - [`choose_pivot`](#choose-pivot)
  - [`recurse`](#recurse)
  - [`par_quicksort`](#par-quicksort)
  - [`merge`](#merge)
  - [`merge_sort`](#merge-sort)
  - [`provide_sorted_batch`](#provide-sorted-batch)
  - [`find_streak`](#find-streak)
  - [`split_for_merge`](#split-for-merge)
  - [`par_merge`](#par-merge)
  - [`merge_recurse`](#merge-recurse)
  - [`par_mergesort`](#par-mergesort)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`InsertionHole`](#insertionhole) | struct |  |
| [`MergeHole`](#mergehole) | struct |  |
| [`TimSortRun`](#timsortrun) | struct | Internal type used by merge_sort. |
| [`MergeSortResult`](#mergesortresult) | enum | The result of merge sort. |
| [`insert_tail`](#insert-tail) | fn | Inserts `v[v.len() - 1]` into pre-sorted sequence `v[..v.len() - 1]` so that whole `v[..]` becomes sorted. |
| [`insert_head`](#insert-head) | fn | Inserts `v[0]` into pre-sorted sequence `v[1..]` so that whole `v[..]` becomes sorted. |
| [`insertion_sort_shift_left`](#insertion-sort-shift-left) | fn | Sort `v` assuming `v[..offset]` is already sorted. |
| [`insertion_sort_shift_right`](#insertion-sort-shift-right) | fn | Sort `v` assuming `v[offset..]` is already sorted. |
| [`partial_insertion_sort`](#partial-insertion-sort) | fn | Partially sorts a slice by shifting several out-of-order elements around. |
| [`heapsort`](#heapsort) | fn | Sorts `v` using heapsort, which guarantees *O*(*n* \* log(*n*)) worst-case. |
| [`partition_in_blocks`](#partition-in-blocks) | fn | Partitions `v` into elements smaller than `pivot`, followed by elements greater than or equal to `pivot`. |
| [`partition`](#partition) | fn | Partitions `v` into elements smaller than `v[pivot]`, followed by elements greater than or equal to `v[pivot]`. |
| [`partition_equal`](#partition-equal) | fn | Partitions `v` into elements equal to `v[pivot]` followed by elements greater than `v[pivot]`. |
| [`break_patterns`](#break-patterns) | fn | Scatters some elements around in an attempt to break patterns that might cause imbalanced partitions in quicksort. |
| [`choose_pivot`](#choose-pivot) | fn | Chooses a pivot in `v` and returns the index and `true` if the slice is likely already sorted. |
| [`recurse`](#recurse) | fn | Sorts `v` recursively. |
| [`par_quicksort`](#par-quicksort) | fn | Sorts `v` using pattern-defeating quicksort in parallel. |
| [`merge`](#merge) | fn | Merges non-decreasing runs `v[..mid]` and `v[mid..]` using `buf` as temporary storage, and stores the result into `v[..]`. |
| [`merge_sort`](#merge-sort) | fn | This merge sort borrows some (but not all) ideas from TimSort, which used to be described in detail [here](https://github.com/python/cpython/blob/main/Objects/listsort.txt). |
| [`provide_sorted_batch`](#provide-sorted-batch) | fn | Takes a range as denoted by start and end, that is already sorted and extends it to the right if necessary with sorts optimized for smaller ranges such as insertion sort. |
| [`find_streak`](#find-streak) | fn | Finds a streak of presorted elements starting at the beginning of the slice. |
| [`split_for_merge`](#split-for-merge) | fn | Splits two sorted slices so that they can be merged in parallel. |
| [`par_merge`](#par-merge) | fn | Merges slices `left` and `right` in parallel and stores the result into `dest`. |
| [`merge_recurse`](#merge-recurse) | fn | Recursively merges pre-sorted chunks inside `v`. |
| [`par_mergesort`](#par-mergesort) | fn | Sorts `v` using merge sort in parallel. |

## Structs

### `InsertionHole<T>`

```rust
struct InsertionHole<T> {
    src: *const T,
    dest: *mut T,
}
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:27-30`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L27-L30)*

#### Trait Implementations

##### `impl<T> Drop for InsertionHole<T>`

- <span id="insertionhole-drop"></span>`fn drop(&mut self)`

##### `impl<T> IntoEither for InsertionHole<T>`

##### `impl<T> Pointable for InsertionHole<T>`

- <span id="insertionhole-pointable-const-align"></span>`const ALIGN: usize`

- <span id="insertionhole-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:1055-1059`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L1055-L1059)*

#### Trait Implementations

##### `impl<T> Drop for MergeHole<T>`

- <span id="mergehole-drop"></span>`fn drop(&mut self)`

##### `impl<T> IntoEither for MergeHole<T>`

##### `impl<T> Pointable for MergeHole<T>`

- <span id="mergehole-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mergehole-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:1202-1205`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L1202-L1205)*

Internal type used by merge_sort.

#### Trait Implementations

##### `impl Clone for TimSortRun`

- <span id="timsortrun-clone"></span>`fn clone(&self) -> TimSortRun` — [`TimSortRun`](#timsortrun)

##### `impl Copy for TimSortRun`

##### `impl Debug for TimSortRun`

- <span id="timsortrun-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for TimSortRun`

##### `impl Pointable for TimSortRun`

- <span id="timsortrun-pointable-const-align"></span>`const ALIGN: usize`

- <span id="timsortrun-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:1074-1081`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L1074-L1081)*

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

##### `impl IntoEither for MergeSortResult`

##### `impl PartialEq for MergeSortResult`

- <span id="mergesortresult-eq"></span>`fn eq(&self, other: &MergeSortResult) -> bool` — [`MergeSortResult`](#mergesortresult)

##### `impl Pointable for MergeSortResult`

- <span id="mergesortresult-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mergesortresult-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:45-96`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L45-L96)*

Inserts `v[v.len() - 1]` into pre-sorted sequence `v[..v.len() - 1]` so that whole `v[..]`
becomes sorted.

### `insert_head`

```rust
unsafe fn insert_head<T, F>(v: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:101-157`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L101-L157)*

Inserts `v[0]` into pre-sorted sequence `v[1..]` so that whole `v[..]` becomes sorted.

This is the integral subroutine of insertion sort.

### `insertion_sort_shift_left`

```rust
fn insertion_sort_shift_left<T, F>(v: &mut [T], offset: usize, is_less: &F)
where
    F: Fn(&T, &T) -> bool
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:164-182`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L164-L182)*

Sort `v` assuming `v[..offset]` is already sorted.

Never inline this function to avoid code bloat. It still optimizes nicely and has practically no
performance impact. Even improving performance in some cases.

### `insertion_sort_shift_right`

```rust
fn insertion_sort_shift_right<T, F>(v: &mut [T], offset: usize, is_less: &F)
where
    F: Fn(&T, &T) -> bool
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:189-208`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L189-L208)*

Sort `v` assuming `v[offset..]` is already sorted.

Never inline this function to avoid code bloat. It still optimizes nicely and has practically no
performance impact. Even improving performance in some cases.

### `partial_insertion_sort`

```rust
fn partial_insertion_sort<T, F>(v: &mut [T], is_less: &F) -> bool
where
    F: Fn(&T, &T) -> bool
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:214-260`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L214-L260)*

Partially sorts a slice by shifting several out-of-order elements around.

Returns `true` if the slice is sorted at the end. This function is *O*(*n*) worst-case.

### `heapsort`

```rust
fn heapsort<T, F>(v: &mut [T], is_less: F)
where
    F: Fn(&T, &T) -> bool
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:264-306`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L264-L306)*

Sorts `v` using heapsort, which guarantees *O*(*n* \* log(*n*)) worst-case.

### `partition_in_blocks`

```rust
fn partition_in_blocks<T, F>(v: &mut [T], pivot: &T, is_less: &F) -> usize
where
    F: Fn(&T, &T) -> bool
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:317-566`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L317-L566)*

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:575-630`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L575-L630)*

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:636-699`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L636-L699)*

Partitions `v` into elements equal to `v[pivot]` followed by elements greater than `v[pivot]`.

Returns the number of elements equal to the pivot. It is assumed that `v` does not contain
elements smaller than the pivot.

### `break_patterns`

```rust
fn break_patterns<T>(v: &mut [T])
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:704-748`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L704-L748)*

Scatters some elements around in an attempt to break patterns that might cause imbalanced
partitions in quicksort.

### `choose_pivot`

```rust
fn choose_pivot<T, F>(v: &mut [T], is_less: &F) -> (usize, bool)
where
    F: Fn(&T, &T) -> bool
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:753-821`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L753-L821)*

Chooses a pivot in `v` and returns the index and `true` if the slice is likely already sorted.

Elements in `v` might be reordered in the process.

### `recurse`

```rust
fn recurse<'a, T, F>(v: &'a mut [T], is_less: &F, pred: Option<&'a mut T>, limit: u32)
where
    T: Send,
    F: Fn(&T, &T) -> bool + Sync
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:829-928`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L829-L928)*

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:933-947`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L933-L947)*

Sorts `v` using pattern-defeating quicksort in parallel.

The algorithm is unstable, in-place, and *O*(*n* \* log(*n*)) worst-case.

### `merge`

```rust
unsafe fn merge<T, F>(v: &mut [T], mid: usize, buf: *mut T, is_less: &F)
where
    F: Fn(&T, &T) -> bool
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:956-1052`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L956-L1052)*

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:1100-1198`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L1100-L1198)*

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:1209-1235`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L1209-L1235)*

Takes a range as denoted by start and end, that is already sorted and extends it to the right if
necessary with sorts optimized for smaller ranges such as insertion sort.

### `find_streak`

```rust
fn find_streak<T, F>(v: &[T], is_less: &F) -> (usize, bool)
where
    F: Fn(&T, &T) -> bool
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:1240-1272`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L1240-L1272)*

Finds a streak of presorted elements starting at the beginning of the slice. Returns the first
value that is not part of said streak, and a bool denoting whether the streak was reversed.
Streaks can be increasing or decreasing.

### `split_for_merge`

```rust
fn split_for_merge<T, F>(left: &[T], right: &[T], is_less: &F) -> (usize, usize)
where
    F: Fn(&T, &T) -> bool
```

*Defined in [`rayon-1.11.0/src/slice/sort.rs:1283-1323`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L1283-L1323)*

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:1333-1421`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L1333-L1421)*

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:1435-1504`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L1435-L1504)*

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

*Defined in [`rayon-1.11.0/src/slice/sort.rs:1510-1608`](../../../../.source_1765521767/rayon-1.11.0/src/slice/sort.rs#L1510-L1608)*

Sorts `v` using merge sort in parallel.

The algorithm is stable, allocates memory, and `O(n log n)` worst-case.
The allocated temporary buffer is of the same length as is `v`.

