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

  Create a new breadcrumb generator.

  

  # Arguments

  

  * `module_path` - The module path segments

  * `crate_name` - The name of the crate for the root link

- <span id="breadcrumbgenerator-generate"></span>`fn generate(&self) -> String`

  Generate breadcrumb navigation markdown.

  

  Returns empty string for root module.

  

  # Example Output

  

  For `module_path = ["error", "types"]` and `crate_name = "docs_md"`:

  ```markdown

  *[docs_md](../../index.md) / [error](../index.md) / [types](index.md)*

  

  ---

  ```

#### Trait Implementations

##### `impl Any for BreadcrumbGenerator<'a>`

- <span id="breadcrumbgenerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BreadcrumbGenerator<'a>`

- <span id="breadcrumbgenerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BreadcrumbGenerator<'a>`

- <span id="breadcrumbgenerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for BreadcrumbGenerator<'a>`

- <span id="breadcrumbgenerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for BreadcrumbGenerator<'a>`

##### `impl<U> Into for BreadcrumbGenerator<'a>`

- <span id="breadcrumbgenerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for BreadcrumbGenerator<'a>`

##### `impl OwoColorize for BreadcrumbGenerator<'a>`

##### `impl Pointable for BreadcrumbGenerator<'a>`

- <span id="breadcrumbgenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="breadcrumbgenerator-pointable-type-init"></span>`type Init = T`

- <span id="breadcrumbgenerator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="breadcrumbgenerator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="breadcrumbgenerator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="breadcrumbgenerator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for BreadcrumbGenerator<'a>`

- <span id="breadcrumbgenerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="breadcrumbgenerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BreadcrumbGenerator<'a>`

- <span id="breadcrumbgenerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="breadcrumbgenerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for BreadcrumbGenerator<'a>`

