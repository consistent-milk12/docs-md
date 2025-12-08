*[anstyle](../index.md) / [reset](index.md)*

---

# Module `reset`

## Structs

### `Reset`

```rust
struct Reset;
```

Reset terminal formatting

#### Implementations

- `fn render(self: Self) -> impl core::fmt::Display + Copy`

#### Trait Implementations

##### `impl Clone for Reset`

- `fn clone(self: &Self) -> Reset` — [`Reset`](../index.md)

##### `impl Copy for Reset`

##### `impl Debug for Reset`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Reset`

- `fn default() -> Reset` — [`Reset`](../index.md)

##### `impl Display for Reset`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Reset`

##### `impl Hash for Reset`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Reset`

- `fn cmp(self: &Self, other: &Reset) -> $crate::cmp::Ordering` — [`Reset`](../index.md)

##### `impl PartialEq for Reset`

- `fn eq(self: &Self, other: &Reset) -> bool` — [`Reset`](../index.md)

##### `impl PartialOrd for Reset`

- `fn partial_cmp(self: &Self, other: &Reset) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Reset`](../index.md)

##### `impl StructuralPartialEq for Reset`

##### `impl<T> ToString for Reset`

- `fn to_string(self: &Self) -> String`

## Constants

### `RESET`

```rust
const RESET: &str;
```

