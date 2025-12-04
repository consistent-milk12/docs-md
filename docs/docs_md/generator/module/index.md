*[docs_md](../../index.md) / [generator](../index.md) / [module](index.md)*

---

# Module `module`

Module markdown rendering for documentation generation.

This module provides the [`ModuleRenderer`](generator/module/index.md) struct which handles rendering
a Rust module's documentation to markdown format, including all its items
organized by type.

## Structs

### `ModuleRenderer<'a>`

```rust
struct ModuleRenderer<'a> {
    // [REDACTED: Private Fields]
}
```

Renders a module to markdown.

This struct handles the complete rendering of a module's documentation page,
including:
- Title (Crate or Module heading)
- Module-level documentation
- Sections for each item type (Modules, Structs, Enums, etc.)

#### Implementations

- `fn new(ctx: &'a GeneratorContext<'a>, current_file: &'a str, is_root: bool) -> Self`
  Create a new module renderer.

- `fn render(self: &Self, item: &Item) -> String`
  Generate the complete markdown content for a module.

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

