*[syn](../index.md) / [token](index.md)*

---

# Module `token`

Tokens representing Rust punctuation, keywords, and delimiters.

The type names in this module can be difficult to keep straight, so we
prefer to use the `Token!` macro instead. This is a type-macro that
expands to the token type of the given token.

# Example

The [`ItemStatic`](../index.md) syntax tree node is defined like this.

```rust
use syn::{Attribute, Expr, Ident, Token, Type, Visibility};

pub struct ItemStatic {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub static_token: Token![static],
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
    pub colon_token: Token![:],
    pub ty: Box<Type>,
    pub eq_token: Token![=],
    pub expr: Box<Expr>,
    pub semi_token: Token![;],
}
```

# Parsing

Keywords and punctuation can be parsed through the `ParseStream::parse`
method. Delimiter tokens are parsed using the `parenthesized!`,
`bracketed!` and `braced!` macros.




```rust
use syn::{Attribute, Result};
use syn::parse::{Parse, ParseStream};

enum ItemStatic {}

// Parse the ItemStatic struct shown above.
impl Parse for ItemStatic {
    fn parse(input: ParseStream) -> Result<Self> {
        use syn::ItemStatic;
        fn parse(input: ParseStream) -> Result<ItemStatic> {
        Ok(ItemStatic {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            static_token: input.parse()?,
            mutability: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            eq_token: input.parse()?,
            expr: input.parse()?,
            semi_token: input.parse()?,
        })
        }
        unimplemented!()
    }
}
```

# Other operations

Every keyword and punctuation token supports the following operations.

- [Peeking] — `input.peek(Token![...])`

- [Parsing] — `input.parse::<Token![...]>()?`

- [Printing] — `quote!( ... #the_token ... )`

- Construction from a `Span` — `let the_token = Token![...](sp)`

- Field access to its span — `let sp = the_token.span`





## Modules

- [`private`](private/index.md) - 

## Structs

### `Underscore`

```rust
struct Underscore {
    pub spans: [Span; 1],
}
```

`_`

Usage:
 wildcard patterns, inferred types, unnamed items in constants, extern crates, use declarations, and destructuring assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Underscore`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Underscore`

##### `impl Debug for Underscore`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Underscore`

- `fn default() -> Self`

##### `impl Deref for Underscore`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Underscore`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Underscore`

##### `impl Hash for Underscore`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Underscore`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Underscore`

