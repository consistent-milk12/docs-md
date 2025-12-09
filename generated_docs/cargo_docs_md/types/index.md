*[cargo_docs_md](../index.md) / [types](index.md)*

---

# Module `types`

Type rendering utilities for converting rustdoc types to string representations.

This module provides the [`TypeRenderer`](#typerenderer) struct to convert the complex type
structures from rustdoc JSON into human-readable Rust type syntax. These
rendered strings are used in the generated markdown documentation.

# Overview

Rustdoc JSON represents types as a tree structure (the `Type` enum). The
[`TypeRenderer`](#typerenderer) walks that tree and produces the string representation
you'd write in code.

# Usage

```ignore
let renderer = TypeRenderer::new(&krate);
let type_string = renderer.render_type(&some_type);
let generics = renderer.render_generics(&generic_params);
```

# Example Transformations

| Rustdoc Type | Rendered String |
|--------------|-----------------|
| `Type::Primitive("u32")` | `"u32"` |
| `Type::BorrowedRef { lifetime: Some("'a"), is_mutable: true, type_: ... }` | `"&'a mut T"` |
| `Type::ResolvedPath { path: "Vec", args: ... }` | `"Vec<T>"` |

# Performance

Uses `Cow<str>` to avoid allocations for simple types like primitives,
generics, and inferred types. Complex types that require string building
return owned strings.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TypeRenderer`](#typerenderer) | struct | Type renderer for converting rustdoc types to Rust syntax strings. |

## Structs

### `TypeRenderer<'a>`

```rust
struct TypeRenderer<'a> {
    krate: &'a rustdoc_types::Crate,
}
```

*Defined in `src/types.rs:65-71`*

Type renderer for converting rustdoc types to Rust syntax strings.

This struct holds a reference to the crate context and provides methods
to render various type constructs into their string representations.

# Design Note

The `krate` field is currently unused because the `Type` enum is self-contained.
However, it is retained for:
- **Future-proofing**: May need to look up items in `krate.index` for enhanced rendering
- **API consistency**: Signals that the renderer is bound to a specific crate context
- **Type safety**: Prevents accidentally mixing renderers across different crates

# Example

```ignore
let renderer = TypeRenderer::new(&krate);
let type_str = renderer.render_type(&some_type);
let generics = renderer.render_generics(&params);
```

#### Fields

- **`krate`**: `&'a rustdoc_types::Crate`

  Reference to the crate for looking up type information.
  
  Reserved for future use (e.g., resolving paths, getting item metadata).

#### Implementations

- <span id="typerenderer-new"></span>`const fn new(krate: &'a Crate) -> Self`

- <span id="typerenderer-get-type-id"></span>`const fn get_type_id(&self, ty: &Type) -> Option<Id>`

- <span id="typerenderer-render-type"></span>`fn render_type<'t>(&self, ty: &'t Type) -> Cow<'t, str>`

- <span id="typerenderer-render-generic-args"></span>`fn render_generic_args(self, args: &GenericArgs) -> String`

- <span id="typerenderer-render-generic-arg"></span>`fn render_generic_arg(self, arg: &GenericArg) -> Cow<'_, str>`

- <span id="typerenderer-render-assoc-item-constraint"></span>`fn render_assoc_item_constraint(self, constraint: &AssocItemConstraint) -> String`

- <span id="typerenderer-render-term"></span>`fn render_term(self, term: &Term) -> Cow<'_, str>`

- <span id="typerenderer-render-generic-bound"></span>`fn render_generic_bound<'t>(&self, bound: &'t GenericBound) -> Cow<'t, str>`

- <span id="typerenderer-render-generics"></span>`fn render_generics(&self, generics: &[GenericParamDef]) -> String`

- <span id="typerenderer-render-generic-param-def"></span>`fn render_generic_param_def(self, param: &GenericParamDef) -> Option<String>`

- <span id="typerenderer-render-where-clause"></span>`fn render_where_clause(&self, where_predicates: &[rustdoc_types::WherePredicate]) -> String`

- <span id="typerenderer-render-where-predicate"></span>`fn render_where_predicate(self, pred: &rustdoc_types::WherePredicate) -> String`

- <span id="typerenderer-collect-linkable-types"></span>`fn collect_linkable_types(&self, ty: &Type) -> Vec<(String, rustdoc_types::Id)>`

- <span id="typerenderer-collect-types-recursive"></span>`fn collect_types_recursive(self, ty: &Type, result: &mut Vec<(String, rustdoc_types::Id)>)`

- <span id="typerenderer-collect-from-generic-args"></span>`fn collect_from_generic_args(self, args: &GenericArgs, result: &mut Vec<(String, rustdoc_types::Id)>)`

#### Trait Implementations

##### `impl Clone for TypeRenderer<'a>`

- <span id="typerenderer-clone"></span>`fn clone(&self) -> TypeRenderer<'a>` — [`TypeRenderer`](#typerenderer)

##### `impl Copy for TypeRenderer<'a>`

##### `impl Debug for TypeRenderer<'a>`

- <span id="typerenderer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for TypeRenderer<'a>`

##### `impl IntoEither for TypeRenderer<'a>`

##### `impl OwoColorize for TypeRenderer<'a>`

##### `impl Pointable for TypeRenderer<'a>`

- <span id="typerenderer-const-align"></span>`const ALIGN: usize`

- <span id="typerenderer-type-init"></span>`type Init = T`

- <span id="typerenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="typerenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="typerenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="typerenderer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TypeRenderer<'a>`

