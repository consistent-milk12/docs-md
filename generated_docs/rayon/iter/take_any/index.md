*[rayon](../../index.md) / [iter](../index.md) / [take_any](index.md)*

---

# Module `take_any`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TakeAny`](#takeany) | struct | `TakeAny` is an iterator that iterates over `n` elements from anywhere in `I`. |
| [`TakeAnyConsumer`](#takeanyconsumer) | struct |  |
| [`TakeAnyFolder`](#takeanyfolder) | struct |  |
| [`checked_decrement`](#checked-decrement) | fn |  |

## Structs

### `TakeAny<I>`

```rust
struct TakeAny<I> {
    base: I,
    count: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any.rs:11-14`](../../../../.source_1765521767/rayon-1.11.0/src/iter/take_any.rs#L11-L14)*

`TakeAny` is an iterator that iterates over `n` elements from anywhere in `I`.
This struct is created by the `take_any()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="takeany-new"></span>`fn new(base: I, count: usize) -> Self`

  Creates a new `TakeAny` iterator.

#### Trait Implementations

##### `impl Any for TakeAny<I>`

- <span id="takeany-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TakeAny<I>`

- <span id="takeany-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TakeAny<I>`

- <span id="takeany-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for TakeAny<I>`

- <span id="takeany-clone"></span>`fn clone(&self) -> TakeAny<I>` — [`TakeAny`](#takeany)

##### `impl CloneToUninit for TakeAny<I>`

- <span id="takeany-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for TakeAny<I>`

- <span id="takeany-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TakeAny<I>`

- <span id="takeany-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TakeAny<I>`

- <span id="takeany-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TakeAny<I>`

##### `impl IntoParallelIterator for TakeAny<I>`

- <span id="takeany-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="takeany-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="takeany-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for TakeAny<I>`

- <span id="takeany-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="takeany-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for TakeAny<I>`

- <span id="takeany-pointable-const-align"></span>`const ALIGN: usize`

- <span id="takeany-pointable-type-init"></span>`type Init = T`

- <span id="takeany-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeany-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeany-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeany-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TakeAny<I>`

- <span id="takeany-toowned-type-owned"></span>`type Owned = T`

- <span id="takeany-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="takeany-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TakeAny<I>`

- <span id="takeany-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="takeany-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TakeAny<I>`

- <span id="takeany-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="takeany-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TakeAnyConsumer<'f, C>`

```rust
struct TakeAnyConsumer<'f, C> {
    base: C,
    count: &'f std::sync::atomic::AtomicUsize,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any.rs:44-47`](../../../../.source_1765521767/rayon-1.11.0/src/iter/take_any.rs#L44-L47)*

#### Trait Implementations

##### `impl Any for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Consumer for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-consumer-type-folder"></span>`type Folder = TakeAnyFolder<'f, <C as Consumer>::Folder>`

- <span id="takeanyconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="takeanyconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="takeanyconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="takeanyconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="takeanyconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TakeAnyConsumer<'f, C>`

##### `impl Pointable for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="takeanyconsumer-pointable-type-init"></span>`type Init = T`

- <span id="takeanyconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanyconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanyconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanyconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="takeanyconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="takeanyconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C> UnindexedConsumer for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="takeanyconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `TakeAnyFolder<'f, C>`

```rust
struct TakeAnyFolder<'f, C> {
    base: C,
    count: &'f std::sync::atomic::AtomicUsize,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any.rs:99-102`](../../../../.source_1765521767/rayon-1.11.0/src/iter/take_any.rs#L99-L102)*

#### Trait Implementations

##### `impl Any for TakeAnyFolder<'f, C>`

- <span id="takeanyfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TakeAnyFolder<'f, C>`

- <span id="takeanyfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TakeAnyFolder<'f, C>`

- <span id="takeanyfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Folder for TakeAnyFolder<'f, C>`

- <span id="takeanyfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="takeanyfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="takeanyfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="takeanyfolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="takeanyfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for TakeAnyFolder<'f, C>`

- <span id="takeanyfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TakeAnyFolder<'f, C>`

- <span id="takeanyfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TakeAnyFolder<'f, C>`

##### `impl Pointable for TakeAnyFolder<'f, C>`

- <span id="takeanyfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="takeanyfolder-pointable-type-init"></span>`type Init = T`

- <span id="takeanyfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanyfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanyfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanyfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for TakeAnyFolder<'f, C>`

- <span id="takeanyfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="takeanyfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TakeAnyFolder<'f, C>`

- <span id="takeanyfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="takeanyfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `checked_decrement`

```rust
fn checked_decrement(u: &std::sync::atomic::AtomicUsize) -> bool
```

*Defined in [`rayon-1.11.0/src/iter/take_any.rs:104-107`](../../../../.source_1765521767/rayon-1.11.0/src/iter/take_any.rs#L104-L107)*

