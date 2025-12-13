*[rayon](../../index.md) / [iter](../index.md) / [take_any_while](index.md)*

---

# Module `take_any_while`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TakeAnyWhile`](#takeanywhile) | struct | `TakeAnyWhile` is an iterator that iterates over elements from anywhere in `I` until the callback returns `false`. |
| [`TakeAnyWhileConsumer`](#takeanywhileconsumer) | struct |  |
| [`TakeAnyWhileFolder`](#takeanywhilefolder) | struct |  |
| [`take`](#take) | fn |  |

## Structs

### `TakeAnyWhile<I, P>`

```rust
struct TakeAnyWhile<I, P> {
    base: I,
    predicate: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any_while.rs:13-16`](../../../../.source_1765633015/rayon-1.11.0/src/iter/take_any_while.rs#L13-L16)*

`TakeAnyWhile` is an iterator that iterates over elements from anywhere in `I`
until the callback returns `false`.
This struct is created by the `take_any_while()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="takeanywhile-new"></span>`fn new(base: I, predicate: P) -> Self`

  Creates a new `TakeAnyWhile` iterator.

#### Trait Implementations

##### `impl Any for TakeAnyWhile<I, P>`

- <span id="takeanywhile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TakeAnyWhile<I, P>`

- <span id="takeanywhile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TakeAnyWhile<I, P>`

- <span id="takeanywhile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, P: clone::Clone> Clone for TakeAnyWhile<I, P>`

- <span id="takeanywhile-clone"></span>`fn clone(&self) -> TakeAnyWhile<I, P>` — [`TakeAnyWhile`](#takeanywhile)

##### `impl CloneToUninit for TakeAnyWhile<I, P>`

- <span id="takeanywhile-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, P> Debug for TakeAnyWhile<I, P>`

- <span id="takeanywhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TakeAnyWhile<I, P>`

- <span id="takeanywhile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TakeAnyWhile<I, P>`

- <span id="takeanywhile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TakeAnyWhile<I, P>`

##### `impl IntoParallelIterator for TakeAnyWhile<I, P>`

- <span id="takeanywhile-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="takeanywhile-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="takeanywhile-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for TakeAnyWhile<I, P>`

- <span id="takeanywhile-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="takeanywhile-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for TakeAnyWhile<I, P>`

- <span id="takeanywhile-pointable-const-align"></span>`const ALIGN: usize`

- <span id="takeanywhile-pointable-type-init"></span>`type Init = T`

- <span id="takeanywhile-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanywhile-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanywhile-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanywhile-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TakeAnyWhile<I, P>`

- <span id="takeanywhile-toowned-type-owned"></span>`type Owned = T`

- <span id="takeanywhile-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="takeanywhile-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TakeAnyWhile<I, P>`

- <span id="takeanywhile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="takeanywhile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TakeAnyWhile<I, P>`

- <span id="takeanywhile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="takeanywhile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TakeAnyWhileConsumer<'p, C, P>`

```rust
struct TakeAnyWhileConsumer<'p, C, P> {
    base: C,
    predicate: &'p P,
    taking: &'p std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any_while.rs:56-60`](../../../../.source_1765633015/rayon-1.11.0/src/iter/take_any_while.rs#L56-L60)*

#### Trait Implementations

##### `impl Any for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, P> Consumer for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-consumer-type-folder"></span>`type Folder = TakeAnyWhileFolder<'p, <C as Consumer>::Folder, P>`

- <span id="takeanywhileconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="takeanywhileconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="takeanywhileconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="takeanywhileconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="takeanywhileconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TakeAnyWhileConsumer<'p, C, P>`

##### `impl Pointable for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="takeanywhileconsumer-pointable-type-init"></span>`type Init = T`

- <span id="takeanywhileconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanywhileconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanywhileconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanywhileconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="takeanywhileconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="takeanywhileconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C, P> UnindexedConsumer for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="takeanywhileconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `TakeAnyWhileFolder<'p, C, P>`

```rust
struct TakeAnyWhileFolder<'p, C, P> {
    base: C,
    predicate: &'p P,
    taking: &'p std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any_while.rs:113-117`](../../../../.source_1765633015/rayon-1.11.0/src/iter/take_any_while.rs#L113-L117)*

#### Trait Implementations

##### `impl Any for TakeAnyWhileFolder<'p, C, P>`

- <span id="takeanywhilefolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TakeAnyWhileFolder<'p, C, P>`

- <span id="takeanywhilefolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TakeAnyWhileFolder<'p, C, P>`

- <span id="takeanywhilefolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, P> Folder for TakeAnyWhileFolder<'p, C, P>`

- <span id="takeanywhilefolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="takeanywhilefolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="takeanywhilefolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="takeanywhilefolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="takeanywhilefolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for TakeAnyWhileFolder<'p, C, P>`

- <span id="takeanywhilefolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TakeAnyWhileFolder<'p, C, P>`

- <span id="takeanywhilefolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TakeAnyWhileFolder<'p, C, P>`

##### `impl Pointable for TakeAnyWhileFolder<'p, C, P>`

- <span id="takeanywhilefolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="takeanywhilefolder-pointable-type-init"></span>`type Init = T`

- <span id="takeanywhilefolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanywhilefolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanywhilefolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanywhilefolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for TakeAnyWhileFolder<'p, C, P>`

- <span id="takeanywhilefolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="takeanywhilefolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TakeAnyWhileFolder<'p, C, P>`

- <span id="takeanywhilefolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="takeanywhilefolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `take`

```rust
fn take<T>(item: &T, taking: &std::sync::atomic::AtomicBool, predicate: &impl Fn(&T) -> bool) -> bool
```

*Defined in [`rayon-1.11.0/src/iter/take_any_while.rs:119-128`](../../../../.source_1765633015/rayon-1.11.0/src/iter/take_any_while.rs#L119-L128)*

