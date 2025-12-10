*[cargo_docs_md](../../index.md) / [generator](../index.md) / [render_shared](index.md)*

---

# Module `render_shared`

Shared rendering functions for documentation generation.

This module contains standalone rendering functions that can be used by both
single-crate ([`ItemRenderer`](super::ItemRenderer)) and multi-crate
([`MultiCrateModuleRenderer`](crate::multi_crate::generator)) renderers.

These functions handle the core markdown generation logic without being tied
to a specific rendering context, avoiding code duplication between the two modes.

## Contents

- [Structs](#structs)
  - [`SourcePathConfig`](#sourcepathconfig)
  - [`CategorizedTraitItems`](#categorizedtraititems)
  - [`TraitRenderer`](#traitrenderer)
- [Traits](#traits)
  - [`DocsProcessor`](#docsprocessor)
- [Functions](#functions)
  - [`transform_cargo_path`](#transform_cargo_path)
  - [`render_source_location`](#render_source_location)
  - [`sanitize_path`](#sanitize_path)
  - [`sanitize_self_param`](#sanitize_self_param)
  - [`render_struct_definition`](#render_struct_definition)
  - [`render_struct_fields`](#render_struct_fields)
  - [`render_enum_definition`](#render_enum_definition)
  - [`render_enum_variant`](#render_enum_variant)
  - [`render_enum_variants_docs`](#render_enum_variants_docs)
  - [`render_function_definition`](#render_function_definition)
  - [`render_constant_definition`](#render_constant_definition)
  - [`render_type_alias_definition`](#render_type_alias_definition)
  - [`render_macro_heading`](#render_macro_heading)
  - [`render_impl_items`](#render_impl_items)
  - [`render_function_type_links_inline`](#render_function_type_links_inline)
  - [`render_impl_function`](#render_impl_function)
  - [`append_docs`](#append_docs)
  - [`render_collapsible_start`](#render_collapsible_start)
  - [`render_collapsible_end`](#render_collapsible_end)
  - [`impl_sort_key`](#impl_sort_key)
  - [`render_union_definition`](#render_union_definition)
  - [`render_union_fields`](#render_union_fields)
  - [`render_static_definition`](#render_static_definition)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SourcePathConfig`](#sourcepathconfig) | struct | Information needed to transform source paths to relative links. |
| [`CategorizedTraitItems`](#categorizedtraititems) | struct | Categorized trait items for structured rendering. |
| [`TraitRenderer`](#traitrenderer) | struct | Unit struct to ornagize trait related functions. |
| [`DocsProcessor`](#docsprocessor) | trait | Check if a render context can resolve documentation. |
| [`transform_cargo_path`](#transform_cargo_path) | fn | Transform an absolute cargo registry path to a relative `.source_*` path. |
| [`render_source_location`](#render_source_location) | fn | Render a source location reference for an item. |
| [`sanitize_path`](#sanitize_path) | fn | Sanitize trait paths by removing macro artifacts. |
| [`sanitize_self_param`](#sanitize_self_param) | fn | Sanitize self parameter in function signatures. |
| [`render_struct_definition`](#render_struct_definition) | fn | Render a struct definition code block to markdown. |
| [`render_struct_fields`](#render_struct_fields) | fn | Render documented struct fields to markdown. |
| [`render_enum_definition`](#render_enum_definition) | fn | Render an enum definition code block to markdown. |
| [`render_enum_variant`](#render_enum_variant) | fn | Render a single enum variant within the definition code block. |
| [`render_enum_variants_docs`](#render_enum_variants_docs) | fn | Render documented enum variants to markdown. |
| [`render_function_definition`](#render_function_definition) | fn | Render a function definition to markdown. |
| [`render_constant_definition`](#render_constant_definition) | fn | Render a constant definition to markdown. |
| [`render_type_alias_definition`](#render_type_alias_definition) | fn | Render a type alias definition to markdown. |
| [`render_macro_heading`](#render_macro_heading) | fn | Render a macro definition to markdown. |
| [`render_impl_items`](#render_impl_items) | fn | Render the items within an impl block. |
| [`render_function_type_links_inline`](#render_function_type_links_inline) | fn | Render type links for a function signature inline (for impl methods). |
| [`render_impl_function`](#render_impl_function) | fn | Render a function signature within an impl block. |
| [`append_docs`](#append_docs) | fn | Append processed documentation to markdown. |
| [`render_collapsible_start`](#render_collapsible_start) | fn | Render the opening of a collapsible `<details>` block with a summary. |
| [`render_collapsible_end`](#render_collapsible_end) | fn | Render the closing of a collapsible `<details>` block. |
| [`impl_sort_key`](#impl_sort_key) | fn | Generate a sort key for an impl block for deterministic ordering. |
| [`render_union_definition`](#render_union_definition) | fn | Render a union definition code block to markdown. |
| [`render_union_fields`](#render_union_fields) | fn | Render union fields documentation. |
| [`render_static_definition`](#render_static_definition) | fn | Render a static definition code block to markdown. |

## Structs

### `SourcePathConfig`

```rust
struct SourcePathConfig {
    pub source_dir_name: String,
    pub depth: usize,
}
```

*Defined in `src/generator/render_shared.rs:30-37`*

Information needed to transform source paths to relative links.

When generating source location references, this config enables transforming
absolute cargo registry paths to relative links pointing to the local
`.source_{timestamp}` directory.

#### Fields

- **`source_dir_name`**: `String`

  The `.source_{timestamp}` directory name (e.g., `.source_1733660400`).

- **`depth`**: `usize`

  Depth of the current markdown file from `generated_docs/`.
  Used to calculate the correct number of `../` prefixes.

#### Implementations

- <span id="sourcepathconfig-new"></span>`fn new(source_dir: &Path, current_file: &str) -> Self`

- <span id="sourcepathconfig-with-depth"></span>`fn with_depth(&self, current_file: &str) -> Self`

#### Trait Implementations

##### `impl Clone for SourcePathConfig`

- <span id="sourcepathconfig-clone"></span>`fn clone(&self) -> SourcePathConfig` — [`SourcePathConfig`](#sourcepathconfig)

##### `impl Debug for SourcePathConfig`

- <span id="sourcepathconfig-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for SourcePathConfig`

##### `impl IntoEither for SourcePathConfig`

##### `impl OwoColorize for SourcePathConfig`

##### `impl Pointable for SourcePathConfig`

- <span id="sourcepathconfig-const-align"></span>`const ALIGN: usize`

- <span id="sourcepathconfig-type-init"></span>`type Init = T`

- <span id="sourcepathconfig-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourcepathconfig-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourcepathconfig-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourcepathconfig-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for SourcePathConfig`

### `CategorizedTraitItems<'a>`

```rust
struct CategorizedTraitItems<'a> {
    pub required_methods: Vec<&'a rustdoc_types::Item>,
    pub provided_methods: Vec<&'a rustdoc_types::Item>,
    pub associated_types: Vec<&'a rustdoc_types::Item>,
    pub associated_consts: Vec<&'a rustdoc_types::Item>,
}
```

*Defined in `src/generator/render_shared.rs:182-194`*

Categorized trait items for structured rendering.

#### Fields

- **`required_methods`**: `Vec<&'a rustdoc_types::Item>`

  Required methods (no default body)

- **`provided_methods`**: `Vec<&'a rustdoc_types::Item>`

  Provided methods (have default body)

- **`associated_types`**: `Vec<&'a rustdoc_types::Item>`

  Associated types

- **`associated_consts`**: `Vec<&'a rustdoc_types::Item>`

  Associated constants

#### Implementations

- <span id="categorizedtraititems-categorize-trait-items"></span>`fn categorize_trait_items(trait_items: &[Id], krate: &'a Crate) -> Self`

#### Trait Implementations

##### `impl Default for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-default"></span>`fn default() -> CategorizedTraitItems<'a>` — [`CategorizedTraitItems`](#categorizedtraititems)

##### `impl Instrument for CategorizedTraitItems<'a>`

##### `impl IntoEither for CategorizedTraitItems<'a>`

##### `impl OwoColorize for CategorizedTraitItems<'a>`

##### `impl Pointable for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-const-align"></span>`const ALIGN: usize`

- <span id="categorizedtraititems-type-init"></span>`type Init = T`

- <span id="categorizedtraititems-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="categorizedtraititems-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="categorizedtraititems-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="categorizedtraititems-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for CategorizedTraitItems<'a>`

### `TraitRenderer`

```rust
struct TraitRenderer;
```

*Defined in `src/generator/render_shared.rs:567`*

Unit struct to ornagize trait related functions.

#### Implementations

- <span id="traitrenderer-render-trait-definition"></span>`fn render_trait_definition(md: &mut String, name: &str, t: &rustdoc_types::Trait, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="traitrenderer-render-trait-item"></span>`fn render_trait_item<F>(md: &mut String, item: &Item, type_renderer: &TypeRenderer<'_>, process_docs: F)` — [`TypeRenderer`](../../types/index.md#typerenderer)

#### Trait Implementations

##### `impl Instrument for TraitRenderer`

##### `impl IntoEither for TraitRenderer`

##### `impl OwoColorize for TraitRenderer`

##### `impl Pointable for TraitRenderer`

- <span id="traitrenderer-const-align"></span>`const ALIGN: usize`

- <span id="traitrenderer-type-init"></span>`type Init = T`

- <span id="traitrenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="traitrenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="traitrenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="traitrenderer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TraitRenderer`

## Traits

### `DocsProcessor`

```rust
trait DocsProcessor { ... }
```

*Defined in `src/generator/render_shared.rs:1102-1105`*

Check if a render context can resolve documentation.

This trait provides a unified way to process docs from different contexts.

#### Required Methods

- `fn process_item_docs(&self, item: &Item) -> Option<String>`

  Process documentation for an item, resolving intra-doc links.

#### Implementors

- `(&T, &str)`

## Functions

### `transform_cargo_path`

```rust
fn transform_cargo_path(absolute_path: &std::path::Path, source_dir_name: &str) -> Option<String>
```

*Defined in `src/generator/render_shared.rs:84-103`*

Transform an absolute cargo registry path to a relative `.source_*` path.

Converts paths like:
`/home/user/.cargo/registry/src/index.crates.io-xxx/serde-1.0.228/src/lib.rs`

To:
`.source_1733660400/serde-1.0.228/src/lib.rs`

Returns `None` if the path doesn't match the expected cargo registry pattern.

### `render_source_location`

```rust
fn render_source_location(span: Option<&rustdoc_types::Span>, source_path_config: Option<&SourcePathConfig>) -> String
```

*Defined in `src/generator/render_shared.rs:132-178`*

Render a source location reference for an item.

Produces a small italicized line showing the source file and line range.
If `source_path_config` is provided, generates a clickable markdown link
relative to the current file's location.

# Arguments

* `span` - The source span from the item
* `source_path_config` - Optional configuration for path transformation

# Returns

A formatted markdown string with the source location, or empty string if span is None.

# Example Output (without config)

```text
*Defined in `/home/user/.cargo/registry/src/.../serde-1.0.228/src/lib.rs:10-25`*
```

# Example Output (with config, depth=2)

```text
*Defined in [`serde-1.0.228/src/lib.rs:10-25`](../../.source_xxx/serde-1.0.228/src/lib.rs#L10-L25)*
```

### `sanitize_path`

```rust
fn sanitize_path(path: &str) -> std::borrow::Cow<'_, str>
```

*Defined in `src/generator/render_shared.rs:249-255`*

Sanitize trait paths by removing macro artifacts.

Rustdoc JSON can contain `$crate::` prefixes from macro expansions
which leak implementation details into documentation. This function
removes these artifacts for cleaner output.

Uses `Cow<str>` to avoid allocation when no changes are needed.

# Examples

```rust
use cargo_docs_md::generator::render_shared::sanitize_path;

assert_eq!(sanitize_path("$crate::clone::Clone"), "clone::Clone");
assert_eq!(sanitize_path("std::fmt::Debug"), "std::fmt::Debug");
```

### `sanitize_self_param`

```rust
fn sanitize_self_param(param: &str) -> std::borrow::Cow<'_, str>
```

*Defined in `src/generator/render_shared.rs:277-287`*

Sanitize self parameter in function signatures.

Converts verbose self type annotations to idiomatic Rust syntax:
- `self: &Self` → `&self`
- `self: &mut Self` → `&mut self`
- `self: Self` → `self`

Uses `Cow<str>` to avoid allocation when no changes are needed.

# Examples

```rust
use cargo_docs_md::generator::render_shared::sanitize_self_param;

assert_eq!(sanitize_self_param("self: &Self"), "&self");
assert_eq!(sanitize_self_param("self: &mut Self"), "&mut self");
assert_eq!(sanitize_self_param("self: Self"), "self");
assert_eq!(sanitize_self_param("x: i32"), "x: i32");
```

### `render_struct_definition`

```rust
fn render_struct_definition(md: &mut String, name: &str, s: &rustdoc_types::Struct, krate: &rustdoc_types::Crate, type_renderer: &crate::types::TypeRenderer<'_>)
```

*Defined in `src/generator/render_shared.rs:301-378`*

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

*Defined in `src/generator/render_shared.rs:392-429`*

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

*Defined in `src/generator/render_shared.rs:443-466`*

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

*Defined in `src/generator/render_shared.rs:471-524`*

Render a single enum variant within the definition code block.

Handles all three variant kinds: plain, tuple, and struct variants.

### `render_enum_variants_docs`

```rust
fn render_enum_variants_docs<F>(md: &mut String, variants: &[rustdoc_types::Id], krate: &rustdoc_types::Crate, process_docs: F)
where
    F: Fn(&rustdoc_types::Item) -> Option<String>
```

*Defined in `src/generator/render_shared.rs:536-564`*

Render documented enum variants to markdown.

Produces a "Variants" section with each documented variant as a bullet point.

# Arguments

* `md` - Output markdown string
* `variants` - Variant IDs from the enum
* `krate` - Crate containing variant definitions
* `process_docs` - Closure to process documentation with intra-doc link resolution

### `render_function_definition`

```rust
fn render_function_definition(md: &mut String, name: &str, f: &rustdoc_types::Function, type_renderer: &crate::types::TypeRenderer<'_>)
```

*Defined in `src/generator/render_shared.rs:709-756`*

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

*Defined in `src/generator/render_shared.rs:770-794`*

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

*Defined in `src/generator/render_shared.rs:807-826`*

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

*Defined in `src/generator/render_shared.rs:837-839`*

Render a macro definition to markdown.

Produces a heading with the macro name and `!` suffix.
Note: We don't show macro rules since rustdoc JSON doesn't provide them.

# Arguments

* `md` - Output markdown string
* `name` - The macro name

### `render_impl_items`

```rust
fn render_impl_items<F, L>(md: &mut String, impl_block: &rustdoc_types::Impl, krate: &rustdoc_types::Crate, type_renderer: &crate::types::TypeRenderer<'_>, process_docs: &Option<F>, create_type_link: &Option<L>, parent_type_name: Option<&str>)
where
    F: Fn(&rustdoc_types::Item) -> Option<String>,
    L: Fn(rustdoc_types::Id) -> Option<String>
```

*Defined in `src/generator/render_shared.rs:855-935`*

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
* `parent_type_name` - Optional type name for generating method anchors

### `render_function_type_links_inline`

```rust
fn render_function_type_links_inline<L>(md: &mut String, f: &rustdoc_types::Function, type_renderer: crate::types::TypeRenderer<'_>, create_link: &L)
where
    L: Fn(rustdoc_types::Id) -> Option<String>
```

*Defined in `src/generator/render_shared.rs:941-980`*

Render type links for a function signature inline (for impl methods).

This is a helper that collects types from function signatures and
creates links for resolvable types, outputting them on the same line.

### `render_impl_function`

```rust
fn render_impl_function(md: &mut String, name: &str, f: &rustdoc_types::Function, type_renderer: crate::types::TypeRenderer<'_>, parent_type_name: Option<&str>)
```

*Defined in `src/generator/render_shared.rs:986-1032`*

Render a function signature within an impl block.

Renders as a bullet point with the full signature including modifiers.
If `parent_type_name` is provided, includes a hidden anchor for deep linking.

### `append_docs`

```rust
fn append_docs(md: &mut String, docs: Option<String>)
```

*Defined in `src/generator/render_shared.rs:1037-1042`*

Append processed documentation to markdown.

Helper function to add documentation with consistent formatting.

### `render_collapsible_start`

```rust
fn render_collapsible_start(summary: &str) -> String
```

*Defined in `src/generator/render_shared.rs:1063-1065`*

Render the opening of a collapsible `<details>` block with a summary.

Produces HTML that creates a collapsible section in markdown. Use with
[`render_collapsible_end`](#render-collapsible-end) to close the block.

# Arguments

* `summary` - The text to display in the summary line (clickable header)

# Example

```rust
use cargo_docs_md::generator::render_shared::render_collapsible_start;

let start = render_collapsible_start("Derived Traits (9 implementations)");
assert!(start.contains("<details>"));
assert!(start.contains("<summary>Derived Traits (9 implementations)</summary>"));
```

### `render_collapsible_end`

```rust
const fn render_collapsible_end() -> &'static str
```

*Defined in `src/generator/render_shared.rs:1079-1081`*

Render the closing of a collapsible `<details>` block.

Returns a static string to close a block opened with [`render_collapsible_start`](#render-collapsible-start).

# Example

```rust
use cargo_docs_md::generator::render_shared::render_collapsible_end;

assert_eq!(render_collapsible_end(), "\n</details>\n\n");
```

### `impl_sort_key`

```rust
fn impl_sort_key(impl_block: &rustdoc_types::Impl, type_renderer: &crate::types::TypeRenderer<'_>) -> String
```

*Defined in `src/generator/render_shared.rs:1087-1097`*

Generate a sort key for an impl block for deterministic ordering.

Combines trait name, generic params, and for-type to create a unique key.

### `render_union_definition`

```rust
fn render_union_definition(md: &mut String, name: &str, u: &rustdoc_types::Union, krate: &rustdoc_types::Crate, type_renderer: &crate::types::TypeRenderer<'_>)
```

*Defined in `src/generator/render_shared.rs:1125-1166`*

Render a union definition code block to markdown.

Produces a heading with the union name and generics, followed by a Rust
code block showing the union definition with all fields.

# Arguments

* `md` - Output markdown string
* `name` - The union name (may differ from item.name for re-exports)
* `u` - The union data from rustdoc
* `krate` - The crate containing field definitions
* `type_renderer` - Type renderer for generics and field types

### `render_union_fields`

```rust
fn render_union_fields<F>(md: &mut String, fields: &[rustdoc_types::Id], krate: &rustdoc_types::Crate, type_renderer: &crate::types::TypeRenderer<'_>, process_docs: F)
where
    F: Fn(&rustdoc_types::Item) -> Option<String>
```

*Defined in `src/generator/render_shared.rs:1180-1225`*

Render union fields documentation.

Creates a "Fields" section with each field's name, type, and documentation.
Only renders if at least one field has documentation.

# Arguments

* `md` - Output markdown string
* `fields` - Field IDs from the union
* `krate` - The crate containing field definitions
* `type_renderer` - Type renderer for field types
* `process_docs` - Callback to process documentation strings

### `render_static_definition`

```rust
fn render_static_definition(md: &mut String, name: &str, s: &rustdoc_types::Static, type_renderer: &crate::types::TypeRenderer<'_>)
```

*Defined in `src/generator/render_shared.rs:1238-1275`*

Render a static definition code block to markdown.

Produces a heading with the static name, followed by a Rust
code block showing the static definition.

# Arguments

* `md` - Output markdown string
* `name` - The static name (may differ from item.name for re-exports)
* `s` - The static data from rustdoc
* `type_renderer` - Type renderer for the static's type

