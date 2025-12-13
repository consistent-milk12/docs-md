*[clap_builder](../../index.md) / [util](../index.md) / [graph](index.md)*

---

# Module `graph`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Child`](#child) | struct |  |
| [`ChildGraph`](#childgraph) | struct |  |

## Structs

### `Child<T>`

```rust
struct Child<T> {
    id: T,
    children: Vec<usize>,
}
```

*Defined in [`clap_builder-4.5.53/src/util/graph.rs:2-5`](../../../../.source_1765633015/clap_builder-4.5.53/src/util/graph.rs#L2-L5)*

#### Implementations

- <span id="child-new"></span>`fn new(id: T) -> Self`

#### Trait Implementations

##### `impl<T> Any for Child<T>`

- <span id="child-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Child<T>`

- <span id="child-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Child<T>`

- <span id="child-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for Child<T>`

- <span id="child-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Child<T>`

- <span id="child-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Child<T>`

- <span id="child-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for Child<T>`

- <span id="child-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="child-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Child<T>`

- <span id="child-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="child-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChildGraph<T>`

```rust
struct ChildGraph<T>(Vec<Child<T>>);
```

*Defined in [`clap_builder-4.5.53/src/util/graph.rs:17`](../../../../.source_1765633015/clap_builder-4.5.53/src/util/graph.rs#L17)*

#### Implementations

- <span id="childgraph-with-capacity"></span>`fn with_capacity(s: usize) -> Self`

- <span id="childgraph-insert"></span>`fn insert(&mut self, req: T) -> usize`

- <span id="childgraph-insert-child"></span>`fn insert_child(&mut self, parent: usize, child: T) -> usize`

- <span id="childgraph-iter"></span>`fn iter(&self) -> impl Iterator<Item = &T>`

- <span id="childgraph-contains"></span>`fn contains(&self, req: &T) -> bool`

#### Trait Implementations

##### `impl<T> Any for ChildGraph<T>`

- <span id="childgraph-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChildGraph<T>`

- <span id="childgraph-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChildGraph<T>`

- <span id="childgraph-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for ChildGraph<T>`

- <span id="childgraph-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ChildGraph<T>`

- <span id="childgraph-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ChildGraph<T>`

- <span id="childgraph-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for ChildGraph<T>`

- <span id="childgraph-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="childgraph-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChildGraph<T>`

- <span id="childgraph-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="childgraph-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

