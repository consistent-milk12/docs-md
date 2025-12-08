*[syn](../index.md) / [lit](index.md)*

---

# Module `lit`

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

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitStr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitStr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitStr`

##### `impl Hash for LitStr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitStr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitStr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LitStr`

##### `impl<T> Spanned for LitStr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitStr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitStr`

### `LitByteStr`

```rust
struct LitByteStr {
    repr: Box<LitRepr>,
}
```

A byte string literal: `b"foo"`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitByteStr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitByteStr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByteStr`

##### `impl Hash for LitByteStr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitByteStr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitByteStr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LitByteStr`

##### `impl<T> Spanned for LitByteStr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitByteStr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByteStr`

### `LitCStr`

```rust
struct LitCStr {
    repr: Box<LitRepr>,
}
```

A nul-terminated C-string literal: `c"foo"`.

#### Implementations

- `fn new(value: &CStr, span: Span) -> Self`

- `fn value(self: &Self) -> CString`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn suffix(self: &Self) -> &str`

- `fn token(self: &Self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitCStr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitCStr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitCStr`

##### `impl Hash for LitCStr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitCStr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitCStr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LitCStr`

##### `impl<T> Spanned for LitCStr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitCStr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitCStr`

### `LitByte`

```rust
struct LitByte {
    repr: Box<LitRepr>,
}
```

A byte literal: `b'f'`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitByte`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitByte`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByte`

##### `impl Hash for LitByte`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitByte`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitByte`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitByte`

##### `impl<T> Spanned for LitByte`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitByte`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByte`

### `LitChar`

```rust
struct LitChar {
    repr: Box<LitRepr>,
}
```

A character literal: `'a'`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitChar`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitChar`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitChar`

##### `impl Hash for LitChar`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitChar`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitChar`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitChar`

##### `impl<T> Spanned for LitChar`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitChar`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn clone(self: &Self) -> Self`

### `LitInt`

```rust
struct LitInt {
    repr: Box<LitIntRepr>,
}
```

An integer literal: `1` or `1u16`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitInt`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitInt`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitInt`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitInt`

##### `impl Hash for LitInt`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitInt`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitInt`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LitInt`

##### `impl<T> Spanned for LitInt`

- `fn span(self: &Self) -> Span`

##### `impl<T> ToString for LitInt`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for crate::lit::LitInt`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn clone(self: &Self) -> Self`

### `LitFloat`

```rust
struct LitFloat {
    repr: Box<LitFloatRepr>,
}
```

A floating point literal: `1f64` or `1.0e10f64`.

Must be finite. May not be infinite or NaN.

#### Implementations

- `fn new(repr: &str, span: Span) -> Self`

- `fn base10_digits(self: &Self) -> &str`

- `fn base10_parse<N>(self: &Self) -> Result<N>` — [`Result`](../index.md)

- `fn suffix(self: &Self) -> &str`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn token(self: &Self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitFloat`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitFloat`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitFloat`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitFloat`

##### `impl Hash for LitFloat`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitFloat`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LitFloat`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LitFloat`

##### `impl<T> Spanned for LitFloat`

- `fn span(self: &Self) -> Span`

##### `impl<T> ToString for LitFloat`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for crate::lit::LitFloat`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn clone(self: &Self) -> Self`

### `LitBool`

```rust
struct LitBool {
    pub value: bool,
    pub span: proc_macro2::Span,
}
```

A boolean literal: `true` or `false`.

#### Implementations

- `fn new(value: bool, span: Span) -> Self`

- `fn value(self: &Self) -> bool`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn token(self: &Self) -> Ident` — [`Ident`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::LitBool`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitBool`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitBool`

##### `impl Hash for crate::LitBool`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitBool`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::LitBool`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitBool`

##### `impl<T> Spanned for LitBool`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitBool`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn new(token: Literal) -> Self`

- `fn from_str(token: Literal, repr: &str) -> Self`

- `fn suffix(self: &Self) -> &str`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

#### Trait Implementations

##### `impl Clone for crate::Lit`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Lit`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Lit`

##### `impl Hash for crate::Lit`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::Lit`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Lit`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Lit`

##### `impl<T> Spanned for Lit`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Lit`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl Token for crate::lit::Lit`

## Macros

### `lit_extra_traits!`

