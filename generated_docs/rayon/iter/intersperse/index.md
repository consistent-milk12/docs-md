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

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:13-19`](../../../../.source_1765521767/rayon-1.11.0/src/iter/intersperse.rs#L13-L19)*

`Intersperse` is an iterator that inserts a particular item between each
item of the adapted iterator.  This struct is created by the
`intersperse()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="intersperse-new"></span>`fn new(base: I, item: <I as >::Item) -> Self` — [`ParallelIterator`](../index.md#paralleliterator)

  Creates a new `Intersperse` iterator

#### Trait Implementations

##### `impl Any for Intersperse<I>`

- <span id="intersperse-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Intersperse<I>`

- <span id="intersperse-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Intersperse<I>`

- <span id="intersperse-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Intersperse<I>`

- <span id="intersperse-clone"></span>`fn clone(&self) -> Intersperse<I>` — [`Intersperse`](#intersperse)

##### `impl CloneToUninit for Intersperse<I>`

- <span id="intersperse-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for Intersperse<I>`

- <span id="intersperse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Intersperse<I>`

- <span id="intersperse-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Intersperse<I>`

- <span id="intersperse-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="intersperse-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="intersperse-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Intersperse<I>`

- <span id="intersperse-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Intersperse<I>`

##### `impl IntoParallelIterator for Intersperse<I>`

- <span id="intersperse-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="intersperse-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intersperse-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Intersperse<I>`

- <span id="intersperse-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="intersperse-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="intersperse-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Intersperse<I>`

- <span id="intersperse-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intersperse-pointable-type-init"></span>`type Init = T`

- <span id="intersperse-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intersperse-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intersperse-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intersperse-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Intersperse<I>`

- <span id="intersperse-toowned-type-owned"></span>`type Owned = T`

- <span id="intersperse-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intersperse-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Intersperse<I>`

- <span id="intersperse-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intersperse-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Intersperse<I>`

- <span id="intersperse-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intersperse-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:109-117`](../../../../.source_1765521767/rayon-1.11.0/src/iter/intersperse.rs#L109-L117)*

#### Implementations

- <span id="intersperseproducer-new"></span>`fn new(base: P, item: <P as >::Item, len: usize) -> Self` — [`Producer`](../plumbing/index.md#producer)

#### Trait Implementations

##### `impl Any for IntersperseProducer<P>`

- <span id="intersperseproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntersperseProducer<P>`

- <span id="intersperseproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntersperseProducer<P>`

- <span id="intersperseproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IntersperseProducer<P>`

- <span id="intersperseproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IntersperseProducer<P>`

- <span id="intersperseproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IntersperseProducer<P>`

##### `impl Pointable for IntersperseProducer<P>`

- <span id="intersperseproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intersperseproducer-pointable-type-init"></span>`type Init = T`

- <span id="intersperseproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intersperseproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intersperseproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intersperseproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for IntersperseProducer<P>`

- <span id="intersperseproducer-producer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="intersperseproducer-producer-type-intoiter"></span>`type IntoIter = IntersperseIter<<P as Producer>::IntoIter>`

- <span id="intersperseproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="intersperseproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="intersperseproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="intersperseproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="intersperseproducer-producer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

##### `impl<U> TryFrom for IntersperseProducer<P>`

- <span id="intersperseproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intersperseproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntersperseProducer<P>`

- <span id="intersperseproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intersperseproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:201-209`](../../../../.source_1765521767/rayon-1.11.0/src/iter/intersperse.rs#L201-L209)*

#### Trait Implementations

##### `impl Any for IntersperseIter<I>`

- <span id="intersperseiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntersperseIter<I>`

- <span id="intersperseiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntersperseIter<I>`

- <span id="intersperseiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> DoubleEndedIterator for IntersperseIter<I>`

- <span id="intersperseiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I> ExactSizeIterator for IntersperseIter<I>`

- <span id="intersperseiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IntersperseIter<I>`

- <span id="intersperseiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IntersperseIter<I>`

- <span id="intersperseiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IntersperseIter<I>`

##### `impl<I> IntoIterator for IntersperseIter<I>`

- <span id="intersperseiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intersperseiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intersperseiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for IntersperseIter<I>`

- <span id="intersperseiter-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intersperseiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intersperseiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Pointable for IntersperseIter<I>`

- <span id="intersperseiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intersperseiter-pointable-type-init"></span>`type Init = T`

- <span id="intersperseiter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intersperseiter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intersperseiter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intersperseiter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for IntersperseIter<I>`

- <span id="intersperseiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intersperseiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntersperseIter<I>`

- <span id="intersperseiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intersperseiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntersperseConsumer<C, T>`

```rust
struct IntersperseConsumer<C, T> {
    base: C,
    item: T,
    clone_first: std::cell::Cell<bool>,
}
```

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:270-274`](../../../../.source_1765521767/rayon-1.11.0/src/iter/intersperse.rs#L270-L274)*

#### Implementations

- <span id="intersperseconsumer-new"></span>`fn new(base: C, item: T) -> Self`

#### Trait Implementations

##### `impl<T> Any for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<C, T> Consumer for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-consumer-type-folder"></span>`type Folder = IntersperseFolder<<C as Consumer>::Folder, T>`

- <span id="intersperseconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="intersperseconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="intersperseconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="intersperseconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="intersperseconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for IntersperseConsumer<C, T>`

##### `impl<T> Pointable for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intersperseconsumer-pointable-type-init"></span>`type Init = T`

- <span id="intersperseconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intersperseconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intersperseconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intersperseconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intersperseconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intersperseconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<C, T> UnindexedConsumer for IntersperseConsumer<C, T>`

- <span id="intersperseconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="intersperseconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `IntersperseFolder<C, T>`

```rust
struct IntersperseFolder<C, T> {
    base: C,
    item: T,
    clone_first: bool,
}
```

*Defined in [`rayon-1.11.0/src/iter/intersperse.rs:346-350`](../../../../.source_1765521767/rayon-1.11.0/src/iter/intersperse.rs#L346-L350)*

#### Trait Implementations

##### `impl<T> Any for IntersperseFolder<C, T>`

- <span id="interspersefolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntersperseFolder<C, T>`

- <span id="interspersefolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntersperseFolder<C, T>`

- <span id="interspersefolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<C, T> Folder for IntersperseFolder<C, T>`

- <span id="interspersefolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="interspersefolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="interspersefolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="interspersefolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="interspersefolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for IntersperseFolder<C, T>`

- <span id="interspersefolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for IntersperseFolder<C, T>`

- <span id="interspersefolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for IntersperseFolder<C, T>`

##### `impl<T> Pointable for IntersperseFolder<C, T>`

- <span id="interspersefolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="interspersefolder-pointable-type-init"></span>`type Init = T`

- <span id="interspersefolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interspersefolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interspersefolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interspersefolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for IntersperseFolder<C, T>`

- <span id="interspersefolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interspersefolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IntersperseFolder<C, T>`

- <span id="interspersefolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interspersefolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

