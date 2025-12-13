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

*Defined in `src/types.rs:67-73`*

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

  Create a new type renderer with the given crate context.

  

  # Arguments

  

  * `krate` - The parsed rustdoc crate containing type definitions

- <span id="typerenderer-get-type-id"></span>`const fn get_type_id(&self, ty: &Type) -> Option<Id>`

  Get the ID of a resolved type, if available.

  

  Returns `Some(Id)` for resolved path types (named types like structs,

  enums, traits), `None` for primitives and other type variants.

  

  # Arguments

  

  * `ty` - The type to extract the ID from

- <span id="typerenderer-render-path-with-args"></span>`fn render_path_with_args<'p>(self, path: &'p str, args: Option<&GenericArgs>) -> Cow<'p, str>`

  Render a sanitized path with optional generic arguments.

  

  Returns `Cow::Borrowed` when possible (no args, no sanitization needed),

  preserving the zero-allocation fast path for simple types.

- <span id="typerenderer-write-bounds-joined"></span>`fn write_bounds_joined(self, out: &mut String, bounds: &[GenericBound])`

  Render bounds joined with " + " directly to a buffer.

  

  Avoids intermediate Vec allocation.

- <span id="typerenderer-write-types-joined"></span>`fn write_types_joined<'t, I>(self, out: &mut String, types: I, sep: &str)`

  Write multiple rendered types separated by a delimiter.

  

  Avoids intermediate Vec allocation.

- <span id="typerenderer-render-type"></span>`fn render_type<'t>(&self, ty: &'t Type) -> Cow<'t, str>`

  Render a rustdoc `Type` to its Rust syntax string representation.

  

  This is the main entry point for type rendering. It handles all variants

  of the `Type` enum and recursively renders nested types.

  

  # Arguments

  

  * `ty` - The type to render

  

  # Returns

  

  A `Cow<str>` representing the type in Rust syntax. Simple types like

  primitives and generics return borrowed strings to avoid allocation.

  

  # Supported Type Variants

  

  - Primitives: `u32`, `bool`, `str`, etc.

  - References: `&T`, `&'a mut T`

  - Pointers: `*const T`, `*mut T`

  - Slices and arrays: `[T]`, `[T; N]`

  - Tuples: `(A, B, C)`

  - Paths: `std::vec::Vec<T>`

  - Function pointers: `fn(A, B) -> C`

  - Trait objects: `dyn Trait + Send`

  - Impl trait: `impl Iterator<Item = T>`

  - Qualified paths: `<T as Trait>::Item`

- <span id="typerenderer-render-generic-args"></span>`fn render_generic_args(self, args: &GenericArgs) -> String`

  Render generic arguments in angle bracket or parenthesized form.

  

  Handles three syntaxes:

  - Angle brackets: `<T, U, Item = V>` (most common)

  - Parenthesized: `(A, B) -> C` (for Fn traits)

  - Return type notation: `(..)` (experimental)

- <span id="typerenderer-render-generic-arg"></span>`fn render_generic_arg(self, arg: &GenericArg) -> Cow<'_, str>`

  Render a single generic argument.

  

  Arguments can be:

  - Lifetimes: `'a`, `'static`

  - Types: `T`, `Vec<u32>`

  - Const values: `N`, `{expr}`

  - Inferred: `_`

- <span id="typerenderer-render-assoc-item-constraint"></span>`fn render_assoc_item_constraint(self, constraint: &AssocItemConstraint) -> String`

  Render an associated type constraint.

  

  These appear in generic bounds:

  - Equality: `Item = u32`

  - Bound: `Item: Display`

- <span id="typerenderer-render-term"></span>`fn render_term(self, term: &Term) -> Cow<'_, str>`

  Render a term, which is either a type or a constant.

  

  Used in associated type constraints like `Item = u32`.

- <span id="typerenderer-render-generic-bound"></span>`fn render_generic_bound<'t>(&self, bound: &'t GenericBound) -> Cow<'t, str>`

  Render a generic bound (trait bound or lifetime bound).

  

  # Examples

  

  - Trait bound: `Clone`, `Iterator<Item = T>`

  - Modified bound: `?Sized`, `~const Drop`

  - Lifetime bound: `'static`, `'a`

