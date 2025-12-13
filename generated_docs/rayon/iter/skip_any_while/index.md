*[rayon](../../index.md) / [iter](../index.md) / [skip_any_while](index.md)*

---

# Module `skip_any_while`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SkipAnyWhile`](#skipanywhile) | struct | `SkipAnyWhile` is an iterator that skips over elements from anywhere in `I` until the callback returns `false`. |
| [`SkipAnyWhileConsumer`](#skipanywhileconsumer) | struct |  |
| [`SkipAnyWhileFolder`](#skipanywhilefolder) | struct |  |
| [`skip`](#skip) | fn |  |

## Structs

### `SkipAnyWhile<I, P>`

```rust
struct SkipAnyWhile<I, P> {
    base: I,
    predicate: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any_while.rs:13-16`](../../../../.source_1765521767/rayon-1.11.0/src/iter/skip_any_while.rs#L13-L16)*

`SkipAnyWhile` is an iterator that skips over elements from anywhere in `I`
until the callback returns `false`.
This struct is created by the `skip_any_while()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="skipanywhile-new"></span>`fn new(base: I, predicate: P) -> Self`

  Creates a new `SkipAnyWhile` iterator.

#### Trait Implementations

##### `impl Any for SkipAnyWhile<I, P>`

- <span id="skipanywhile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SkipAnyWhile<I, P>`

- <span id="skipanywhile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SkipAnyWhile<I, P>`

- <span id="skipanywhile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, P: clone::Clone> Clone for SkipAnyWhile<I, P>`

- <span id="skipanywhile-clone"></span>`fn clone(&self) -> SkipAnyWhile<I, P>` — [`SkipAnyWhile`](#skipanywhile)

##### `impl CloneToUninit for SkipAnyWhile<I, P>`

- <span id="skipanywhile-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, P> Debug for SkipAnyWhile<I, P>`

- <span id="skipanywhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SkipAnyWhile<I, P>`

- <span id="skipanywhile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SkipAnyWhile<I, P>`

- <span id="skipanywhile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SkipAnyWhile<I, P>`

##### `impl IntoParallelIterator for SkipAnyWhile<I, P>`

- <span id="skipanywhile-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="skipanywhile-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skipanywhile-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for SkipAnyWhile<I, P>`

- <span id="skipanywhile-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skipanywhile-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for SkipAnyWhile<I, P>`

- <span id="skipanywhile-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skipanywhile-pointable-type-init"></span>`type Init = T`

- <span id="skipanywhile-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanywhile-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanywhile-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanywhile-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for SkipAnyWhile<I, P>`

- <span id="skipanywhile-toowned-type-owned"></span>`type Owned = T`

- <span id="skipanywhile-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="skipanywhile-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SkipAnyWhile<I, P>`

- <span id="skipanywhile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skipanywhile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SkipAnyWhile<I, P>`

- <span id="skipanywhile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skipanywhile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SkipAnyWhileConsumer<'p, C, P>`

```rust
struct SkipAnyWhileConsumer<'p, C, P> {
    base: C,
    predicate: &'p P,
    skipping: &'p std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any_while.rs:56-60`](../../../../.source_1765521767/rayon-1.11.0/src/iter/skip_any_while.rs#L56-L60)*

#### Trait Implementations

##### `impl Any for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, P> Consumer for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-consumer-type-folder"></span>`type Folder = SkipAnyWhileFolder<'p, <C as Consumer>::Folder, P>`

- <span id="skipanywhileconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="skipanywhileconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="skipanywhileconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="skipanywhileconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="skipanywhileconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SkipAnyWhileConsumer<'p, C, P>`

##### `impl Pointable for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skipanywhileconsumer-pointable-type-init"></span>`type Init = T`

- <span id="skipanywhileconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanywhileconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanywhileconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanywhileconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skipanywhileconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skipanywhileconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C, P> UnindexedConsumer for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="skipanywhileconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `SkipAnyWhileFolder<'p, C, P>`

```rust
struct SkipAnyWhileFolder<'p, C, P> {
    base: C,
    predicate: &'p P,
    skipping: &'p std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any_while.rs:113-117`](../../../../.source_1765521767/rayon-1.11.0/src/iter/skip_any_while.rs#L113-L117)*

#### Trait Implementations

##### `impl Any for SkipAnyWhileFolder<'p, C, P>`

- <span id="skipanywhilefolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SkipAnyWhileFolder<'p, C, P>`

- <span id="skipanywhilefolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SkipAnyWhileFolder<'p, C, P>`

- <span id="skipanywhilefolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, P> Folder for SkipAnyWhileFolder<'p, C, P>`

- <span id="skipanywhilefolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="skipanywhilefolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="skipanywhilefolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="skipanywhilefolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="skipanywhilefolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for SkipAnyWhileFolder<'p, C, P>`

- <span id="skipanywhilefolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SkipAnyWhileFolder<'p, C, P>`

- <span id="skipanywhilefolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SkipAnyWhileFolder<'p, C, P>`

##### `impl Pointable for SkipAnyWhileFolder<'p, C, P>`

- <span id="skipanywhilefolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skipanywhilefolder-pointable-type-init"></span>`type Init = T`

- <span id="skipanywhilefolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanywhilefolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanywhilefolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanywhilefolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SkipAnyWhileFolder<'p, C, P>`

- <span id="skipanywhilefolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skipanywhilefolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SkipAnyWhileFolder<'p, C, P>`

- <span id="skipanywhilefolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skipanywhilefolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `skip`

```rust
fn skip<T>(item: &T, skipping: &std::sync::atomic::AtomicBool, predicate: &impl Fn(&T) -> bool) -> bool
```

*Defined in [`rayon-1.11.0/src/iter/skip_any_while.rs:119-128`](../../../../.source_1765521767/rayon-1.11.0/src/iter/skip_any_while.rs#L119-L128)*

