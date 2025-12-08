*[cargo_docs_md](../../index.md) / [generator](../index.md) / [render_shared](index.md)*

---

# Module `render_shared`

Shared rendering functions for documentation generation.

This module contains standalone rendering functions that can be used by both
single-crate ([`ItemRenderer`](super::ItemRenderer)) and multi-crate
([`MultiCrateModuleRenderer`](crate::multi_crate::generator)) renderers.

These functions handle the core markdown generation logic without being tied
to a specific rendering context, avoiding code duplication between the two modes.

## Traits

### `DocsProcessor`

```rust
trait DocsProcessor { ... }
```

Check if a render context can resolve documentation.

This trait provides a unified way to process docs from different contexts.

#### Required Methods

- `fn process_item_docs(self: &Self, item: &Item) -> Option<String>`

  Process documentation for an item, resolving intra-doc links.

## Functions

### `render_struct_definition`

```rust
fn render_struct_definition(md: &mut String, name: &str, s: &rustdoc_types::Struct, krate: &rustdoc_types::Crate, type_renderer: &crate::types::TypeRenderer<'_>)
```

Render a struct definition code block to markdown.

Produces a heading with the struct name and generics, followed by a Rust
code block showing the struct definition.

# Arguments

* `md` - Output markdown string
* `name` - The struct name (may differ from item.name for re-exports)
* `s` - The struct data from rustdoc
* `krate` - The crate containing field definitions
* `type_renderer` - Type renderer for generics and field types

### `render_struct_fields`

```rust
fn render_struct_fields<F>(md: &mut String, fields: &[rustdoc_types::Id], krate: &rustdoc_types::Crate, type_renderer: &crate::types::TypeRenderer<'_>, process_docs: F)
where
    F: Fn(&rustdoc_types::Item) -> Option<String>
```

Render documented struct fields to markdown.

Produces a "Fields" section with each documented field as a bullet point
showing the field name, type, and documentation.

# Arguments

* `md` - Output markdown string
* `fields` - Field IDs from the struct
* `krate` - Crate containing field definitions
* `type_renderer` - Type renderer for field types
* `process_docs` - Closure to process documentation with intra-doc link resolution

### `render_enum_definition`

```rust
fn render_enum_definition(md: &mut String, name: &str, e: &rustdoc_types::Enum, krate: &rustdoc_types::Crate, type_renderer: &crate::types::TypeRenderer<'_>)
```

Render an enum definition code block to markdown.

Produces a heading with the enum name and generics, followed by a Rust
code block showing the enum definition with all variants.

# Arguments

* `md` - Output markdown string
* `name` - The enum name (may differ from item.name for re-exports)
* `e` - The enum data from rustdoc
* `krate` - The crate containing variant definitions
* `type_renderer` - Type renderer for generics and variant types

### `render_enum_variant`

```rust
fn render_enum_variant(md: &mut String, variant: &rustdoc_types::Item, krate: &rustdoc_types::Crate, type_renderer: &crate::types::TypeRenderer<'_>)
```

Render a single enum variant within the definition code block.

Handles all three variant kinds: plain, tuple, and struct variants.

### `render_enum_variants_docs`

```rust
fn render_enum_variants_docs<F>(md: &mut String, variants: &[rustdoc_types::Id], krate: &rustdoc_types::Crate, process_docs: F)
where
    F: Fn(&rustdoc_types::Item) -> Option<String>
```

Render documented enum variants to markdown.

Produces a "Variants" section with each documented variant as a bullet point.

# Arguments

* `md` - Output markdown string
* `variants` - Variant IDs from the enum
* `krate` - Crate containing variant definitions
* `process_docs` - Closure to process documentation with intra-doc link resolution

### `render_trait_definition`

```rust
fn render_trait_definition(md: &mut String, name: &str, t: &rustdoc_types::Trait, type_renderer: &crate::types::TypeRenderer<'_>)
```

Render a trait definition code block to markdown.

Produces a heading with the trait name and generics, followed by a Rust
code block showing the trait signature with supertraits.

# Arguments

* `md` - Output markdown string
* `name` - The trait name
* `t` - The trait data from rustdoc
* `type_renderer` - Type renderer for generics and bounds

### `render_trait_item`

```rust
fn render_trait_item<F>(md: &mut String, item: &rustdoc_types::Item, type_renderer: &crate::types::TypeRenderer<'_>, process_docs: F)
where
    F: Fn(&rustdoc_types::Item) -> Option<String>
```

