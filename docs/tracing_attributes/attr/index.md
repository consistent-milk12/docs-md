*[tracing_attributes](../index.md) / [attr](index.md)*

---

# Module `attr`

## Modules

- [`kw`](kw/index.md) - 

## Structs

### `EventArgs`

```rust
struct EventArgs {
    level: Option<Level>,
    mode: FormatMode,
}
```

Arguments to `#[instrument(err(...))]` and `#[instrument(ret(...))]` which describe how the
return value event should be emitted.

#### Implementations

- `fn level(self: &Self, default: Level) -> Level` — [`Level`](#level)

#### Trait Implementations

##### `impl Clone for EventArgs`

- `fn clone(self: &Self) -> EventArgs` — [`EventArgs`](#eventargs)

##### `impl Debug for EventArgs`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for EventArgs`

- `fn default() -> EventArgs` — [`EventArgs`](#eventargs)

##### `impl Parse for EventArgs`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

### `InstrumentArgs`

```rust
struct InstrumentArgs {
    level: Option<Level>,
    name: Option<LitStrOrIdent>,
    target: Option<LitStrOrIdent>,
    parent: Option<syn::Expr>,
    follows_from: Option<syn::Expr>,
    skips: std::collections::HashSet<syn::Ident>,
    skip_all: bool,
    fields: Option<Fields>,
    err_args: Option<EventArgs>,
    ret_args: Option<EventArgs>,
    parse_warnings: Vec<syn::Error>,
}
```

#### Fields

- **`parse_warnings`**: `Vec<syn::Error>`

  Errors describing any unrecognized parse inputs that we skipped.

#### Implementations

- `fn level(self: &Self) -> Level` — [`Level`](#level)

- `fn target(self: &Self) -> impl ToTokens`

- `fn warnings(self: &Self) -> impl ToTokens`

#### Trait Implementations

##### `impl Clone for InstrumentArgs`

- `fn clone(self: &Self) -> InstrumentArgs` — [`InstrumentArgs`](#instrumentargs)

##### `impl Debug for InstrumentArgs`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for InstrumentArgs`

- `fn default() -> InstrumentArgs` — [`InstrumentArgs`](#instrumentargs)

##### `impl Parse for InstrumentArgs`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

### `StrArg<T>`

```rust
struct StrArg<T> {
    value: LitStrOrIdent,
    _p: std::marker::PhantomData<T>,
}
```

#### Trait Implementations

##### `impl<T: Parse> Parse for StrArg<T>`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

### `ExprArg<T>`

```rust
struct ExprArg<T> {
    value: syn::Expr,
    _p: std::marker::PhantomData<T>,
}
```

#### Trait Implementations

##### `impl<T: Parse> Parse for ExprArg<T>`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

### `Skips`

```rust
struct Skips(std::collections::HashSet<syn::Ident>);
```

#### Trait Implementations

##### `impl Parse for Skips`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

### `Fields`

```rust
struct Fields(syn::punctuated::Punctuated<Field, $crate::token::Comma>);
```

#### Trait Implementations

##### `impl Clone for Fields`

- `fn clone(self: &Self) -> Fields` — [`Fields`](#fields)

##### `impl Debug for Fields`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Parse for Fields`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<T> Spanned for Fields`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Fields`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Field`

```rust
struct Field {
    name: FieldName,
    value: Option<syn::Expr>,
    kind: FieldKind,
}
```

#### Trait Implementations

##### `impl Clone for Field`

- `fn clone(self: &Self) -> Field` — [`Field`](#field)

##### `impl Debug for Field`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Parse for Field`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<T> Spanned for Field`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Field`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Enums

### `LitStrOrIdent`

```rust
enum LitStrOrIdent {
    LitStr(syn::LitStr),
    Ident(syn::Ident),
}
```

#### Trait Implementations

##### `impl Clone for LitStrOrIdent`

- `fn clone(self: &Self) -> LitStrOrIdent` — [`LitStrOrIdent`](#litstrorident)

##### `impl Debug for LitStrOrIdent`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Parse for LitStrOrIdent`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<T> Spanned for LitStrOrIdent`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for LitStrOrIdent`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `FormatMode`

```rust
enum FormatMode {
    Default,
    Display,
    Debug,
}
```

#### Trait Implementations

##### `impl Clone for FormatMode`

- `fn clone(self: &Self) -> FormatMode` — [`FormatMode`](#formatmode)

##### `impl Debug for FormatMode`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for FormatMode`

- `fn default() -> FormatMode` — [`FormatMode`](#formatmode)

##### `impl Eq for FormatMode`

##### `impl Hash for FormatMode`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for FormatMode`

- `fn eq(self: &Self, other: &FormatMode) -> bool` — [`FormatMode`](#formatmode)

##### `impl StructuralPartialEq for FormatMode`

### `FieldKind`

```rust
enum FieldKind {
    Debug,
    Display,
    Value,
}
```

#### Trait Implementations

##### `impl Clone for FieldKind`

- `fn clone(self: &Self) -> FieldKind` — [`FieldKind`](#fieldkind)

##### `impl Debug for FieldKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for FieldKind`

##### `impl PartialEq for FieldKind`

- `fn eq(self: &Self, other: &FieldKind) -> bool` — [`FieldKind`](#fieldkind)

##### `impl<T> Spanned for FieldKind`

- `fn span(self: &Self) -> Span`

##### `impl StructuralPartialEq for FieldKind`

##### `impl ToTokens for FieldKind`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `FieldName`

```rust
enum FieldName {
    Expr(syn::Expr),
    Punctuated(syn::punctuated::Punctuated<syn::Ident, $crate::token::Dot>),
}
```

#### Trait Implementations

##### `impl Clone for FieldName`

- `fn clone(self: &Self) -> FieldName` — [`FieldName`](#fieldname)

##### `impl Debug for FieldName`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Spanned for FieldName`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for FieldName`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Level`

```rust
enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Path(syn::Path),
}
```

#### Trait Implementations

##### `impl Clone for Level`

- `fn clone(self: &Self) -> Level` — [`Level`](#level)

##### `impl Debug for Level`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Parse for Level`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<T> Spanned for Level`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Level`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

