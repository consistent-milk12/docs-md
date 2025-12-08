*[docs_md](../../index.md) / [multi_crate](../index.md) / [context](index.md)*

---

# Module `context`

Multi-crate generation context.

This module provides [`MultiCrateContext`](../../index.md) which holds shared state
during multi-crate documentation generation, and [`SingleCrateView`](../index.md)
which provides a single-crate interface for existing rendering code.

## Structs

### `MultiCrateContext<'a>`

```rust
struct MultiCrateContext<'a> {
    crates: &'a crate::multi_crate::CrateCollection,
    registry: crate::multi_crate::UnifiedLinkRegistry,
    args: &'a crate::Args,
    cross_crate_impls: std::collections::HashMap<String, std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>,
}
```

Shared context for multi-crate documentation generation.

Holds references to all crates, the unified link registry, and
CLI configuration. Used by [`MultiCrateGenerator`](../../index.md) to coordinate
generation across crates.


#### Fields

- **`crates`**: `&'a crate::multi_crate::CrateCollection`

  All crates being documented.

- **`registry`**: `crate::multi_crate::UnifiedLinkRegistry`

  Unified link registry for cross-crate resolution.

- **`args`**: `&'a crate::Args`

  CLI arguments.

- **`cross_crate_impls`**: `std::collections::HashMap<String, std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>`

  Pre-computed cross-crate impl blocks.
  
  Maps target crate name -> type name -> impl blocks from other crates.
  This is computed once during construction rather than per-view.

#### Implementations

- `fn new(crates: &'a CrateCollection, args: &'a Args) -> Self` — [`CrateCollection`](../../index.md), [`Args`](../../index.md)

- `fn build_cross_crate_impls(crates: &'a CrateCollection) -> HashMap<String, HashMap<String, Vec<&'a Impl>>>` — [`CrateCollection`](../../index.md)

- `const fn crates(self: &Self) -> &CrateCollection` — [`CrateCollection`](../../index.md)

