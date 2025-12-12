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

##### `impl IntoEither for BlocksCallback<S, C>`

##### `impl Pointable for BlocksCallback<S, C>`

- <span id="blockscallback-pointable-const-align"></span>`const ALIGN: usize`

- <span id="blockscallback-pointable-type-init"></span>`type Init = T`

- <span id="blockscallback-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="blockscallback-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="blockscallback-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="blockscallback-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, S, C> ProducerCallback for BlocksCallback<S, C>`

- <span id="blockscallback-producercallback-type-output"></span>`type Output = <C as Consumer>::Result`

- <span id="blockscallback-callback"></span>`fn callback<P: Producer<Item = T>>(self, producer: P) -> <Self as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

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

##### `impl<I: clone::Clone> Clone for ExponentialBlocks<I>`

- <span id="exponentialblocks-clone"></span>`fn clone(&self) -> ExponentialBlocks<I>` — [`ExponentialBlocks`](#exponentialblocks)

##### `impl<I: fmt::Debug> Debug for ExponentialBlocks<I>`

- <span id="exponentialblocks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for ExponentialBlocks<I>`

##### `impl IntoParallelIterator for ExponentialBlocks<I>`

- <span id="exponentialblocks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="exponentialblocks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="exponentialblocks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for ExponentialBlocks<I>`

- <span id="exponentialblocks-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="exponentialblocks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for ExponentialBlocks<I>`

- <span id="exponentialblocks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="exponentialblocks-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/iter/blocks.rs:101-104`](../../../../.source_1765521767/rayon-1.11.0/src/iter/blocks.rs#L101-L104)*

`UniformBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of constant sizes.

This struct is created by the `by_uniform_blocks()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="uniformblocks-new"></span>`fn new(base: I, block_size: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for UniformBlocks<I>`

- <span id="uniformblocks-clone"></span>`fn clone(&self) -> UniformBlocks<I>` — [`UniformBlocks`](#uniformblocks)

##### `impl<I: fmt::Debug> Debug for UniformBlocks<I>`

- <span id="uniformblocks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for UniformBlocks<I>`

##### `impl IntoParallelIterator for UniformBlocks<I>`

- <span id="uniformblocks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="uniformblocks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="uniformblocks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for UniformBlocks<I>`

- <span id="uniformblocks-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="uniformblocks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for UniformBlocks<I>`

- <span id="uniformblocks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="uniformblocks-pointable-type-init"></span>`type Init = T`

- <span id="uniformblocks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="uniformblocks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="uniformblocks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="uniformblocks-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `exponential_size`

```rust
fn exponential_size(size: &usize) -> Option<usize>
```

*Defined in [`rayon-1.11.0/src/iter/blocks.rs:89-91`](../../../../.source_1765521767/rayon-1.11.0/src/iter/blocks.rs#L89-L91)*

