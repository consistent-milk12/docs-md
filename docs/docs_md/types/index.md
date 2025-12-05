*[docs_md](../index.md) / [types](index.md)*

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

## Structs

### `TypeRenderer<'a>`

```rust
struct TypeRenderer<'a> {
    krate: &'a rustdoc_types::Crate,
}
```

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

- `const fn new(krate: &'a Crate) -> Self`

- `fn render_type<'t>(self: &Self, ty: &'t Type) -> Cow<'t, str>`

- `fn render_generic_args(self: &Self, args: &GenericArgs) -> String`

- `fn render_generic_arg<'t>(self: &Self, arg: &'t GenericArg) -> Cow<'t, str>`

- `fn render_assoc_item_constraint(self: &Self, constraint: &AssocItemConstraint) -> String`

- `fn render_term<'t>(self: &Self, term: &'t Term) -> Cow<'t, str>`

- `fn render_generic_bound<'t>(self: &Self, bound: &'t GenericBound) -> Cow<'t, str>`

- `fn render_generics(self: &Self, generics: &[GenericParamDef]) -> String`

- `fn render_generic_param_def(self: &Self, param: &GenericParamDef) -> Option<String>`

- `fn render_where_clause(self: &Self, where_predicates: &[rustdoc_types::WherePredicate]) -> String`

- `fn render_where_predicate(self: &Self, pred: &rustdoc_types::WherePredicate) -> String`

- `fn collect_linkable_types(self: &Self, ty: &Type) -> Vec<(String, rustdoc_types::Id)>`

- `fn collect_types_recursive(self: &Self, ty: &Type, result: &mut Vec<(String, rustdoc_types::Id)>)`

- `fn collect_from_generic_args(self: &Self, args: &GenericArgs, result: &mut Vec<(String, rustdoc_types::Id)>)`

#### Trait Implementations

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> TypeRenderer<'a>` — [`TypeRenderer`](../../types/index.md)

##### `impl Copy<'a>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

