*[rayon](../../index.md) / [iter](../index.md) / [sum](index.md)*

---

# Module `sum`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SumConsumer`](#sumconsumer) | struct |  |
| [`SumFolder`](#sumfolder) | struct |  |
| [`sum`](#sum) | fn |  |
| [`add`](#add) | fn |  |

## Structs

### `SumConsumer<S: Send>`

```rust
struct SumConsumer<S: Send> {
    _marker: std::marker::PhantomData<*const S>,
}
```

*Defined in [`rayon-1.11.0/src/iter/sum.rs:19-21`](../../../../.source_1765521767/rayon-1.11.0/src/iter/sum.rs#L19-L21)*

#### Implementations

- <span id="sumconsumer-new"></span>`fn new() -> SumConsumer<S>` — [`SumConsumer`](#sumconsumer)

#### Trait Implementations

##### `impl Any for SumConsumer<S>`

- <span id="sumconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SumConsumer<S>`

- <span id="sumconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SumConsumer<S>`

- <span id="sumconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S, T> Consumer for SumConsumer<S>`

- <span id="sumconsumer-consumer-type-folder"></span>`type Folder = SumFolder<S>`

- <span id="sumconsumer-consumer-type-reducer"></span>`type Reducer = SumConsumer<S>`

- <span id="sumconsumer-consumer-type-result"></span>`type Result = S`

- <span id="sumconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, Self)`

- <span id="sumconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="sumconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for SumConsumer<S>`

- <span id="sumconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SumConsumer<S>`

- <span id="sumconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SumConsumer<S>`

##### `impl Pointable for SumConsumer<S>`

- <span id="sumconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sumconsumer-pointable-type-init"></span>`type Init = T`

- <span id="sumconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sumconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sumconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sumconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<S> Reducer for SumConsumer<S>`

- <span id="sumconsumer-reducer-reduce"></span>`fn reduce(self, left: S, right: S) -> S`

##### `impl<S: Send> Send for SumConsumer<S>`

##### `impl<U> TryFrom for SumConsumer<S>`

- <span id="sumconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sumconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SumConsumer<S>`

- <span id="sumconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sumconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<S, T> UnindexedConsumer for SumConsumer<S>`

- <span id="sumconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="sumconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `SumFolder<S>`

```rust
struct SumFolder<S> {
    sum: S,
}
```

*Defined in [`rayon-1.11.0/src/iter/sum.rs:78-80`](../../../../.source_1765521767/rayon-1.11.0/src/iter/sum.rs#L78-L80)*

#### Trait Implementations

##### `impl Any for SumFolder<S>`

- <span id="sumfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SumFolder<S>`

- <span id="sumfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SumFolder<S>`

- <span id="sumfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S, T> Folder for SumFolder<S>`

- <span id="sumfolder-folder-type-result"></span>`type Result = S`

- <span id="sumfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="sumfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="sumfolder-folder-complete"></span>`fn complete(self) -> S`

- <span id="sumfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for SumFolder<S>`

- <span id="sumfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SumFolder<S>`

- <span id="sumfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SumFolder<S>`

##### `impl Pointable for SumFolder<S>`

- <span id="sumfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sumfolder-pointable-type-init"></span>`type Init = T`

- <span id="sumfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sumfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sumfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sumfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SumFolder<S>`

- <span id="sumfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sumfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SumFolder<S>`

- <span id="sumfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sumfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `sum`

```rust
fn sum<PI, S>(pi: PI) -> S
where
    PI: ParallelIterator,
    S: Send + Sum<<PI as >::Item> + Sum
```

*Defined in [`rayon-1.11.0/src/iter/sum.rs:7-13`](../../../../.source_1765521767/rayon-1.11.0/src/iter/sum.rs#L7-L13)*

### `add`

```rust
fn add<T: Sum>(left: T, right: T) -> T
```

*Defined in [`rayon-1.11.0/src/iter/sum.rs:15-17`](../../../../.source_1765521767/rayon-1.11.0/src/iter/sum.rs#L15-L17)*

