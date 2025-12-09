*[tracing_attributes](../index.md) / [attr](index.md)*

---

# Module `attr`

## Contents

- [Modules](#modules)
  - [`kw`](#kw)
- [Structs](#structs)
  - [`EventArgs`](#eventargs)
  - [`InstrumentArgs`](#instrumentargs)
  - [`StrArg`](#strarg)
  - [`ExprArg`](#exprarg)
  - [`Skips`](#skips)
  - [`Fields`](#fields)
  - [`Field`](#field)
- [Enums](#enums)
  - [`LitStrOrIdent`](#litstrorident)
  - [`FormatMode`](#formatmode)
  - [`FieldKind`](#fieldkind)
  - [`FieldName`](#fieldname)
  - [`Level`](#level)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`kw`](#kw) | mod |  |
| [`EventArgs`](#eventargs) | struct | Arguments to `#[instrument(err(...))]` and `#[instrument(ret(...))]` which describe how the |
| [`InstrumentArgs`](#instrumentargs) | struct |  |
| [`StrArg`](#strarg) | struct |  |
| [`ExprArg`](#exprarg) | struct |  |
| [`Skips`](#skips) | struct |  |
| [`Fields`](#fields) | struct |  |
| [`Field`](#field) | struct |  |
| [`LitStrOrIdent`](#litstrorident) | enum |  |
| [`FormatMode`](#formatmode) | enum |  |
| [`FieldKind`](#fieldkind) | enum |  |
| [`FieldName`](#fieldname) | enum |  |
| [`Level`](#level) | enum |  |

## Modules

- [`kw`](kw/index.md)

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

- <span id="eventargs-level"></span>`fn level(&self, default: Level) -> Level` — [`Level`](#level)

#### Trait Implementations

##### `impl Clone for EventArgs`

- <span id="eventargs-clone"></span>`fn clone(&self) -> EventArgs` — [`EventArgs`](#eventargs)

##### `impl Debug for EventArgs`

- <span id="eventargs-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EventArgs`

- <span id="eventargs-default"></span>`fn default() -> EventArgs` — [`EventArgs`](#eventargs)

##### `impl Parse for EventArgs`

- <span id="eventargs-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

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

- <span id="instrumentargs-level"></span>`fn level(&self) -> Level` — [`Level`](#level)

- <span id="instrumentargs-target"></span>`fn target(&self) -> impl ToTokens`

- <span id="instrumentargs-warnings"></span>`fn warnings(&self) -> impl ToTokens`

#### Trait Implementations

##### `impl Clone for InstrumentArgs`

- <span id="instrumentargs-clone"></span>`fn clone(&self) -> InstrumentArgs` — [`InstrumentArgs`](#instrumentargs)

##### `impl Debug for InstrumentArgs`

- <span id="instrumentargs-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for InstrumentArgs`

- <span id="instrumentargs-default"></span>`fn default() -> InstrumentArgs` — [`InstrumentArgs`](#instrumentargs)

##### `impl Parse for InstrumentArgs`

- <span id="instrumentargs-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

### `StrArg<T>`

```rust
struct StrArg<T> {
    value: LitStrOrIdent,
    _p: std::marker::PhantomData<T>,
}
```

#### Trait Implementations

##### `impl<T: Parse> Parse for StrArg<T>`

- <span id="strarg-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

### `ExprArg<T>`

```rust
struct ExprArg<T> {
    value: syn::Expr,
    _p: std::marker::PhantomData<T>,
}
```

#### Trait Implementations

##### `impl<T: Parse> Parse for ExprArg<T>`

- <span id="exprarg-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

### `Skips`

```rust
struct Skips(std::collections::HashSet<syn::Ident>);
```

#### Trait Implementations

##### `impl Parse for Skips`

- <span id="skips-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

### `Fields`

```rust
struct Fields(syn::punctuated::Punctuated<Field, token::Comma>);
```

#### Trait Implementations

##### `impl Clone for Fields`

- <span id="fields-clone"></span>`fn clone(&self) -> Fields` — [`Fields`](#fields)

##### `impl Debug for Fields`

- <span id="fields-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for Fields`

- <span id="fields-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<T> Spanned for Fields`

- <span id="fields-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Fields`

- <span id="fields-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

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

- <span id="field-clone"></span>`fn clone(&self) -> Field` — [`Field`](#field)

##### `impl Debug for Field`

- <span id="field-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for Field`

- <span id="field-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<T> Spanned for Field`

- <span id="field-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Field`

- <span id="field-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

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

- <span id="litstrorident-clone"></span>`fn clone(&self) -> LitStrOrIdent` — [`LitStrOrIdent`](#litstrorident)

##### `impl Debug for LitStrOrIdent`

- <span id="litstrorident-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for LitStrOrIdent`

- <span id="litstrorident-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<T> Spanned for LitStrOrIdent`

- <span id="litstrorident-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for LitStrOrIdent`

- <span id="litstrorident-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

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

- <span id="formatmode-clone"></span>`fn clone(&self) -> FormatMode` — [`FormatMode`](#formatmode)

##### `impl Debug for FormatMode`

- <span id="formatmode-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FormatMode`

- <span id="formatmode-default"></span>`fn default() -> FormatMode` — [`FormatMode`](#formatmode)

##### `impl Eq for FormatMode`

##### `impl Hash for FormatMode`

- <span id="formatmode-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for FormatMode`

- <span id="formatmode-eq"></span>`fn eq(&self, other: &FormatMode) -> bool` — [`FormatMode`](#formatmode)

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

- <span id="fieldkind-clone"></span>`fn clone(&self) -> FieldKind` — [`FieldKind`](#fieldkind)

##### `impl Debug for FieldKind`

- <span id="fieldkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FieldKind`

##### `impl PartialEq for FieldKind`

- <span id="fieldkind-eq"></span>`fn eq(&self, other: &FieldKind) -> bool` — [`FieldKind`](#fieldkind)

##### `impl<T> Spanned for FieldKind`

- <span id="fieldkind-span"></span>`fn span(&self) -> Span`

##### `impl StructuralPartialEq for FieldKind`

##### `impl ToTokens for FieldKind`

- <span id="fieldkind-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FieldName`

```rust
enum FieldName {
    Expr(syn::Expr),
    Punctuated(syn::punctuated::Punctuated<syn::Ident, token::Dot>),
}
```

#### Trait Implementations

##### `impl Clone for FieldName`

- <span id="fieldname-clone"></span>`fn clone(&self) -> FieldName` — [`FieldName`](#fieldname)

##### `impl Debug for FieldName`

- <span id="fieldname-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Spanned for FieldName`

- <span id="fieldname-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for FieldName`

- <span id="fieldname-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

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

- <span id="level-clone"></span>`fn clone(&self) -> Level` — [`Level`](#level)

##### `impl Debug for Level`

- <span id="level-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for Level`

- <span id="level-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<T> Spanned for Level`

- <span id="level-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Level`

- <span id="level-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

