*[docs_md](../../index.md) / [generator](../index.md) / [breadcrumbs](index.md)*

---

# Module `breadcrumbs`

Breadcrumb navigation generation for nested module pages.

This module provides [`BreadcrumbGenerator`](generator/breadcrumbs/index.md) which creates navigation
links showing the path from crate root to the current module.

## Structs

### `BreadcrumbGenerator<'a>`

```rust
struct BreadcrumbGenerator<'a> {
    // [REDACTED: Private Fields]
}
```

Generates breadcrumb navigation for nested module pages.

Creates a navigation line showing the path from the crate root to
the current module, with each segment being a clickable link.

#### Implementations

- `fn new(module_path: &'a [String], crate_name: &'a str) -> Self`
  Create a new breadcrumb generator.

- `fn generate(self: &Self) -> String`
  Generate breadcrumb navigation markdown.

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

