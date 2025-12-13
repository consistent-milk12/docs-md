*[rayon](../../index.md) / [iter](../index.md) / [enumerate](index.md)*

---

# Module `enumerate`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Enumerate`](#enumerate) | struct | `Enumerate` is an iterator that returns the current count along with the element. |
| [`EnumerateProducer`](#enumerateproducer) | struct |  |

## Structs

### `Enumerate<I>`

```rust
struct Enumerate<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/enumerate.rs:12-14`](../../../../.source_1765633015/rayon-1.11.0/src/iter/enumerate.rs#L12-L14)*

`Enumerate` is an iterator that returns the current count along with the element.
This struct is created by the `enumerate()` method on [`IndexedParallelIterator`](../index.md)


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

- <span id="enumerate-clone"></span>`fn clone(&self) -> Enumerate<I>` — [`Enumerate`](#enumerate)

##### `impl CloneToUninit for Enumerate<I>`

- <span id="enumerate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Enumerate<I>`

- <span id="enumerate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Enumerate<I>`

- <span id="enumerate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Enumerate<I>`

- <span id="enumerate-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="enumerate-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="enumerate-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

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

- <span id="enumerate-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

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

### `EnumerateProducer<P>`

```rust
struct EnumerateProducer<P> {
    base: P,
    offset: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/enumerate.rs:82-85`](../../../../.source_1765633015/rayon-1.11.0/src/iter/enumerate.rs#L82-L85)*

#### Trait Implementations

##### `impl Any for EnumerateProducer<P>`

- <span id="enumerateproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EnumerateProducer<P>`

- <span id="enumerateproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EnumerateProducer<P>`

- <span id="enumerateproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for EnumerateProducer<P>`

- <span id="enumerateproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EnumerateProducer<P>`

- <span id="enumerateproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for EnumerateProducer<P>`

##### `impl Pointable for EnumerateProducer<P>`

- <span id="enumerateproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="enumerateproducer-pointable-type-init"></span>`type Init = T`

- <span id="enumerateproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="enumerateproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="enumerateproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="enumerateproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for EnumerateProducer<P>`

- <span id="enumerateproducer-producer-type-item"></span>`type Item = (usize, <P as Producer>::Item)`

- <span id="enumerateproducer-producer-type-intoiter"></span>`type IntoIter = Zip<Range<usize>, <P as Producer>::IntoIter>`

- <span id="enumerateproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="enumerateproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="enumerateproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="enumerateproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<U> TryFrom for EnumerateProducer<P>`

- <span id="enumerateproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="enumerateproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EnumerateProducer<P>`

- <span id="enumerateproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="enumerateproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

