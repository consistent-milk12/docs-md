*[rustversion](../index.md) / [version](index.md)*

---

# Module `version`

## Structs

### `Version`

```rust
struct Version {
    pub minor: u16,
    pub patch: u16,
    pub channel: Channel,
}
```

#### Trait Implementations

##### `impl Clone for Version`

- `fn clone(self: &Self) -> Version` — [`Version`](#version)

##### `impl Copy for Version`

##### `impl Debug for Version`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PartialEq for crate::version::Version`

- `fn eq(self: &Self, rhs: &Bound) -> bool` — [`Bound`](../bound/index.md)

##### `impl PartialOrd for crate::version::Version`

- `fn partial_cmp(self: &Self, rhs: &Bound) -> Option<Ordering>` — [`Bound`](../bound/index.md)

##### `impl StructuralPartialEq for Version`

## Enums

### `Channel`

```rust
enum Channel {
    Stable,
    Beta,
    Nightly(crate::date::Date),
    Dev,
}
```

#### Trait Implementations

##### `impl Clone for Channel`

- `fn clone(self: &Self) -> Channel` — [`Channel`](#channel)

##### `impl Copy for Channel`

##### `impl Debug for Channel`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PartialEq for Channel`

- `fn eq(self: &Self, other: &Channel) -> bool` — [`Channel`](#channel)

##### `impl StructuralPartialEq for Channel`

