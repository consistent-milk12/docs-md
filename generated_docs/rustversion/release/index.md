*[rustversion](../index.md) / [release](index.md)*

---

# Module `release`

## Structs

### `Release`

```rust
struct Release {
    pub minor: u16,
    pub patch: Option<u16>,
}
```

#### Trait Implementations

##### `impl Clone for Release`

- `fn clone(self: &Self) -> Release` — [`Release`](#release)

##### `impl Copy for Release`

##### `impl Debug for Release`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Release`

##### `impl Ord for Release`

- `fn cmp(self: &Self, other: &Release) -> $crate::cmp::Ordering` — [`Release`](#release)

##### `impl PartialEq for Release`

- `fn eq(self: &Self, other: &Release) -> bool` — [`Release`](#release)

##### `impl PartialOrd for Release`

- `fn partial_cmp(self: &Self, other: &Release) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Release`](#release)

##### `impl StructuralPartialEq for Release`

## Functions

### `parse`

```rust
fn parse(paren: proc_macro::Group, iter: &'_ mut IterImpl) -> std::result::Result<Release, Error>
```

### `try_parse`

```rust
fn try_parse(iter: &'_ mut IterImpl) -> std::result::Result<Release, ()>
```

