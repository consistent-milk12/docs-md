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
  - [`string_len`](#string_len)
  - [`osstring_len`](#osstring_len)
  - [`fast_collect`](#fast_collect)
- [Macros](#macros)
  - [`extend!`](#extend)
  - [`extend_reserved!`](#extend_reserved)

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
| [`string_len`](#string_len) | fn | Computes the total string length of a `fast_collect` result. |
| [`osstring_len`](#osstring_len) | fn | Computes the total OS-string length of a `fast_collect` result. |
| [`fast_collect`](#fast_collect) | fn |  |
| [`extend!`](#extend) | macro | Performs a generic `par_extend` by collecting to a `LinkedList<Vec<_>>` in |
| [`extend_reserved!`](#extend_reserved) | macro |  |

## Structs

### `ListVecConsumer`

```rust
struct ListVecConsumer;
```

#### Trait Implementations

##### `impl<T: Send> Consumer for ListVecConsumer`

- <span id="listvecconsumer-folder"></span>`type Folder = ListVecFolder<T>`

- <span id="listvecconsumer-reducer"></span>`type Reducer = ListReducer`

- <span id="listvecconsumer-result"></span>`type Result = LinkedList<Vec<T>>`

- <span id="listvecconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="listvecconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="listvecconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for ListVecConsumer`

##### `impl<T> Pointable for ListVecConsumer`

- <span id="listvecconsumer-align"></span>`const ALIGN: usize`

- <span id="listvecconsumer-init"></span>`type Init = T`

- <span id="listvecconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="listvecconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="listvecconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="listvecconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> UnindexedConsumer for ListVecConsumer`

- <span id="listvecconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="listvecconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `ListVecFolder<T>`

```rust
struct ListVecFolder<T> {
    vec: Vec<T>,
}
```

#### Trait Implementations

##### `impl<T> Folder for ListVecFolder<T>`

- <span id="listvecfolder-result"></span>`type Result = LinkedList<Vec<T>>`

- <span id="listvecfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="listvecfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="listvecfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="listvecfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for ListVecFolder<T>`

##### `impl<T> Pointable for ListVecFolder<T>`

- <span id="listvecfolder-align"></span>`const ALIGN: usize`

- <span id="listvecfolder-init"></span>`type Init = T`

- <span id="listvecfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="listvecfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="listvecfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="listvecfolder-drop"></span>`unsafe fn drop(ptr: usize)`

### `ListConsumer`

```rust
struct ListConsumer;
```

#### Trait Implementations

##### `impl<T: Send> Consumer for ListConsumer`

- <span id="listconsumer-folder"></span>`type Folder = ListFolder<T>`

- <span id="listconsumer-reducer"></span>`type Reducer = ListReducer`

- <span id="listconsumer-result"></span>`type Result = LinkedList<T>`

- <span id="listconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="listconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="listconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for ListConsumer`

##### `impl<T> Pointable for ListConsumer`

- <span id="listconsumer-align"></span>`const ALIGN: usize`

- <span id="listconsumer-init"></span>`type Init = T`

- <span id="listconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="listconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="listconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="listconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> UnindexedConsumer for ListConsumer`

- <span id="listconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="listconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `ListFolder<T>`

```rust
struct ListFolder<T> {
    list: std::collections::LinkedList<T>,
}
```

#### Trait Implementations

##### `impl<T> Folder for ListFolder<T>`

- <span id="listfolder-result"></span>`type Result = LinkedList<T>`

- <span id="listfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="listfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="listfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="listfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for ListFolder<T>`

##### `impl<T> Pointable for ListFolder<T>`

- <span id="listfolder-align"></span>`const ALIGN: usize`

- <span id="listfolder-init"></span>`type Init = T`

- <span id="listfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="listfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="listfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="listfolder-drop"></span>`unsafe fn drop(ptr: usize)`

### `ListReducer`

```rust
struct ListReducer;
```

#### Trait Implementations

##### `impl<T> IntoEither for ListReducer`

##### `impl<T> Pointable for ListReducer`

- <span id="listreducer-align"></span>`const ALIGN: usize`

- <span id="listreducer-init"></span>`type Init = T`

- <span id="listreducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="listreducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="listreducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="listreducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for ListReducer`

- <span id="listreducer-reduce"></span>`fn reduce(self, left: LinkedList<T>, right: LinkedList<T>) -> LinkedList<T>`

### `ListStringConsumer`

```rust
struct ListStringConsumer;
```

#### Trait Implementations

##### `impl Consumer for ListStringConsumer`

- <span id="liststringconsumer-folder"></span>`type Folder = ListStringFolder`

- <span id="liststringconsumer-reducer"></span>`type Reducer = ListReducer`

- <span id="liststringconsumer-result"></span>`type Result = LinkedList<String>`

- <span id="liststringconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="liststringconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="liststringconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for ListStringConsumer`

##### `impl<T> Pointable for ListStringConsumer`

- <span id="liststringconsumer-align"></span>`const ALIGN: usize`

- <span id="liststringconsumer-init"></span>`type Init = T`

- <span id="liststringconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="liststringconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="liststringconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="liststringconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl UnindexedConsumer for ListStringConsumer`

- <span id="liststringconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="liststringconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `ListStringFolder`

```rust
struct ListStringFolder {
    string: String,
}
```

#### Trait Implementations

##### `impl Folder for ListStringFolder`

- <span id="liststringfolder-result"></span>`type Result = LinkedList<String>`

- <span id="liststringfolder-consume"></span>`fn consume(self, item: char) -> Self`

- <span id="liststringfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="liststringfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="liststringfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for ListStringFolder`

##### `impl<T> Pointable for ListStringFolder`

- <span id="liststringfolder-align"></span>`const ALIGN: usize`

- <span id="liststringfolder-init"></span>`type Init = T`

- <span id="liststringfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="liststringfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="liststringfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="liststringfolder-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `len`

```rust
fn len<T>(vecs: &either::Either<Vec<T>, std::collections::LinkedList<Vec<T>>>) -> usize
```

Computes the total length of a `fast_collect` result.

### `string_len`

```rust
fn string_len<T: AsRef<str>>(vecs: &either::Either<Vec<T>, std::collections::LinkedList<Vec<T>>>) -> usize
```

Computes the total string length of a `fast_collect` result.

### `osstring_len`

```rust
fn osstring_len<T: AsRef<std::ffi::OsStr>>(vecs: &either::Either<Vec<T>, std::collections::LinkedList<Vec<T>>>) -> usize
```

Computes the total OS-string length of a `fast_collect` result.

### `fast_collect`

```rust
fn fast_collect<I, T>(pi: I) -> either::Either<Vec<T>, std::collections::LinkedList<Vec<T>>>
where
    I: IntoParallelIterator<Item = T>,
    T: Send
```

## Macros

### `extend!`

Performs a generic `par_extend` by collecting to a `LinkedList<Vec<_>>` in
parallel, then extending the collection sequentially.

### `extend_reserved!`

