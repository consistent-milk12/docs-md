*[cargo_docs_md](../../index.md) / [generator](../index.md) / [items](index.md)*

---

# Module `items`

Item rendering for documentation generation.

This module provides the [`ItemRenderer`](#itemrenderer) struct which handles rendering
individual Rust items (structs, enums, traits, functions, macros, constants,
and type aliases) to markdown format.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ItemRenderer`](#itemrenderer) | struct | Renders individual Rust items to markdown. |

## Structs

### `ItemRenderer<'a>`

```rust
struct ItemRenderer<'a> {
    ctx: &'a dyn RenderContext,
    current_file: &'a str,
    type_renderer: crate::types::TypeRenderer<'a>,
}
```

*Defined in `src/generator/items.rs:29-39`*

Renders individual Rust items to markdown.

This struct provides methods for rendering each type of documentable item:
- Structs with fields and implementations
- Enums with variants and implementations
- Traits with methods and associated types
- Functions with signatures
- Macros
- Constants
- Type aliases

The renderer is generic over [`RenderContext`](../context/index.md), allowing it to work with
both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.

#### Fields

- **`ctx`**: `&'a dyn RenderContext`

  Reference to the render context (either single-crate or multi-crate).

- **`current_file`**: `&'a str`

  Path of the current file being generated (for relative link calculation).

- **`type_renderer`**: `crate::types::TypeRenderer<'a>`

  Type renderer for converting rustdoc types to strings.
  Stored once to avoid redundant construction in each render method.

#### Implementations

- <span id="itemrenderer-new"></span>`fn new(ctx: &'a dyn RenderContext, current_file: &'a str) -> Self` — [`RenderContext`](../context/index.md#rendercontext)

  Create a new item renderer with the given context.

  

  # Arguments

  

  * `ctx` - Render context (implements `RenderContext` trait)

  * `current_file` - Path of the current file (for relative link calculation)

- <span id="itemrenderer-process-docs"></span>`fn process_docs(&self, item: &Item) -> Option<String>`

  Process documentation string to resolve intra-doc links.

  

  Delegates to the render context's `process_docs` method, which handles

  both single-crate and multi-crate link resolution.

- <span id="itemrenderer-maybe-render-source-location"></span>`fn maybe_render_source_location(&self, item: &Item) -> String`

  Render source location if enabled in config.

  

  Returns the source location string if `source_locations` is enabled,

  otherwise returns an empty string. Uses the source path config to

  generate clickable links to the `.source_*` directory when available.

- <span id="itemrenderer-resolve-use-target"></span>`fn resolve_use_target<'b>(&self, item: &'b Item) -> Option<(&'b str, &'b Item)>`

  Resolve a Use item to its target, returning the alias name and target item.

  

  For non-Use items, returns the item's own name and itself.

  For Use items, resolves the target and returns the alias name with the target.

- <span id="itemrenderer-render-struct"></span>`fn render_struct(&self, md: &mut String, item_id: Id, item: &Item)`

  Render a struct definition to markdown.

  

  Produces a section with:

  - Heading with struct name and generics

  - Rust code block showing the struct definition

  - Documentation from doc comments

  - Fields section (for structs with documented fields)

  - Implementations section (inherent and trait impls)

  

  # Struct Kinds

  

  Rust has three kinds of structs, each rendered differently:

  - **Unit**: `struct Foo;`

  - **Tuple**: `struct Foo(T, U);`

  - **Plain** (named fields): `struct Foo { field: T }`

  

  # Re-exports

  

  Also handles `pub use` re-exports where the item is a Use pointing to a struct.

- <span id="itemrenderer-render-enum"></span>`fn render_enum(&self, md: &mut String, item_id: Id, item: &Item)`

  Render an enum definition to markdown.

  

  Produces a section with:

  - Heading with enum name and generics

  - Rust code block showing the enum definition with all variants

  - Documentation from doc comments

  - Variants section (for variants with documentation)

  - Implementations section (inherent and trait impls)

  

  # Variant Kinds

  

  Rust enums support three variant kinds:

  - **Plain**: `Variant` (no data)

  - **Tuple**: `Variant(T, U)` (positional data)

  - **Struct**: `Variant { field: T }` (named fields)

  

  # Re-exports

  

  Also handles `pub use` re-exports where the item is a Use pointing to an enum.

- <span id="itemrenderer-render-trait"></span>`fn render_trait(&self, md: &mut String, item_id: Id, item: &Item)`

  Render a trait definition to markdown.

  

  Produces a section with:

  - Heading with trait name and generics

  - Rust code block showing trait signature with supertraits

  - Documentation from doc comments

  - Associated Types section (if any)

  - Associated Constants section (if any)

  - Required Methods section (methods without default impl)

  - Provided Methods section (methods with default impl)

  - Implementors section (types that implement this trait)

  

  # Re-exports

  

  Also handles `pub use` re-exports where the item is a Use pointing to a trait.

- <span id="itemrenderer-render-trait-implementors"></span>`fn render_trait_implementors(&self, md: &mut String, trait_id: Id)`

  Render the implementors section for a trait.

  

  Uses `Trait.implementations` field for direct lookup instead of scanning

  all items in the crate index, providing O(k) performance where k is the

  number of implementors.

- <span id="itemrenderer-render-function"></span>`fn render_function(&self, md: &mut String, item: &Item)`

  Render a standalone function to markdown.

  

  Produces a section with:

  - Heading with function name

  - Rust code block showing full signature

  - Type links for parameter and return types (when resolvable)

  - Documentation from doc comments

  

  # Function Modifiers

  

  The signature includes applicable modifiers:

  - `const fn` - Compile-time evaluable

  - `async fn` - Returns a Future

  - `unsafe fn` - Requires unsafe block to call

  

  # Re-exports

  

  Also handles `pub use` re-exports where the item is a Use pointing to a function.

- <span id="itemrenderer-render-function-type-links"></span>`fn render_function_type_links(&self, md: &mut String, f: &rustdoc_types::Function)`

  Render linked types used in a function signature.

  

  Collects all types from parameters and return type, creates links

  for resolvable types, and outputs a "**Types:**" line.

- <span id="itemrenderer-render-macro"></span>`fn render_macro(&self, md: &mut String, item: &Item)`

  Render a macro definition to markdown.

  

  Produces a section with:

  - Heading with macro name and `!` suffix

  - Documentation from doc comments

  

  Note: We don't show macro rules/implementation since rustdoc JSON

  doesn't provide the full macro definition, only metadata.

  

  # Re-exports

  

  Also handles `pub use` re-exports where the item is a Use pointing to a macro.

- <span id="itemrenderer-render-constant"></span>`fn render_constant(&self, md: &mut String, item: &Item)`

  Render a constant definition to markdown.

  

  Produces a section with:

  - Heading with constant name

  - Rust code block showing `const NAME: Type = value;`

  - Documentation from doc comments

  

  The value may be omitted if rustdoc couldn't determine it

  (e.g., for complex const expressions).

  

  # Re-exports

  

  Also handles `pub use` re-exports where the item is a Use pointing to a constant.

- <span id="itemrenderer-render-type-alias"></span>`fn render_type_alias(&self, md: &mut String, item: &Item)`

  Render a type alias to markdown.

  

  Produces a section with:

  - Heading with alias name and generics

  - Rust code block showing `type Name<T> = TargetType;`

  - Documentation from doc comments

  

  # Re-exports

  

  Also handles `pub use` re-exports where the item is a Use pointing to a type alias.

- <span id="itemrenderer-render-union"></span>`fn render_union(&self, md: &mut String, item_id: Id, item: &Item)`

  Render a union definition to markdown.

  

  Produces a section with:

  - Heading with union name and generics

  - Rust code block showing the union definition with all fields

  - Documentation from doc comments

  - Fields section (for unions with documented fields)

  - Implementations section (inherent and trait impls)

  

  # Union Fields

  

  Unlike structs, all union fields share the same memory location.

  Only one field can be active at a time, and accessing fields

  requires `unsafe` code.

  

  # Re-exports

  

  Also handles `pub use` re-exports where the item is a Use pointing to a union.

- <span id="itemrenderer-render-static"></span>`fn render_static(&self, md: &mut String, item: &Item)`

  Render a static variable to markdown.

  

  Produces a section with:

  - Heading with static name

  - Rust code block showing `static [mut] NAME: Type = expr;`

  - Documentation from doc comments

  

  # Static Modifiers

  

  The signature includes applicable modifiers:

  - `static mut` - Mutable static (access requires unsafe)

  - `unsafe static` - Unsafe static in extern block

  

  # Re-exports

  

  Also handles `pub use` re-exports where the item is a Use pointing to a static.

#### Trait Implementations

##### `impl Any for ItemRenderer<'a>`

- <span id="itemrenderer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemRenderer<'a>`

- <span id="itemrenderer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemRenderer<'a>`

- <span id="itemrenderer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ItemRenderer<'a>`

- <span id="itemrenderer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for ItemRenderer<'a>`

##### `impl<U> Into for ItemRenderer<'a>`

- <span id="itemrenderer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ItemRenderer<'a>`

##### `impl OwoColorize for ItemRenderer<'a>`

##### `impl Pointable for ItemRenderer<'a>`

- <span id="itemrenderer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itemrenderer-pointable-type-init"></span>`type Init = T`

- <span id="itemrenderer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itemrenderer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itemrenderer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itemrenderer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ItemRenderer<'a>`

- <span id="itemrenderer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemrenderer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemRenderer<'a>`

- <span id="itemrenderer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemrenderer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for ItemRenderer<'a>`