- <span id="typerenderer-render-generics"></span>`fn render_generics(&self, generics: &[GenericParamDef]) -> String`

  Render a list of generic parameter definitions.

  

  Produces the `<T, U, const N: usize>` portion of a signature.

  

  # Arguments

  

  * `generics` - The list of generic parameters from rustdoc

  

  # Returns

  

  A string like `<T, U>` or empty string if no generics.

  

  # Filtering

  

  Synthetic parameters (generated by the compiler for `impl Trait`)

  are filtered out since they don't appear in the source.

- <span id="typerenderer-render-generic-param-def"></span>`fn render_generic_param_def(self, param: &GenericParamDef) -> Option<String>`

  Render a single generic parameter definition.

  

  Returns `None` for synthetic parameters (compiler-generated).

  

  # Parameter Kinds

  

  - Lifetime: `'a`, `'a: 'b + 'c`

  - Type: `T`, `T: Clone + Send`

  - Const: `const N: usize`

- <span id="typerenderer-render-where-clause"></span>`fn render_where_clause(&self, where_predicates: &[WherePredicate]) -> String`

  Render where clause predicates.

  

  Produces the `where T: Clone, U: Send` portion of a signature.

  

  # Arguments

  

  * `where_predicates` - The list of where predicates from rustdoc

  

  # Returns

  

  A formatted where clause string, or empty string if no predicates.

  

  # Format

  

  ```text

  where

      T: Clone,

      U: Send

  ```

- <span id="typerenderer-render-where-predicate"></span>`fn render_where_predicate(self, pred: &WherePredicate) -> String`

  Render a single where predicate.

  

  # Predicate Types

  

  - Bound: `T: Clone + Send`

  - Lifetime: `'a: 'b + 'c`

  - Equality: `<T as Iterator>::Item = u32`

- <span id="typerenderer-collect-linkable-types"></span>`fn collect_linkable_types(&self, ty: &Type) -> Vec<(String, rustdoc_types::Id)>`

  Collect all linkable type names from a type.

  

  This extracts type names that could potentially be linked to their definitions.

  Returns a set of (`ype_name`, `type_id`) pairs for `ResolvedPath` types.

  

  # Linkable Types

  

  - `ResolvedPath` types (e.g., `Vec`, `HashMap`, `MyStruct`)

  - Nested types within generics, references, slices, etc.

  

  # Excluded

  

  - Primitives (e.g., `u32`, `bool`)

  - Generic parameters (e.g., `T`, `U`)

  - Inferred types (`_`)

- <span id="typerenderer-collect-types-recursive"></span>`fn collect_types_recursive(self, ty: &Type, result: &mut Vec<(String, rustdoc_types::Id)>)`

  Recursively collect linkable types from a type tree.

- <span id="typerenderer-collect-from-generic-args"></span>`fn collect_from_generic_args(self, args: &GenericArgs, result: &mut Vec<(String, rustdoc_types::Id)>)`

  Collect types from generic arguments.

#### Trait Implementations

##### `impl Any for TypeRenderer<'a>`

- <span id="typerenderer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeRenderer<'a>`

- <span id="typerenderer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeRenderer<'a>`

- <span id="typerenderer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TypeRenderer<'a>`

- <span id="typerenderer-clone"></span>`fn clone(&self) -> TypeRenderer<'a>` — [`TypeRenderer`](#typerenderer)

##### `impl CloneToUninit for TypeRenderer<'a>`

- <span id="typerenderer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for TypeRenderer<'a>`

##### `impl Debug for TypeRenderer<'a>`

- <span id="typerenderer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TypeRenderer<'a>`

- <span id="typerenderer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for TypeRenderer<'a>`

##### `impl<U> Into for TypeRenderer<'a>`

- <span id="typerenderer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TypeRenderer<'a>`

##### `impl OwoColorize for TypeRenderer<'a>`

##### `impl Pointable for TypeRenderer<'a>`

- <span id="typerenderer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="typerenderer-pointable-type-init"></span>`type Init = T`

- <span id="typerenderer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="typerenderer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="typerenderer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="typerenderer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TypeRenderer<'a>`

- <span id="typerenderer-toowned-type-owned"></span>`type Owned = T`

- <span id="typerenderer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typerenderer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TypeRenderer<'a>`

- <span id="typerenderer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typerenderer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeRenderer<'a>`

- <span id="typerenderer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typerenderer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for TypeRenderer<'a>`

