*[docs_md](../index.md) / [linker](index.md)*

---

# Module `linker`

Cross-reference linking for markdown documentation.

This module provides the `LinkRegistry` which maps rustdoc item IDs to their
corresponding markdown file paths. This enables creating clickable links
between items in the generated documentation.

# How It Works

1. During initialization, `LinkRegistry::build()` traverses the entire crate
   and records where each item's documentation will be written.

2. During markdown generation, `create_link()` is called to generate
   relative links from one file to another.

# Path Formats

The registry supports two output formats:

- **Flat**: `module.md`, `parent__child.md` (double-underscore separators)
- **Nested**: `module/index.md`, `parent/child/index.md` (directory structure)

# Example

```ignore
let registry = LinkRegistry::build(&krate, true); // flat format
let link = registry.create_link(&some_id, "index.md");
// Returns: Some("[`ItemName`](module.md)")
```

## Structs

### `LinkRegistry`

```rust
struct LinkRegistry {
}
```

Registry mapping item IDs to their documentation file paths.

This is the central data structure for cross-reference resolution.
It's built once during generation and queried whenever we need to
create links between items.

#### Implementations

- `fn build(krate: &Crate, flat_format: bool) -> Self`
  Build a link registry by traversing all items in the crate.

- `fn get_path(self: &Self, id: Id) -> Option<&String>`
  Get the file path where an item is documented.

- `fn get_name(self: &Self, id: Id) -> Option<&String>`
  Get the display name for an item.

- `fn create_link(self: &Self, id: Id, from_path: &str) -> Option<String>`
  Create a markdown link to an item from a given source file.

- `fn compute_relative_path(from: &str, to: &str) -> String`
  Compute the relative path from one file to another.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl OwoColorize<D>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> LinkRegistry`

