*[syn](../index.md) / [mac](index.md)*

---

# Module `mac`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Macro`](#macro) | struct | A macro invocation: `println!("{}", mac)`. |
| [`MacroDelimiter`](#macrodelimiter) | enum | A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`. |
| [`parse_delimiter`](#parse-delimiter) | fn |  |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `Macro`

```rust
struct Macro {
    pub path: crate::path::Path,
    pub bang_token: token::Not,
    pub delimiter: MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}
```

*Defined in [`syn-2.0.111/src/mac.rs:14-23`](../../../.source_1765210505/syn-2.0.111/src/mac.rs#L14-L23)*

A macro invocation: `println!("{}", mac)`.

#### Implementations

- <span id="macro-parse-body"></span>`fn parse_body<T: Parse>(&self) -> Result<T>` — [`Result`](../error/index.md#result)

- <span id="macro-parse-body-with"></span>`fn parse_body_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](../error/index.md#result), [`Parser`](../parse/index.md#parser)

#### Trait Implementations

##### `impl Clone for crate::Macro`

- <span id="cratemacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Macro`

- <span id="cratemacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Macro`

##### `impl Hash for crate::Macro`

- <span id="cratemacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::mac::Macro`

- <span id="cratemacmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Macro`

- <span id="cratemacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Macro`

##### `impl Spanned for Macro`

- <span id="macro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::mac::Macro`

- <span id="cratemacmacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `MacroDelimiter`

```rust
enum MacroDelimiter {
    Paren(crate::token::Paren),
    Brace(crate::token::Brace),
    Bracket(crate::token::Bracket),
}
```

*Defined in [`syn-2.0.111/src/mac.rs:25-33`](../../../.source_1765210505/syn-2.0.111/src/mac.rs#L25-L33)*

A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`.

#### Implementations

- <span id="cratemacmacrodelimiter-surround"></span>`fn surround(&self, tokens: &mut TokenStream, inner: TokenStream)`

#### Trait Implementations

##### `impl Clone for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MacroDelimiter`

##### `impl Hash for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-eq"></span>`fn eq(&self, other: &Self) -> bool`

## Functions

### `parse_delimiter`

```rust
fn parse_delimiter(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(MacroDelimiter, proc_macro2::TokenStream)>
```

*Defined in [`syn-2.0.111/src/mac.rs:153-170`](../../../.source_1765210505/syn-2.0.111/src/mac.rs#L153-L170)*

