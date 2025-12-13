*[rayon](../../index.md) / [iter](../index.md) / [extend](index.md)*

---

# Module `extend`

## Contents

- [Structs](#structs)
  - [`ListVecConsumer`](#listvecconsumer)
  - [`ListVecFolder`](#listvecfolder)
  - [`ListConsumer`](#listconsumer)
  - [`ListFolder`](#listfolder)
  - [`ListReducer`](#listreducer)
  - [`ListStringConsumer`](#liststringconsumer)
  - [`ListStringFolder`](#liststringfolder)
- [Functions](#functions)
  - [`len`](#len)
  - [`string_len`](#string-len)
  - [`osstring_len`](#osstring-len)
  - [`fast_collect`](#fast-collect)
- [Macros](#macros)
  - [`extend!`](#extend)
  - [`extend_reserved!`](#extend-reserved)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ListVecConsumer`](#listvecconsumer) | struct |  |
| [`ListVecFolder`](#listvecfolder) | struct |  |
| [`ListConsumer`](#listconsumer) | struct |  |
| [`ListFolder`](#listfolder) | struct |  |
| [`ListReducer`](#listreducer) | struct |  |
| [`ListStringConsumer`](#liststringconsumer) | struct |  |
| [`ListStringFolder`](#liststringfolder) | struct |  |
| [`len`](#len) | fn | Computes the total length of a `fast_collect` result. |
| [`string_len`](#string-len) | fn | Computes the total string length of a `fast_collect` result. |
| [`osstring_len`](#osstring-len) | fn | Computes the total OS-string length of a `fast_collect` result. |
| [`fast_collect`](#fast-collect) | fn |  |
| [`extend!`](#extend) | macro | Performs a generic `par_extend` by collecting to a `LinkedList<Vec<_>>` in parallel, then extending the collection sequentially. |
| [`extend_reserved!`](#extend-reserved) | macro |  |

## Structs

### `ListVecConsumer`

```rust
struct ListVecConsumer;
```

*Defined in [`rayon-1.11.0/src/iter/extend.rs:84`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L84)*

#### Trait Implementations

##### `impl Any for ListVecConsumer`

- <span id="listvecconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ListVecConsumer`

- <span id="listvecconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ListVecConsumer`

- <span id="listvecconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Send> Consumer for ListVecConsumer`

- <span id="listvecconsumer-consumer-type-folder"></span>`type Folder = ListVecFolder<T>`

- <span id="listvecconsumer-consumer-type-reducer"></span>`type Reducer = ListReducer`

- <span id="listvecconsumer-consumer-type-result"></span>`type Result = LinkedList<Vec<T>>`

- <span id="listvecconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="listvecconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="listvecconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ListVecConsumer`

- <span id="listvecconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ListVecConsumer`

- <span id="listvecconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ListVecConsumer`

##### `impl Pointable for ListVecConsumer`

- <span id="listvecconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="listvecconsumer-pointable-type-init"></span>`type Init = T`

- <span id="listvecconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="listvecconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="listvecconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="listvecconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ListVecConsumer`

- <span id="listvecconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="listvecconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ListVecConsumer`

- <span id="listvecconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="listvecconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: Send> UnindexedConsumer for ListVecConsumer`

- <span id="listvecconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="listvecconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `ListVecFolder<T>`

```rust
struct ListVecFolder<T> {
    vec: Vec<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/extend.rs:86-88`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L86-L88)*

#### Trait Implementations

##### `impl<T> Any for ListVecFolder<T>`

- <span id="listvecfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ListVecFolder<T>`

- <span id="listvecfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ListVecFolder<T>`

- <span id="listvecfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Folder for ListVecFolder<T>`

- <span id="listvecfolder-folder-type-result"></span>`type Result = LinkedList<Vec<T>>`

- <span id="listvecfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="listvecfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="listvecfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="listvecfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ListVecFolder<T>`

- <span id="listvecfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ListVecFolder<T>`

- <span id="listvecfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ListVecFolder<T>`

##### `impl<T> Pointable for ListVecFolder<T>`

- <span id="listvecfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="listvecfolder-pointable-type-init"></span>`type Init = T`

- <span id="listvecfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="listvecfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="listvecfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="listvecfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for ListVecFolder<T>`

- <span id="listvecfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="listvecfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ListVecFolder<T>`

- <span id="listvecfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="listvecfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ListConsumer`

```rust
struct ListConsumer;
```

*Defined in [`rayon-1.11.0/src/iter/extend.rs:313`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L313)*

#### Trait Implementations

##### `impl Any for ListConsumer`

- <span id="listconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ListConsumer`

- <span id="listconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ListConsumer`

- <span id="listconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Send> Consumer for ListConsumer`

- <span id="listconsumer-consumer-type-folder"></span>`type Folder = ListFolder<T>`

- <span id="listconsumer-consumer-type-reducer"></span>`type Reducer = ListReducer`

- <span id="listconsumer-consumer-type-result"></span>`type Result = LinkedList<T>`

- <span id="listconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="listconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="listconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ListConsumer`

- <span id="listconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ListConsumer`

- <span id="listconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ListConsumer`

##### `impl Pointable for ListConsumer`

- <span id="listconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="listconsumer-pointable-type-init"></span>`type Init = T`

- <span id="listconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="listconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="listconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="listconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ListConsumer`

- <span id="listconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="listconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ListConsumer`

- <span id="listconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="listconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: Send> UnindexedConsumer for ListConsumer`

- <span id="listconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="listconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `ListFolder<T>`

```rust
struct ListFolder<T> {
    list: std::collections::LinkedList<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/extend.rs:315-317`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L315-L317)*

#### Trait Implementations

##### `impl<T> Any for ListFolder<T>`

- <span id="listfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ListFolder<T>`

- <span id="listfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ListFolder<T>`

- <span id="listfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Folder for ListFolder<T>`

- <span id="listfolder-folder-type-result"></span>`type Result = LinkedList<T>`

- <span id="listfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="listfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="listfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="listfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ListFolder<T>`

- <span id="listfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ListFolder<T>`

- <span id="listfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ListFolder<T>`

##### `impl<T> Pointable for ListFolder<T>`

- <span id="listfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="listfolder-pointable-type-init"></span>`type Init = T`

- <span id="listfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="listfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="listfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="listfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for ListFolder<T>`

- <span id="listfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="listfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ListFolder<T>`

- <span id="listfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="listfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ListReducer`

```rust
struct ListReducer;
```

*Defined in [`rayon-1.11.0/src/iter/extend.rs:319`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L319)*

#### Trait Implementations

##### `impl Any for ListReducer`

- <span id="listreducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ListReducer`

- <span id="listreducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ListReducer`

- <span id="listreducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ListReducer`

- <span id="listreducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ListReducer`

- <span id="listreducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ListReducer`

##### `impl Pointable for ListReducer`

- <span id="listreducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="listreducer-pointable-type-init"></span>`type Init = T`

- <span id="listreducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="listreducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="listreducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="listreducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for ListReducer`

- <span id="listreducer-reducer-reduce"></span>`fn reduce(self, left: LinkedList<T>, right: LinkedList<T>) -> LinkedList<T>`

##### `impl<U> TryFrom for ListReducer`

- <span id="listreducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="listreducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ListReducer`

- <span id="listreducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="listreducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ListStringConsumer`

```rust
struct ListStringConsumer;
```

*Defined in [`rayon-1.11.0/src/iter/extend.rs:437`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L437)*

#### Trait Implementations

##### `impl Any for ListStringConsumer`

- <span id="liststringconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ListStringConsumer`

- <span id="liststringconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ListStringConsumer`

- <span id="liststringconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Consumer for ListStringConsumer`

- <span id="liststringconsumer-consumer-type-folder"></span>`type Folder = ListStringFolder`

- <span id="liststringconsumer-consumer-type-reducer"></span>`type Reducer = ListReducer`

- <span id="liststringconsumer-consumer-type-result"></span>`type Result = LinkedList<String>`

- <span id="liststringconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="liststringconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="liststringconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ListStringConsumer`

- <span id="liststringconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ListStringConsumer`

- <span id="liststringconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ListStringConsumer`

##### `impl Pointable for ListStringConsumer`

- <span id="liststringconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="liststringconsumer-pointable-type-init"></span>`type Init = T`

- <span id="liststringconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="liststringconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="liststringconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="liststringconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ListStringConsumer`

- <span id="liststringconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="liststringconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ListStringConsumer`

- <span id="liststringconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="liststringconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UnindexedConsumer for ListStringConsumer`

- <span id="liststringconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="liststringconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `ListStringFolder`

```rust
struct ListStringFolder {
    string: String,
}
```

*Defined in [`rayon-1.11.0/src/iter/extend.rs:439-441`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L439-L441)*

#### Trait Implementations

##### `impl Any for ListStringFolder`

- <span id="liststringfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ListStringFolder`

- <span id="liststringfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ListStringFolder`

- <span id="liststringfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Folder for ListStringFolder`

- <span id="liststringfolder-folder-type-result"></span>`type Result = LinkedList<String>`

- <span id="liststringfolder-folder-consume"></span>`fn consume(self, item: char) -> Self`

- <span id="liststringfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="liststringfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="liststringfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ListStringFolder`

- <span id="liststringfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ListStringFolder`

- <span id="liststringfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ListStringFolder`

##### `impl Pointable for ListStringFolder`

- <span id="liststringfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="liststringfolder-pointable-type-init"></span>`type Init = T`

- <span id="liststringfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="liststringfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="liststringfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="liststringfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ListStringFolder`

- <span id="liststringfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="liststringfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ListStringFolder`

- <span id="liststringfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="liststringfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `len`

```rust
fn len<T>(vecs: &either::Either<Vec<T>, std::collections::LinkedList<Vec<T>>>) -> usize
```

*Defined in [`rayon-1.11.0/src/iter/extend.rs:42-47`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L42-L47)*

Computes the total length of a `fast_collect` result.

### `string_len`

```rust
fn string_len<T: AsRef<str>>(vecs: &either::Either<Vec<T>, std::collections::LinkedList<Vec<T>>>) -> usize
```

*Defined in [`rayon-1.11.0/src/iter/extend.rs:50-56`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L50-L56)*

Computes the total string length of a `fast_collect` result.

### `osstring_len`

```rust
fn osstring_len<T: AsRef<std::ffi::OsStr>>(vecs: &either::Either<Vec<T>, std::collections::LinkedList<Vec<T>>>) -> usize
```

*Defined in [`rayon-1.11.0/src/iter/extend.rs:59-65`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L59-L65)*

Computes the total OS-string length of a `fast_collect` result.

### `fast_collect`

```rust
fn fast_collect<I, T>(pi: I) -> either::Either<Vec<T>, std::collections::LinkedList<Vec<T>>>
where
    I: IntoParallelIterator<Item = T>,
    T: Send
```

*Defined in [`rayon-1.11.0/src/iter/extend.rs:67-82`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L67-L82)*

## Macros

### `extend!`

*Defined in [`rayon-1.11.0/src/iter/extend.rs:15-29`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L15-L29)*

Performs a generic `par_extend` by collecting to a `LinkedList<Vec<_>>` in
parallel, then extending the collection sequentially.

### `extend_reserved!`

*Defined in [`rayon-1.11.0/src/iter/extend.rs:30-39`](../../../../.source_1765521767/rayon-1.11.0/src/iter/extend.rs#L30-L39)*

