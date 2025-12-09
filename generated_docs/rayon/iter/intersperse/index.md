*[rayon](../../index.md) / [iter](../index.md) / [intersperse](index.md)*

---

# Module `intersperse`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Intersperse`](#intersperse) | struct | `Intersperse` is an iterator that inserts a particular item between each item of the adapted iterator. |
| [`IntersperseProducer`](#intersperseproducer) | struct |  |
| [`IntersperseIter`](#intersperseiter) | struct |  |
| [`IntersperseConsumer`](#intersperseconsumer) | struct |  |
| [`IntersperseFolder`](#interspersefolder) | struct |  |

## Structs

### `Intersperse<I>`

```rust
struct Intersperse<I>
where
    I: ParallelIterator {
    base: I,
    item: <I as >::Item,
}
```

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:13-19`](../../../../.source_1765210505/rayon-1.11.0/src/iter/intersperse.rs#L13-L19)*

`Intersperse` is an iterator that inserts a particular item between each
item of the adapted iterator.  This struct is created by the
`intersperse()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="intersperse-new"></span>`fn new(base: I, item: <I as >::Item) -> Self` — [`ParallelIterator`](../index.md)

#### Trait Implementations

##### `impl<I> Clone for Intersperse<I>`

- <span id="intersperse-clone"></span>`fn clone(&self) -> Intersperse<I>` — [`Intersperse`](#intersperse)

##### `impl<I> Debug for Intersperse<I>`

- <span id="intersperse-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Intersperse<I>`

- <span id="intersperse-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="intersperse-len"></span>`fn len(&self) -> usize`

- <span id="intersperse-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Intersperse<I>`

##### `impl<T> IntoParallelIterator for Intersperse<I>`

- <span id="intersperse-type-iter"></span>`type Iter = T`

- <span id="intersperse-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intersperse-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Intersperse<I>`

- <span id="intersperse-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="intersperse-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="intersperse-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Intersperse<I>`

- <span id="intersperse-const-align"></span>`const ALIGN: usize`

- <span id="intersperse-type-init"></span>`type Init = T`

- <span id="intersperse-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intersperse-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intersperse-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intersperse-drop"></span>`unsafe fn drop(ptr: usize)`

### `IntersperseProducer<P>`

```rust
struct IntersperseProducer<P>
where
    P: Producer {
    base: P,
    item: <P as >::Item,
    len: usize,
    clone_first: bool,
}
```

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:109-117`](../../../../.source_1765210505/rayon-1.11.0/src/iter/intersperse.rs#L109-L117)*

#### Implementations

- <span id="intersperseproducer-new"></span>`fn new(base: P, item: <P as >::Item, len: usize) -> Self` — [`Producer`](../plumbing/index.md)

#### Trait Implementations

##### `impl<T> IntoEither for IntersperseProducer<P>`

##### `impl<T> Pointable for IntersperseProducer<P>`

- <span id="intersperseproducer-const-align"></span>`const ALIGN: usize`

- <span id="intersperseproducer-type-init"></span>`type Init = T`

- <span id="intersperseproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intersperseproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intersperseproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intersperseproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for IntersperseProducer<P>`

- <span id="intersperseproducer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="intersperseproducer-type-intoiter"></span>`type IntoIter = IntersperseIter<<P as Producer>::IntoIter>`

- <span id="intersperseproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="intersperseproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="intersperseproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="intersperseproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="intersperseproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `IntersperseIter<I>`

```rust
struct IntersperseIter<I>
where
    I: Iterator {
    base: std::iter::Fuse<I>,
    item: <I as >::Item,
    clone_first: bool,
    clone_last: bool,
}
```

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:201-209`](../../../../.source_1765210505/rayon-1.11.0/src/iter/intersperse.rs#L201-L209)*

#### Trait Implementations

##### `impl<I> DoubleEndedIterator for IntersperseIter<I>`

- <span id="intersperseiter-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I> ExactSizeIterator for IntersperseIter<I>`

- <span id="intersperseiter-len"></span>`fn len(&self) -> usize`

##### `impl<T> IntoEither for IntersperseIter<I>`

##### `impl<I> IntoIterator for IntersperseIter<I>`

- <span id="intersperseiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intersperseiter-type-intoiter"></span>`type IntoIter = I`

- <span id="intersperseiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for IntersperseIter<I>`

- <span id="intersperseiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intersperseiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intersperseiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for IntersperseIter<I>`

- <span id="intersperseiter-const-align"></span>`const ALIGN: usize`

- <span id="intersperseiter-type-init"></span>`type Init = T`

- <span id="intersperseiter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intersperseiter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intersperseiter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intersperseiter-drop"></span>`unsafe fn drop(ptr: usize)`

### `IntersperseConsumer<C, T>`

```rust
struct IntersperseConsumer<C, T> {
    base: C,
    item: T,
    clone_first: std::cell::Cell<bool>,
}
```

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:270-274`](../../../../.source_1765210505/rayon-1.11.0/src/iter/intersperse.rs#L270-L274)*

#### Implementations

- <span id="intersperseconsumer-new"></span>`fn new(base: C, item: T) -> Self`

#### Trait Implementations

##### `impl<C, T> Consumer for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-type-folder"></span>`type Folder = IntersperseFolder<<C as Consumer>::Folder, T>`

- <span id="intersperseconsumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="intersperseconsumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="intersperseconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="intersperseconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="intersperseconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for IntersperseConsumer<C, T>`

##### `impl<T> Pointable for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-const-align"></span>`const ALIGN: usize`

- <span id="intersperseconsumer-type-init"></span>`type Init = T`

- <span id="intersperseconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intersperseconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intersperseconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intersperseconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<C, T> UnindexedConsumer for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="intersperseconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `IntersperseFolder<C, T>`

```rust
struct IntersperseFolder<C, T> {
    base: C,
    item: T,
    clone_first: bool,
}
```

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:346-350`](../../../../.source_1765210505/rayon-1.11.0/src/iter/intersperse.rs#L346-L350)*

#### Trait Implementations

##### `impl<C, T> Folder for IntersperseFolder<C, T>`

- <span id="interspersefolder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="interspersefolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="interspersefolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="interspersefolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="interspersefolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for IntersperseFolder<C, T>`

##### `impl<T> Pointable for IntersperseFolder<C, T>`

- <span id="interspersefolder-const-align"></span>`const ALIGN: usize`

- <span id="interspersefolder-type-init"></span>`type Init = T`

- <span id="interspersefolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interspersefolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interspersefolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interspersefolder-drop"></span>`unsafe fn drop(ptr: usize)`

