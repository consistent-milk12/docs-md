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

#### Implementations

- <span id="child-new"></span>`fn new(id: T) -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for Child<T>`

- <span id="child-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ChildGraph<T>`

```rust
struct ChildGraph<T>(Vec<Child<T>>);
```

#### Implementations

- <span id="childgraph-with-capacity"></span>`fn with_capacity(s: usize) -> Self`

- <span id="childgraph-insert"></span>`fn insert(&mut self, req: T) -> usize`

- <span id="childgraph-insert-child"></span>`fn insert_child(&mut self, parent: usize, child: T) -> usize`

- <span id="childgraph-iter"></span>`fn iter(&self) -> impl Iterator<Item = &T>`

- <span id="childgraph-contains"></span>`fn contains(&self, req: &T) -> bool`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for ChildGraph<T>`

- <span id="childgraph-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

