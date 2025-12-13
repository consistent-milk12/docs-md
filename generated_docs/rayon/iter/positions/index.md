*[rayon](../../index.md) / [iter](../index.md) / [positions](index.md)*

---

# Module `positions`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Positions`](#positions) | struct | `Positions` takes a predicate `predicate` and filters out elements that match, yielding their indices. |
| [`PositionsConsumer`](#positionsconsumer) | struct |  |
| [`PositionsFolder`](#positionsfolder) | struct |  |

## Structs

### `Positions<I, P>`

```rust
struct Positions<I, P> {
    base: I,
    predicate: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/positions.rs:14-17`](../../../../.source_1765633015/rayon-1.11.0/src/iter/positions.rs#L14-L17)*

`Positions` takes a predicate `predicate` and filters out elements that match,
yielding their indices.

This struct is created by the `positions()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="positions-new"></span>`fn new(base: I, predicate: P) -> Self`

  Create a new `Positions` iterator.

#### Trait Implementations

##### `impl Any for Positions<I, P>`

- <span id="positions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Positions<I, P>`

- <span id="positions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Positions<I, P>`

- <span id="positions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, P: clone::Clone> Clone for Positions<I, P>`

- <span id="positions-clone"></span>`fn clone(&self) -> Positions<I, P>` — [`Positions`](#positions)

##### `impl CloneToUninit for Positions<I, P>`

- <span id="positions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, P> Debug for Positions<I, P>`

- <span id="positions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Positions<I, P>`

- <span id="positions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Positions<I, P>`

- <span id="positions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Positions<I, P>`

##### `impl IntoParallelIterator for Positions<I, P>`

- <span id="positions-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="positions-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="positions-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for Positions<I, P>`

- <span id="positions-paralleliterator-type-item"></span>`type Item = usize`

- <span id="positions-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for Positions<I, P>`

- <span id="positions-pointable-const-align"></span>`const ALIGN: usize`

- <span id="positions-pointable-type-init"></span>`type Init = T`

- <span id="positions-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="positions-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="positions-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="positions-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Positions<I, P>`

- <span id="positions-toowned-type-owned"></span>`type Owned = T`

- <span id="positions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="positions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Positions<I, P>`

- <span id="positions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="positions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Positions<I, P>`

- <span id="positions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="positions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PositionsConsumer<'p, C, P>`

```rust
struct PositionsConsumer<'p, C, P> {
    base: C,
    predicate: &'p P,
    offset: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/positions.rs:53-57`](../../../../.source_1765633015/rayon-1.11.0/src/iter/positions.rs#L53-L57)*

#### Implementations

- <span id="positionsconsumer-new"></span>`fn new(base: C, predicate: &'p P, offset: usize) -> Self`

#### Trait Implementations

##### `impl Any for PositionsConsumer<'p, C, P>`

- <span id="positionsconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PositionsConsumer<'p, C, P>`

- <span id="positionsconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PositionsConsumer<'p, C, P>`

- <span id="positionsconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, P> Consumer for PositionsConsumer<'p, C, P>`

- <span id="positionsconsumer-consumer-type-folder"></span>`type Folder = PositionsFolder<'p, <C as Consumer>::Folder, P>`

- <span id="positionsconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="positionsconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="positionsconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="positionsconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="positionsconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for PositionsConsumer<'p, C, P>`

- <span id="positionsconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PositionsConsumer<'p, C, P>`

- <span id="positionsconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PositionsConsumer<'p, C, P>`

##### `impl Pointable for PositionsConsumer<'p, C, P>`

- <span id="positionsconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="positionsconsumer-pointable-type-init"></span>`type Init = T`

- <span id="positionsconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="positionsconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="positionsconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="positionsconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for PositionsConsumer<'p, C, P>`

- <span id="positionsconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="positionsconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PositionsConsumer<'p, C, P>`

- <span id="positionsconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="positionsconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PositionsFolder<'p, F, P>`

```rust
struct PositionsFolder<'p, F, P> {
    base: F,
    predicate: &'p P,
    offset: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/positions.rs:100-104`](../../../../.source_1765633015/rayon-1.11.0/src/iter/positions.rs#L100-L104)*

#### Trait Implementations

##### `impl Any for PositionsFolder<'p, F, P>`

- <span id="positionsfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PositionsFolder<'p, F, P>`

- <span id="positionsfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PositionsFolder<'p, F, P>`

- <span id="positionsfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F, P, T> Folder for PositionsFolder<'_, F, P>`

- <span id="positionsfolder-folder-type-result"></span>`type Result = <F as Folder>::Result`

- <span id="positionsfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="positionsfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="positionsfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for PositionsFolder<'p, F, P>`

- <span id="positionsfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PositionsFolder<'p, F, P>`

- <span id="positionsfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PositionsFolder<'p, F, P>`

##### `impl Pointable for PositionsFolder<'p, F, P>`

- <span id="positionsfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="positionsfolder-pointable-type-init"></span>`type Init = T`

- <span id="positionsfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="positionsfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="positionsfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="positionsfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for PositionsFolder<'p, F, P>`

- <span id="positionsfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="positionsfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PositionsFolder<'p, F, P>`

- <span id="positionsfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="positionsfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