- `const fn registry(self: &Self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../../index.md)

- `const fn args(self: &Self) -> &Args` — [`Args`](../../index.md)

- `fn single_crate_view(self: &'a Self, crate_name: &str) -> Option<SingleCrateView<'a>>` — [`SingleCrateView`](../index.md)

- `fn find_item(self: &Self, id: &Id) -> Option<(&str, &Item)>`

- `fn get_cross_crate_impls(self: &Self, target_crate: &str) -> Option<&HashMap<String, Vec<&'a Impl>>>`

- `fn get_impl_target_path(impl_block: &Impl) -> Option<String>`

#### Trait Implementations

##### `impl<T> Instrument for MultiCrateContext<'a>`

##### `impl<T> IntoEither for MultiCrateContext<'a>`

##### `impl<D> OwoColorize for MultiCrateContext<'a>`

##### `impl<T> Pointable for MultiCrateContext<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for MultiCrateContext<'a>`

### `SingleCrateView<'a>`

```rust
struct SingleCrateView<'a> {
    crate_name: &'a str,
    krate: &'a rustdoc_types::Crate,
    registry: &'a crate::multi_crate::UnifiedLinkRegistry,
    args: &'a crate::Args,
    ctx: &'a MultiCrateContext<'a>,
    impl_map: std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>,
    cross_crate_impls: Option<&'a std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>,
    type_name_to_id: std::collections::HashMap<String, rustdoc_types::Id>,
}
```

View of a single crate within multi-crate context.

Provides an interface similar to [`GeneratorContext`](../../generator/index.md) but uses
[`UnifiedLinkRegistry`](../../index.md) for cross-crate link resolution. This
allows existing rendering code to work with minimal changes.


#### Fields

- **`crate_name`**: `&'a str`

  Name of this crate (borrowed from the context).

- **`krate`**: `&'a rustdoc_types::Crate`

  The crate being rendered.

- **`registry`**: `&'a crate::multi_crate::UnifiedLinkRegistry`

  Unified registry for link resolution.

- **`args`**: `&'a crate::Args`

  CLI arguments.

- **`ctx`**: `&'a MultiCrateContext<'a>`

  Reference to the parent multi-crate context for cross-crate lookups.

- **`impl_map`**: `std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>`

  Map from type ID to impl blocks (local crate only).

- **`cross_crate_impls`**: `Option<&'a std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>`

  Reference to pre-computed cross-crate impl blocks from context.
  Maps type name to impl blocks from other crates.

- **`type_name_to_id`**: `std::collections::HashMap<String, rustdoc_types::Id>`

  Map from type name to type ID for cross-crate impl lookup.

#### Implementations

- `fn new(crate_name: &'a str, krate: &'a Crate, registry: &'a UnifiedLinkRegistry, args: &'a Args, ctx: &'a MultiCrateContext<'a>) -> Self` — [`UnifiedLinkRegistry`](../../index.md), [`Args`](../../index.md), [`MultiCrateContext`](../../index.md)

- `fn build_impl_map(self: &mut Self)`

- `fn build_type_name_map(self: &mut Self)`

- `const fn get_impl_target_id(impl_block: &Impl) -> Option<Id>`

- `fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

- `const fn crate_name(self: &Self) -> &str`

- `const fn krate(self: &Self) -> &Crate`

- `const fn registry(self: &Self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../../index.md)

- `const fn args(self: &Self) -> &Args` — [`Args`](../../index.md)

- `fn get_impls(self: &Self, id: Id) -> Option<&Vec<&'a Impl>>`

- `fn get_all_impls(self: &Self, id: Id) -> Vec<&'a Impl>`

- `fn get_impls_from_crate(self: &Self, id: Id, source_krate: &'a Crate) -> Vec<&'a Impl>`

- `const fn get_impl_target_id_from_type(ty: &rustdoc_types::Type) -> Option<Id>`

- `const fn should_include_item(self: &Self, item: &rustdoc_types::Item) -> bool`

- `fn count_modules(self: &Self) -> usize`

- `fn create_link(self: &Self, to_crate: &str, to_id: Id, from_path: &str) -> Option<String>`

- `fn resolve_name(self: &Self, name: &str) -> Option<(String, Id)>`

- `fn lookup_item_across_crates(self: &Self, id: &Id) -> Option<(&str, &Item)>`

- `fn get_crate(self: &Self, name: &str) -> Option<&Crate>`

- `fn resolve_external_path(self: &Self, path: &str) -> Option<(&str, &Item, Id)>`

- `fn process_backtick_links(self: &Self, docs: &str, item_links: &HashMap<String, Id>, current_file: &str) -> String`

- `fn process_plain_links(self: &Self, docs: &str, current_file: &str) -> String`

- `fn resolve_plain_link(self: &Self, link_text: &str, current_file: &str) -> Option<String>`

- `fn resolve_link(self: &Self, link_text: &str, item_links: &HashMap<String, Id>, current_file: &str) -> Option<String>`

- `fn build_link_to_id(self: &Self, id: Id, current_file: &str, display_name: &str, anchor: Option<&str>) -> Option<String>`

- `fn resolve_crate_path(self: &Self, path_without_crate: &str, display_name: &str, current_file: &str) -> Option<String>`

- `fn split_type_and_anchor(path: &str) -> (&str, Option<&str>)`

- `fn build_markdown_link(self: &Self, current_file: &str, target_crate: &str, target_path: &str, display_name: &str, anchor: Option<&str>) -> String`

- `fn compute_cross_crate_path(current_local: &str, target_crate: &str, target_path: &str) -> String`

- `fn strip_crate_prefix(path: &str) -> &str`

- `fn looks_like_external_reference(link_text: &str) -> bool`

#### Trait Implementations

##### `impl<T> Instrument for SingleCrateView<'a>`

##### `impl<T> IntoEither for SingleCrateView<'a>`

##### `impl ItemAccess for SingleCrateView<'_>`

- `fn krate(self: &Self) -> &Crate`

- `fn crate_name(self: &Self) -> &str`

- `fn get_item(self: &Self, id: &Id) -> Option<&Item>`

- `fn get_impls(self: &Self, id: &Id) -> Option<&[&Impl]>`

- `fn crate_version(self: &Self) -> Option<&str>`

##### `impl ItemFilter for SingleCrateView<'_>`

- `fn should_include_item(self: &Self, item: &Item) -> bool`

- `fn include_private(self: &Self) -> bool`

- `fn include_blanket_impls(self: &Self) -> bool`

##### `impl LinkResolver for SingleCrateView<'_>`

- `fn link_registry(self: &Self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../../index.md)

- `fn process_docs(self: &Self, item: &Item, current_file: &str) -> Option<String>`

- `fn create_link(self: &Self, id: Id, current_file: &str) -> Option<String>`

##### `impl<D> OwoColorize for SingleCrateView<'a>`

##### `impl<T> Pointable for SingleCrateView<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> RenderContext for SingleCrateView<'a>`

##### `impl<T> WithSubscriber for SingleCrateView<'a>`

