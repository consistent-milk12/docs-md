*[rayon](../../index.md) / [iter](../index.md) / [skip_any](index.md)*

---

# Module `skip_any`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SkipAny`](#skipany) | struct | `SkipAny` is an iterator that skips over `n` elements from anywhere in `I`. |
| [`SkipAnyConsumer`](#skipanyconsumer) | struct |  |
| [`SkipAnyFolder`](#skipanyfolder) | struct |  |
| [`checked_decrement`](#checked-decrement) | fn |  |

## Structs

### `SkipAny<I>`

```rust
struct SkipAny<I> {
    base: I,
    count: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any.rs:11-14`](../../../../.source_1765633015/rayon-1.11.0/src/iter/skip_any.rs#L11-L14)*

`SkipAny` is an iterator that skips over `n` elements from anywhere in `I`.
This struct is created by the `skip_any()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="skipany-new"></span>`fn new(base: I, count: usize) -> Self`

  Creates a new `SkipAny` iterator.

#### Trait Implementations

##### `impl Any for SkipAny<I>`

- <span id="skipany-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SkipAny<I>`

- <span id="skipany-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SkipAny<I>`

- <span id="skipany-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for SkipAny<I>`

- <span id="skipany-clone"></span>`fn clone(&self) -> SkipAny<I>` — [`SkipAny`](#skipany)

##### `impl CloneToUninit for SkipAny<I>`

- <span id="skipany-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for SkipAny<I>`

- <span id="skipany-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SkipAny<I>`

- <span id="skipany-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SkipAny<I>`

- <span id="skipany-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SkipAny<I>`

##### `impl IntoParallelIterator for SkipAny<I>`

- <span id="skipany-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="skipany-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skipany-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for SkipAny<I>`

- <span id="skipany-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skipany-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for SkipAny<I>`

- <span id="skipany-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skipany-pointable-type-init"></span>`type Init = T`

- <span id="skipany-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipany-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipany-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipany-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for SkipAny<I>`

- <span id="skipany-toowned-type-owned"></span>`type Owned = T`

- <span id="skipany-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="skipany-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SkipAny<I>`

- <span id="skipany-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skipany-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SkipAny<I>`

- <span id="skipany-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skipany-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SkipAnyConsumer<'f, C>`

```rust
struct SkipAnyConsumer<'f, C> {
    base: C,
    count: &'f std::sync::atomic::AtomicUsize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any.rs:44-47`](../../../../.source_1765633015/rayon-1.11.0/src/iter/skip_any.rs#L44-L47)*

#### Trait Implementations

##### `impl Any for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Consumer for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-consumer-type-folder"></span>`type Folder = SkipAnyFolder<'f, <C as Consumer>::Folder>`

- <span id="skipanyconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="skipanyconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="skipanyconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="skipanyconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="skipanyconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SkipAnyConsumer<'f, C>`

##### `impl Pointable for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skipanyconsumer-pointable-type-init"></span>`type Init = T`

- <span id="skipanyconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanyconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanyconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanyconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skipanyconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skipanyconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C> UnindexedConsumer for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="skipanyconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `SkipAnyFolder<'f, C>`

```rust
struct SkipAnyFolder<'f, C> {
    base: C,
    count: &'f std::sync::atomic::AtomicUsize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any.rs:99-102`](../../../../.source_1765633015/rayon-1.11.0/src/iter/skip_any.rs#L99-L102)*

#### Trait Implementations

##### `impl Any for SkipAnyFolder<'f, C>`

- <span id="skipanyfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SkipAnyFolder<'f, C>`

- <span id="skipanyfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SkipAnyFolder<'f, C>`

- <span id="skipanyfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Folder for SkipAnyFolder<'f, C>`

- <span id="skipanyfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="skipanyfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="skipanyfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="skipanyfolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="skipanyfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for SkipAnyFolder<'f, C>`

- <span id="skipanyfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SkipAnyFolder<'f, C>`

- <span id="skipanyfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SkipAnyFolder<'f, C>`

##### `impl Pointable for SkipAnyFolder<'f, C>`

- <span id="skipanyfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skipanyfolder-pointable-type-init"></span>`type Init = T`

- <span id="skipanyfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanyfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanyfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanyfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SkipAnyFolder<'f, C>`

- <span id="skipanyfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skipanyfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SkipAnyFolder<'f, C>`

- <span id="skipanyfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skipanyfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `checked_decrement`

```rust
fn checked_decrement(u: &std::sync::atomic::AtomicUsize) -> bool
```

*Defined in [`rayon-1.11.0/src/iter/skip_any.rs:104-107`](../../../../.source_1765633015/rayon-1.11.0/src/iter/skip_any.rs#L104-L107)*

