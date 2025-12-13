*[cargo_docs_md](../../index.md) / [generator](../index.md) / [render_shared](index.md)*

---

# Module `render_shared`

Shared rendering functions for documentation generation.

This module contains standalone rendering functions that can be used by both
single-crate ([`ItemRenderer`](super::ItemRenderer)) and multi-crate
([`MultiCrateModuleRenderer`](crate::multi_crate::generator)) renderers.

These functions handle the core markdown generation logic without being tied
to a specific rendering context, avoiding code duplication between the two modes.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SourcePathConfig`](#sourcepathconfig) | struct | Information needed to transform source paths to relative links. |
| [`CategorizedTraitItems`](#categorizedtraititems) | struct | Categorized trait items for structured rendering. |
| [`RendererUtils`](#rendererutils) | struct | Unit struct to organize path related utility functions related to renderer functions. |
| [`TraitRenderer`](#traitrenderer) | struct | Unit struct to organize trait related functions. |
| [`RendererInternals`](#rendererinternals) | struct | Unit struct containing renderer functions. |
| [`DocsProcessor`](#docsprocessor) | trait | Check if a render context can resolve documentation. |

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

  Create a new source path config.

  

  # Arguments

  

  * `source_dir` - Full path to the `.source_*` directory

  * `current_file` - Path of the current markdown file relative to output dir

- <span id="sourcepathconfig-with-depth"></span>`fn with_depth(&self, current_file: &str) -> Self`

  Create a config with a specific depth (for file-specific configs).

#### Trait Implementations

##### `impl Any for SourcePathConfig`

- <span id="sourcepathconfig-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourcePathConfig`

- <span id="sourcepathconfig-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourcePathConfig`

- <span id="sourcepathconfig-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SourcePathConfig`

