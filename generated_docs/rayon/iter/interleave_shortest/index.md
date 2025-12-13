*[rayon](../../index.md) / [iter](../index.md) / [interleave_shortest](index.md)*

---

# Module `interleave_shortest`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`InterleaveShortest`](#interleaveshortest) | struct | `InterleaveShortest` is an iterator that works similarly to `Interleave`, but this version stops returning elements once one of the iterators run out. |

## Structs

### `InterleaveShortest<I, J>`

```rust
struct InterleaveShortest<I, J> {
    interleave: Interleave<Take<I>, Take<J>>,
}
```

*Defined in [`rayon-1.11.0/src/iter/interleave_shortest.rs:14-16`](../../../../.source_1765633015/rayon-1.11.0/src/iter/interleave_shortest.rs#L14-L16)*

`InterleaveShortest` is an iterator that works similarly to
`Interleave`, but this version stops returning elements once one
of the iterators run out.

This struct is created by the `interleave_shortest()` method on
[`IndexedParallelIterator`](../index.md).


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

- <span id="interleaveshortest-clone"></span>`fn clone(&self) -> InterleaveShortest<I, J>` — [`InterleaveShortest`](#interleaveshortest)

##### `impl CloneToUninit for InterleaveShortest<I, J>`

- <span id="interleaveshortest-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, J: fmt::Debug> Debug for InterleaveShortest<I, J>`

- <span id="interleaveshortest-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for InterleaveShortest<I, J>`

- <span id="interleaveshortest-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, J> IndexedParallelIterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="interleaveshortest-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="interleaveshortest-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

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

- <span id="interleaveshortest-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

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

