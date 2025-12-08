*[clap_builder](../../index.md) / [util](../index.md) / [graph](index.md)*

---

# Module `graph`

## Structs

### `Child<T>`

```rust
struct Child<T> {
    id: T,
    children: Vec<usize>,
}
```

#### Implementations

- `fn new(id: T) -> Self`

#### Trait Implementations

##### `impl<T: $crate::fmt::Debug> Debug for Child<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ChildGraph<T>`

```rust
struct ChildGraph<T>(Vec<Child<T>>);
```

#### Implementations

- `fn with_capacity(s: usize) -> Self`

- `fn insert(self: &mut Self, req: T) -> usize`

- `fn insert_child(self: &mut Self, parent: usize, child: T) -> usize`

- `fn iter(self: &Self) -> impl Iterator<Item = &T>`

- `fn contains(self: &Self, req: &T) -> bool`

#### Trait Implementations

##### `impl<T: $crate::fmt::Debug> Debug for ChildGraph<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

