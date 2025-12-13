*[rayon](../../index.md) / [iter](../index.md) / [while_some](index.md)*

---

# Module `while_some`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WhileSome`](#whilesome) | struct | `WhileSome` is an iterator that yields the `Some` elements of an iterator, halting as soon as any `None` is produced. |
| [`WhileSomeConsumer`](#whilesomeconsumer) | struct |  |
| [`WhileSomeFolder`](#whilesomefolder) | struct |  |

## Structs

### `WhileSome<I>`

```rust
struct WhileSome<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/while_some.rs:13-15`](../../../../.source_1765633015/rayon-1.11.0/src/iter/while_some.rs#L13-L15)*

`WhileSome` is an iterator that yields the `Some` elements of an iterator,
halting as soon as any `None` is produced.

This struct is created by the `while_some()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="whilesome-new"></span>`fn new(base: I) -> Self`

  Creates a new `WhileSome` iterator.

#### Trait Implementations

##### `impl Any for WhileSome<I>`

- <span id="whilesome-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WhileSome<I>`

- <span id="whilesome-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WhileSome<I>`

- <span id="whilesome-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for WhileSome<I>`

- <span id="whilesome-clone"></span>`fn clone(&self) -> WhileSome<I>` — [`WhileSome`](#whilesome)

##### `impl CloneToUninit for WhileSome<I>`

- <span id="whilesome-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for WhileSome<I>`

- <span id="whilesome-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WhileSome<I>`

- <span id="whilesome-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WhileSome<I>`

- <span id="whilesome-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WhileSome<I>`

##### `impl IntoParallelIterator for WhileSome<I>`

- <span id="whilesome-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="whilesome-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="whilesome-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for WhileSome<I>`

- <span id="whilesome-paralleliterator-type-item"></span>`type Item = T`

- <span id="whilesome-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for WhileSome<I>`

- <span id="whilesome-pointable-const-align"></span>`const ALIGN: usize`

- <span id="whilesome-pointable-type-init"></span>`type Init = T`

- <span id="whilesome-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="whilesome-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="whilesome-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="whilesome-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for WhileSome<I>`

- <span id="whilesome-toowned-type-owned"></span>`type Owned = T`

- <span id="whilesome-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="whilesome-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WhileSome<I>`

- <span id="whilesome-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="whilesome-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WhileSome<I>`

- <span id="whilesome-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="whilesome-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WhileSomeConsumer<'f, C>`

```rust
struct WhileSomeConsumer<'f, C> {
    base: C,
    full: &'f std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/while_some.rs:47-50`](../../../../.source_1765633015/rayon-1.11.0/src/iter/while_some.rs#L47-L50)*

#### Trait Implementations

##### `impl Any for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Consumer for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-consumer-type-folder"></span>`type Folder = WhileSomeFolder<'f, <C as Consumer>::Folder>`

- <span id="whilesomeconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="whilesomeconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="whilesomeconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="whilesomeconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="whilesomeconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WhileSomeConsumer<'f, C>`

##### `impl Pointable for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="whilesomeconsumer-pointable-type-init"></span>`type Init = T`

- <span id="whilesomeconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="whilesomeconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="whilesomeconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="whilesomeconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="whilesomeconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="whilesomeconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C> UnindexedConsumer for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="whilesomeconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `WhileSomeFolder<'f, C>`

```rust
struct WhileSomeFolder<'f, C> {
    base: C,
    full: &'f std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/while_some.rs:102-105`](../../../../.source_1765633015/rayon-1.11.0/src/iter/while_some.rs#L102-L105)*

#### Trait Implementations

##### `impl Any for WhileSomeFolder<'f, C>`

- <span id="whilesomefolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WhileSomeFolder<'f, C>`

- <span id="whilesomefolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WhileSomeFolder<'f, C>`

- <span id="whilesomefolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Folder for WhileSomeFolder<'f, C>`

- <span id="whilesomefolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="whilesomefolder-folder-consume"></span>`fn consume(self, item: Option<T>) -> Self`

- <span id="whilesomefolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="whilesomefolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="whilesomefolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for WhileSomeFolder<'f, C>`

- <span id="whilesomefolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WhileSomeFolder<'f, C>`

- <span id="whilesomefolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WhileSomeFolder<'f, C>`

##### `impl Pointable for WhileSomeFolder<'f, C>`

- <span id="whilesomefolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="whilesomefolder-pointable-type-init"></span>`type Init = T`

- <span id="whilesomefolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="whilesomefolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="whilesomefolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="whilesomefolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WhileSomeFolder<'f, C>`

- <span id="whilesomefolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="whilesomefolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WhileSomeFolder<'f, C>`

- <span id="whilesomefolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="whilesomefolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

