*[rayon](../../index.md) / [iter](../index.md) / [blocks](index.md)*

---

# Module `blocks`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BlocksCallback`](#blockscallback) | struct |  |
| [`ExponentialBlocks`](#exponentialblocks) | struct | `ExponentialBlocks` is a parallel iterator that consumes itself as a sequence |
| [`UniformBlocks`](#uniformblocks) | struct | `UniformBlocks` is a parallel iterator that consumes itself as a sequence |
| [`exponential_size`](#exponential_size) | fn |  |

## Structs

### `BlocksCallback<S, C>`

```rust
struct BlocksCallback<S, C> {
    sizes: S,
    consumer: C,
    len: usize,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for BlocksCallback<S, C>`

##### `impl<T> Pointable for BlocksCallback<S, C>`

- <span id="blockscallback-align"></span>`const ALIGN: usize`

- <span id="blockscallback-init"></span>`type Init = T`

- <span id="blockscallback-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="blockscallback-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="blockscallback-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="blockscallback-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, S, C> ProducerCallback for BlocksCallback<S, C>`

- <span id="blockscallback-output"></span>`type Output = <C as Consumer>::Result`

- <span id="blockscallback-callback"></span>`fn callback<P: Producer<Item = T>>(self, producer: P) -> <Self as >::Output` — [`ProducerCallback`](../plumbing/index.md)

### `ExponentialBlocks<I>`

```rust
struct ExponentialBlocks<I> {
    base: I,
}
```

`ExponentialBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of increasing sizes (exponentially).

This struct is created by the `by_exponential_blocks()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="exponentialblocks-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for ExponentialBlocks<I>`

- <span id="exponentialblocks-clone"></span>`fn clone(&self) -> ExponentialBlocks<I>` — [`ExponentialBlocks`](../index.md)

##### `impl<I: fmt::Debug> Debug for ExponentialBlocks<I>`

- <span id="exponentialblocks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for ExponentialBlocks<I>`

##### `impl<T> IntoParallelIterator for ExponentialBlocks<I>`

- <span id="exponentialblocks-iter"></span>`type Iter = T`

- <span id="exponentialblocks-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="exponentialblocks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for ExponentialBlocks<I>`

- <span id="exponentialblocks-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="exponentialblocks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for ExponentialBlocks<I>`

- <span id="exponentialblocks-align"></span>`const ALIGN: usize`

- <span id="exponentialblocks-init"></span>`type Init = T`

- <span id="exponentialblocks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="exponentialblocks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="exponentialblocks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="exponentialblocks-drop"></span>`unsafe fn drop(ptr: usize)`

### `UniformBlocks<I>`

```rust
struct UniformBlocks<I> {
    base: I,
    block_size: usize,
}
```

`UniformBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of constant sizes.

This struct is created by the `by_uniform_blocks()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="uniformblocks-new"></span>`fn new(base: I, block_size: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for UniformBlocks<I>`

- <span id="uniformblocks-clone"></span>`fn clone(&self) -> UniformBlocks<I>` — [`UniformBlocks`](../index.md)

##### `impl<I: fmt::Debug> Debug for UniformBlocks<I>`

- <span id="uniformblocks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for UniformBlocks<I>`

##### `impl<T> IntoParallelIterator for UniformBlocks<I>`

- <span id="uniformblocks-iter"></span>`type Iter = T`

- <span id="uniformblocks-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="uniformblocks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for UniformBlocks<I>`

- <span id="uniformblocks-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="uniformblocks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for UniformBlocks<I>`

- <span id="uniformblocks-align"></span>`const ALIGN: usize`

- <span id="uniformblocks-init"></span>`type Init = T`

- <span id="uniformblocks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="uniformblocks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="uniformblocks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="uniformblocks-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `exponential_size`

```rust
fn exponential_size(size: &usize) -> Option<usize>
```

