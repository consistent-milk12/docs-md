*[thiserror_impl](../index.md) / [attr](index.md)*

---

# Module `attr`

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

- `fn expand_shorthand(self: &mut Self, fields: &[Field<'_>], container: ContainerKind) -> Result<()>` — [`Field`](../ast/index.md), [`ContainerKind`](../ast/index.md)

#### Trait Implementations

##### `impl<'a> Clone for Display<'a>`

- `fn clone(self: &Self) -> Display<'a>` — [`Display`](#display)

##### `impl<T> Spanned for Display<'a>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Display<'_>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Source<'a>`

```rust
struct Source<'a> {
    pub original: &'a syn::Attribute,
    pub span: proc_macro2::Span,
}
```

#### Trait Implementations

##### `impl<'a> Clone for Source<'a>`

- `fn clone(self: &Self) -> Source<'a>` — [`Source`](#source)

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

- `fn clone(self: &Self) -> From<'a>` — [`From`](#from)

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

- `fn clone(self: &Self) -> Transparent<'a>` — [`Transparent`](#transparent)

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

- `fn clone(self: &Self) -> Fmt<'a>` — [`Fmt`](#fmt)

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

- `fn clone(self: &Self) -> Trait` — [`Trait`](#trait)

##### `impl Copy for Trait`

##### `impl Debug for Trait`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Trait`

##### `impl Ord for Trait`

- `fn cmp(self: &Self, other: &Trait) -> $crate::cmp::Ordering` — [`Trait`](#trait)

##### `impl PartialEq for Trait`

- `fn eq(self: &Self, other: &Trait) -> bool` — [`Trait`](#trait)

##### `impl PartialOrd for Trait`

- `fn partial_cmp(self: &Self, other: &Trait) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Trait`](#trait)

##### `impl<T> Spanned for Trait`

- `fn span(self: &Self) -> Span`

##### `impl StructuralPartialEq for Trait`

##### `impl ToTokens for Trait`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

