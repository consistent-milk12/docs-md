*[thiserror_impl](../index.md) / [attr](index.md)*

---

# Module `attr`

## Contents

- [Structs](#structs)
  - [`Attrs`](#attrs)
  - [`Display`](#display)
  - [`Source`](#source)
  - [`From`](#from)
  - [`Transparent`](#transparent)
  - [`Fmt`](#fmt)
- [Enums](#enums)
  - [`Trait`](#trait)
- [Functions](#functions)
  - [`get`](#get)
  - [`parse_error_attribute`](#parse_error_attribute)
  - [`parse_token_expr`](#parse_token_expr)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Attrs`](#attrs) | struct |  |
| [`Display`](#display) | struct |  |
| [`Source`](#source) | struct |  |
| [`From`](#from) | struct |  |
| [`Transparent`](#transparent) | struct |  |
| [`Fmt`](#fmt) | struct |  |
| [`Trait`](#trait) | enum |  |
| [`get`](#get) | fn |  |
| [`parse_error_attribute`](#parse_error_attribute) | fn |  |
| [`parse_token_expr`](#parse_token_expr) | fn |  |

## Structs

### `Attrs<'a>`

```rust
struct Attrs<'a> {
    pub display: Option<Display<'a>>,
    pub source: Option<Source<'a>>,
    pub backtrace: Option<&'a syn::Attribute>,
    pub from: Option<From<'a>>,
    pub transparent: Option<Transparent<'a>>,
    pub fmt: Option<Fmt<'a>>,
}
```

### `Display<'a>`

```rust
struct Display<'a> {
    pub original: &'a syn::Attribute,
    pub fmt: syn::LitStr,
    pub args: proc_macro2::TokenStream,
    pub requires_fmt_machinery: bool,
    pub has_bonus_display: bool,
    pub infinite_recursive: bool,
    pub implied_bounds: std::collections::BTreeSet<(usize, Trait)>,
    pub bindings: Vec<(syn::Ident, proc_macro2::TokenStream)>,
}
```

#### Implementations

- <span id="crateattrdisplay-expand-shorthand"></span>`fn expand_shorthand(&mut self, fields: &[Field<'_>], container: ContainerKind) -> Result<()>` — [`Field`](../ast/index.md), [`ContainerKind`](../ast/index.md)

#### Trait Implementations

##### `impl<'a> Clone for Display<'a>`

- <span id="display-clone"></span>`fn clone(&self) -> Display<'a>` — [`Display`](#display)

##### `impl<T> Spanned for Display<'a>`

- <span id="display-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Display<'_>`

- <span id="display-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Source<'a>`

```rust
struct Source<'a> {
    pub original: &'a syn::Attribute,
    pub span: proc_macro2::Span,
}
```

#### Trait Implementations

##### `impl<'a> Clone for Source<'a>`

- <span id="source-clone"></span>`fn clone(&self) -> Source<'a>` — [`Source`](#source)

##### `impl<'a> Copy for Source<'a>`

### `From<'a>`

```rust
struct From<'a> {
    pub original: &'a syn::Attribute,
    pub span: proc_macro2::Span,
}
```

#### Trait Implementations

##### `impl<'a> Clone for From<'a>`

- <span id="from-clone"></span>`fn clone(&self) -> From<'a>` — [`From`](#from)

##### `impl<'a> Copy for From<'a>`

### `Transparent<'a>`

```rust
struct Transparent<'a> {
    pub original: &'a syn::Attribute,
    pub span: proc_macro2::Span,
}
```

#### Trait Implementations

##### `impl<'a> Clone for Transparent<'a>`

- <span id="transparent-clone"></span>`fn clone(&self) -> Transparent<'a>` — [`Transparent`](#transparent)

##### `impl<'a> Copy for Transparent<'a>`

### `Fmt<'a>`

```rust
struct Fmt<'a> {
    pub original: &'a syn::Attribute,
    pub path: syn::ExprPath,
}
```

#### Trait Implementations

##### `impl<'a> Clone for Fmt<'a>`

- <span id="fmt-clone"></span>`fn clone(&self) -> Fmt<'a>` — [`Fmt`](#fmt)

## Enums

### `Trait`

```rust
enum Trait {
    Debug,
    Display,
    Octal,
    LowerHex,
    UpperHex,
    Pointer,
    Binary,
    LowerExp,
    UpperExp,
}
```

#### Trait Implementations

##### `impl Clone for Trait`

- <span id="trait-clone"></span>`fn clone(&self) -> Trait` — [`Trait`](#trait)

##### `impl Copy for Trait`

##### `impl Debug for Trait`

- <span id="trait-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Trait`

##### `impl Ord for Trait`

- <span id="trait-cmp"></span>`fn cmp(&self, other: &Trait) -> cmp::Ordering` — [`Trait`](#trait)

##### `impl PartialEq for Trait`

- <span id="trait-eq"></span>`fn eq(&self, other: &Trait) -> bool` — [`Trait`](#trait)

##### `impl PartialOrd for Trait`

- <span id="trait-partial-cmp"></span>`fn partial_cmp(&self, other: &Trait) -> option::Option<cmp::Ordering>` — [`Trait`](#trait)

##### `impl<T> Spanned for Trait`

- <span id="trait-span"></span>`fn span(&self) -> Span`

##### `impl StructuralPartialEq for Trait`

##### `impl ToTokens for Trait`

- <span id="trait-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Functions

### `get`

```rust
fn get(input: &[syn::Attribute]) -> syn::Result<Attrs<'_>>
```

### `parse_error_attribute`

```rust
fn parse_error_attribute<'a>(attrs: &mut Attrs<'a>, attr: &'a syn::Attribute) -> syn::Result<()>
```

### `parse_token_expr`

```rust
fn parse_token_expr(input: syn::parse::ParseStream<'_>, begin_expr: bool) -> syn::Result<proc_macro2::TokenStream>
```

