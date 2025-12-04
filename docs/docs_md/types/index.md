*[docs_md](../index.md) / [types](index.md)*

---

# Module `types`

Type rendering utilities for converting rustdoc types to string representations.

This module provides the [`TypeRenderer`](docs_md/types/index.md) struct to convert the complex type
structures from rustdoc JSON into human-readable Rust type syntax. These
rendered strings are used in the generated markdown documentation.

# Overview

Rustdoc JSON represents types as a tree structure (the `Type` enum). The
[`TypeRenderer`](docs_md/types/index.md) walks that tree and produces the string representation
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

## Structs

### `TypeRenderer<'a>`

```rust
struct TypeRenderer<'a> {
    // [REDACTED: Private Fields]
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

#### Implementations

- `fn new(krate: &'a Crate) -> Self`
  Create a new type renderer with the given crate context.

- `fn render_type(self: &Self, ty: &Type) -> String`
  Render a rustdoc `Type` to its Rust syntax string representation.

- `fn render_generic_bound(self: &Self, bound: &GenericBound) -> String`
  Render a generic bound (trait bound or lifetime bound).

- `fn render_generics(self: &Self, generics: &[GenericParamDef]) -> String`
  Render a list of generic parameter definitions.

- `fn render_where_clause(self: &Self, where_predicates: &[rustdoc_types::WherePredicate]) -> String`
  Render where clause predicates.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> TypeRenderer<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'a>`

##### `impl OwoColorize<D>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

