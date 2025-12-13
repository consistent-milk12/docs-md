*[rayon](../../index.md) / [iter](../index.md) / [splitter](index.md)*

---

# Module `splitter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Split`](#split) | struct | `Split` is a parallel iterator using arbitrary data and a splitting function. |
| [`SplitProducer`](#splitproducer) | struct |  |
| [`split`](#split) | fn | The `split` function takes arbitrary data and a closure that knows how to split it, and turns this into a `ParallelIterator`. |

## Structs

### `Split<D, S>`

```rust
struct Split<D, S> {
    data: D,
    splitter: S,
}
```

*Defined in [`rayon-1.11.0/src/iter/splitter.rs:117-120`](../../../../.source_1765633015/rayon-1.11.0/src/iter/splitter.rs#L117-L120)*

`Split` is a parallel iterator using arbitrary data and a splitting function.
This struct is created by the [`split()`](#split) function.

#### Trait Implementations

##### `impl Any for Split<D, S>`

- <span id="split-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Split<D, S>`

- <span id="split-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Split<D, S>`

- <span id="split-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<D: clone::Clone, S: clone::Clone> Clone for Split<D, S>`

- <span id="split-clone"></span>`fn clone(&self) -> Split<D, S>` — [`Split`](#split)

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

- <span id="split-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

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

### `SplitProducer<'a, D, S>`

```rust
struct SplitProducer<'a, D, S> {
    data: D,
    splitter: &'a S,
}
```

*Defined in [`rayon-1.11.0/src/iter/splitter.rs:147-150`](../../../../.source_1765633015/rayon-1.11.0/src/iter/splitter.rs#L147-L150)*

#### Trait Implementations

##### `impl Any for SplitProducer<'a, D, S>`

- <span id="splitproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitProducer<'a, D, S>`

- <span id="splitproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitProducer<'a, D, S>`

- <span id="splitproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SplitProducer<'a, D, S>`

- <span id="splitproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SplitProducer<'a, D, S>`

- <span id="splitproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SplitProducer<'a, D, S>`

##### `impl Pointable for SplitProducer<'a, D, S>`

- <span id="splitproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitproducer-pointable-type-init"></span>`type Init = T`

- <span id="splitproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SplitProducer<'a, D, S>`

- <span id="splitproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SplitProducer<'a, D, S>`

- <span id="splitproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<D, S> UnindexedProducer for SplitProducer<'a, D, S>`

- <span id="splitproducer-unindexedproducer-type-item"></span>`type Item = D`

- <span id="splitproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="splitproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

## Functions

### `split`

```rust
fn split<D, S>(data: D, splitter: S) -> Split<D, S>
where
    D: Send,
    S: Fn(D) -> (D, Option<D>) + Sync
```

*Defined in [`rayon-1.11.0/src/iter/splitter.rs:106-112`](../../../../.source_1765633015/rayon-1.11.0/src/iter/splitter.rs#L106-L112)*

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


