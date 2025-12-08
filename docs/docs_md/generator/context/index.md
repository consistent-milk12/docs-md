*[docs_md](../../index.md) / [generator](../index.md) / [context](index.md)*

---

# Module `context`

Shared context for documentation generation.

This module provides the [`GeneratorContext`](../index.md) struct which holds all shared
state needed during markdown generation, including the crate data, lookup
maps, and configuration options.

# Trait Hierarchy

The rendering context is split into focused traits for better abstraction:

- [`ItemAccess`](../index.md) - Core data access (crate, items, impls)
- [`ItemFilter`](../index.md) - Visibility and filtering logic
- [`LinkResolver`](../index.md) - Link creation and documentation processing
- [`RenderContext`](../index.md) - Combined super-trait for convenience

This allows components to depend only on the traits they need, improving
testability and reducing coupling.

## Structs

### `GeneratorContext<'a>`

```rust
struct GeneratorContext<'a> {
    pub krate: &'a rustdoc_types::Crate,
    crate_name: String,
    pub impl_map: std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>,
    pub link_registry: crate::linker::LinkRegistry,
    pub args: &'a crate::Args,
}
```

Shared context containing all data needed for documentation generation.

This struct is passed to all rendering components and provides:
- Access to the parsed crate data
- Impl block lookup for rendering implementations
- Link registry for cross-references
- CLI configuration options

#### Fields

- **`krate`**: `&'a rustdoc_types::Crate`

  The parsed rustdoc JSON crate.

- **`crate_name`**: `String`

  The crate name (extracted from root module).

- **`impl_map`**: `std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>`

  Maps type IDs to all impl blocks for that type.
  
  Used for rendering "Implementations" and "Trait Implementations" sections.

- **`link_registry`**: `crate::linker::LinkRegistry`

  Registry for creating cross-reference links between items.

- **`args`**: `&'a crate::Args`

  CLI arguments containing output path, format, and options.

#### Implementations

- `fn new(krate: &'a Crate, args: &'a Args) -> Self` — [`Args`](../../index.md)

- `fn build_impl_map(krate: &'a Crate) -> HashMap<Id, Vec<&'a Impl>>`

- `fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

- `const fn get_type_id(ty: &rustdoc_types::Type) -> Option<Id>`

- `const fn should_include_item(self: &Self, item: &Item) -> bool`

- `fn count_modules(self: &Self, item: &Item) -> usize`

#### Trait Implementations

##### `impl<T> Instrument for GeneratorContext<'a>`

##### `impl<T> IntoEither for GeneratorContext<'a>`

##### `impl ItemAccess for GeneratorContext<'_>`

- `fn krate(self: &Self) -> &Crate`

- `fn crate_name(self: &Self) -> &str`

- `fn get_item(self: &Self, id: &Id) -> Option<&Item>`

- `fn get_impls(self: &Self, id: &Id) -> Option<&[&Impl]>`

- `fn crate_version(self: &Self) -> Option<&str>`

##### `impl ItemFilter for GeneratorContext<'_>`

- `fn should_include_item(self: &Self, item: &Item) -> bool`

- `fn include_private(self: &Self) -> bool`

- `fn include_blanket_impls(self: &Self) -> bool`

##### `impl LinkResolver for GeneratorContext<'_>`

- `fn link_registry(self: &Self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../../index.md)

- `fn process_docs(self: &Self, item: &Item, current_file: &str) -> Option<String>`

- `fn create_link(self: &Self, id: Id, current_file: &str) -> Option<String>`

##### `impl<D> OwoColorize for GeneratorContext<'a>`

##### `impl<T> Pointable for GeneratorContext<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> RenderContext for GeneratorContext<'a>`

##### `impl<T> WithSubscriber for GeneratorContext<'a>`

## Traits

### `ItemAccess`

```rust
trait ItemAccess { ... }
```

Core data access for crate documentation.

Provides read-only access to the crate structure, items, and impl blocks.

#### Required Methods

- `fn krate(self: &Self) -> &Crate`

  Get the crate being documented.

- `fn crate_name(self: &Self) -> &str`

  Get the crate name.

- `fn get_item(self: &Self, id: &Id) -> Option<&Item>`

  Get an item by its ID.

- `fn get_impls(self: &Self, id: &Id) -> Option<&[&Impl]>`

  Get impl blocks for a type.

- `fn crate_version(self: &Self) -> Option<&str>`

  Get the crate version for display in headers.

### `ItemFilter`

```rust
trait ItemFilter { ... }
```

Item visibility and filtering logic.

Determines which items should be included in the generated documentation.

#### Required Methods

- `fn should_include_item(self: &Self, item: &Item) -> bool`

  Check if an item should be included based on visibility.

- `fn include_private(self: &Self) -> bool`

  Whether private items should be included.

- `fn include_blanket_impls(self: &Self) -> bool`

  Whether blanket trait implementations should be included.

### `LinkResolver`

```rust
trait LinkResolver { ... }
```

Link creation and documentation processing.

Handles intra-doc link resolution and markdown link generation.

#### Required Methods

- `fn link_registry(self: &Self) -> Option<&LinkRegistry>`

  Get the link registry for single-crate mode.

- `fn process_docs(self: &Self, item: &Item, current_file: &str) -> Option<String>`

  Process documentation string with intra-doc link resolution.

- `fn create_link(self: &Self, id: Id, current_file: &str) -> Option<String>`

  Create a markdown link to an item.

### `RenderContext`

```rust
trait RenderContext: ItemAccess + ItemFilter + LinkResolver { ... }
```

Combined rendering context trait.

This trait combines [`ItemAccess`](../index.md), [`ItemFilter`](../index.md), and [`LinkResolver`](../index.md)
for components that need full access to the rendering context.

Most renderers should use this trait for convenience, but components
with limited requirements can depend on individual sub-traits.

