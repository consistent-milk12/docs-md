*[rayon](../../index.md) / [iter](../index.md) / [len](index.md)*

---

# Module `len`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MinLen`](#minlen) | struct | `MinLen` is an iterator that imposes a minimum length on iterator splits. |
| [`MinLenProducer`](#minlenproducer) | struct |  |
| [`MaxLen`](#maxlen) | struct | `MaxLen` is an iterator that imposes a maximum length on iterator splits. |
| [`MaxLenProducer`](#maxlenproducer) | struct |  |

## Structs

### `MinLen<I>`

```rust
struct MinLen<I> {
    base: I,
    min: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/len.rs:10-13`](../../../../.source_1765521767/rayon-1.11.0/src/iter/len.rs#L10-L13)*

`MinLen` is an iterator that imposes a minimum length on iterator splits.
This struct is created by the `with_min_len()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="minlen-new"></span>`fn new(base: I, min: usize) -> Self`

  Creates a new `MinLen` iterator.

#### Trait Implementations

##### `impl Any for MinLen<I>`

- <span id="minlen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MinLen<I>`

- <span id="minlen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MinLen<I>`

- <span id="minlen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for MinLen<I>`

- <span id="minlen-clone"></span>`fn clone(&self) -> MinLen<I>` — [`MinLen`](#minlen)

##### `impl CloneToUninit for MinLen<I>`

- <span id="minlen-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for MinLen<I>`

- <span id="minlen-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MinLen<I>`

- <span id="minlen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for MinLen<I>`

- <span id="minlen-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="minlen-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="minlen-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for MinLen<I>`

- <span id="minlen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MinLen<I>`

##### `impl IntoParallelIterator for MinLen<I>`

- <span id="minlen-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="minlen-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="minlen-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for MinLen<I>`

- <span id="minlen-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="minlen-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="minlen-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for MinLen<I>`

- <span id="minlen-pointable-const-align"></span>`const ALIGN: usize`

- <span id="minlen-pointable-type-init"></span>`type Init = T`

- <span id="minlen-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="minlen-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="minlen-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="minlen-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for MinLen<I>`

- <span id="minlen-toowned-type-owned"></span>`type Owned = T`

- <span id="minlen-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="minlen-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MinLen<I>`

- <span id="minlen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="minlen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MinLen<I>`

- <span id="minlen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="minlen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MinLenProducer<P>`

```rust
struct MinLenProducer<P> {
    base: P,
    min: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/len.rs:88-91`](../../../../.source_1765521767/rayon-1.11.0/src/iter/len.rs#L88-L91)*

#### Trait Implementations

##### `impl Any for MinLenProducer<P>`

- <span id="minlenproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MinLenProducer<P>`

- <span id="minlenproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MinLenProducer<P>`

- <span id="minlenproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MinLenProducer<P>`

- <span id="minlenproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MinLenProducer<P>`

- <span id="minlenproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MinLenProducer<P>`

##### `impl Pointable for MinLenProducer<P>`

- <span id="minlenproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="minlenproducer-pointable-type-init"></span>`type Init = T`

- <span id="minlenproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="minlenproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="minlenproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="minlenproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for MinLenProducer<P>`

- <span id="minlenproducer-producer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="minlenproducer-producer-type-intoiter"></span>`type IntoIter = <P as Producer>::IntoIter`

- <span id="minlenproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="minlenproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="minlenproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="minlenproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="minlenproducer-producer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

##### `impl<U> TryFrom for MinLenProducer<P>`

- <span id="minlenproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="minlenproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MinLenProducer<P>`

- <span id="minlenproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="minlenproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MaxLen<I>`

```rust
struct MaxLen<I> {
    base: I,
    max: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/len.rs:140-143`](../../../../.source_1765521767/rayon-1.11.0/src/iter/len.rs#L140-L143)*

`MaxLen` is an iterator that imposes a maximum length on iterator splits.
This struct is created by the `with_max_len()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="maxlen-new"></span>`fn new(base: I, max: usize) -> Self`

  Creates a new `MaxLen` iterator.

#### Trait Implementations

##### `impl Any for MaxLen<I>`

- <span id="maxlen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MaxLen<I>`

- <span id="maxlen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MaxLen<I>`

- <span id="maxlen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for MaxLen<I>`

- <span id="maxlen-clone"></span>`fn clone(&self) -> MaxLen<I>` — [`MaxLen`](#maxlen)

##### `impl CloneToUninit for MaxLen<I>`

- <span id="maxlen-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for MaxLen<I>`

- <span id="maxlen-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MaxLen<I>`

- <span id="maxlen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for MaxLen<I>`

- <span id="maxlen-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="maxlen-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="maxlen-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for MaxLen<I>`

- <span id="maxlen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MaxLen<I>`

##### `impl IntoParallelIterator for MaxLen<I>`

- <span id="maxlen-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="maxlen-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="maxlen-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for MaxLen<I>`

- <span id="maxlen-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="maxlen-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="maxlen-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for MaxLen<I>`

- <span id="maxlen-pointable-const-align"></span>`const ALIGN: usize`

- <span id="maxlen-pointable-type-init"></span>`type Init = T`

- <span id="maxlen-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="maxlen-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="maxlen-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="maxlen-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for MaxLen<I>`

- <span id="maxlen-toowned-type-owned"></span>`type Owned = T`

- <span id="maxlen-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="maxlen-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MaxLen<I>`

- <span id="maxlen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="maxlen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MaxLen<I>`

- <span id="maxlen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="maxlen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MaxLenProducer<P>`

```rust
struct MaxLenProducer<P> {
    base: P,
    max: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/len.rs:218-221`](../../../../.source_1765521767/rayon-1.11.0/src/iter/len.rs#L218-L221)*

#### Trait Implementations

##### `impl Any for MaxLenProducer<P>`

- <span id="maxlenproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MaxLenProducer<P>`

- <span id="maxlenproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MaxLenProducer<P>`

- <span id="maxlenproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MaxLenProducer<P>`

- <span id="maxlenproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MaxLenProducer<P>`

- <span id="maxlenproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MaxLenProducer<P>`

##### `impl Pointable for MaxLenProducer<P>`

- <span id="maxlenproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="maxlenproducer-pointable-type-init"></span>`type Init = T`

- <span id="maxlenproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="maxlenproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="maxlenproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="maxlenproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for MaxLenProducer<P>`

- <span id="maxlenproducer-producer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="maxlenproducer-producer-type-intoiter"></span>`type IntoIter = <P as Producer>::IntoIter`

- <span id="maxlenproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="maxlenproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="maxlenproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="maxlenproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="maxlenproducer-producer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

##### `impl<U> TryFrom for MaxLenProducer<P>`

- <span id="maxlenproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="maxlenproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MaxLenProducer<P>`

- <span id="maxlenproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="maxlenproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

