*[rayon](../../index.md) / [iter](../index.md) / [extend](index.md)*

---

# Module `extend`

## Structs

### `ListVecConsumer`

```rust
struct ListVecConsumer;
```

#### Trait Implementations

##### `impl<T: Send> Consumer for ListVecConsumer`

- `type Folder = ListVecFolder<T>`

- `type Reducer = ListReducer`

- `type Result = LinkedList<Vec<T>>`

- `fn split_at(self: Self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ListVecConsumer`

##### `impl<T> Pointable for ListVecConsumer`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: Send> UnindexedConsumer for ListVecConsumer`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `ListVecFolder<T>`

```rust
struct ListVecFolder<T> {
    vec: Vec<T>,
}
```

#### Trait Implementations

##### `impl<T> Folder for ListVecFolder<T>`

- `type Result = LinkedList<Vec<T>>`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ListVecFolder<T>`

##### `impl<T> Pointable for ListVecFolder<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ListConsumer`

```rust
struct ListConsumer;
```

#### Trait Implementations

##### `impl<T: Send> Consumer for ListConsumer`

- `type Folder = ListFolder<T>`

- `type Reducer = ListReducer`

- `type Result = LinkedList<T>`

- `fn split_at(self: Self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ListConsumer`

##### `impl<T> Pointable for ListConsumer`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: Send> UnindexedConsumer for ListConsumer`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `ListFolder<T>`

```rust
struct ListFolder<T> {
    list: std::collections::LinkedList<T>,
}
```

#### Trait Implementations

##### `impl<T> Folder for ListFolder<T>`

- `type Result = LinkedList<T>`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ListFolder<T>`

##### `impl<T> Pointable for ListFolder<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ListReducer`

```rust
struct ListReducer;
```

#### Trait Implementations

##### `impl<T> IntoEither for ListReducer`

##### `impl<T> Pointable for ListReducer`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for ListReducer`

- `fn reduce(self: Self, left: LinkedList<T>, right: LinkedList<T>) -> LinkedList<T>`

### `ListStringConsumer`

```rust
struct ListStringConsumer;
```

#### Trait Implementations

##### `impl Consumer for ListStringConsumer`

- `type Folder = ListStringFolder`

- `type Reducer = ListReducer`

- `type Result = LinkedList<String>`

- `fn split_at(self: Self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ListStringConsumer`

##### `impl<T> Pointable for ListStringConsumer`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl UnindexedConsumer for ListStringConsumer`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `ListStringFolder`

```rust
struct ListStringFolder {
    string: String,
}
```

#### Trait Implementations

##### `impl Folder for ListStringFolder`

- `type Result = LinkedList<String>`

- `fn consume(self: Self, item: char) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ListStringFolder`

##### `impl<T> Pointable for ListStringFolder`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