Render a single trait item (method, associated type, or constant).

Each item is rendered as a bullet point with its signature in backticks.
For methods, the first line of documentation is included.

# Arguments

* `md` - Output markdown string
* `item` - The trait item (function, assoc type, or assoc const)
* `type_renderer` - Type renderer for types
* `process_docs` - Closure to process documentation with intra-doc link resolution

### `render_function_definition`

```rust
fn render_function_definition(md: &mut String, name: &str, f: &rustdoc_types::Function, type_renderer: &crate::types::TypeRenderer<'_>)
```

Render a function definition to markdown.

Produces a heading with the function name, followed by a Rust code block
showing the full signature with modifiers (const, async, unsafe).

# Arguments

* `md` - Output markdown string
* `name` - The function name
* `f` - The function data from rustdoc
* `type_renderer` - Type renderer for parameter and return types

### `render_constant_definition`

```rust
fn render_constant_definition(md: &mut String, name: &str, type_: &rustdoc_types::Type, const_: &rustdoc_types::Constant, type_renderer: &crate::types::TypeRenderer<'_>)
```

Render a constant definition to markdown.

Produces a heading with the constant name, followed by a Rust code block
showing `const NAME: Type = value;`.

# Arguments

* `md` - Output markdown string
* `name` - The constant name
* `type_` - The constant's type
* `const_` - The constant data including value
* `type_renderer` - Type renderer for the type

### `render_type_alias_definition`

```rust
fn render_type_alias_definition(md: &mut String, name: &str, ta: &rustdoc_types::TypeAlias, type_renderer: &crate::types::TypeRenderer<'_>)
```

Render a type alias definition to markdown.

Produces a heading with the alias name and generics, followed by a Rust
code block showing `type Name<T> = TargetType;`.

# Arguments

* `md` - Output markdown string
* `name` - The type alias name
* `ta` - The type alias data from rustdoc
* `type_renderer` - Type renderer for generics and the aliased type

### `render_macro_heading`

```rust
fn render_macro_heading(md: &mut String, name: &str)
```

Render a macro definition to markdown.

Produces a heading with the macro name and `!` suffix.
Note: We don't show macro rules since rustdoc JSON doesn't provide them.

# Arguments

* `md` - Output markdown string
* `name` - The macro name

### `render_impl_items`

```rust
fn render_impl_items<F, L>(md: &mut String, impl_block: &rustdoc_types::Impl, krate: &rustdoc_types::Crate, type_renderer: &crate::types::TypeRenderer<'_>, process_docs: &Option<F>, create_type_link: &Option<L>)
where
    F: Fn(&rustdoc_types::Item) -> Option<String>,
    L: Fn(rustdoc_types::Id) -> Option<String>
```

Render the items within an impl block.

This renders all methods, associated constants, and associated types
within an impl block as bullet points.

# Arguments

* `md` - Output markdown string
* `impl_block` - The impl block to render items from
* `krate` - The crate containing item definitions
* `type_renderer` - Type renderer for types
* `process_docs` - Optional closure to process documentation
* `create_type_link` - Optional closure to create links for types `(id -> Option<markdown_link>)`

### `render_function_type_links_inline`

```rust
fn render_function_type_links_inline<L>(md: &mut String, f: &rustdoc_types::Function, type_renderer: crate::types::TypeRenderer<'_>, create_link: &L)
where
    L: Fn(rustdoc_types::Id) -> Option<String>
```

Render type links for a function signature inline (for impl methods).

This is a helper that collects types from function signatures and
creates links for resolvable types, outputting them on the same line.

### `render_impl_function`

```rust
fn render_impl_function(md: &mut String, name: &str, f: &rustdoc_types::Function, type_renderer: crate::types::TypeRenderer<'_>)
```

Render a function signature within an impl block.

Renders as a bullet point with the full signature including modifiers.

### `append_docs`

```rust
fn append_docs(md: &mut String, docs: Option<String>)
```

Append processed documentation to markdown.

Helper function to add documentation with consistent formatting.

### `impl_sort_key`

```rust
fn impl_sort_key(impl_block: &rustdoc_types::Impl, type_renderer: &crate::types::TypeRenderer<'_>) -> String
```

Generate a sort key for an impl block for deterministic ordering.

Combines trait name, generic params, and for-type to create a unique key.

