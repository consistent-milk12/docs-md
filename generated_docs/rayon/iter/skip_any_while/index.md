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

*Defined in [`rayon-1.11.0/src/iter/skip_any_while.rs:13-16`](../../../../.source_1765210505/rayon-1.11.0/src/iter/skip_any_while.rs#L13-L16)*

`SkipAnyWhile` is an iterator that skips over elements from anywhere in `I`
until the callback returns `false`.
This struct is created by the `skip_any_while()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="skipanywhile-new"></span>`fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for SkipAnyWhile<I, P>`

- <span id="skipanywhile-clone"></span>`fn clone(&self) -> SkipAnyWhile<I, P>` — [`SkipAnyWhile`](#skipanywhile)

##### `impl<I: fmt::Debug, P> Debug for SkipAnyWhile<I, P>`

- <span id="skipanywhile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for SkipAnyWhile<I, P>`

##### `impl IntoParallelIterator for SkipAnyWhile<I, P>`

- <span id="skipanywhile-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="skipanywhile-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skipanywhile-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for SkipAnyWhile<I, P>`

- <span id="skipanywhile-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skipanywhile-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for SkipAnyWhile<I, P>`

- <span id="skipanywhile-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skipanywhile-pointable-type-init"></span>`type Init = T`

- <span id="skipanywhile-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanywhile-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanywhile-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanywhile-drop"></span>`unsafe fn drop(ptr: usize)`

### `SkipAnyWhileConsumer<'p, C, P>`

```rust
struct SkipAnyWhileConsumer<'p, C, P> {
    base: C,
    predicate: &'p P,
    skipping: &'p std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any_while.rs:56-60`](../../../../.source_1765210505/rayon-1.11.0/src/iter/skip_any_while.rs#L56-L60)*

#### Trait Implementations

##### `impl<T, C, P> Consumer for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-consumer-type-folder"></span>`type Folder = SkipAnyWhileFolder<'p, <C as Consumer>::Folder, P>`

- <span id="skipanywhileconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="skipanywhileconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="skipanywhileconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="skipanywhileconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="skipanywhileconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for SkipAnyWhileConsumer<'p, C, P>`

##### `impl Pointable for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skipanywhileconsumer-pointable-type-init"></span>`type Init = T`

- <span id="skipanywhileconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanywhileconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanywhileconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanywhileconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, C, P> UnindexedConsumer for SkipAnyWhileConsumer<'p, C, P>`

- <span id="skipanywhileconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="skipanywhileconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `SkipAnyWhileFolder<'p, C, P>`

```rust
struct SkipAnyWhileFolder<'p, C, P> {
    base: C,
    predicate: &'p P,
    skipping: &'p std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any_while.rs:113-117`](../../../../.source_1765210505/rayon-1.11.0/src/iter/skip_any_while.rs#L113-L117)*

#### Trait Implementations

##### `impl<T, C, P> Folder for SkipAnyWhileFolder<'p, C, P>`

- <span id="skipanywhilefolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="skipanywhilefolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="skipanywhilefolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="skipanywhilefolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="skipanywhilefolder-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for SkipAnyWhileFolder<'p, C, P>`

##### `impl Pointable for SkipAnyWhileFolder<'p, C, P>`

- <span id="skipanywhilefolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skipanywhilefolder-pointable-type-init"></span>`type Init = T`

- <span id="skipanywhilefolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanywhilefolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanywhilefolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanywhilefolder-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `skip`

```rust
fn skip<T>(item: &T, skipping: &std::sync::atomic::AtomicBool, predicate: &impl Fn(&T) -> bool) -> bool
```

*Defined in [`rayon-1.11.0/src/iter/skip_any_while.rs:119-128`](../../../../.source_1765210505/rayon-1.11.0/src/iter/skip_any_while.rs#L119-L128)*