- <span id="sourcepathconfig-clone"></span>`fn clone(&self) -> SourcePathConfig` — [`SourcePathConfig`](#sourcepathconfig)

##### `impl CloneToUninit for SourcePathConfig`

- <span id="sourcepathconfig-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SourcePathConfig`

- <span id="sourcepathconfig-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SourcePathConfig`

- <span id="sourcepathconfig-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SourcePathConfig`

##### `impl<U> Into for SourcePathConfig`

- <span id="sourcepathconfig-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SourcePathConfig`

##### `impl OwoColorize for SourcePathConfig`

##### `impl Pointable for SourcePathConfig`

- <span id="sourcepathconfig-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourcepathconfig-pointable-type-init"></span>`type Init = T`

- <span id="sourcepathconfig-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourcepathconfig-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourcepathconfig-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourcepathconfig-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for SourcePathConfig`

- <span id="sourcepathconfig-toowned-type-owned"></span>`type Owned = T`

- <span id="sourcepathconfig-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sourcepathconfig-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SourcePathConfig`

- <span id="sourcepathconfig-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourcepathconfig-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourcePathConfig`

- <span id="sourcepathconfig-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourcepathconfig-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in `src/generator/render_shared.rs:76-88`*

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

  Categorize trait items into required/provided methods, types and constants.

#### Trait Implementations

##### `impl Any for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Default for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-default"></span>`fn default() -> CategorizedTraitItems<'a>` — [`CategorizedTraitItems`](#categorizedtraititems)

##### `impl<T> From for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for CategorizedTraitItems<'a>`

##### `impl<U> Into for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CategorizedTraitItems<'a>`

##### `impl OwoColorize for CategorizedTraitItems<'a>`

##### `impl Pointable for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-pointable-const-align"></span>`const ALIGN: usize`

- <span id="categorizedtraititems-pointable-type-init"></span>`type Init = T`

- <span id="categorizedtraititems-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="categorizedtraititems-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="categorizedtraititems-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="categorizedtraititems-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="categorizedtraititems-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="categorizedtraititems-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for CategorizedTraitItems<'a>`

### `RendererUtils`

```rust
struct RendererUtils;
```

*Defined in `src/generator/render_shared.rs:127`*

Unit struct to organize path related utility functions related to renderer functions.

#### Implementations

- <span id="rendererutils-sanitize-path"></span>`fn sanitize_path(path: &str) -> Cow<'_, str>`

  Sanitize trait paths by removing macro artifacts.

  

  Rustdoc JSON can contain `$crate::` prefixes from macro expansions

  which leak implementation details into documentation. This function

  removes these artifacts for cleaner output.

  

  Uses `Cow<str>` to avoid allocation when no changes are needed.

  

  # Examples

  

  ```rust

  use cargo_docs_md::generator::render_shared::RendererUtils;

  

  assert_eq!(RendererUtils::sanitize_path("$crate::clone::Clone"), "clone::Clone");

  assert_eq!(RendererUtils::sanitize_path("std::fmt::Debug"), "std::fmt::Debug");

  ```

- <span id="rendererutils-sanitize-self-param"></span>`fn sanitize_self_param(param: &str) -> Cow<'_, str>`

  Sanitize self parameter in function signatures.

  

  Converts verbose self type annotations to idiomatic Rust syntax:

  - `self: &Self` → `&self`

  - `self: &mut Self` → `&mut self`

  - `self: Self` → `self`

  

  Uses `Cow<str>` to avoid allocation when no changes are needed.

  

  # Examples

  

  ```rust

  use cargo_docs_md::generator::render_shared::RendererUtils;

  

  assert_eq!(RendererUtils::sanitize_self_param("self: &Self"), "&self");

  assert_eq!(RendererUtils::sanitize_self_param("self: &mut Self"), "&mut self");

  assert_eq!(RendererUtils::sanitize_self_param("self: Self"), "self");

  assert_eq!(RendererUtils::sanitize_self_param("x: i32"), "x: i32");

  ```

- <span id="rendererutils-write-tuple-fields"></span>`fn write_tuple_fields(out: &mut String, fields: &[Option<Id>], krate: &Crate, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Write tuple field types directly to buffer, comma-separated.

  

  Avoids intermediate `Vec` allocation by writing directly to the output buffer.

  Handles `Option<Id>` fields from rustdoc's representation of tuple structs/variants

  (where `None` indicates a private field).

  

  # Arguments

  

  * `out` - Output buffer to write to

  * `fields` - Slice of optional field IDs from rustdoc

  * `krate` - Crate containing field definitions

  * `type_renderer` - Type renderer for field types

- <span id="rendererutils-transform-cargo-path"></span>`fn transform_cargo_path(absolute_path: &Path, source_dir_name: &str) -> Option<String>`

  Transform an absolute cargo registry path to a relative `.source_*` path.

  

  Converts paths like:

  `/home/user/.cargo/registry/src/index.crates.io-xxx/serde-1.0.228/src/lib.rs`

  

  To:

  `.source_1733660400/serde-1.0.228/src/lib.rs`

  

  Returns `None` if the path doesn't match the expected cargo registry pattern.

#### Trait Implementations

##### `impl Any for RendererUtils`

- <span id="rendererutils-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RendererUtils`

- <span id="rendererutils-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RendererUtils`

- <span id="rendererutils-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for RendererUtils`

- <span id="rendererutils-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for RendererUtils`

##### `impl<U> Into for RendererUtils`

- <span id="rendererutils-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for RendererUtils`

##### `impl OwoColorize for RendererUtils`

##### `impl Pointable for RendererUtils`

- <span id="rendererutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rendererutils-pointable-type-init"></span>`type Init = T`

- <span id="rendererutils-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rendererutils-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rendererutils-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rendererutils-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for RendererUtils`

- <span id="rendererutils-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rendererutils-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RendererUtils`

- <span id="rendererutils-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rendererutils-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for RendererUtils`

### `TraitRenderer`

```rust
struct TraitRenderer;
```

*Defined in `src/generator/render_shared.rs:254`*

Unit struct to organize trait related functions.

#### Implementations

- <span id="traitrenderer-write-trait-bounds"></span>`fn write_trait_bounds(out: &mut String, bounds: &[rustdoc_types::GenericBound], type_renderer: TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Write trait bounds with `: ` prefix directly to buffer.

  

  Avoids intermediate `Vec` allocation for trait supertrait bounds.

  Writes nothing if bounds are empty.

  

  # Arguments

  

  * `out` - Output buffer to write to

  * `bounds` - Slice of generic bounds from the trait

  * `type_renderer` - Type renderer for bounds (passed by value as it's Copy)

- <span id="traitrenderer-render-trait-definition"></span>`fn render_trait_definition(md: &mut String, name: &str, t: &rustdoc_types::Trait, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render a trait definition code block to markdown.

  

  Produces a heading with the trait name and generics, followed by a Rust

  code block showing the trait signature with supertraits.

  

  # Arguments

  

  * `md` - Output markdown string

  * `name` - The trait name

  * `t` - The trait data from rustdoc

  * `type_renderer` - Type renderer for generics and bounds

- <span id="traitrenderer-render-trait-item"></span>`fn render_trait_item<F>(md: &mut String, item: &Item, type_renderer: &TypeRenderer<'_>, process_docs: F)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render a single trait item (method, associated type, or constant).

  

  Each item is rendered as a bullet point with its signature in backticks.

  For methods, the first line of documentation is included.

  

  # Arguments

  

  * `md` - Output markdown string

  * `item` - The trait item (function, assoc type, or assoc const)

  * `type_renderer` - Type renderer for types

  * `process_docs` - Closure to process documentation with intra-doc link resolution

#### Trait Implementations

##### `impl Any for TraitRenderer`

- <span id="traitrenderer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitRenderer`

- <span id="traitrenderer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitRenderer`

- <span id="traitrenderer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for TraitRenderer`

- <span id="traitrenderer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for TraitRenderer`

##### `impl<U> Into for TraitRenderer`

- <span id="traitrenderer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TraitRenderer`

##### `impl OwoColorize for TraitRenderer`

##### `impl Pointable for TraitRenderer`

- <span id="traitrenderer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="traitrenderer-pointable-type-init"></span>`type Init = T`

- <span id="traitrenderer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="traitrenderer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="traitrenderer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="traitrenderer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for TraitRenderer`

- <span id="traitrenderer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traitrenderer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitRenderer`

- <span id="traitrenderer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traitrenderer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for TraitRenderer`

### `RendererInternals`

```rust
struct RendererInternals;
```

*Defined in `src/generator/render_shared.rs:418`*

Unit struct containing renderer functions.
Helpful because free functions are annoying.

#### Implementations

- <span id="rendererinternals-render-struct-definition"></span>`fn render_struct_definition(md: &mut String, name: &str, s: &rustdoc_types::Struct, krate: &Crate, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render a struct definition code block to markdown.

  

  Produces a heading with the struct name and generics, followed by a Rust

  code block showing the struct definition.

  

  # Arguments

  

  * `md` - Output markdown string

  * `name` - The struct name (may differ from item.name for re-exports)

  * `s` - The struct data from rustdoc

  * `krate` - The crate containing field definitions

  * `type_renderer` - Type renderer for generics and field types

- <span id="rendererinternals-render-struct-fields"></span>`fn render_struct_fields<F>(md: &mut String, fields: &[Id], krate: &Crate, type_renderer: &TypeRenderer<'_>, process_docs: F)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render documented struct fields to markdown.

  

  Produces a "Fields" section with each documented field as a bullet point

  showing the field name, type, and documentation.

  

  # Arguments

  

  * `md` - Output markdown string

  * `fields` - Field IDs from the struct

  * `krate` - Crate containing field definitions

  * `type_renderer` - Type renderer for field types

  * `process_docs` - Closure to process documentation with intra-doc link resolution

- <span id="rendererinternals-render-enum-definition"></span>`fn render_enum_definition(md: &mut String, name: &str, e: &rustdoc_types::Enum, krate: &Crate, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render an enum definition code block to markdown.

  

  Produces a heading with the enum name and generics, followed by a Rust

  code block showing the enum definition with all variants.

  

  # Arguments

  

  * `md` - Output markdown string

  * `name` - The enum name (may differ from item.name for re-exports)

  * `e` - The enum data from rustdoc

  * `krate` - The crate containing variant definitions

  * `type_renderer` - Type renderer for generics and variant types

- <span id="rendererinternals-render-enum-variant"></span>`fn render_enum_variant(md: &mut String, variant: &Item, krate: &Crate, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render a single enum variant within the definition code block.

  

  Handles all three variant kinds: plain, tuple, and struct variants.

- <span id="rendererinternals-render-enum-variants-docs"></span>`fn render_enum_variants_docs<F>(md: &mut String, variants: &[Id], krate: &Crate, process_docs: F)`

  Render documented enum variants to markdown.

  

  Produces a "Variants" section with each documented variant as a bullet point.

  

  # Arguments

  

  * `md` - Output markdown string

  * `variants` - Variant IDs from the enum

  * `krate` - Crate containing variant definitions

  * `process_docs` - Closure to process documentation with intra-doc link resolution

- <span id="rendererinternals-render-function-definition"></span>`fn render_function_definition(md: &mut String, name: &str, f: &rustdoc_types::Function, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render a function definition to markdown.

  

  Produces a heading with the function name, followed by a Rust code block

  showing the full signature with modifiers (const, async, unsafe).

  

  # Arguments

  

  * `md` - Output markdown string

  * `name` - The function name

  * `f` - The function data from rustdoc

  * `type_renderer` - Type renderer for parameter and return types

- <span id="rendererinternals-render-constant-definition"></span>`fn render_constant_definition(md: &mut String, name: &str, type_: &rustdoc_types::Type, const_: &rustdoc_types::Constant, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render a constant definition to markdown.

  

  Produces a heading with the constant name, followed by a Rust code block

  showing `const NAME: Type = value;`.

  

  # Arguments

  

  * `md` - Output markdown string

  * `name` - The constant name

  * `type_` - The constant's type

  * `const_` - The constant data including value

  * `type_renderer` - Type renderer for the type

- <span id="rendererinternals-render-type-alias-definition"></span>`fn render_type_alias_definition(md: &mut String, name: &str, ta: &rustdoc_types::TypeAlias, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render a type alias definition to markdown.

  

  Produces a heading with the alias name and generics, followed by a Rust

  code block showing `type Name<T> = TargetType;`.

  

  # Arguments

  

  * `md` - Output markdown string

  * `name` - The type alias name

  * `ta` - The type alias data from rustdoc

  * `type_renderer` - Type renderer for generics and the aliased type

- <span id="rendererinternals-render-macro-heading"></span>`fn render_macro_heading(md: &mut String, name: &str)`

  Render a macro definition to markdown.

  

  Produces a heading with the macro name and `!` suffix.

  Note: We don't show macro rules since rustdoc JSON doesn't provide them.

  

  # Arguments

  

  * `md` - Output markdown string

  * `name` - The macro name

- <span id="rendererinternals-render-impl-items"></span>`fn render_impl_items<F, L>(md: &mut String, impl_block: &Impl, krate: &Crate, type_renderer: &TypeRenderer<'_>, process_docs: &Option<F>, create_type_link: &Option<L>, parent_type_name: Option<&str>, impl_ctx: ImplContext<'_>, full_method_docs: bool)` — [`TypeRenderer`](../../types/index.md#typerenderer), [`ImplContext`](../../linker/index.md#implcontext)

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

  * `impl_ctx` - Context for anchor generation (inherent vs trait impl)

- <span id="rendererinternals-extract-method-summary"></span>`fn extract_method_summary(docs: &str, full_method_docs: bool) -> String`

  Extract method documentation summary for impl blocks.

  

  The extraction strategy is:

  1. If `full_method_docs` is true, return the entire documentation

  2. If the docs contain code examples (triple-backtick blocks), return full docs to preserve them

  3. Otherwise, extract just the first paragraph (lines until first blank line)

  

  This ensures important code examples are never lost while keeping summaries

  concise for methods without examples.

- <span id="rendererinternals-render-function-type-links-inline"></span>`fn render_function_type_links_inline<L>(md: &mut String, f: &rustdoc_types::Function, type_renderer: TypeRenderer<'_>, create_link: &L)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render type links for a function signature inline (for impl methods).

  

  This is a helper that collects types from function signatures and

  creates links for resolvable types, outputting them on the same line.

- <span id="rendererinternals-render-impl-function"></span>`fn render_impl_function(md: &mut String, name: &str, f: &rustdoc_types::Function, type_renderer: TypeRenderer<'_>, parent_type_name: Option<&str>, impl_ctx: ImplContext<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer), [`ImplContext`](../../linker/index.md#implcontext)

  Render a function signature within an impl block.

  

  Renders as a bullet point with the full signature including modifiers.

  If `parent_type_name` is provided, includes a hidden anchor for deep linking.

  The `impl_ctx` parameter ensures unique anchors when the same method name

  appears in multiple trait implementations (e.g., `fmt` in Debug and Display).

- <span id="rendererinternals-append-docs"></span>`fn append_docs(md: &mut String, docs: Option<String>)`

  Append processed documentation to markdown.

  

  Helper function to add documentation with consistent formatting.

- <span id="rendererinternals-render-collapsible-start"></span>`fn render_collapsible_start(summary: &str) -> String`

  Render the opening of a collapsible `<details>` block with a summary.

  

  Produces HTML that creates a collapsible section in markdown. Use with

  `render_collapsible_end` to close the block.

  

  # Arguments

  

  * `summary` - The text to display in the summary line (clickable header)

  

  # Example

  

  ```rust

  use cargo_docs_md::generator::render_shared::RendererInternals;

  

  let start = RendererInternals::render_collapsible_start("Derived Traits (9 implementations)");

  assert!(start.contains("<details>"));

  assert!(start.contains("<summary>Derived Traits (9 implementations)</summary>"));

  ```

- <span id="rendererinternals-render-collapsible-end"></span>`const fn render_collapsible_end() -> &'static str`

  Render the closing of a collapsible `<details>` block.

  

  Returns a static string to close a block opened with `render_collapsible_start`.

  

  # Example

  

  ```rust

  use cargo_docs_md::generator::render_shared::RendererInternals;

  

  assert_eq!(RendererInternals::render_collapsible_end(), "\n</details>\n\n");

  ```

- <span id="rendererinternals-impl-sort-key"></span>`fn impl_sort_key(impl_block: &Impl, type_renderer: &TypeRenderer<'_>) -> String` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Generate a sort key for an impl block for deterministic ordering.

  

  Combines trait name, generic params, and for-type to create a unique key.

- <span id="rendererinternals-render-source-location"></span>`fn render_source_location(span: Option<&Span>, source_path_config: Option<&SourcePathConfig>) -> String` — [`SourcePathConfig`](#sourcepathconfig)

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

- <span id="rendererinternals-render-union-definition"></span>`fn render_union_definition(md: &mut String, name: &str, u: &rustdoc_types::Union, krate: &Crate, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render a union definition code block to markdown.

  

  Produces a heading with the union name and generics, followed by a Rust

  code block showing the union definition with all fields.

  

  # Arguments

  

  * `md` - Output markdown string

  * `name` - The union name (may differ from item.name for re-exports)

  * `u` - The union data from rustdoc

  * `krate` - The crate containing field definitions

  * `type_renderer` - Type renderer for generics and field types

- <span id="rendererinternals-render-union-fields"></span>`fn render_union_fields<F>(md: &mut String, fields: &[Id], krate: &Crate, type_renderer: &TypeRenderer<'_>, process_docs: F)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render union fields documentation.

  

  Creates a "Fields" section with each field's name, type, and documentation.

  Only renders if at least one field has documentation.

  

  # Arguments

  

  * `md` - Output markdown string

  * `fields` - Field IDs from the union

  * `krate` - The crate containing field definitions

  * `type_renderer` - Type renderer for field types

  * `process_docs` - Callback to process documentation strings

- <span id="rendererinternals-render-static-definition"></span>`fn render_static_definition(md: &mut String, name: &str, s: &rustdoc_types::Static, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Render a static definition code block to markdown.

  

  Produces a heading with the static name, followed by a Rust

  code block showing the static definition.

  

  # Arguments

  

  * `md` - Output markdown string

  * `name` - The static name (may differ from item.name for re-exports)

  * `s` - The static data from rustdoc

  * `type_renderer` - Type renderer for the static's type

#### Trait Implementations

##### `impl Any for RendererInternals`

- <span id="rendererinternals-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RendererInternals`

- <span id="rendererinternals-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RendererInternals`

- <span id="rendererinternals-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for RendererInternals`

- <span id="rendererinternals-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for RendererInternals`

##### `impl<U> Into for RendererInternals`

- <span id="rendererinternals-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for RendererInternals`

##### `impl OwoColorize for RendererInternals`

##### `impl Pointable for RendererInternals`

- <span id="rendererinternals-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rendererinternals-pointable-type-init"></span>`type Init = T`

- <span id="rendererinternals-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rendererinternals-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rendererinternals-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rendererinternals-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for RendererInternals`

- <span id="rendererinternals-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rendererinternals-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RendererInternals`

- <span id="rendererinternals-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rendererinternals-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for RendererInternals`

## Traits

### `DocsProcessor`

```rust
trait DocsProcessor { ... }
```

*Defined in `src/generator/render_shared.rs:1385-1388`*

Check if a render context can resolve documentation.

This trait provides a unified way to process docs from different contexts.

#### Required Methods

- `fn process_item_docs(&self, item: &Item) -> Option<String>`

  Process documentation for an item, resolving intra-doc links.

#### Implementors

- `(&T, &str)`

