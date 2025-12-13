*[rayon](../../index.md) / [iter](../index.md) / [find_first_last](index.md)*

---

# Module `find_first_last`

## Contents

- [Structs](#structs)
  - [`FindConsumer`](#findconsumer)
  - [`FindFolder`](#findfolder)
  - [`FindReducer`](#findreducer)
- [Enums](#enums)
  - [`MatchPosition`](#matchposition)
- [Functions](#functions)
  - [`better_position`](#better-position)
  - [`find_first`](#find-first)
  - [`find_last`](#find-last)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FindConsumer`](#findconsumer) | struct |  |
| [`FindFolder`](#findfolder) | struct |  |
| [`FindReducer`](#findreducer) | struct |  |
| [`MatchPosition`](#matchposition) | enum |  |
| [`better_position`](#better-position) | fn | Returns true if pos1 is a better match than pos2 according to MatchPosition |
| [`find_first`](#find-first) | fn |  |
| [`find_last`](#find-last) | fn |  |

## Structs

### `FindConsumer<'p, P>`

```rust
struct FindConsumer<'p, P> {
    find_op: &'p P,
    lower_bound: std::cell::Cell<usize>,
    upper_bound: usize,
    match_position: MatchPosition,
    best_found: &'p std::sync::atomic::AtomicUsize,
}
```

*Defined in [`rayon-1.11.0/src/iter/find_first_last/mod.rs:61-67`](../../../../.source_1765633015/rayon-1.11.0/src/iter/find_first_last/mod.rs#L61-L67)*

#### Implementations

- <span id="findconsumer-new"></span>`fn new(find_op: &'p P, match_position: MatchPosition, best_found: &'p AtomicUsize) -> Self` — [`MatchPosition`](#matchposition)

- <span id="findconsumer-current-index"></span>`fn current_index(&self) -> usize`

#### Trait Implementations

##### `impl Any for FindConsumer<'p, P>`

- <span id="findconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FindConsumer<'p, P>`

- <span id="findconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FindConsumer<'p, P>`

- <span id="findconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, P> Consumer for FindConsumer<'p, P>`

- <span id="findconsumer-consumer-type-folder"></span>`type Folder = FindFolder<'p, T, P>`

- <span id="findconsumer-consumer-type-reducer"></span>`type Reducer = FindReducer`

- <span id="findconsumer-consumer-type-result"></span>`type Result = Option<T>`

- <span id="findconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="findconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="findconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FindConsumer<'p, P>`

- <span id="findconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FindConsumer<'p, P>`

- <span id="findconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FindConsumer<'p, P>`

##### `impl Pointable for FindConsumer<'p, P>`

- <span id="findconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="findconsumer-pointable-type-init"></span>`type Init = T`

- <span id="findconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FindConsumer<'p, P>`

- <span id="findconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="findconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FindConsumer<'p, P>`

- <span id="findconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="findconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, P> UnindexedConsumer for FindConsumer<'p, P>`

- <span id="findconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="findconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FindFolder<'p, T, P>`

```rust
struct FindFolder<'p, T, P> {
    find_op: &'p P,
    boundary: usize,
    match_position: MatchPosition,
    best_found: &'p std::sync::atomic::AtomicUsize,
    item: Option<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/find_first_last/mod.rs:166-172`](../../../../.source_1765633015/rayon-1.11.0/src/iter/find_first_last/mod.rs#L166-L172)*

#### Trait Implementations

##### `impl<T> Any for FindFolder<'p, T, P>`

- <span id="findfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FindFolder<'p, T, P>`

- <span id="findfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FindFolder<'p, T, P>`

- <span id="findfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P: 'p + Fn(&T) -> bool, T> Folder for FindFolder<'p, T, P>`

- <span id="findfolder-folder-type-result"></span>`type Result = Option<T>`

- <span id="findfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="findfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="findfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FindFolder<'p, T, P>`

- <span id="findfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for FindFolder<'p, T, P>`

- <span id="findfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for FindFolder<'p, T, P>`

##### `impl<T> Pointable for FindFolder<'p, T, P>`

- <span id="findfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="findfolder-pointable-type-init"></span>`type Init = T`

- <span id="findfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for FindFolder<'p, T, P>`

- <span id="findfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="findfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for FindFolder<'p, T, P>`

- <span id="findfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="findfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FindReducer`

```rust
struct FindReducer {
    match_position: MatchPosition,
}
```

*Defined in [`rayon-1.11.0/src/iter/find_first_last/mod.rs:219-221`](../../../../.source_1765633015/rayon-1.11.0/src/iter/find_first_last/mod.rs#L219-L221)*

#### Trait Implementations

##### `impl Any for FindReducer`

- <span id="findreducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FindReducer`

- <span id="findreducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FindReducer`

- <span id="findreducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for FindReducer`

- <span id="findreducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FindReducer`

- <span id="findreducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FindReducer`

##### `impl Pointable for FindReducer`

- <span id="findreducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="findreducer-pointable-type-init"></span>`type Init = T`

- <span id="findreducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findreducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findreducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findreducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for FindReducer`

- <span id="findreducer-reducer-reduce"></span>`fn reduce(self, left: Option<T>, right: Option<T>) -> Option<T>`

##### `impl<U> TryFrom for FindReducer`

- <span id="findreducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="findreducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FindReducer`

- <span id="findreducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="findreducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `MatchPosition`

```rust
enum MatchPosition {
    Leftmost,
    Rightmost,
}
```

*Defined in [`rayon-1.11.0/src/iter/find_first_last/mod.rs:27-30`](../../../../.source_1765633015/rayon-1.11.0/src/iter/find_first_last/mod.rs#L27-L30)*

#### Trait Implementations

##### `impl Any for MatchPosition`

- <span id="matchposition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchPosition`

- <span id="matchposition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchPosition`

- <span id="matchposition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MatchPosition`

- <span id="matchposition-clone"></span>`fn clone(&self) -> MatchPosition` — [`MatchPosition`](#matchposition)

##### `impl CloneToUninit for MatchPosition`

- <span id="matchposition-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MatchPosition`

##### `impl<T> From for MatchPosition`

- <span id="matchposition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchPosition`

- <span id="matchposition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MatchPosition`

##### `impl Pointable for MatchPosition`

- <span id="matchposition-pointable-const-align"></span>`const ALIGN: usize`

- <span id="matchposition-pointable-type-init"></span>`type Init = T`

- <span id="matchposition-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="matchposition-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="matchposition-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="matchposition-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for MatchPosition`

- <span id="matchposition-toowned-type-owned"></span>`type Owned = T`

- <span id="matchposition-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="matchposition-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MatchPosition`

- <span id="matchposition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matchposition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchPosition`

- <span id="matchposition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matchposition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `better_position`

```rust
fn better_position(pos1: usize, pos2: usize, mp: MatchPosition) -> bool
```

*Defined in [`rayon-1.11.0/src/iter/find_first_last/mod.rs:34-39`](../../../../.source_1765633015/rayon-1.11.0/src/iter/find_first_last/mod.rs#L34-L39)*

Returns true if pos1 is a better match than pos2 according to MatchPosition

### `find_first`

```rust
fn find_first<I, P>(pi: I, find_op: P) -> Option<<I as >::Item>
where
    I: ParallelIterator,
    P: Fn(&<I as >::Item) -> bool + Sync
```

*Defined in [`rayon-1.11.0/src/iter/find_first_last/mod.rs:41-49`](../../../../.source_1765633015/rayon-1.11.0/src/iter/find_first_last/mod.rs#L41-L49)*

### `find_last`

```rust
fn find_last<I, P>(pi: I, find_op: P) -> Option<<I as >::Item>
where
    I: ParallelIterator,
    P: Fn(&<I as >::Item) -> bool + Sync
```

*Defined in [`rayon-1.11.0/src/iter/find_first_last/mod.rs:51-59`](../../../../.source_1765633015/rayon-1.11.0/src/iter/find_first_last/mod.rs#L51-L59)*