- `fn eq(self: &Self, _other: &Underscore) -> bool` — [`Underscore`](#underscore)

##### `impl<P, T> Receiver for Underscore`

- `type Target = T`

##### `impl<T> Sealed for Underscore`

##### `impl<T> Spanned for Underscore`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Underscore`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Underscore`

### `Group`

```rust
struct Group {
    pub span: proc_macro2::Span,
}
```

None-delimited group

#### Implementations

- `fn surround<F>(self: &Self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Clone for Group`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Group`

##### `impl Debug for Group`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Group`

- `fn default() -> Self`

##### `impl Eq for Group`

##### `impl Hash for Group`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl PartialEq for Group`

- `fn eq(self: &Self, _other: &Group) -> bool` — [`Group`](#group)

##### `impl Sealed for Group`

##### `impl Token for Group`

### `Abstract`

```rust
struct Abstract {
    pub span: Span,
}
```

`abstract`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Abstract`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Abstract`

##### `impl Debug for Abstract`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Abstract`

- `fn default() -> Self`

##### `impl Eq for Abstract`

##### `impl Hash for Abstract`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Abstract`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Abstract`

- `fn eq(self: &Self, _other: &Abstract) -> bool` — [`Abstract`](#abstract)

##### `impl<T> Sealed for Abstract`

##### `impl<T> Spanned for Abstract`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Abstract`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Abstract`

### `As`

```rust
struct As {
    pub span: Span,
}
```

`as`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for As`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for As`

##### `impl Debug for As`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for As`

- `fn default() -> Self`

##### `impl Eq for As`

##### `impl Hash for As`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for As`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for As`

- `fn eq(self: &Self, _other: &As) -> bool` — [`As`](#as)

##### `impl Sealed for As`

##### `impl<T> Spanned for As`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for As`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for As`

### `Async`

```rust
struct Async {
    pub span: Span,
}
```

`async`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Async`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Async`

##### `impl Debug for Async`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Async`

- `fn default() -> Self`

##### `impl Eq for Async`

##### `impl Hash for Async`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Async`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Async`

- `fn eq(self: &Self, _other: &Async) -> bool` — [`Async`](#async)

##### `impl<T> Sealed for Async`

##### `impl<T> Spanned for Async`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Async`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Async`

### `Auto`

```rust
struct Auto {
    pub span: Span,
}
```

`auto`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Auto`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Auto`

##### `impl Debug for Auto`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Auto`

- `fn default() -> Self`

##### `impl Eq for Auto`

##### `impl Hash for Auto`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Auto`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Auto`

- `fn eq(self: &Self, _other: &Auto) -> bool` — [`Auto`](#auto)

##### `impl<T> Sealed for Auto`

##### `impl<T> Spanned for Auto`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Auto`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Auto`

### `Await`

```rust
struct Await {
    pub span: Span,
}
```

`await`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Await`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Await`

##### `impl Debug for Await`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Await`

- `fn default() -> Self`

##### `impl Eq for Await`

##### `impl Hash for Await`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Await`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Await`

- `fn eq(self: &Self, _other: &Await) -> bool` — [`Await`](#await)

##### `impl Sealed for Await`

##### `impl<T> Spanned for Await`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Await`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Await`

### `Become`

```rust
struct Become {
    pub span: Span,
}
```

`become`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Become`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Become`

##### `impl Debug for Become`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Become`

- `fn default() -> Self`

##### `impl Eq for Become`

##### `impl Hash for Become`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Become`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Become`

- `fn eq(self: &Self, _other: &Become) -> bool` — [`Become`](#become)

##### `impl<T> Sealed for Become`

##### `impl<T> Spanned for Become`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Become`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Become`

### `Box`

```rust
struct Box {
    pub span: Span,
}
```

`box`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Box`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Box`

##### `impl Debug for Box`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Box`

- `fn default() -> Self`

##### `impl Eq for Box`

##### `impl Hash for Box`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Box`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Box`

- `fn eq(self: &Self, _other: &Box) -> bool` — [`Box`](#box)

##### `impl Sealed for Box`

##### `impl<T> Spanned for Box`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Box`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Box`

### `Break`

```rust
struct Break {
    pub span: Span,
}
```

`break`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Break`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Break`

##### `impl Debug for Break`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Break`

- `fn default() -> Self`

##### `impl Eq for Break`

##### `impl Hash for Break`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Break`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Break`

- `fn eq(self: &Self, _other: &Break) -> bool` — [`Break`](#break)

##### `impl<T> Sealed for Break`

##### `impl<T> Spanned for Break`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Break`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Break`

### `Const`

```rust
struct Const {
    pub span: Span,
}
```

`const`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Const`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Const`

##### `impl Debug for Const`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Const`

- `fn default() -> Self`

##### `impl Eq for Const`

##### `impl Hash for Const`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Const`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Const`

- `fn eq(self: &Self, _other: &Const) -> bool` — [`Const`](#const)

##### `impl Sealed for Const`

##### `impl<T> Spanned for Const`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Const`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Const`

### `Continue`

```rust
struct Continue {
    pub span: Span,
}
```

`continue`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Continue`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Continue`

##### `impl Debug for Continue`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Continue`

- `fn default() -> Self`

##### `impl Eq for Continue`

##### `impl Hash for Continue`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Continue`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Continue`

- `fn eq(self: &Self, _other: &Continue) -> bool` — [`Continue`](#continue)

##### `impl Sealed for Continue`

##### `impl<T> Spanned for Continue`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Continue`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Continue`

### `Crate`

```rust
struct Crate {
    pub span: Span,
}
```

`crate`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Crate`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Crate`

##### `impl Debug for Crate`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Crate`

- `fn default() -> Self`

##### `impl Eq for Crate`

##### `impl Hash for Crate`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Crate`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Crate`

- `fn eq(self: &Self, _other: &Crate) -> bool` — [`Crate`](#crate)

##### `impl<T> Sealed for Crate`

##### `impl<T> Spanned for Crate`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Crate`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Crate`

### `Default`

```rust
struct Default {
    pub span: Span,
}
```

`default`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Default`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Default`

##### `impl Debug for Default`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Default`

- `fn default() -> Self`

##### `impl Eq for Default`

##### `impl Hash for Default`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Default`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Default`

- `fn eq(self: &Self, _other: &Default) -> bool` — [`Default`](#default)

##### `impl<T> Sealed for Default`

##### `impl<T> Spanned for Default`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Default`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Default`

### `Do`

```rust
struct Do {
    pub span: Span,
}
```

`do`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Do`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Do`

##### `impl Debug for Do`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Do`

- `fn default() -> Self`

##### `impl Eq for Do`

##### `impl Hash for Do`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Do`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Do`

- `fn eq(self: &Self, _other: &Do) -> bool` — [`Do`](#do)

##### `impl<T> Sealed for Do`

##### `impl<T> Spanned for Do`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Do`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Do`

### `Dyn`

```rust
struct Dyn {
    pub span: Span,
}
```

`dyn`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Dyn`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Dyn`

##### `impl Debug for Dyn`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dyn`

- `fn default() -> Self`

##### `impl Eq for Dyn`

##### `impl Hash for Dyn`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Dyn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Dyn`

- `fn eq(self: &Self, _other: &Dyn) -> bool` — [`Dyn`](#dyn)

##### `impl Sealed for Dyn`

##### `impl<T> Spanned for Dyn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Dyn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Dyn`

### `Else`

```rust
struct Else {
    pub span: Span,
}
```

`else`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Else`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Else`

##### `impl Debug for Else`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Else`

- `fn default() -> Self`

##### `impl Eq for Else`

##### `impl Hash for Else`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Else`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Else`

- `fn eq(self: &Self, _other: &Else) -> bool` — [`Else`](#else)

##### `impl Sealed for Else`

##### `impl<T> Spanned for Else`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Else`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Else`

### `Enum`

```rust
struct Enum {
    pub span: Span,
}
```

`enum`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Enum`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Enum`

##### `impl Debug for Enum`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Enum`

- `fn default() -> Self`

##### `impl Eq for Enum`

##### `impl Hash for Enum`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Enum`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Enum`

- `fn eq(self: &Self, _other: &Enum) -> bool` — [`Enum`](#enum)

##### `impl<T> Sealed for Enum`

##### `impl<T> Spanned for Enum`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Enum`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Enum`

### `Extern`

```rust
struct Extern {
    pub span: Span,
}
```

`extern`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Extern`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Extern`

##### `impl Debug for Extern`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Extern`

- `fn default() -> Self`

##### `impl Eq for Extern`

##### `impl Hash for Extern`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Extern`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Extern`

- `fn eq(self: &Self, _other: &Extern) -> bool` — [`Extern`](#extern)

##### `impl Sealed for Extern`

##### `impl<T> Spanned for Extern`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Extern`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Extern`

### `Final`

```rust
struct Final {
    pub span: Span,
}
```

`final`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Final`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Final`

##### `impl Debug for Final`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Final`

- `fn default() -> Self`

##### `impl Eq for Final`

##### `impl Hash for Final`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Final`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Final`

- `fn eq(self: &Self, _other: &Final) -> bool` — [`Final`](#final)

##### `impl<T> Sealed for Final`

##### `impl<T> Spanned for Final`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Final`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Final`

### `Fn`

```rust
struct Fn {
    pub span: Span,
}
```

`fn`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Fn`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Fn`

##### `impl Debug for Fn`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Fn`

- `fn default() -> Self`

##### `impl Eq for Fn`

##### `impl Hash for Fn`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Fn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Fn`

- `fn eq(self: &Self, _other: &Fn) -> bool` — [`Fn`](#fn)

##### `impl Sealed for Fn`

##### `impl<T> Spanned for Fn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Fn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Fn`

### `For`

```rust
struct For {
    pub span: Span,
}
```

`for`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for For`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for For`

##### `impl Debug for For`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for For`

- `fn default() -> Self`

##### `impl Eq for For`

##### `impl Hash for For`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for For`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for For`

- `fn eq(self: &Self, _other: &For) -> bool` — [`For`](#for)

##### `impl Sealed for For`

##### `impl<T> Spanned for For`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for For`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for For`

### `If`

```rust
struct If {
    pub span: Span,
}
```

`if`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for If`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for If`

##### `impl Debug for If`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for If`

- `fn default() -> Self`

##### `impl Eq for If`

##### `impl Hash for If`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for If`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for If`

- `fn eq(self: &Self, _other: &If) -> bool` — [`If`](#if)

##### `impl Sealed for If`

##### `impl<T> Spanned for If`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for If`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for If`

### `Impl`

```rust
struct Impl {
    pub span: Span,
}
```

`impl`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Impl`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Impl`

##### `impl Debug for Impl`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Impl`

- `fn default() -> Self`

##### `impl Eq for Impl`

##### `impl Hash for Impl`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Impl`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Impl`

- `fn eq(self: &Self, _other: &Impl) -> bool` — [`Impl`](#impl)

##### `impl<T> Sealed for Impl`

##### `impl<T> Spanned for Impl`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Impl`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Impl`

### `In`

```rust
struct In {
    pub span: Span,
}
```

`in`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for In`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for In`

##### `impl Debug for In`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for In`

- `fn default() -> Self`

##### `impl Eq for In`

##### `impl Hash for In`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for In`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for In`

- `fn eq(self: &Self, _other: &In) -> bool` — [`In`](#in)

##### `impl Sealed for In`

##### `impl<T> Spanned for In`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for In`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for In`

### `Let`

```rust
struct Let {
    pub span: Span,
}
```

`let`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Let`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Let`

##### `impl Debug for Let`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Let`

- `fn default() -> Self`

##### `impl Eq for Let`

##### `impl Hash for Let`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Let`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Let`

- `fn eq(self: &Self, _other: &Let) -> bool` — [`Let`](#let)

##### `impl<T> Sealed for Let`

##### `impl<T> Spanned for Let`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Let`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Let`

### `Loop`

```rust
struct Loop {
    pub span: Span,
}
```

`loop`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Loop`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Loop`

##### `impl Debug for Loop`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Loop`

- `fn default() -> Self`

##### `impl Eq for Loop`

##### `impl Hash for Loop`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Loop`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Loop`

- `fn eq(self: &Self, _other: &Loop) -> bool` — [`Loop`](#loop)

##### `impl<T> Sealed for Loop`

##### `impl<T> Spanned for Loop`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Loop`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Loop`

### `Macro`

```rust
struct Macro {
    pub span: Span,
}
```

`macro`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Macro`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Macro`

##### `impl Debug for Macro`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Macro`

- `fn default() -> Self`

##### `impl Eq for Macro`

##### `impl Hash for Macro`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Macro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Macro`

- `fn eq(self: &Self, _other: &Macro) -> bool` — [`Macro`](#macro)

##### `impl<T> Sealed for Macro`

##### `impl<T> Spanned for Macro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Macro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Macro`

### `Match`

```rust
struct Match {
    pub span: Span,
}
```

`match`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Match`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Match`

##### `impl Debug for Match`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Match`

- `fn default() -> Self`

##### `impl Eq for Match`

##### `impl Hash for Match`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Match`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Match`

- `fn eq(self: &Self, _other: &Match) -> bool` — [`Match`](#match)

##### `impl Sealed for Match`

##### `impl<T> Spanned for Match`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Match`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Match`

### `Mod`

```rust
struct Mod {
    pub span: Span,
}
```

`mod`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Mod`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Mod`

##### `impl Debug for Mod`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Mod`

- `fn default() -> Self`

##### `impl Eq for Mod`

##### `impl Hash for Mod`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Mod`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Mod`

- `fn eq(self: &Self, _other: &Mod) -> bool` — [`Mod`](#mod)

##### `impl<T> Sealed for Mod`

##### `impl<T> Spanned for Mod`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Mod`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Mod`

### `Move`

```rust
struct Move {
    pub span: Span,
}
```

`move`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Move`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Move`

##### `impl Debug for Move`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Move`

- `fn default() -> Self`

##### `impl Eq for Move`

##### `impl Hash for Move`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Move`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Move`

- `fn eq(self: &Self, _other: &Move) -> bool` — [`Move`](#move)

##### `impl<T> Sealed for Move`

##### `impl<T> Spanned for Move`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Move`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Move`

### `Mut`

```rust
struct Mut {
    pub span: Span,
}
```

`mut`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Mut`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Mut`

##### `impl Debug for Mut`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Mut`

- `fn default() -> Self`

##### `impl Eq for Mut`

##### `impl Hash for Mut`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Mut`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Mut`

- `fn eq(self: &Self, _other: &Mut) -> bool` — [`Mut`](#mut)

##### `impl Sealed for Mut`

##### `impl<T> Spanned for Mut`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Mut`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Mut`

### `Override`

```rust
struct Override {
    pub span: Span,
}
```

`override`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Override`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Override`

##### `impl Debug for Override`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Override`

- `fn default() -> Self`

##### `impl Eq for Override`

##### `impl Hash for Override`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Override`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Override`

- `fn eq(self: &Self, _other: &Override) -> bool` — [`Override`](#override)

##### `impl Sealed for Override`

##### `impl<T> Spanned for Override`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Override`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Override`

### `Priv`

```rust
struct Priv {
    pub span: Span,
}
```

`priv`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Priv`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Priv`

##### `impl Debug for Priv`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Priv`

- `fn default() -> Self`

##### `impl Eq for Priv`

##### `impl Hash for Priv`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Priv`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Priv`

- `fn eq(self: &Self, _other: &Priv) -> bool` — [`Priv`](#priv)

##### `impl Sealed for Priv`

##### `impl<T> Spanned for Priv`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Priv`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Priv`

### `Pub`

```rust
struct Pub {
    pub span: Span,
}
```

`pub`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Pub`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Pub`

##### `impl Debug for Pub`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Pub`

- `fn default() -> Self`

##### `impl Eq for Pub`

##### `impl Hash for Pub`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Pub`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Pub`

- `fn eq(self: &Self, _other: &Pub) -> bool` — [`Pub`](#pub)

##### `impl<T> Sealed for Pub`

##### `impl<T> Spanned for Pub`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Pub`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Pub`

### `Raw`

```rust
struct Raw {
    pub span: Span,
}
```

`raw`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Raw`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Raw`

##### `impl Debug for Raw`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Raw`

- `fn default() -> Self`

##### `impl Eq for Raw`

##### `impl Hash for Raw`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Raw`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Raw`

- `fn eq(self: &Self, _other: &Raw) -> bool` — [`Raw`](#raw)

##### `impl<T> Sealed for Raw`

##### `impl<T> Spanned for Raw`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Raw`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Raw`

### `Ref`

```rust
struct Ref {
    pub span: Span,
}
```

`ref`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Ref`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Ref`

##### `impl Debug for Ref`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Ref`

- `fn default() -> Self`

##### `impl Eq for Ref`

##### `impl Hash for Ref`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Ref`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Ref`

- `fn eq(self: &Self, _other: &Ref) -> bool` — [`Ref`](#ref)

##### `impl<T> Sealed for Ref`

##### `impl<T> Spanned for Ref`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Ref`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Ref`

### `Return`

```rust
struct Return {
    pub span: Span,
}
```

`return`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Return`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Return`

##### `impl Debug for Return`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Return`

- `fn default() -> Self`

##### `impl Eq for Return`

##### `impl Hash for Return`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Return`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Return`

- `fn eq(self: &Self, _other: &Return) -> bool` — [`Return`](#return)

##### `impl Sealed for Return`

##### `impl<T> Spanned for Return`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Return`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Return`

### `SelfType`

```rust
struct SelfType {
    pub span: Span,
}
```

`Self`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for SelfType`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for SelfType`

##### `impl Debug for SelfType`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SelfType`

- `fn default() -> Self`

##### `impl Eq for SelfType`

##### `impl Hash for SelfType`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for SelfType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for SelfType`

- `fn eq(self: &Self, _other: &SelfType) -> bool` — [`SelfType`](#selftype)

##### `impl<T> Sealed for SelfType`

##### `impl<T> Spanned for SelfType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for SelfType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for SelfType`

### `SelfValue`

```rust
struct SelfValue {
    pub span: Span,
}
```

`self`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for SelfValue`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for SelfValue`

##### `impl Debug for SelfValue`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SelfValue`

- `fn default() -> Self`

##### `impl Eq for SelfValue`

##### `impl Hash for SelfValue`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for SelfValue`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for SelfValue`

- `fn eq(self: &Self, _other: &SelfValue) -> bool` — [`SelfValue`](#selfvalue)

##### `impl<T> Sealed for SelfValue`

##### `impl<T> Spanned for SelfValue`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for SelfValue`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for SelfValue`

### `Static`

```rust
struct Static {
    pub span: Span,
}
```

`static`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Static`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Static`

##### `impl Debug for Static`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Static`

- `fn default() -> Self`

##### `impl Eq for Static`

##### `impl Hash for Static`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Static`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Static`

- `fn eq(self: &Self, _other: &Static) -> bool` — [`Static`](#static)

##### `impl Sealed for Static`

##### `impl<T> Spanned for Static`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Static`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Static`

### `Struct`

```rust
struct Struct {
    pub span: Span,
}
```

`struct`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Struct`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Struct`

##### `impl Debug for Struct`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Struct`

- `fn default() -> Self`

##### `impl Eq for Struct`

##### `impl Hash for Struct`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Struct`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Struct`

- `fn eq(self: &Self, _other: &Struct) -> bool` — [`Struct`](#struct)

##### `impl Sealed for Struct`

##### `impl<T> Spanned for Struct`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Struct`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Struct`

### `Super`

```rust
struct Super {
    pub span: Span,
}
```

`super`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Super`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Super`

##### `impl Debug for Super`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Super`

- `fn default() -> Self`

##### `impl Eq for Super`

##### `impl Hash for Super`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Super`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Super`

- `fn eq(self: &Self, _other: &Super) -> bool` — [`Super`](#super)

##### `impl Sealed for Super`

##### `impl<T> Spanned for Super`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Super`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Super`

### `Trait`

```rust
struct Trait {
    pub span: Span,
}
```

`trait`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Trait`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Trait`

##### `impl Debug for Trait`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Trait`

- `fn default() -> Self`

##### `impl Eq for Trait`

##### `impl Hash for Trait`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Trait`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Trait`

- `fn eq(self: &Self, _other: &Trait) -> bool` — [`Trait`](#trait)

##### `impl Sealed for Trait`

##### `impl<T> Spanned for Trait`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Trait`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Trait`

### `Try`

```rust
struct Try {
    pub span: Span,
}
```

`try`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Try`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Try`

##### `impl Debug for Try`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Try`

- `fn default() -> Self`

##### `impl Eq for Try`

##### `impl Hash for Try`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Try`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Try`

- `fn eq(self: &Self, _other: &Try) -> bool` — [`Try`](#try)

##### `impl Sealed for Try`

##### `impl<T> Spanned for Try`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Try`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Try`

### `Type`

```rust
struct Type {
    pub span: Span,
}
```

`type`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Type`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Type`

##### `impl Debug for Type`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Type`

- `fn default() -> Self`

##### `impl Eq for Type`

##### `impl Hash for Type`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Type`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Type`

- `fn eq(self: &Self, _other: &Type) -> bool` — [`Type`](#type)

##### `impl Sealed for Type`

##### `impl<T> Spanned for Type`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Type`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Type`

### `Typeof`

```rust
struct Typeof {
    pub span: Span,
}
```

`typeof`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Typeof`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Typeof`

##### `impl Debug for Typeof`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Typeof`

- `fn default() -> Self`

##### `impl Eq for Typeof`

##### `impl Hash for Typeof`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Typeof`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Typeof`

- `fn eq(self: &Self, _other: &Typeof) -> bool` — [`Typeof`](#typeof)

##### `impl Sealed for Typeof`

##### `impl<T> Spanned for Typeof`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Typeof`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Typeof`

### `Union`

```rust
struct Union {
    pub span: Span,
}
```

`union`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Union`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Union`

##### `impl Debug for Union`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Union`

- `fn default() -> Self`

##### `impl Eq for Union`

##### `impl Hash for Union`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Union`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Union`

- `fn eq(self: &Self, _other: &Union) -> bool` — [`Union`](#union)

##### `impl Sealed for Union`

##### `impl<T> Spanned for Union`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Union`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Union`

### `Unsafe`

```rust
struct Unsafe {
    pub span: Span,
}
```

`unsafe`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Unsafe`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Unsafe`

##### `impl Debug for Unsafe`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Unsafe`

- `fn default() -> Self`

##### `impl Eq for Unsafe`

##### `impl Hash for Unsafe`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Unsafe`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Unsafe`

- `fn eq(self: &Self, _other: &Unsafe) -> bool` — [`Unsafe`](#unsafe)

##### `impl Sealed for Unsafe`

##### `impl<T> Spanned for Unsafe`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Unsafe`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Unsafe`

### `Unsized`

```rust
struct Unsized {
    pub span: Span,
}
```

`unsized`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Unsized`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Unsized`

##### `impl Debug for Unsized`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Unsized`

- `fn default() -> Self`

##### `impl Eq for Unsized`

##### `impl Hash for Unsized`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Unsized`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Unsized`

- `fn eq(self: &Self, _other: &Unsized) -> bool` — [`Unsized`](#unsized)

##### `impl<T> Sealed for Unsized`

##### `impl<T> Spanned for Unsized`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Unsized`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Unsized`

### `Use`

```rust
struct Use {
    pub span: Span,
}
```

`use`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Use`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Use`

##### `impl Debug for Use`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Use`

- `fn default() -> Self`

##### `impl Eq for Use`

##### `impl Hash for Use`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Use`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Use`

- `fn eq(self: &Self, _other: &Use) -> bool` — [`Use`](#use)

##### `impl<T> Sealed for Use`

##### `impl<T> Spanned for Use`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Use`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Use`

### `Virtual`

```rust
struct Virtual {
    pub span: Span,
}
```

`virtual`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Virtual`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Virtual`

##### `impl Debug for Virtual`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Virtual`

- `fn default() -> Self`

##### `impl Eq for Virtual`

##### `impl Hash for Virtual`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Virtual`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Virtual`

- `fn eq(self: &Self, _other: &Virtual) -> bool` — [`Virtual`](#virtual)

##### `impl<T> Sealed for Virtual`

##### `impl<T> Spanned for Virtual`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Virtual`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Virtual`

### `Where`

```rust
struct Where {
    pub span: Span,
}
```

`where`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Where`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Where`

##### `impl Debug for Where`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Where`

- `fn default() -> Self`

##### `impl Eq for Where`

##### `impl Hash for Where`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Where`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Where`

- `fn eq(self: &Self, _other: &Where) -> bool` — [`Where`](#where)

##### `impl<T> Sealed for Where`

##### `impl<T> Spanned for Where`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Where`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Where`

### `While`

```rust
struct While {
    pub span: Span,
}
```

`while`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for While`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for While`

##### `impl Debug for While`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for While`

- `fn default() -> Self`

##### `impl Eq for While`

##### `impl Hash for While`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for While`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for While`

- `fn eq(self: &Self, _other: &While) -> bool` — [`While`](#while)

##### `impl<T> Sealed for While`

##### `impl<T> Spanned for While`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for While`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for While`

### `Yield`

```rust
struct Yield {
    pub span: Span,
}
```

`yield`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Yield`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Yield`

##### `impl Debug for Yield`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Yield`

- `fn default() -> Self`

##### `impl Eq for Yield`

##### `impl Hash for Yield`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Yield`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Yield`

- `fn eq(self: &Self, _other: &Yield) -> bool` — [`Yield`](#yield)

##### `impl Sealed for Yield`

##### `impl<T> Spanned for Yield`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Yield`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Yield`

### `And`

```rust
struct And {
    pub spans: [Span; 1],
}
```

`&`

Usage:
 bitwise and logical AND, borrow, references, reference patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for And`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for And`

##### `impl Debug for And`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for And`

- `fn default() -> Self`

##### `impl Deref for And`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for And`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for And`

##### `impl Hash for And`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for And`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for And`

- `fn eq(self: &Self, _other: &And) -> bool` — [`And`](#and)

##### `impl<P, T> Receiver for And`

- `type Target = T`

##### `impl Sealed for And`

##### `impl<T> Spanned for And`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for And`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for And`

### `AndAnd`

```rust
struct AndAnd {
    pub spans: [Span; 2],
}
```

`&&`

Usage:
 lazy AND, borrow, references, reference patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for AndAnd`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for AndAnd`

##### `impl Debug for AndAnd`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AndAnd`

- `fn default() -> Self`

##### `impl Eq for AndAnd`

##### `impl Hash for AndAnd`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for AndAnd`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for AndAnd`

- `fn eq(self: &Self, _other: &AndAnd) -> bool` — [`AndAnd`](#andand)

##### `impl Sealed for AndAnd`

##### `impl<T> Spanned for AndAnd`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for AndAnd`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for AndAnd`

### `AndEq`

```rust
struct AndEq {
    pub spans: [Span; 2],
}
```

`&=`

Usage:
 bitwise AND assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for AndEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for AndEq`

##### `impl Debug for AndEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AndEq`

- `fn default() -> Self`

##### `impl Eq for AndEq`

##### `impl Hash for AndEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for AndEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for AndEq`

- `fn eq(self: &Self, _other: &AndEq) -> bool` — [`AndEq`](#andeq)

##### `impl<T> Sealed for AndEq`

##### `impl<T> Spanned for AndEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for AndEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for AndEq`

### `At`

```rust
struct At {
    pub spans: [Span; 1],
}
```

`@`

Usage:
 subpattern binding.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for At`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for At`

##### `impl Debug for At`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for At`

- `fn default() -> Self`

##### `impl Deref for At`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for At`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for At`

##### `impl Hash for At`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for At`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for At`

- `fn eq(self: &Self, _other: &At) -> bool` — [`At`](#at)

##### `impl<P, T> Receiver for At`

- `type Target = T`

##### `impl<T> Sealed for At`

##### `impl<T> Spanned for At`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for At`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for At`

### `Caret`

```rust
struct Caret {
    pub spans: [Span; 1],
}
```

`^`

Usage:
 bitwise and logical XOR.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Caret`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Caret`

##### `impl Debug for Caret`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Caret`

- `fn default() -> Self`

##### `impl Deref for Caret`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Caret`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Caret`

##### `impl Hash for Caret`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Caret`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Caret`

- `fn eq(self: &Self, _other: &Caret) -> bool` — [`Caret`](#caret)

##### `impl<P, T> Receiver for Caret`

- `type Target = T`

##### `impl Sealed for Caret`

##### `impl<T> Spanned for Caret`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Caret`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Caret`

### `CaretEq`

```rust
struct CaretEq {
    pub spans: [Span; 2],
}
```

`^=`

Usage:
 bitwise XOR assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for CaretEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for CaretEq`

##### `impl Debug for CaretEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CaretEq`

- `fn default() -> Self`

##### `impl Eq for CaretEq`

##### `impl Hash for CaretEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for CaretEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for CaretEq`

- `fn eq(self: &Self, _other: &CaretEq) -> bool` — [`CaretEq`](#careteq)

##### `impl<T> Sealed for CaretEq`

##### `impl<T> Spanned for CaretEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for CaretEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for CaretEq`

### `Colon`

```rust
struct Colon {
    pub spans: [Span; 1],
}
```

`:`

Usage:
 various separators.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Colon`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Colon`

##### `impl Debug for Colon`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Colon`

- `fn default() -> Self`

##### `impl Deref for Colon`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Colon`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Colon`

##### `impl Hash for Colon`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Colon`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Colon`

- `fn eq(self: &Self, _other: &Colon) -> bool` — [`Colon`](#colon)

##### `impl<P, T> Receiver for Colon`

- `type Target = T`

##### `impl<T> Sealed for Colon`

##### `impl<T> Spanned for Colon`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Colon`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Colon`

### `Comma`

```rust
struct Comma {
    pub spans: [Span; 1],
}
```

`,`

Usage:
 various separators.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Comma`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Comma`

##### `impl Debug for Comma`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Comma`

- `fn default() -> Self`

##### `impl Deref for Comma`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Comma`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Comma`

##### `impl Hash for Comma`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Comma`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Comma`

- `fn eq(self: &Self, _other: &Comma) -> bool` — [`Comma`](#comma)

##### `impl<P, T> Receiver for Comma`

- `type Target = T`

##### `impl Sealed for Comma`

##### `impl<T> Spanned for Comma`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Comma`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Comma`

### `Dollar`

```rust
struct Dollar {
    pub spans: [Span; 1],
}
```

`$`

Usage:
 macros.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Dollar`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Dollar`

##### `impl Debug for Dollar`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dollar`

- `fn default() -> Self`

##### `impl Deref for Dollar`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Dollar`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Dollar`

##### `impl Hash for Dollar`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Dollar`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Dollar`

- `fn eq(self: &Self, _other: &Dollar) -> bool` — [`Dollar`](#dollar)

##### `impl<P, T> Receiver for Dollar`

- `type Target = T`

##### `impl<T> Sealed for Dollar`

##### `impl<T> Spanned for Dollar`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Dollar`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Dollar`

### `Dot`

```rust
struct Dot {
    pub spans: [Span; 1],
}
```

`.`

Usage:
 field access, tuple index.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Dot`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Dot`

##### `impl Debug for Dot`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dot`

- `fn default() -> Self`

##### `impl Deref for Dot`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Dot`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Dot`

##### `impl Hash for Dot`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Dot`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Dot`

- `fn eq(self: &Self, _other: &Dot) -> bool` — [`Dot`](#dot)

##### `impl<P, T> Receiver for Dot`

- `type Target = T`

##### `impl Sealed for Dot`

##### `impl<T> Spanned for Dot`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Dot`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Dot`

### `DotDot`

```rust
struct DotDot {
    pub spans: [Span; 2],
}
```

`..`

Usage:
 range, struct expressions, patterns, range patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for DotDot`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for DotDot`

##### `impl Debug for DotDot`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DotDot`

- `fn default() -> Self`

##### `impl Eq for DotDot`

##### `impl Hash for DotDot`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for DotDot`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for DotDot`

- `fn eq(self: &Self, _other: &DotDot) -> bool` — [`DotDot`](#dotdot)

##### `impl<T> Sealed for DotDot`

##### `impl<T> Spanned for DotDot`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for DotDot`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for DotDot`

### `DotDotDot`

```rust
struct DotDotDot {
    pub spans: [Span; 3],
}
```

`...`

Usage:
 variadic functions, range patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for DotDotDot`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for DotDotDot`

##### `impl Debug for DotDotDot`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DotDotDot`

- `fn default() -> Self`

##### `impl Eq for DotDotDot`

##### `impl Hash for DotDotDot`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for DotDotDot`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for DotDotDot`

- `fn eq(self: &Self, _other: &DotDotDot) -> bool` — [`DotDotDot`](#dotdotdot)

##### `impl<T> Sealed for DotDotDot`

##### `impl<T> Spanned for DotDotDot`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for DotDotDot`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for DotDotDot`

### `DotDotEq`

```rust
struct DotDotEq {
    pub spans: [Span; 3],
}
```

`..=`

Usage:
 inclusive range, range patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for DotDotEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for DotDotEq`

##### `impl Debug for DotDotEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DotDotEq`

- `fn default() -> Self`

##### `impl Eq for DotDotEq`

##### `impl Hash for DotDotEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for DotDotEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for DotDotEq`

- `fn eq(self: &Self, _other: &DotDotEq) -> bool` — [`DotDotEq`](#dotdoteq)

##### `impl Sealed for DotDotEq`

##### `impl<T> Spanned for DotDotEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for DotDotEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for DotDotEq`

### `Eq`

```rust
struct Eq {
    pub spans: [Span; 1],
}
```

`=`

Usage:
 assignment, attributes, various type definitions.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Eq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Eq`

##### `impl Debug for Eq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Eq`

- `fn default() -> Self`

##### `impl Deref for Eq`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Eq`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Eq`

##### `impl Hash for Eq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Eq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Eq`

- `fn eq(self: &Self, _other: &Eq) -> bool` — [`Eq`](#eq)

##### `impl<P, T> Receiver for Eq`

- `type Target = T`

##### `impl<T> Sealed for Eq`

##### `impl<T> Spanned for Eq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Eq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Eq`

### `EqEq`

```rust
struct EqEq {
    pub spans: [Span; 2],
}
```

`==`

Usage:
 equal.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for EqEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for EqEq`

##### `impl Debug for EqEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EqEq`

- `fn default() -> Self`

##### `impl Eq for EqEq`

##### `impl Hash for EqEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for EqEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for EqEq`

- `fn eq(self: &Self, _other: &EqEq) -> bool` — [`EqEq`](#eqeq)

##### `impl<T> Sealed for EqEq`

##### `impl<T> Spanned for EqEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for EqEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for EqEq`

### `FatArrow`

```rust
struct FatArrow {
    pub spans: [Span; 2],
}
```

`=>`

Usage:
 match arms, macros.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for FatArrow`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for FatArrow`

##### `impl Debug for FatArrow`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FatArrow`

- `fn default() -> Self`

##### `impl Eq for FatArrow`

##### `impl Hash for FatArrow`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for FatArrow`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for FatArrow`

- `fn eq(self: &Self, _other: &FatArrow) -> bool` — [`FatArrow`](#fatarrow)

##### `impl<T> Sealed for FatArrow`

##### `impl<T> Spanned for FatArrow`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for FatArrow`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for FatArrow`

### `Ge`

```rust
struct Ge {
    pub spans: [Span; 2],
}
```

`>=`

Usage:
 greater than or equal to, generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Ge`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Ge`

##### `impl Debug for Ge`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Ge`

- `fn default() -> Self`

##### `impl Eq for Ge`

##### `impl Hash for Ge`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Ge`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Ge`

- `fn eq(self: &Self, _other: &Ge) -> bool` — [`Ge`](#ge)

##### `impl Sealed for Ge`

##### `impl<T> Spanned for Ge`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Ge`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Ge`

### `Gt`

```rust
struct Gt {
    pub spans: [Span; 1],
}
```

`>`

Usage:
 greater than, generics, paths.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Gt`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Gt`

##### `impl Debug for Gt`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Gt`

- `fn default() -> Self`

##### `impl Deref for Gt`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Gt`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Gt`

##### `impl Hash for Gt`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Gt`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Gt`

- `fn eq(self: &Self, _other: &Gt) -> bool` — [`Gt`](#gt)

##### `impl<P, T> Receiver for Gt`

- `type Target = T`

##### `impl Sealed for Gt`

##### `impl<T> Spanned for Gt`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Gt`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Gt`

### `LArrow`

```rust
struct LArrow {
    pub spans: [Span; 2],
}
```

`<-`

Usage:
 unused.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for LArrow`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for LArrow`

##### `impl Debug for LArrow`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LArrow`

- `fn default() -> Self`

##### `impl Eq for LArrow`

##### `impl Hash for LArrow`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for LArrow`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for LArrow`

- `fn eq(self: &Self, _other: &LArrow) -> bool` — [`LArrow`](#larrow)

##### `impl Sealed for LArrow`

##### `impl<T> Spanned for LArrow`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for LArrow`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for LArrow`

### `Le`

```rust
struct Le {
    pub spans: [Span; 2],
}
```

`<=`

Usage:
 less than or equal to.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Le`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Le`

##### `impl Debug for Le`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Le`

- `fn default() -> Self`

##### `impl Eq for Le`

##### `impl Hash for Le`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Le`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Le`

- `fn eq(self: &Self, _other: &Le) -> bool` — [`Le`](#le)

##### `impl<T> Sealed for Le`

##### `impl<T> Spanned for Le`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Le`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Le`

### `Lt`

```rust
struct Lt {
    pub spans: [Span; 1],
}
```

`<`

Usage:
 less than, generics, paths.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Lt`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Lt`

##### `impl Debug for Lt`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Lt`

- `fn default() -> Self`

##### `impl Deref for Lt`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Lt`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Lt`

##### `impl Hash for Lt`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Lt`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Lt`

- `fn eq(self: &Self, _other: &Lt) -> bool` — [`Lt`](#lt)

##### `impl<P, T> Receiver for Lt`

- `type Target = T`

##### `impl Sealed for Lt`

##### `impl<T> Spanned for Lt`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Lt`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Lt`

### `Minus`

```rust
struct Minus {
    pub spans: [Span; 1],
}
```

`-`

Usage:
 subtraction, negation.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Minus`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Minus`

##### `impl Debug for Minus`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Minus`

- `fn default() -> Self`

##### `impl Deref for Minus`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Minus`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Minus`

##### `impl Hash for Minus`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Minus`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Minus`

- `fn eq(self: &Self, _other: &Minus) -> bool` — [`Minus`](#minus)

##### `impl<P, T> Receiver for Minus`

- `type Target = T`

##### `impl<T> Sealed for Minus`

##### `impl<T> Spanned for Minus`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Minus`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Minus`

### `MinusEq`

```rust
struct MinusEq {
    pub spans: [Span; 2],
}
```

`-=`

Usage:
 subtraction assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for MinusEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for MinusEq`

##### `impl Debug for MinusEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MinusEq`

- `fn default() -> Self`

##### `impl Eq for MinusEq`

##### `impl Hash for MinusEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for MinusEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for MinusEq`

- `fn eq(self: &Self, _other: &MinusEq) -> bool` — [`MinusEq`](#minuseq)

##### `impl Sealed for MinusEq`

##### `impl<T> Spanned for MinusEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for MinusEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for MinusEq`

### `Ne`

```rust
struct Ne {
    pub spans: [Span; 2],
}
```

`!=`

Usage:
 not equal.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Ne`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Ne`

##### `impl Debug for Ne`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Ne`

- `fn default() -> Self`

##### `impl Eq for Ne`

##### `impl Hash for Ne`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Ne`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Ne`

- `fn eq(self: &Self, _other: &Ne) -> bool` — [`Ne`](#ne)

##### `impl Sealed for Ne`

##### `impl<T> Spanned for Ne`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Ne`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Ne`

### `Not`

```rust
struct Not {
    pub spans: [Span; 1],
}
```

`!`

Usage:
 bitwise and logical NOT, macro calls, inner attributes, never type, negative impls.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Not`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Not`

##### `impl Debug for Not`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Not`

- `fn default() -> Self`

##### `impl Deref for Not`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Not`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Not`

##### `impl Hash for Not`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Not`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Not`

- `fn eq(self: &Self, _other: &Not) -> bool` — [`Not`](#not)

##### `impl<P, T> Receiver for Not`

- `type Target = T`

##### `impl<T> Sealed for Not`

##### `impl<T> Spanned for Not`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Not`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Not`

### `Or`

```rust
struct Or {
    pub spans: [Span; 1],
}
```

`|`

Usage:
 bitwise and logical OR, closures, patterns in match, if let, and while let.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Or`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Or`

##### `impl Debug for Or`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Or`

- `fn default() -> Self`

##### `impl Deref for Or`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Or`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Or`

##### `impl Hash for Or`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Or`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Or`

- `fn eq(self: &Self, _other: &Or) -> bool` — [`Or`](#or)

##### `impl<P, T> Receiver for Or`

- `type Target = T`

##### `impl<T> Sealed for Or`

##### `impl<T> Spanned for Or`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Or`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Or`

### `OrEq`

```rust
struct OrEq {
    pub spans: [Span; 2],
}
```

`|=`

Usage:
 bitwise OR assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for OrEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for OrEq`

##### `impl Debug for OrEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OrEq`

- `fn default() -> Self`

##### `impl Eq for OrEq`

##### `impl Hash for OrEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for OrEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for OrEq`

- `fn eq(self: &Self, _other: &OrEq) -> bool` — [`OrEq`](#oreq)

##### `impl<T> Sealed for OrEq`

##### `impl<T> Spanned for OrEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for OrEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for OrEq`

### `OrOr`

```rust
struct OrOr {
    pub spans: [Span; 2],
}
```

`||`

Usage:
 lazy OR, closures.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for OrOr`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for OrOr`

##### `impl Debug for OrOr`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OrOr`

- `fn default() -> Self`

##### `impl Eq for OrOr`

##### `impl Hash for OrOr`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for OrOr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for OrOr`

- `fn eq(self: &Self, _other: &OrOr) -> bool` — [`OrOr`](#oror)

##### `impl<T> Sealed for OrOr`

##### `impl<T> Spanned for OrOr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for OrOr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for OrOr`

### `PathSep`

```rust
struct PathSep {
    pub spans: [Span; 2],
}
```

`::`

Usage:
 path separator.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for PathSep`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for PathSep`

##### `impl Debug for PathSep`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathSep`

- `fn default() -> Self`

##### `impl Eq for PathSep`

##### `impl Hash for PathSep`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for PathSep`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for PathSep`

- `fn eq(self: &Self, _other: &PathSep) -> bool` — [`PathSep`](#pathsep)

##### `impl Sealed for PathSep`

##### `impl<T> Spanned for PathSep`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for PathSep`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for PathSep`

### `Percent`

```rust
struct Percent {
    pub spans: [Span; 1],
}
```

`%`

Usage:
 remainder.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Percent`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Percent`

##### `impl Debug for Percent`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Percent`

- `fn default() -> Self`

##### `impl Deref for Percent`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Percent`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Percent`

##### `impl Hash for Percent`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Percent`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Percent`

- `fn eq(self: &Self, _other: &Percent) -> bool` — [`Percent`](#percent)

##### `impl<P, T> Receiver for Percent`

- `type Target = T`

##### `impl<T> Sealed for Percent`

##### `impl<T> Spanned for Percent`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Percent`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Percent`

### `PercentEq`

```rust
struct PercentEq {
    pub spans: [Span; 2],
}
```

`%=`

Usage:
 remainder assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for PercentEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for PercentEq`

##### `impl Debug for PercentEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PercentEq`

- `fn default() -> Self`

##### `impl Eq for PercentEq`

##### `impl Hash for PercentEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for PercentEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for PercentEq`

- `fn eq(self: &Self, _other: &PercentEq) -> bool` — [`PercentEq`](#percenteq)

##### `impl<T> Sealed for PercentEq`

##### `impl<T> Spanned for PercentEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for PercentEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for PercentEq`

### `Plus`

```rust
struct Plus {
    pub spans: [Span; 1],
}
```

`+`

Usage:
 addition, trait bounds, macro Kleene matcher.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Plus`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Plus`

##### `impl Debug for Plus`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Plus`

- `fn default() -> Self`

##### `impl Deref for Plus`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Plus`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Plus`

##### `impl Hash for Plus`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Plus`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Plus`

- `fn eq(self: &Self, _other: &Plus) -> bool` — [`Plus`](#plus)

##### `impl<P, T> Receiver for Plus`

- `type Target = T`

##### `impl Sealed for Plus`

##### `impl<T> Spanned for Plus`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Plus`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Plus`

### `PlusEq`

```rust
struct PlusEq {
    pub spans: [Span; 2],
}
```

`+=`

Usage:
 addition assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for PlusEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for PlusEq`

##### `impl Debug for PlusEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PlusEq`

- `fn default() -> Self`

##### `impl Eq for PlusEq`

##### `impl Hash for PlusEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for PlusEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for PlusEq`

- `fn eq(self: &Self, _other: &PlusEq) -> bool` — [`PlusEq`](#pluseq)

##### `impl Sealed for PlusEq`

##### `impl<T> Spanned for PlusEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for PlusEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for PlusEq`

### `Pound`

```rust
struct Pound {
    pub spans: [Span; 1],
}
```

`#`

Usage:
 attributes.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Pound`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Pound`

##### `impl Debug for Pound`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Pound`

- `fn default() -> Self`

##### `impl Deref for Pound`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Pound`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Pound`

##### `impl Hash for Pound`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Pound`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Pound`

- `fn eq(self: &Self, _other: &Pound) -> bool` — [`Pound`](#pound)

##### `impl<P, T> Receiver for Pound`

- `type Target = T`

##### `impl Sealed for Pound`

##### `impl<T> Spanned for Pound`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Pound`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Pound`

### `Question`

```rust
struct Question {
    pub spans: [Span; 1],
}
```

`?`

Usage:
 question mark operator, questionably sized, macro Kleene matcher.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Question`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Question`

##### `impl Debug for Question`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Question`

- `fn default() -> Self`

##### `impl Deref for Question`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Question`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Question`

##### `impl Hash for Question`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Question`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Question`

- `fn eq(self: &Self, _other: &Question) -> bool` — [`Question`](#question)

##### `impl<P, T> Receiver for Question`

- `type Target = T`

##### `impl<T> Sealed for Question`

##### `impl<T> Spanned for Question`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Question`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Question`

### `RArrow`

```rust
struct RArrow {
    pub spans: [Span; 2],
}
```

`->`

Usage:
 function return type, closure return type, function pointer type.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for RArrow`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for RArrow`

##### `impl Debug for RArrow`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RArrow`

- `fn default() -> Self`

##### `impl Eq for RArrow`

##### `impl Hash for RArrow`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for RArrow`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for RArrow`

- `fn eq(self: &Self, _other: &RArrow) -> bool` — [`RArrow`](#rarrow)

##### `impl<T> Sealed for RArrow`

##### `impl<T> Spanned for RArrow`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for RArrow`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for RArrow`

### `Semi`

```rust
struct Semi {
    pub spans: [Span; 1],
}
```

`;`

Usage:
 terminator for various items and statements, array types.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Semi`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Semi`

##### `impl Debug for Semi`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Semi`

- `fn default() -> Self`

##### `impl Deref for Semi`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Semi`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Semi`

##### `impl Hash for Semi`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Semi`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Semi`

- `fn eq(self: &Self, _other: &Semi) -> bool` — [`Semi`](#semi)

##### `impl<P, T> Receiver for Semi`

- `type Target = T`

##### `impl Sealed for Semi`

##### `impl<T> Spanned for Semi`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Semi`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Semi`

### `Shl`

```rust
struct Shl {
    pub spans: [Span; 2],
}
```

`<<`

Usage:
 shift left, nested generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Shl`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Shl`

##### `impl Debug for Shl`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Shl`

- `fn default() -> Self`

##### `impl Eq for Shl`

##### `impl Hash for Shl`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Shl`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Shl`

- `fn eq(self: &Self, _other: &Shl) -> bool` — [`Shl`](#shl)

##### `impl<T> Sealed for Shl`

##### `impl<T> Spanned for Shl`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Shl`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Shl`

### `ShlEq`

```rust
struct ShlEq {
    pub spans: [Span; 3],
}
```

`<<=`

Usage:
 shift left assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for ShlEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for ShlEq`

##### `impl Debug for ShlEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ShlEq`

- `fn default() -> Self`

##### `impl Eq for ShlEq`

##### `impl Hash for ShlEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for ShlEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for ShlEq`

- `fn eq(self: &Self, _other: &ShlEq) -> bool` — [`ShlEq`](#shleq)

##### `impl Sealed for ShlEq`

##### `impl<T> Spanned for ShlEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for ShlEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for ShlEq`

### `Shr`

```rust
struct Shr {
    pub spans: [Span; 2],
}
```

`>>`

Usage:
 shift right, nested generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Shr`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Shr`

##### `impl Debug for Shr`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Shr`

- `fn default() -> Self`

##### `impl Eq for Shr`

##### `impl Hash for Shr`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Shr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Shr`

- `fn eq(self: &Self, _other: &Shr) -> bool` — [`Shr`](#shr)

##### `impl<T> Sealed for Shr`

##### `impl<T> Spanned for Shr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Shr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Shr`

### `ShrEq`

```rust
struct ShrEq {
    pub spans: [Span; 3],
}
```

`>>=`

Usage:
 shift right assignment, nested generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for ShrEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for ShrEq`

##### `impl Debug for ShrEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ShrEq`

- `fn default() -> Self`

##### `impl Eq for ShrEq`

##### `impl Hash for ShrEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for ShrEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for ShrEq`

- `fn eq(self: &Self, _other: &ShrEq) -> bool` — [`ShrEq`](#shreq)

##### `impl Sealed for ShrEq`

##### `impl<T> Spanned for ShrEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for ShrEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for ShrEq`

### `Slash`

```rust
struct Slash {
    pub spans: [Span; 1],
}
```

`/`

Usage:
 division.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Slash`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Slash`

##### `impl Debug for Slash`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Slash`

- `fn default() -> Self`

##### `impl Deref for Slash`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Slash`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Slash`

##### `impl Hash for Slash`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Slash`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Slash`

- `fn eq(self: &Self, _other: &Slash) -> bool` — [`Slash`](#slash)

##### `impl<P, T> Receiver for Slash`

- `type Target = T`

##### `impl<T> Sealed for Slash`

##### `impl<T> Spanned for Slash`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Slash`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Slash`

### `SlashEq`

```rust
struct SlashEq {
    pub spans: [Span; 2],
}
```

`/=`

Usage:
 division assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for SlashEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for SlashEq`

##### `impl Debug for SlashEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SlashEq`

- `fn default() -> Self`

##### `impl Eq for SlashEq`

##### `impl Hash for SlashEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for SlashEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for SlashEq`

- `fn eq(self: &Self, _other: &SlashEq) -> bool` — [`SlashEq`](#slasheq)

##### `impl Sealed for SlashEq`

##### `impl<T> Spanned for SlashEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for SlashEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for SlashEq`

### `Star`

```rust
struct Star {
    pub spans: [Span; 1],
}
```

`*`

Usage:
 multiplication, dereference, raw pointers, macro Kleene matcher, use wildcards.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Star`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Star`

##### `impl Debug for Star`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Star`

- `fn default() -> Self`

##### `impl Deref for Star`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Star`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Star`

##### `impl Hash for Star`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Star`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Star`

- `fn eq(self: &Self, _other: &Star) -> bool` — [`Star`](#star)

##### `impl<P, T> Receiver for Star`

- `type Target = T`

##### `impl Sealed for Star`

##### `impl<T> Spanned for Star`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Star`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Star`

### `StarEq`

```rust
struct StarEq {
    pub spans: [Span; 2],
}
```

`*=`

Usage:
 multiplication assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for StarEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for StarEq`

##### `impl Debug for StarEq`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StarEq`

- `fn default() -> Self`

##### `impl Eq for StarEq`

##### `impl Hash for StarEq`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for StarEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for StarEq`

- `fn eq(self: &Self, _other: &StarEq) -> bool` — [`StarEq`](#stareq)

##### `impl Sealed for StarEq`

##### `impl<T> Spanned for StarEq`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for StarEq`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for StarEq`

### `Tilde`

```rust
struct Tilde {
    pub spans: [Span; 1],
}
```

`~`

Usage:
 unused since before Rust 1.0.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Clone for Tilde`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Tilde`

##### `impl Debug for Tilde`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Tilde`

- `fn default() -> Self`

##### `impl Deref for Tilde`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Tilde`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Eq for Tilde`

##### `impl Hash for Tilde`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl Parse for Tilde`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Tilde`

- `fn eq(self: &Self, _other: &Tilde) -> bool` — [`Tilde`](#tilde)

##### `impl<P, T> Receiver for Tilde`

- `type Target = T`

##### `impl Sealed for Tilde`

##### `impl<T> Spanned for Tilde`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Tilde`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for Tilde`

### `Brace`

```rust
struct Brace {
    pub span: DelimSpan,
}
```

`{`&hellip;`}`

#### Implementations

- `fn surround<F>(self: &Self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Clone for Brace`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Brace`

##### `impl Debug for Brace`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Brace`

- `fn default() -> Self`

##### `impl Eq for Brace`

##### `impl Hash for Brace`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl PartialEq for Brace`

- `fn eq(self: &Self, _other: &Brace) -> bool` — [`Brace`](#brace)

##### `impl Sealed for Brace`

##### `impl Token for Brace`

### `Bracket`

```rust
struct Bracket {
    pub span: DelimSpan,
}
```

``&hellip;``

#### Implementations

- `fn surround<F>(self: &Self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Clone for Bracket`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Bracket`

##### `impl Debug for Bracket`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bracket`

- `fn default() -> Self`

##### `impl Eq for Bracket`

##### `impl Hash for Bracket`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl PartialEq for Bracket`

- `fn eq(self: &Self, _other: &Bracket) -> bool` — [`Bracket`](#bracket)

##### `impl Sealed for Bracket`

##### `impl Token for Bracket`

### `Paren`

```rust
struct Paren {
    pub span: DelimSpan,
}
```

`(`&hellip;`)`

#### Implementations

- `fn surround<F>(self: &Self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Clone for Paren`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Paren`

##### `impl Debug for Paren`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Paren`

- `fn default() -> Self`

##### `impl Eq for Paren`

##### `impl Hash for Paren`

- `fn hash<H: Hasher>(self: &Self, _state: &mut H)`

##### `impl PartialEq for Paren`

- `fn eq(self: &Self, _other: &Paren) -> bool` — [`Paren`](#paren)

##### `impl Sealed for Paren`

##### `impl Token for Paren`

## Traits

### `Token`

```rust
trait Token: private::Sealed { ... }
```

Marker trait for types that represent single tokens.

This trait is sealed and cannot be implemented for types outside of Syn.

## Macros

### `impl_low_level_token!`

### `define_keywords!`

### `impl_deref_if_len_is_1!`

### `define_punctuation_structs!`

### `define_punctuation!`

### `define_delimiters!`

