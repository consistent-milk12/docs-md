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


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unix`](#unix) | mod |  |
| [`Width`](#width) | struct |  |
| [`Height`](#height) | struct |  |
| [`terminal_size`](#terminal_size) | fn |  |
| [`terminal_size_of`](#terminal_size_of) | fn |  |
| [`terminal_size_using_fd`](#terminal_size_using_fd) | fn |  |

## Modules

- [`unix`](unix/index.md)

## Structs

### `Width`

```rust
struct Width(u16);
```

#### Trait Implementations

##### `impl Clone for Width`

- <span id="width-clone"></span>`fn clone(&self) -> Width` — [`Width`](#width)

##### `impl Copy for Width`

##### `impl Debug for Width`

- <span id="width-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Width`

##### `impl Ord for Width`

- <span id="width-cmp"></span>`fn cmp(&self, other: &Width) -> cmp::Ordering` — [`Width`](#width)

##### `impl PartialEq for Width`

- <span id="width-eq"></span>`fn eq(&self, other: &Width) -> bool` — [`Width`](#width)

##### `impl PartialOrd for Width`

- <span id="width-partial-cmp"></span>`fn partial_cmp(&self, other: &Width) -> option::Option<cmp::Ordering>` — [`Width`](#width)

##### `impl StructuralPartialEq for Width`

### `Height`

```rust
struct Height(u16);
```

#### Trait Implementations

##### `impl Clone for Height`

- <span id="height-clone"></span>`fn clone(&self) -> Height` — [`Height`](#height)

##### `impl Copy for Height`

##### `impl Debug for Height`

- <span id="height-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Height`

##### `impl Ord for Height`

- <span id="height-cmp"></span>`fn cmp(&self, other: &Height) -> cmp::Ordering` — [`Height`](#height)

##### `impl PartialEq for Height`

- <span id="height-eq"></span>`fn eq(&self, other: &Height) -> bool` — [`Height`](#height)

##### `impl PartialOrd for Height`

- <span id="height-partial-cmp"></span>`fn partial_cmp(&self, other: &Height) -> option::Option<cmp::Ordering>` — [`Height`](#height)

##### `impl StructuralPartialEq for Height`

## Functions

