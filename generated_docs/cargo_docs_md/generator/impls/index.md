*[cargo_docs_md](../../index.md) / [generator](../index.md) / [impls](index.md)*

---

# Module `impls`

Implementation block rendering for documentation generation.

This module provides the [`ImplRenderer`](#implrenderer) struct which handles rendering
impl blocks (both inherent and trait implementations) to markdown format.

# Path Types in `rustdoc_types::Impl`

When working with impl blocks, there are two different path representations:

- **`Impl.trait_: Option<Path>`** - The trait being implemented
  - `Path.path` is a `String` like `"Clone"` or `"std::fmt::Debug"`
  - Use for display/rendering in documentation output
  - Extract trait name with `path.rsplit("::").next()`

- **`ItemSummary.path: Vec<String>`** - Structured path from `krate.paths`
  - Example: `["std", "clone", "Clone"]`
  - Use for lookups and resolution via item IDs

The `Path.path` string is already formatted for display, while
`ItemSummary.path` is structured for programmatic manipulation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImplUtils`](#implutils) | struct | Utility functions to work with impl's and generics |
| [`ImplRenderer`](#implrenderer) | struct | Renders impl blocks to markdown. |
| [`BLANKET_TRAITS`](#blanket-traits) | const | Blanket trait implementations to filter from output. |
| [`TRIVIAL_DERIVE_TRAITS`](#trivial-derive-traits) | const | Trivial derive trait implementations that can be collapsed. |
| [`TRIVIAL_DERIVE_DESCRIPTIONS`](#trivial-derive-descriptions) | const | Short descriptions for trivial derive traits, used in summary tables. |

## Structs

### `ImplUtils`

```rust
struct ImplUtils;
```

*Defined in `src/generator/impls.rs:104`*

Utility functions to work with impl's and generics

#### Implementations

- <span id="implutils-is-trivial-derive-impl"></span>`fn is_trivial_derive_impl(impl_block: &Impl) -> bool`

  Check if an impl block is for a trivial derive trait.

  

  Returns `true` if the impl is for one of the commonly derived traits

  `(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)`.

  

  # Examples

  

  ```rust

  use rustdoc_types::Impl;

  // For an impl block with trait "Clone", returns true

  // For an impl block with trait "Display", returns false

  ```

- <span id="implutils-get-trivial-derive-description"></span>`fn get_trivial_derive_description(trait_name: &str) -> Option<&'static str>`

  Get the description for a trivial derive trait.

  

  Returns `None` if the trait is not in the trivial derives list.

- <span id="implutils-is-blanket-impl"></span>`fn is_blanket_impl(impl_block: &Impl) -> bool`

  Check if an impl block is for a blanket trait that should be filtered.

  

  Returns `true` if the impl is for one of the commonly auto-derived traits

  that add noise to documentation (From, Into, Any, Borrow, etc.).

- <span id="implutils-is-generic-type"></span>`fn is_generic_type(ty: &Type) -> bool`

  Check if a type is generic (contains a type parameter like `T`).

  

  This is used to determine whether to show generic parameters in impl blocks.

  For blanket impls like `impl<T> Trait for T`, we only show the generics if

  the `for_` type is actually generic. When the impl is instantiated for a

  concrete type like `TocEntry`, we hide the generics.

  

  # Examples

  

  ```text

  // Generic type - returns true

  is_generic_type(&Type::Generic("T")) == true

  

  // Concrete type - returns false

  is_generic_type(&Type::ResolvedPath { name: "TocEntry", .. }) == false

  

  // Container with generic - returns true

  is_generic_type(&Type::ResolvedPath {

      name: "Vec",

      args: Some(GenericArgs::AngleBracketed { args: [Type::Generic("T")] })

  }) == true

  ```

- <span id="implutils-generic-args-contain-generic"></span>`fn generic_args_contain_generic(args: &GenericArgs) -> bool`

  Check if generic args contain any generic type parameters.

- <span id="implutils-extract-impl-signature-generics"></span>`fn extract_impl_signature_generics(impl_block: &Impl) -> HashSet<String>`

  Extract generic parameter names that appear in the impl signature (for_ and trait).

  

  This extracts generics from only the visible parts of the impl header:

  1. The `for_` type: `impl<T> Trait for Foo<T>` → extracts `T` from `Foo<T>`

  2. The trait path: `impl<U> Trait<U> for Foo` → extracts `U` from `Trait<U>`

  

  Unlike `extract_impl_visible_generics`, this does NOT extract from where clauses.

  Use this for rendering purposes when deciding which generic params to show in

  `impl<...>`. Generics that only appear in where clauses should not be shown in

  the impl header since the where clause itself is not rendered.

  

  # Examples

  

  ```text

  impl<T: Sized> IntoEither for T

    └─ for_ type is Generic("T") → extracts ["T"]

    └─ When rendered for `Either<L, R>`, for_ becomes `Either<L, R>`

    └─ Extracts ["L", "R"], NOT ["T"]

    └─ T is only in where clause, so `impl<T>` should NOT be shown

  

  impl<T> Clone for Wrapper<T>

    └─ for_ type has `Wrapper<T>` → extracts ["T"]

    └─ Result: render as `impl<T>`

  

  impl<T, U> Trait<U> for Foo<T>

    └─ for_ type has `Foo<T>` → extracts ["T"]

    └─ trait path has `Trait<U>` → extracts ["U"]

    └─ Result: render as `impl<T, U>` (both visible)

  ```

- <span id="implutils-extract-impl-visible-generics"></span>`fn extract_impl_visible_generics(impl_block: &Impl) -> HashSet<String>`

  Extract all generic parameter names visible in an impl block's signature.

  

  # Why This Is Needed

  

  Rustdoc JSON stores impl generics in two places that can have DIFFERENT names:

  - `impl_block.generics.params` - the declared params (e.g., `T` from `impl<T>`)

  - `impl_block.for_` - the actual type (e.g., `StyledObject<D>`)

  

  For blanket impls, these names often differ! The compiler transforms:

  ```text

  impl<D: Display> ToString for StyledObject<D>

  ```

  Into JSON where `generics.params` has `T` but `for_` has `StyledObject<D>`.

  If we blindly use `generics.params`, we'd render `impl<T> ToString for StyledObject<D>`.

  

  # Solution

  

  Extract generics from what's VISIBLE in the signature:

  1. The `for_` type: `Foo<T>` → extracts `T`

  2. The trait path: `Iterator<Item = U>` → extracts `U`

  3. Where clauses: `where T: Clone` → extracts `T`

  

  Then filter `generics.params` to only include names that appear in visible locations.

  

  **Note:** For rendering the impl header, use `extract_impl_signature_generics` instead,

  which excludes where clause generics.

  

  # Examples

  

  ```text

  impl<T> Clone for Wrapper<T>

    └─ for_ type has `Wrapper<T>` → extracts ["T"]

    └─ Result: render as `impl<T>`

  

  impl<T, U> Trait<U> for Foo<T> where T: Clone

    └─ for_ type has `Foo<T>` → extracts ["T"]

    └─ trait path has `Trait<U>` → extracts ["U"]

    └─ where clause has `T: Clone` → extracts ["T"]

    └─ Result: render as `impl<T, U>` (both visible)

  

  impl<D: Display> ToString for StyledObject<D>

    └─ for_ type has `StyledObject<D>` → extracts ["D"]

    └─ Result: render as `impl<D>` (not `impl<T>`)

  ```

- <span id="implutils-extract-type-generics-into"></span>`fn extract_type_generics_into(ty: &Type, names: &mut HashSet<String>)`

  Extract generic parameter names from a type into a set.

  

  Recursively traverses the type structure to find all `Type::Generic` variants.

  This handles all the ways generics can be nested in complex types.

  

  # Type Tree Visualization

  

  ```text

  HashMap<K, Vec<T>>

  └─ ResolvedPath("HashMap")

     └─ args: [K, Vec<T>]

              │  └─ ResolvedPath("Vec")

              │     └─ args: [T]

              │              └─ Generic("T") ← extracted!

              └─ Generic("K") ← extracted!

  ```

- <span id="implutils-extract-generic-args-into"></span>`fn extract_generic_args_into(args: &GenericArgs, names: &mut HashSet<String>)`

  Extract generic parameter names from generic arguments.

  

  Generic arguments come in three forms:

  

  # Forms

  

  ```text

  AngleBracketed: Vec<T, U>           → args: [T, U], constraints: []

                  Iterator<Item = T>  → args: [], constraints: [Item = T]

  

  Parenthesized:  Fn(T, U) -> V       → inputs: [T, U], output: Some(V)

                  FnOnce(T)           → inputs: [T], output: None

  

  ReturnTypeNotation: method(..)      → (no generics to extract)

  ```

- <span id="implutils-extract-term-generics-into"></span>`fn extract_term_generics_into(term: &Term, names: &mut HashSet<String>)`

  Extract generic parameter names from a type or const term.

  

  A `Term` appears in associated type bindings like `Iterator<Item = T>`.

  - `Term::Type(T)` - the `T` in `Item = T`

  - `Term::Constant(N)` - const generics like `Item = 42`

- <span id="implutils-extract-bound-generics-into"></span>`fn extract_bound_generics_into(bound: &GenericBound, names: &mut HashSet<String>)`

  Extract generic parameter names from a generic bound.

  

  Bounds appear in where clauses: `where T: Clone + Iterator<Item = U>`

  

  We only care about `TraitBound` (not `Outlives` like `'a: 'b`).

  From the trait bound, we extract any generic args on the trait itself.

  

  # Example

  

  ```text

  where T: Iterator<Item = U>

           └─ TraitBound { trait_: Path("Iterator"), args: [Item = U] }

                                                               └─ extracts U

  ```

- <span id="implutils-has-hidden-generic-refs"></span>`fn has_hidden_generic_refs(impl_block: &Impl, krate: &Crate) -> bool`

  Check if an impl has associated types referencing generics NOT visible in the signature.

  

  # The Problem

  

  Some impl blocks have generics that only appear in associated types, not in the

  visible signature. When rendered, this produces confusing output:

  

  ```text

  impl DoubleEndedIterator for Foo    ← No `I` visible!

      type Item = <I as Iterator>::Item   ← Where does `I` come from?

  ```

  

  # Detection Strategy

  

  1. Extract "visible" generics from: `for_` type, trait path, where clause

  2. Extract "declared" generics from: `impl_block.generics.params`

  3. Compute "hidden" = declared - visible

  4. Check if any associated type references a hidden generic

  

  # Examples

  

  ```text

  // FILTER THIS - `I` is hidden but used in associated type

  impl<I: Iterator> DoubleEndedIterator for Foo {

      type Item = <I as Iterator>::Item;

  }

  visible = {}        (Foo has no generics)

  declared = {I}      (from impl<I>)

  hidden = {I}        (declared - visible)

  assoc type uses I   → FILTER!

  

  // KEEP THIS - `T` is visible in for_ type

  impl<T> Iterator for Wrapper<T> {

      type Item = T;

  }

  visible = {T}       (from Wrapper<T>)

  declared = {T}      (from impl<T>)

  hidden = {}         (all visible)

  → KEEP!

  

  // KEEP THIS - `U` is visible in trait path

  impl<T, U> Trait<U> for Foo<T> {

      type Output = (T, U);

  }

  visible = {T, U}    (T from Foo<T>, U from Trait<U>)

  declared = {T, U}   (from impl<T, U>)

  hidden = {}         (all visible)

  → KEEP!

  ```

#### Trait Implementations

##### `impl Any for ImplUtils`

- <span id="implutils-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplUtils`

- <span id="implutils-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplUtils`

- <span id="implutils-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ImplUtils`

- <span id="implutils-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for ImplUtils`

##### `impl<U> Into for ImplUtils`

- <span id="implutils-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ImplUtils`

##### `impl OwoColorize for ImplUtils`

##### `impl Pointable for ImplUtils`

- <span id="implutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="implutils-pointable-type-init"></span>`type Init = T`

- <span id="implutils-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implutils-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implutils-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implutils-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ImplUtils`

- <span id="implutils-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implutils-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplUtils`

- <span id="implutils-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implutils-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for ImplUtils`

### `ImplRenderer<'a>`

```rust
struct ImplRenderer<'a> {
    ctx: &'a dyn RenderContext,
    current_file: &'a str,
    type_renderer: crate::types::TypeRenderer<'a>,
}
```

*Defined in `src/generator/impls.rs:731-740`*

Renders impl blocks to markdown.

This struct handles:
- Inherent implementations (`impl MyType { ... }`)
- Trait implementations (`impl Trait for MyType { ... }`)
- Method signatures within impl blocks
- Associated types and constants

The renderer is generic over [`RenderContext`](../context/index.md), allowing it to work with
both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.

#### Fields

- **`ctx`**: `&'a dyn RenderContext`

  Reference to the render context (either single-crate or multi-crate).

- **`current_file`**: `&'a str`

  Path of the current file being generated (for relative link calculation).

- **`type_renderer`**: `crate::types::TypeRenderer<'a>`

  Cached type renderer to avoid repeated construction.

#### Implementations

- <span id="implrenderer-new"></span>`fn new(ctx: &'a dyn RenderContext, current_file: &'a str) -> Self` — [`RenderContext`](../context/index.md#rendercontext)

  Create a new impl renderer with the given context.

  

  # Arguments

  

  * `ctx` - Render context (implements `RenderContext` trait)

  * `current_file` - Path of the current file (for relative link calculation)

- <span id="implrenderer-process-docs"></span>`fn process_docs(&self, item: &Item) -> Option<String>`

  Process documentation string to resolve intra-doc links.

  

  Delegates to the render context's `process_docs` method, which handles

  both single-crate and multi-crate link resolution.

- <span id="implrenderer-render-impl-blocks"></span>`fn render_impl_blocks(&self, md: &mut String, item_id: Id)`

  Render impl blocks for a given type.

  

  This method looks up all impl blocks for a type and renders them

  in two sections:

  

  1. **Implementations** - Inherent impls (methods defined directly on the type)

  2. **Trait Implementations** - Trait impls (`impl Trait for Type`)

  

  # Impl Block Categories

  

  - **Inherent**: `impl MyType { fn method(&self) {} }`

  - **Trait**: `impl Clone for MyType { ... }`

  - **Synthetic**: Auto-derived by compiler (Send, Sync) - skipped

- <span id="implrenderer-render-trivial-derives-collapsed"></span>`fn render_trivial_derives_collapsed(md: &mut String, impls: &[&&Impl])`

  Render trivial derive trait implementations in a collapsible block.

  

  Groups `Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord`

  implementations into a `<details>` block with a summary table.

- <span id="implrenderer-render-trait-impl"></span>`fn render_trait_impl(&self, md: &mut String, impl_block: &Impl)`

  Render a single trait implementation block.

- <span id="implrenderer-render-impl-methods"></span>`fn render_impl_methods(&self, md: &mut String, impl_block: &Impl)`

  Render the items (methods, constants, types) within an impl block.

  

  Each item is rendered as a bullet point. Items can be:

  - **Functions/Methods**: Full signature with modifiers

  - **Associated Constants**: `const NAME: Type`

  - **Associated Types**: `type Name = Type`

  

  For methods, the first line of documentation is included as a brief summary.

  Type links are added for resolvable types in method signatures.

  Method anchors are generated for deep linking (e.g., `#typename-methodname`).

- <span id="implrenderer-render-generic-args-for-impl"></span>`fn render_generic_args_for_impl(&self, args: &GenericArgs) -> String`

  Render generic arguments for impl block signatures.

  

  This handles the different forms of generic arguments:

  - **Angle bracketed**: `<T, U, Item = V>` (most common)

  - **Parenthesized**: `(A, B) -> C` (for Fn traits)

  - **Return type notation**: `(..)` (experimental)

#### Trait Implementations

##### `impl Any for ImplRenderer<'a>`

- <span id="implrenderer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplRenderer<'a>`

- <span id="implrenderer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplRenderer<'a>`

- <span id="implrenderer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ImplRenderer<'a>`

- <span id="implrenderer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for ImplRenderer<'a>`

##### `impl<U> Into for ImplRenderer<'a>`

- <span id="implrenderer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ImplRenderer<'a>`

##### `impl OwoColorize for ImplRenderer<'a>`

##### `impl Pointable for ImplRenderer<'a>`

- <span id="implrenderer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="implrenderer-pointable-type-init"></span>`type Init = T`

- <span id="implrenderer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implrenderer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implrenderer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implrenderer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ImplRenderer<'a>`

- <span id="implrenderer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implrenderer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplRenderer<'a>`

- <span id="implrenderer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implrenderer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for ImplRenderer<'a>`

## Constants

### `BLANKET_TRAITS`
```rust
const BLANKET_TRAITS: &[&str];
```

*Defined in `src/generator/impls.rs:41-55`*

Blanket trait implementations to filter from output.

These are automatically derived by the compiler and add noise to documentation
without providing useful information. Users who want them can use `--include-blanket-impls`.

### `TRIVIAL_DERIVE_TRAITS`
```rust
const TRIVIAL_DERIVE_TRAITS: &[&str];
```

*Defined in `src/generator/impls.rs:70-86`*

Trivial derive trait implementations that can be collapsed.

These are traits commonly derived via `#[derive(...)]` that have standard,
predictable implementations. When `RenderConfig.hide_trivial_derives` is enabled,
these are grouped into a collapsible `<details>` block with a summary table.

The list includes:
- **Cloning**: `Clone`, `Copy`
- **Formatting**: `Debug`
- **Default values**: `Default`
- **Equality**: `PartialEq`, `Eq`
- **Hashing**: `Hash`
- **Ordering**: `PartialOrd`, `Ord`

### `TRIVIAL_DERIVE_DESCRIPTIONS`
```rust
const TRIVIAL_DERIVE_DESCRIPTIONS: &[(&str, &str)];
```

*Defined in `src/generator/impls.rs:91-101`*

Short descriptions for trivial derive traits, used in summary tables.

Maps trait names to brief descriptions for the collapsible summary table.

