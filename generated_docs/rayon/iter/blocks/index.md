*[rayon](../../index.md) / [iter](../index.md) / [blocks](index.md)*

---

# Module `blocks`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BlocksCallback`](#blockscallback) | struct |  |
| [`ExponentialBlocks`](#exponentialblocks) | struct | `ExponentialBlocks` is a parallel iterator that consumes itself as a sequence of parallel blocks of increasing sizes (exponentially). |
| [`UniformBlocks`](#uniformblocks) | struct | `UniformBlocks` is a parallel iterator that consumes itself as a sequence of parallel blocks of constant sizes. |
| [`exponential_size`](#exponential-size) | fn |  |

## Structs

### `BlocksCallback<S, C>`

```rust
struct BlocksCallback<S, C> {
    sizes: S,
    consumer: C,
    len: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/blocks.rs:4-8`](../../../../.source_1765521767/rayon-1.11.0/src/iter/blocks.rs#L4-L8)*

#### Trait Implementations

##### `impl Any for BlocksCallback<S, C>`

- <span id="blockscallback-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BlocksCallback<S, C>`

- <span id="blockscallback-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BlocksCallback<S, C>`

- <span id="blockscallback-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for BlocksCallback<S, C>`

- <span id="blockscallback-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BlocksCallback<S, C>`

- <span id="blockscallback-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for BlocksCallback<S, C>`

##### `impl Pointable for BlocksCallback<S, C>`

- <span id="blockscallback-pointable-const-align"></span>`const ALIGN: usize`

- <span id="blockscallback-pointable-type-init"></span>`type Init = T`

- <span id="blockscallback-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="blockscallback-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="blockscallback-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="blockscallback-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, S, C> ProducerCallback for BlocksCallback<S, C>`

- <span id="blockscallback-producercallback-type-output"></span>`type Output = <C as Consumer>::Result`

- <span id="blockscallback-producercallback-callback"></span>`fn callback<P: Producer<Item = T>>(self, producer: P) -> <Self as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> TryFrom for BlocksCallback<S, C>`

- <span id="blockscallback-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="blockscallback-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BlocksCallback<S, C>`

- <span id="blockscallback-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="blockscallback-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExponentialBlocks<I>`

```rust
struct ExponentialBlocks<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/blocks.rs:59-61`](../../../../.source_1765521767/rayon-1.11.0/src/iter/blocks.rs#L59-L61)*

`ExponentialBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of increasing sizes (exponentially).

This struct is created by the `by_exponential_blocks()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="exponentialblocks-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl Any for ExponentialBlocks<I>`

- <span id="exponentialblocks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExponentialBlocks<I>`

- <span id="exponentialblocks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExponentialBlocks<I>`

- <span id="exponentialblocks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for ExponentialBlocks<I>`

- <span id="exponentialblocks-clone"></span>`fn clone(&self) -> ExponentialBlocks<I>` — [`ExponentialBlocks`](#exponentialblocks)

##### `impl CloneToUninit for ExponentialBlocks<I>`

- <span id="exponentialblocks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for ExponentialBlocks<I>`

- <span id="exponentialblocks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ExponentialBlocks<I>`

- <span id="exponentialblocks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ExponentialBlocks<I>`

- <span id="exponentialblocks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ExponentialBlocks<I>`

##### `impl IntoParallelIterator for ExponentialBlocks<I>`

- <span id="exponentialblocks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="exponentialblocks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="exponentialblocks-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for ExponentialBlocks<I>`

- <span id="exponentialblocks-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="exponentialblocks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for ExponentialBlocks<I>`

- <span id="exponentialblocks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="exponentialblocks-pointable-type-init"></span>`type Init = T`

- <span id="exponentialblocks-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="exponentialblocks-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="exponentialblocks-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="exponentialblocks-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for ExponentialBlocks<I>`

- <span id="exponentialblocks-toowned-type-owned"></span>`type Owned = T`

- <span id="exponentialblocks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exponentialblocks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ExponentialBlocks<I>`

- <span id="exponentialblocks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exponentialblocks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExponentialBlocks<I>`

- <span id="exponentialblocks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exponentialblocks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UniformBlocks<I>`

```rust
struct UniformBlocks<I> {
    base: I,
    block_size: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/blocks.rs:101-104`](../../../../.source_1765521767/rayon-1.11.0/src/iter/blocks.rs#L101-L104)*

`UniformBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of constant sizes.

This struct is created by the `by_uniform_blocks()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="uniformblocks-new"></span>`fn new(base: I, block_size: usize) -> Self`

#### Trait Implementations

##### `impl Any for UniformBlocks<I>`

- <span id="uniformblocks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UniformBlocks<I>`

- <span id="uniformblocks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UniformBlocks<I>`

- <span id="uniformblocks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for UniformBlocks<I>`

- <span id="uniformblocks-clone"></span>`fn clone(&self) -> UniformBlocks<I>` — [`UniformBlocks`](#uniformblocks)

##### `impl CloneToUninit for UniformBlocks<I>`

- <span id="uniformblocks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for UniformBlocks<I>`

- <span id="uniformblocks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UniformBlocks<I>`

- <span id="uniformblocks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UniformBlocks<I>`

- <span id="uniformblocks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UniformBlocks<I>`

##### `impl IntoParallelIterator for UniformBlocks<I>`

- <span id="uniformblocks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="uniformblocks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="uniformblocks-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for UniformBlocks<I>`

- <span id="uniformblocks-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="uniformblocks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for UniformBlocks<I>`

- <span id="uniformblocks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="uniformblocks-pointable-type-init"></span>`type Init = T`

- <span id="uniformblocks-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="uniformblocks-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="uniformblocks-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="uniformblocks-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for UniformBlocks<I>`

- <span id="uniformblocks-toowned-type-owned"></span>`type Owned = T`

- <span id="uniformblocks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="uniformblocks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UniformBlocks<I>`

- <span id="uniformblocks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uniformblocks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UniformBlocks<I>`

- <span id="uniformblocks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uniformblocks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `exponential_size`

```rust
fn exponential_size(size: &usize) -> Option<usize>
```

*Defined in [`rayon-1.11.0/src/iter/blocks.rs:89-91`](../../../../.source_1765521767/rayon-1.11.0/src/iter/blocks.rs#L89-L91)*

