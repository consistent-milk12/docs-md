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
  - [`parse_error_attribute`](#parse-error-attribute)
  - [`parse_token_expr`](#parse-token-expr)

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
| [`parse_error_attribute`](#parse-error-attribute) | fn |  |
| [`parse_token_expr`](#parse-token-expr) | fn |  |

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

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:11-18`](../../../.source_1765210505/thiserror-impl-2.0.17/src/attr.rs#L11-L18)*

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

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:21-30`](../../../.source_1765210505/thiserror-impl-2.0.17/src/attr.rs#L21-L30)*

#### Implementations

- <span id="crateattrdisplay-expand-shorthand"></span>`fn expand_shorthand(&mut self, fields: &[Field<'_>], container: ContainerKind) -> Result<()>` — [`Field`](../ast/index.md#field), [`ContainerKind`](../ast/index.md#containerkind)

#### Trait Implementations

##### `impl Clone for Display<'a>`

- <span id="display-clone"></span>`fn clone(&self) -> Display<'a>` — [`Display`](#display)

##### `impl Spanned for Display<'a>`

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

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:33-36`](../../../.source_1765210505/thiserror-impl-2.0.17/src/attr.rs#L33-L36)*

#### Trait Implementations

##### `impl Clone for Source<'a>`

- <span id="source-clone"></span>`fn clone(&self) -> Source<'a>` — [`Source`](#source)

##### `impl Copy for Source<'a>`

### `From<'a>`

```rust
struct From<'a> {
    pub original: &'a syn::Attribute,
    pub span: proc_macro2::Span,
}
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:39-42`](../../../.source_1765210505/thiserror-impl-2.0.17/src/attr.rs#L39-L42)*

#### Trait Implementations

##### `impl Clone for From<'a>`

- <span id="from-clone"></span>`fn clone(&self) -> From<'a>` — [`From`](#from)

##### `impl Copy for From<'a>`

### `Transparent<'a>`

```rust
struct Transparent<'a> {
    pub original: &'a syn::Attribute,
    pub span: proc_macro2::Span,
}
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:45-48`](../../../.source_1765210505/thiserror-impl-2.0.17/src/attr.rs#L45-L48)*

#### Trait Implementations

##### `impl Clone for Transparent<'a>`

- <span id="transparent-clone"></span>`fn clone(&self) -> Transparent<'a>` — [`Transparent`](#transparent)

##### `impl Copy for Transparent<'a>`

### `Fmt<'a>`

```rust
struct Fmt<'a> {
    pub original: &'a syn::Attribute,
    pub path: syn::ExprPath,
}
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:51-54`](../../../.source_1765210505/thiserror-impl-2.0.17/src/attr.rs#L51-L54)*

#### Trait Implementations

##### `impl Clone for Fmt<'a>`

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

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:57-67`](../../../.source_1765210505/thiserror-impl-2.0.17/src/attr.rs#L57-L67)*

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

##### `impl Spanned for Trait`

- <span id="trait-span"></span>`fn span(&self) -> Span`

##### `impl StructuralPartialEq for Trait`

##### `impl ToTokens for Trait`

- <span id="trait-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Functions

### `get`

```rust
fn get(input: &[syn::Attribute]) -> syn::Result<Attrs<'_>>
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:69-122`](../../../.source_1765210505/thiserror-impl-2.0.17/src/attr.rs#L69-L122)*

### `parse_error_attribute`

```rust
fn parse_error_attribute<'a>(attrs: &mut Attrs<'a>, attr: &'a syn::Attribute) -> syn::Result<()>
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:124-194`](../../../.source_1765210505/thiserror-impl-2.0.17/src/attr.rs#L124-L194)*

### `parse_token_expr`

```rust
fn parse_token_expr(input: syn::parse::ParseStream<'_>, begin_expr: bool) -> syn::Result<proc_macro2::TokenStream>
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:196-300`](../../../.source_1765210505/thiserror-impl-2.0.17/src/attr.rs#L196-L300)*

