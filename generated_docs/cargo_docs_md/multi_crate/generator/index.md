*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [generator](index.md)*

---

# Module `generator`

Multi-crate documentation generator.

This module provides [`MultiCrateGenerator`](../../index.md) which orchestrates
documentation generation across multiple crates with cross-crate linking.

## Structs

### `MultiCrateGenerator<'a>`

```rust
struct MultiCrateGenerator<'a> {
    ctx: crate::multi_crate::MultiCrateContext<'a>,
    args: &'a crate::Args,
}
```

Generator for multi-crate documentation.

Produces a directory structure with one subdirectory per crate,
each containing nested markdown files with cross-crate linking.

# Output Structure

```text
output/
├── tracing/
│   ├── index.md
│   └── span/
│       └── index.md
├── tracing_core/
│   ├── index.md
│   └── subscriber/
│       └── index.md
└── SUMMARY.md        # If --mdbook enabled
```

#### Fields

- **`ctx`**: `crate::multi_crate::MultiCrateContext<'a>`

  Multi-crate context with unified registry.

- **`args`**: `&'a crate::Args`

  CLI arguments.

#### Implementations

- `fn new(crates: &'a CrateCollection, args: &'a Args) -> Self` — [`CrateCollection`](../../index.md), [`Args`](../../index.md)

- `fn generate(self: &Self) -> Result<(), Error>` — [`Error`](../../error/index.md)

- `fn collect_rendered_items(self: &Self) -> HashMap<String, HashSet<Id>>`

- `fn collect_crate_items(view: &SingleCrateView<'_>, ids: &mut HashSet<Id>)` — [`SingleCrateView`](../index.md)

- `fn collect_module_items(view: &SingleCrateView<'_>, item: &Item, ids: &mut HashSet<Id>)` — [`SingleCrateView`](../index.md)

- `fn generate_crate(self: &Self, view: &SingleCrateView<'_>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](../index.md), [`Error`](../../error/index.md)

- `fn generate_module(view: &SingleCrateView<'_>, item: &Item, parent_dir: &Path, module_path: Vec<String>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](../index.md), [`Error`](../../error/index.md)

- `fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](../../error/index.md)

#### Trait Implementations

##### `impl<T> Instrument for MultiCrateGenerator<'a>`

##### `impl<T> IntoEither for MultiCrateGenerator<'a>`

##### `impl<D> OwoColorize for MultiCrateGenerator<'a>`

##### `impl<T> Pointable for MultiCrateGenerator<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for MultiCrateGenerator<'a>`

### `MultiCrateModuleRenderer<'a>`

```rust
struct MultiCrateModuleRenderer<'a> {
    view: &'a crate::multi_crate::context::SingleCrateView<'a>,
    file_path: &'a str,
    is_root: bool,
    type_renderer: crate::types::TypeRenderer<'a>,
}
```

Module renderer for multi-crate context.

Wraps the standard module rendering with multi-crate link resolution.

This renderer handles special cases that aren't covered by the standard
`ModuleRenderer`, particularly re-exports (`pub use`) which need to
resolve items across crate boundaries.

#### Fields

- **`view`**: `&'a crate::multi_crate::context::SingleCrateView<'a>`

  Single-crate view for this crate (implements `RenderContext`).

- **`file_path`**: `&'a str`

  Current file path for link resolution.

- **`is_root`**: `bool`

  Whether this is the crate root.

- **`type_renderer`**: `crate::types::TypeRenderer<'a>`

  Cached type renderer to avoid repeated construction.

#### Implementations

- `const fn new(view: &'a SingleCrateView<'a>, file_path: &'a str, is_root: bool) -> Self` — [`SingleCrateView`](../index.md)

- `fn render(self: &Self, item: &Item) -> String`

- `fn render_module_contents(self: &Self, md: &mut String, module: &rustdoc_types::Module, _parent: &Item)`

- `fn render_modules_section(md: &mut String, modules: &[&Item])`

- `fn render_structs_section(self: &Self, md: &mut String, structs: &[(&Id, &Item)])`

- `fn render_enums_section(self: &Self, md: &mut String, enums: &[(&Id, &Item)])`

- `fn render_traits_section(self: &Self, md: &mut String, traits: &[&Item])`

- `fn render_functions_section(self: &Self, md: &mut String, functions: &[&Item])`

- `fn render_type_aliases_section(self: &Self, md: &mut String, types: &[&Item])`

- `fn render_constants_section(self: &Self, md: &mut String, constants: &[&Item])`

- `fn render_macros_section(self: &Self, md: &mut String, macros: &[&Item])`

- `fn get_item_name_and_summary(item: &Item) -> (String, String)`

- `fn render_struct(self: &Self, md: &mut String, item_id: Id, item: &Item)`

- `fn render_enum(self: &Self, md: &mut String, item_id: Id, item: &Item)`

- `fn render_trait(self: &Self, md: &mut String, item: &Item)`

- `fn render_function(self: &Self, md: &mut String, item: &Item)`

- `fn render_constant(self: &Self, md: &mut String, item: &Item)`

- `fn render_type_alias(self: &Self, md: &mut String, item: &Item)`

- `fn render_macro(self: &Self, md: &mut String, item: &Item)`

- `fn expand_glob_reexport<'b>(self: &Self, modules: &mut Vec<&'b Item>, structs: &mut Vec<(&'b Id, &'b Item)>, enums: &mut Vec<(&'b Id, &'b Item)>, traits: &mut Vec<&'b Item>, functions: &mut Vec<&'b Item>, types: &mut Vec<&'b Item>, constants: &mut Vec<&'b Item>, macros: &mut Vec<&'b Item>, use_item: &rustdoc_types::Use, seen_items: &mut HashSet<Id>)`

- `fn render_impl_blocks(self: &Self, md: &mut String, item_id: Id, source_crate_name: Option<&str>)`

#### Trait Implementations

##### `impl<T> Instrument for MultiCrateModuleRenderer<'a>`

##### `impl<T> IntoEither for MultiCrateModuleRenderer<'a>`

##### `impl<D> OwoColorize for MultiCrateModuleRenderer<'a>`

##### `impl<T> Pointable for MultiCrateModuleRenderer<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for MultiCrateModuleRenderer<'a>`

