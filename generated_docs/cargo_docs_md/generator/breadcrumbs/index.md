*[cargo_docs_md](../../index.md) / [generator](../index.md) / [breadcrumbs](index.md)*

---

# Module `breadcrumbs`

Breadcrumb navigation generation for nested module pages.

This module provides [`BreadcrumbGenerator`](#breadcrumbgenerator) which creates navigation
links showing the path from crate root to the current module.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BreadcrumbGenerator`](#breadcrumbgenerator) | struct | Generates breadcrumb navigation for nested module pages. |

## Structs

### `BreadcrumbGenerator<'a>`

```rust
struct BreadcrumbGenerator<'a> {
    module_path: &'a [String],
    crate_name: &'a str,
}
```

*Defined in `src/generator/breadcrumbs.rs:10-16`*

Generates breadcrumb navigation for nested module pages.

Creates a navigation line showing the path from the crate root to
the current module, with each segment being a clickable link.

#### Fields

- **`module_path`**: `&'a [String]`

  The module path segments (e.g., `["error", "types"]`).

- **`crate_name`**: `&'a str`

  The name of the crate for the root link.

#### Implementations

- <span id="breadcrumbgenerator-new"></span>`const fn new(module_path: &'a [String], crate_name: &'a str) -> Self`

- <span id="breadcrumbgenerator-generate"></span>`fn generate(&self) -> String`

#### Trait Implementations

##### `impl Instrument for BreadcrumbGenerator<'a>`

##### `impl IntoEither for BreadcrumbGenerator<'a>`

##### `impl OwoColorize for BreadcrumbGenerator<'a>`

##### `impl Pointable for BreadcrumbGenerator<'a>`

- <span id="breadcrumbgenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="breadcrumbgenerator-pointable-type-init"></span>`type Init = T`

- <span id="breadcrumbgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="breadcrumbgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="breadcrumbgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="breadcrumbgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for BreadcrumbGenerator<'a>`

