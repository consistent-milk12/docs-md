*[rayon](../../index.md) / [iter](../index.md) / [panic_fuse](index.md)*

---

# Module `panic_fuse`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PanicFuse`](#panicfuse) | struct | `PanicFuse` is an adaptor that wraps an iterator with a fuse in case of panics, to halt all threads as soon as possible. |
| [`Fuse`](#fuse) | struct | Helper that sets a bool to `true` if dropped while unwinding. |
| [`PanicFuseProducer`](#panicfuseproducer) | struct |  |
| [`PanicFuseIter`](#panicfuseiter) | struct |  |
| [`PanicFuseConsumer`](#panicfuseconsumer) | struct |  |
| [`PanicFuseFolder`](#panicfusefolder) | struct |  |
| [`PanicFuseReducer`](#panicfusereducer) | struct |  |

## Structs

### `PanicFuse<I>`

```rust
struct PanicFuse<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:14-16`](../../../../.source_1765633015/rayon-1.11.0/src/iter/panic_fuse.rs#L14-L16)*

`PanicFuse` is an adaptor that wraps an iterator with a fuse in case
of panics, to halt all threads as soon as possible.

This struct is created by the `panic_fuse()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="panicfuse-new"></span>`fn new(base: I) -> PanicFuse<I>` — [`PanicFuse`](#panicfuse)

  Creates a new `PanicFuse` iterator.

#### Trait Implementations

##### `impl Any for PanicFuse<I>`

- <span id="panicfuse-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PanicFuse<I>`

- <span id="panicfuse-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PanicFuse<I>`

- <span id="panicfuse-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for PanicFuse<I>`

- <span id="panicfuse-clone"></span>`fn clone(&self) -> PanicFuse<I>` — [`PanicFuse`](#panicfuse)

##### `impl CloneToUninit for PanicFuse<I>`

- <span id="panicfuse-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for PanicFuse<I>`

- <span id="panicfuse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PanicFuse<I>`

- <span id="panicfuse-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for PanicFuse<I>`

- <span id="panicfuse-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="panicfuse-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="panicfuse-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for PanicFuse<I>`

- <span id="panicfuse-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PanicFuse<I>`

##### `impl IntoParallelIterator for PanicFuse<I>`

- <span id="panicfuse-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="panicfuse-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="panicfuse-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for PanicFuse<I>`

- <span id="panicfuse-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="panicfuse-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="panicfuse-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for PanicFuse<I>`

- <span id="panicfuse-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfuse-pointable-type-init"></span>`type Init = T`

- <span id="panicfuse-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuse-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuse-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuse-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for PanicFuse<I>`

- <span id="panicfuse-toowned-type-owned"></span>`type Owned = T`

- <span id="panicfuse-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="panicfuse-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PanicFuse<I>`

- <span id="panicfuse-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="panicfuse-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PanicFuse<I>`

- <span id="panicfuse-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="panicfuse-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Fuse<'a>`

```rust
struct Fuse<'a>(&'a std::sync::atomic::AtomicBool);
```

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:20`](../../../../.source_1765633015/rayon-1.11.0/src/iter/panic_fuse.rs#L20)*

Helper that sets a bool to `true` if dropped while unwinding.

#### Implementations

- <span id="fuse-panicked"></span>`fn panicked(&self) -> bool`

#### Trait Implementations

##### `impl Any for Fuse<'a>`

- <span id="fuse-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Fuse<'a>`

- <span id="fuse-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fuse<'a>`

- <span id="fuse-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Fuse<'a>`

- <span id="fuse-clone"></span>`fn clone(&self) -> Fuse<'a>` — [`Fuse`](#fuse)

##### `impl CloneToUninit for Fuse<'a>`

- <span id="fuse-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Drop for Fuse<'a>`

- <span id="fuse-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Fuse<'a>`

- <span id="fuse-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Fuse<'a>`

- <span id="fuse-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Fuse<'a>`

##### `impl Pointable for Fuse<'a>`

- <span id="fuse-pointable-const-align"></span>`const ALIGN: usize`

- <span id="fuse-pointable-type-init"></span>`type Init = T`

- <span id="fuse-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fuse-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fuse-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fuse-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Fuse<'a>`

- <span id="fuse-toowned-type-owned"></span>`type Owned = T`

- <span id="fuse-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fuse-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Fuse<'a>`

- <span id="fuse-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fuse-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fuse<'a>`

- <span id="fuse-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fuse-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PanicFuseProducer<'a, P>`

```rust
struct PanicFuseProducer<'a, P> {
    base: P,
    fuse: Fuse<'a>,
}
```

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:122-125`](../../../../.source_1765633015/rayon-1.11.0/src/iter/panic_fuse.rs#L122-L125)*

#### Trait Implementations

##### `impl Any for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PanicFuseProducer<'a, P>`

##### `impl Pointable for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfuseproducer-pointable-type-init"></span>`type Init = T`

- <span id="panicfuseproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuseproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuseproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuseproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-producer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="panicfuseproducer-producer-type-intoiter"></span>`type IntoIter = PanicFuseIter<'a, <P as Producer>::IntoIter>`

- <span id="panicfuseproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="panicfuseproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="panicfuseproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="panicfuseproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="panicfuseproducer-producer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

##### `impl<U> TryFrom for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="panicfuseproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="panicfuseproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PanicFuseIter<'a, I>`

```rust
struct PanicFuseIter<'a, I> {
    base: I,
    fuse: Fuse<'a>,
}
```

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:174-177`](../../../../.source_1765633015/rayon-1.11.0/src/iter/panic_fuse.rs#L174-L177)*

#### Trait Implementations

##### `impl Any for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> DoubleEndedIterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I> ExactSizeIterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PanicFuseIter<'a, I>`

##### `impl<I> IntoIterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="panicfuseiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="panicfuseiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="panicfuseiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="panicfuseiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Pointable for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfuseiter-pointable-type-init"></span>`type Init = T`

- <span id="panicfuseiter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuseiter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuseiter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuseiter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="panicfuseiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="panicfuseiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PanicFuseConsumer<'a, C>`

```rust
struct PanicFuseConsumer<'a, C> {
    base: C,
    fuse: Fuse<'a>,
}
```

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:223-226`](../../../../.source_1765633015/rayon-1.11.0/src/iter/panic_fuse.rs#L223-L226)*

#### Trait Implementations

##### `impl Any for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Consumer for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-consumer-type-folder"></span>`type Folder = PanicFuseFolder<'a, <C as Consumer>::Folder>`

- <span id="panicfuseconsumer-consumer-type-reducer"></span>`type Reducer = PanicFuseReducer<'a, <C as Consumer>::Reducer>`

- <span id="panicfuseconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="panicfuseconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="panicfuseconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="panicfuseconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PanicFuseConsumer<'a, C>`

##### `impl Pointable for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfuseconsumer-pointable-type-init"></span>`type Init = T`

- <span id="panicfuseconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuseconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuseconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuseconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="panicfuseconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="panicfuseconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C> UnindexedConsumer for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="panicfuseconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `PanicFuseFolder<'a, C>`

```rust
struct PanicFuseFolder<'a, C> {
    base: C,
    fuse: Fuse<'a>,
}
```

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:285-288`](../../../../.source_1765633015/rayon-1.11.0/src/iter/panic_fuse.rs#L285-L288)*

#### Trait Implementations

##### `impl Any for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Folder for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="panicfusefolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="panicfusefolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="panicfusefolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="panicfusefolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PanicFuseFolder<'a, C>`

##### `impl Pointable for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfusefolder-pointable-type-init"></span>`type Init = T`

- <span id="panicfusefolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfusefolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfusefolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfusefolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="panicfusefolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="panicfusefolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PanicFuseReducer<'a, C>`

```rust
struct PanicFuseReducer<'a, C> {
    base: C,
    _fuse: Fuse<'a>,
}
```

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:326-329`](../../../../.source_1765633015/rayon-1.11.0/src/iter/panic_fuse.rs#L326-L329)*

#### Trait Implementations

##### `impl Any for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PanicFuseReducer<'a, C>`

##### `impl Pointable for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfusereducer-pointable-type-init"></span>`type Init = T`

- <span id="panicfusereducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfusereducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfusereducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfusereducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, C> Reducer for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-reducer-reduce"></span>`fn reduce(self, left: T, right: T) -> T`

##### `impl<U> TryFrom for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="panicfusereducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="panicfusereducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

