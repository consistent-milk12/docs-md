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


## Modules

- [`unix`](unix/index.md) - 

## Structs

### `Width`

```rust
struct Width(u16);
```

#### Trait Implementations

##### `impl Clone for Width`

- `fn clone(self: &Self) -> Width` — [`Width`](#width)

##### `impl Copy for Width`

##### `impl Debug for Width`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Width`

##### `impl Ord for Width`

- `fn cmp(self: &Self, other: &Width) -> $crate::cmp::Ordering` — [`Width`](#width)

##### `impl PartialEq for Width`

- `fn eq(self: &Self, other: &Width) -> bool` — [`Width`](#width)

##### `impl PartialOrd for Width`

- `fn partial_cmp(self: &Self, other: &Width) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Width`](#width)

##### `impl StructuralPartialEq for Width`

### `Height`

```rust
struct Height(u16);
```

#### Trait Implementations

##### `impl Clone for Height`

- `fn clone(self: &Self) -> Height` — [`Height`](#height)

##### `impl Copy for Height`

##### `impl Debug for Height`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Height`

##### `impl Ord for Height`

- `fn cmp(self: &Self, other: &Height) -> $crate::cmp::Ordering` — [`Height`](#height)

##### `impl PartialEq for Height`

- `fn eq(self: &Self, other: &Height) -> bool` — [`Height`](#height)

##### `impl PartialOrd for Height`

- `fn partial_cmp(self: &Self, other: &Height) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Height`](#height)

##### `impl StructuralPartialEq for Height`

## Functions

