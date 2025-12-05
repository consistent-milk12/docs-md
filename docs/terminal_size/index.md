# Crate `terminal_size`

A simple utility for getting the size of a terminal.

Works on Linux, macOS, Windows, and illumos.

This crate requires a minimum Rust version of 1.63.0 (2022-08-11).

# Example

```rust
use terminal_size::{Width, Height, terminal_size};

let size = terminal_size();
if let Some((Width(w), Height(h))) = size {
    println!("Your terminal is {} cols wide and {} lines tall", w, h);
} else {
    println!("Unable to get terminal size");
}
```


## Structs

### `Width`

```rust
struct Width(u16);
```

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Width` — [`Width`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Width) -> $crate::cmp::Ordering` — [`Width`](../index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Width) -> bool` — [`Width`](../index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Width) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Width`](../index.md)

##### `impl StructuralPartialEq`

### `Height`

```rust
struct Height(u16);
```

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Height` — [`Height`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Height) -> $crate::cmp::Ordering` — [`Height`](../index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Height) -> bool` — [`Height`](../index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Height) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Height`](../index.md)

##### `impl StructuralPartialEq`

## Functions

