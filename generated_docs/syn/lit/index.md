*[syn](../index.md) / [lit](index.md)*

---

# Module `lit`

## Contents

- [Modules](#modules)
  - [`debug_impls`](#debug_impls)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
  - [`value`](#value)
- [Structs](#structs)
  - [`LitStr`](#litstr)
  - [`LitByteStr`](#litbytestr)
  - [`LitCStr`](#litcstr)
  - [`LitByte`](#litbyte)
  - [`LitChar`](#litchar)
  - [`LitRepr`](#litrepr)
  - [`LitInt`](#litint)
  - [`LitIntRepr`](#litintrepr)
  - [`LitFloat`](#litfloat)
  - [`LitFloatRepr`](#litfloatrepr)
  - [`LitBool`](#litbool)
- [Enums](#enums)
  - [`Lit`](#lit)
- [Macros](#macros)
  - [`lit_extra_traits!`](#lit_extra_traits)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`debug_impls`](#debug_impls) | mod |  |
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`value`](#value) | mod |  |
| [`LitStr`](#litstr) | struct | A UTF-8 string literal: `"foo"`. |
| [`LitByteStr`](#litbytestr) | struct | A byte string literal: `b"foo"`. |
| [`LitCStr`](#litcstr) | struct | A nul-terminated C-string literal: `c"foo"`. |
| [`LitByte`](#litbyte) | struct | A byte literal: `b'f'`. |
| [`LitChar`](#litchar) | struct | A character literal: `'a'`. |
| [`LitRepr`](#litrepr) | struct |  |
| [`LitInt`](#litint) | struct | An integer literal: `1` or `1u16`. |
| [`LitIntRepr`](#litintrepr) | struct |  |
| [`LitFloat`](#litfloat) | struct | A floating point literal: `1f64` or `1.0e10f64`. |
| [`LitFloatRepr`](#litfloatrepr) | struct |  |
| [`LitBool`](#litbool) | struct | A boolean literal: `true` or `false`. |
| [`Lit`](#lit) | enum | A Rust literal such as a string or integer or boolean. |
| [`lit_extra_traits!`](#lit_extra_traits) | macro |  |

## Modules

- [`debug_impls`](debug_impls/index.md) - 
- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 
- [`value`](value/index.md) - 

## Structs

### `LitStr`

```rust
struct LitStr {
    repr: Box<LitRepr>,
}
```

A UTF-8 string literal: `"foo"`.

#### Implementations

- <span id="litstr-new"></span>`fn new(value: &str, span: Span) -> Self`

- <span id="litstr-value"></span>`fn value(&self) -> String`

- <span id="litstr-parse"></span>`fn parse<T: Parse>(&self) -> Result<T>` — [`Result`](../index.md)

- <span id="litstr-parse-with"></span>`fn parse_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](../index.md), [`Parser`](../parse/index.md)

- <span id="litstr-span"></span>`fn span(&self) -> Span`

- <span id="litstr-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litstr-suffix"></span>`fn suffix(&self) -> &str`

- <span id="litstr-token"></span>`fn token(&self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitStr`

- <span id="litstr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitStr`

- <span id="cratelitlitstr-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitStr`

##### `impl Hash for LitStr`

- <span id="litstr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitStr`

- <span id="cratelitlitstr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitStr`

- <span id="litstr-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for LitStr`

##### `impl<T> Spanned for LitStr`

- <span id="litstr-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitStr`

- <span id="cratelitlitstr-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitStr`

### `LitByteStr`

```rust
struct LitByteStr {
    repr: Box<LitRepr>,
}
```

A byte string literal: `b"foo"`.

#### Implementations

- <span id="litbytestr-new"></span>`fn new(value: &[u8], span: Span) -> Self`

- <span id="litbytestr-value"></span>`fn value(&self) -> Vec<u8>`

- <span id="litbytestr-span"></span>`fn span(&self) -> Span`

- <span id="litbytestr-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litbytestr-suffix"></span>`fn suffix(&self) -> &str`

- <span id="litbytestr-token"></span>`fn token(&self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitByteStr`

- <span id="litbytestr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByteStr`

##### `impl Hash for LitByteStr`

- <span id="litbytestr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitByteStr`

- <span id="litbytestr-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitByteStr`

##### `impl<T> Spanned for LitByteStr`

- <span id="litbytestr-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByteStr`

### `LitCStr`

```rust
struct LitCStr {
    repr: Box<LitRepr>,
}
```

A nul-terminated C-string literal: `c"foo"`.

#### Implementations

- <span id="litcstr-new"></span>`fn new(value: &CStr, span: Span) -> Self`

- <span id="litcstr-value"></span>`fn value(&self) -> CString`

- <span id="litcstr-span"></span>`fn span(&self) -> Span`

- <span id="litcstr-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litcstr-suffix"></span>`fn suffix(&self) -> &str`

- <span id="litcstr-token"></span>`fn token(&self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitCStr`

- <span id="litcstr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitCStr`

- <span id="cratelitlitcstr-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitCStr`

##### `impl Hash for LitCStr`

- <span id="litcstr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitCStr`

- <span id="cratelitlitcstr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitCStr`

- <span id="litcstr-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for LitCStr`

##### `impl<T> Spanned for LitCStr`

- <span id="litcstr-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitCStr`

- <span id="cratelitlitcstr-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitCStr`

### `LitByte`

```rust
struct LitByte {
    repr: Box<LitRepr>,
}
```

A byte literal: `b'f'`.

#### Implementations

- <span id="cratelitlitbyte-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitByte`

- <span id="litbyte-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitByte`

- <span id="cratelitlitbyte-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByte`

##### `impl Hash for LitByte`

- <span id="litbyte-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitByte`

- <span id="cratelitlitbyte-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitByte`

- <span id="litbyte-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitByte`

##### `impl<T> Spanned for LitByte`

- <span id="litbyte-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitByte`

- <span id="cratelitlitbyte-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByte`

### `LitChar`

```rust
struct LitChar {
    repr: Box<LitRepr>,
}
```

A character literal: `'a'`.

#### Implementations

- <span id="litchar-new"></span>`fn new(value: char, span: Span) -> Self`

- <span id="litchar-value"></span>`fn value(&self) -> char`

- <span id="litchar-span"></span>`fn span(&self) -> Span`

- <span id="litchar-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litchar-suffix"></span>`fn suffix(&self) -> &str`

- <span id="litchar-token"></span>`fn token(&self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitChar`

- <span id="litchar-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitChar`

- <span id="cratelitlitchar-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitChar`

##### `impl Hash for LitChar`

- <span id="litchar-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitChar`

- <span id="cratelitlitchar-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitChar`

- <span id="litchar-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for LitChar`

##### `impl<T> Spanned for LitChar`

- <span id="litchar-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitChar`

- <span id="cratelitlitchar-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitChar`

### `LitRepr`

```rust
struct LitRepr {
    token: proc_macro2::Literal,
    suffix: Box<str>,
}
```

#### Trait Implementations

##### `impl Clone for LitRepr`

- <span id="litrepr-clone"></span>`fn clone(&self) -> Self`

### `LitInt`

```rust
struct LitInt {
    repr: Box<LitIntRepr>,
}
```

An integer literal: `1` or `1u16`.

#### Implementations

- <span id="litint-new"></span>`fn new(repr: &str, span: Span) -> Self`

- <span id="litint-base10-digits"></span>`fn base10_digits(&self) -> &str`

- <span id="litint-base10-parse"></span>`fn base10_parse<N>(&self) -> Result<N>` — [`Result`](../index.md)

- <span id="litint-suffix"></span>`fn suffix(&self) -> &str`

- <span id="litint-span"></span>`fn span(&self) -> Span`

- <span id="litint-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litint-token"></span>`fn token(&self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitInt`

- <span id="litint-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitInt`

- <span id="cratelitlitint-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitInt`

- <span id="litint-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitInt`

##### `impl Hash for LitInt`

- <span id="litint-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitInt`

- <span id="cratelitlitint-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitInt`

- <span id="litint-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for LitInt`

##### `impl<T> Spanned for LitInt`

- <span id="litint-span"></span>`fn span(&self) -> Span`

##### `impl<T> ToString for LitInt`

- <span id="litint-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lit::LitInt`

- <span id="cratelitlitint-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitInt`

### `LitIntRepr`

```rust
struct LitIntRepr {
    token: proc_macro2::Literal,
    digits: Box<str>,
    suffix: Box<str>,
}
```

#### Trait Implementations

##### `impl Clone for LitIntRepr`

- <span id="litintrepr-clone"></span>`fn clone(&self) -> Self`

### `LitFloat`

```rust
struct LitFloat {
    repr: Box<LitFloatRepr>,
}
```

A floating point literal: `1f64` or `1.0e10f64`.

Must be finite. May not be infinite or NaN.

#### Implementations

- <span id="cratelitlitfloat-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitFloat`

- <span id="litfloat-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitFloat`

- <span id="cratelitlitfloat-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitFloat`

- <span id="litfloat-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitFloat`

##### `impl Hash for LitFloat`

- <span id="litfloat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitFloat`

- <span id="cratelitlitfloat-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitFloat`

- <span id="litfloat-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitFloat`

##### `impl<T> Spanned for LitFloat`

- <span id="litfloat-span"></span>`fn span(&self) -> Span`

##### `impl<T> ToString for LitFloat`

- <span id="litfloat-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lit::LitFloat`

- <span id="cratelitlitfloat-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitFloat`

### `LitFloatRepr`

```rust
struct LitFloatRepr {
    token: proc_macro2::Literal,
    digits: Box<str>,
    suffix: Box<str>,
}
```

#### Trait Implementations

##### `impl Clone for LitFloatRepr`

- <span id="litfloatrepr-clone"></span>`fn clone(&self) -> Self`

### `LitBool`

```rust
struct LitBool {
    pub value: bool,
    pub span: proc_macro2::Span,
}
```

A boolean literal: `true` or `false`.

#### Implementations

- <span id="litbool-new"></span>`fn new(value: bool, span: Span) -> Self`

- <span id="litbool-value"></span>`fn value(&self) -> bool`

- <span id="litbool-span"></span>`fn span(&self) -> Span`

- <span id="litbool-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litbool-token"></span>`fn token(&self) -> Ident` — [`Ident`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::LitBool`

- <span id="cratelitbool-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitBool`

- <span id="cratelitlitbool-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitBool`

##### `impl Hash for crate::LitBool`

- <span id="cratelitbool-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitBool`

- <span id="cratelitlitbool-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::LitBool`

- <span id="cratelitbool-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitBool`

##### `impl<T> Spanned for LitBool`

- <span id="litbool-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitBool`

- <span id="cratelitlitbool-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitBool`

## Enums

### `Lit`

```rust
enum Lit {
    Str(LitStr),
    ByteStr(LitByteStr),
    CStr(LitCStr),
    Byte(LitByte),
    Char(LitChar),
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
    Verbatim(proc_macro2::Literal),
}
```

A Rust literal such as a string or integer or boolean.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Str`**

  A UTF-8 string literal: `"foo"`.

- **`ByteStr`**

  A byte string literal: `b"foo"`.

- **`CStr`**

  A nul-terminated C-string literal: `c"foo"`.

- **`Byte`**

  A byte literal: `b'f'`.

- **`Char`**

  A character literal: `'a'`.

- **`Int`**

  An integer literal: `1` or `1u16`.

- **`Float`**

  A floating point literal: `1f64` or `1.0e10f64`.
  
  Must be finite. May not be infinite or NaN.

- **`Bool`**

  A boolean literal: `true` or `false`.

- **`Verbatim`**

  A raw token literal not interpreted by Syn.

#### Implementations

- <span id="cratelitlit-new"></span>`fn new(token: Literal) -> Self`

- <span id="cratelitlit-from-str"></span>`fn from_str(token: Literal, repr: &str) -> Self`

- <span id="cratelitlit-suffix"></span>`fn suffix(&self) -> &str`

- <span id="cratelitlit-span"></span>`fn span(&self) -> Span`

- <span id="cratelitlit-set-span"></span>`fn set_span(&mut self, span: Span)`

#### Trait Implementations

##### `impl Clone for crate::Lit`

- <span id="cratelit-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Lit`

- <span id="cratelit-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Lit`

##### `impl Hash for crate::Lit`

- <span id="cratelit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::Lit`

- <span id="cratelitlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Lit`

- <span id="cratelit-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::Lit`

##### `impl<T> Spanned for Lit`

- <span id="lit-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Lit`

- <span id="lit-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl Token for crate::lit::Lit`

## Macros

### `lit_extra_traits!`

