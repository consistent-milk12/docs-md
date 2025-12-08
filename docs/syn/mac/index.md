*[syn](../index.md) / [mac](index.md)*

---

# Module `mac`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `Macro`

```rust
struct Macro {
    pub path: crate::path::Path,
    pub bang_token: $crate::token::Not,
    pub delimiter: MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}
```

A macro invocation: `println!("{}", mac)`.

#### Implementations

- `fn parse_body<T: Parse>(self: &Self) -> Result<T>` — [`Result`](../error/index.md)

- `fn parse_body_with<F: Parser>(self: &Self, parser: F) -> Result<<F as >::Output>` — [`Result`](../error/index.md), [`Parser`](../parse/index.md)

#### Trait Implementations

##### `impl Clone for crate::Macro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Macro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Macro`

##### `impl Hash for crate::Macro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::mac::Macro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::Macro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Macro`

##### `impl<T> Spanned for Macro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::mac::Macro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Enums

### `MacroDelimiter`

```rust
enum MacroDelimiter {
    Paren(crate::token::Paren),
    Brace(crate::token::Brace),
    Bracket(crate::token::Bracket),
}
```

A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`.

#### Implementations

- `fn span(self: &Self) -> &DelimSpan`

- `fn is_brace(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::MacroDelimiter`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::MacroDelimiter`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MacroDelimiter`

##### `impl Hash for crate::MacroDelimiter`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::MacroDelimiter`

- `fn eq(self: &Self, other: &Self) -> bool`

## Functions

### `parse_delimiter`

```rust
fn parse_delimiter(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(MacroDelimiter, proc_macro2::TokenStream)>
```

