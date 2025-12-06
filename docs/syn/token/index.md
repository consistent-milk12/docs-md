*[syn](../index.md) / [token](index.md)*

---

# Module `token`

Tokens representing Rust punctuation, keywords, and delimiters.

The type names in this module can be difficult to keep straight, so we
prefer to use the [`Token!`](#token) macro instead. This is a type-macro that
expands to the token type of the given token.

# Example

The [`ItemStatic`](../item/index.md) syntax tree node is defined like this.

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
method. Delimiter tokens are parsed using the [`parenthesized!`](#parenthesized),
[`bracketed!`](#bracketed) and [`braced!`](#braced) macros.




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

- Construction from a [`Span`](#span) — `let the_token = Token![...](sp)`

- Field access to its span — `let sp = the_token.span`





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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Underscore`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Underscore`

##### `impl Default for Underscore`

- `fn default() -> Self`

##### `impl Deref for Underscore`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Underscore`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Underscore`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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

##### `impl Default for Group`

- `fn default() -> Self`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Abstract`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Abstract`

##### `impl Default for Abstract`

- `fn default() -> Self`

##### `impl Parse for Abstract`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Abstract`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for As`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for As`

##### `impl Default for As`

- `fn default() -> Self`

##### `impl Parse for As`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for As`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Async`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Async`

##### `impl Default for Async`

- `fn default() -> Self`

##### `impl Parse for Async`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Async`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Auto`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Auto`

##### `impl Default for Auto`

- `fn default() -> Self`

##### `impl Parse for Auto`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Auto`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Await`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Await`

##### `impl Default for Await`

- `fn default() -> Self`

##### `impl Parse for Await`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Become`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Become`

##### `impl Default for Become`

- `fn default() -> Self`

##### `impl Parse for Become`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Box`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Box`

##### `impl Default for Box`

- `fn default() -> Self`

##### `impl Parse for Box`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Break`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Break`

##### `impl Default for Break`

- `fn default() -> Self`

##### `impl Parse for Break`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Break`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Const`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Const`

##### `impl Default for Const`

- `fn default() -> Self`

##### `impl Parse for Const`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Const`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Continue`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Continue`

##### `impl Default for Continue`

- `fn default() -> Self`

##### `impl Parse for Continue`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Crate`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Crate`

##### `impl Default for Crate`

- `fn default() -> Self`

##### `impl Parse for Crate`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Default`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Default`

##### `impl Default for Default`

- `fn default() -> Self`

##### `impl Parse for Default`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Do`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Do`

##### `impl Default for Do`

- `fn default() -> Self`

##### `impl Parse for Do`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Dyn`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Dyn`

##### `impl Default for Dyn`

- `fn default() -> Self`

##### `impl Parse for Dyn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Else`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Else`

##### `impl Default for Else`

- `fn default() -> Self`

##### `impl Parse for Else`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Else`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Enum`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Enum`

##### `impl Default for Enum`

- `fn default() -> Self`

##### `impl Parse for Enum`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Extern`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Extern`

##### `impl Default for Extern`

- `fn default() -> Self`

##### `impl Parse for Extern`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Extern`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Final`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Final`

##### `impl Default for Final`

- `fn default() -> Self`

##### `impl Parse for Final`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Final`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Fn`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Fn`

##### `impl Default for Fn`

- `fn default() -> Self`

##### `impl Parse for Fn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Fn`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for For`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for For`

##### `impl Default for For`

- `fn default() -> Self`

##### `impl Parse for For`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for If`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for If`

##### `impl Default for If`

- `fn default() -> Self`

##### `impl Parse for If`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Impl`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Impl`

##### `impl Default for Impl`

- `fn default() -> Self`

##### `impl Parse for Impl`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for In`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for In`

##### `impl Default for In`

- `fn default() -> Self`

##### `impl Parse for In`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Let`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Let`

##### `impl Default for Let`

- `fn default() -> Self`

##### `impl Parse for Let`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Loop`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Loop`

##### `impl Default for Loop`

- `fn default() -> Self`

##### `impl Parse for Loop`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Macro`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Macro`

##### `impl Default for Macro`

- `fn default() -> Self`

##### `impl Parse for Macro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Macro`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Match`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Match`

##### `impl Default for Match`

- `fn default() -> Self`

##### `impl Parse for Match`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Mod`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Mod`

##### `impl Default for Mod`

- `fn default() -> Self`

##### `impl Parse for Mod`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Mod`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Move`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Move`

##### `impl Default for Move`

- `fn default() -> Self`

##### `impl Parse for Move`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Mut`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Mut`

##### `impl Default for Mut`

- `fn default() -> Self`

##### `impl Parse for Mut`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Override`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Override`

##### `impl Default for Override`

- `fn default() -> Self`

##### `impl Parse for Override`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Override`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Priv`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Priv`

##### `impl Default for Priv`

- `fn default() -> Self`

##### `impl Parse for Priv`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Priv`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Pub`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Pub`

##### `impl Default for Pub`

- `fn default() -> Self`

##### `impl Parse for Pub`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Raw`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Raw`

##### `impl Default for Raw`

- `fn default() -> Self`

##### `impl Parse for Raw`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Raw`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Ref`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Ref`

##### `impl Default for Ref`

- `fn default() -> Self`

##### `impl Parse for Ref`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Return`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Return`

##### `impl Default for Return`

- `fn default() -> Self`

##### `impl Parse for Return`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Return`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for SelfType`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for SelfType`

##### `impl Default for SelfType`

- `fn default() -> Self`

##### `impl Parse for SelfType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for SelfType`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for SelfValue`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for SelfValue`

##### `impl Default for SelfValue`

- `fn default() -> Self`

##### `impl Parse for SelfValue`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Static`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Static`

##### `impl Default for Static`

- `fn default() -> Self`

##### `impl Parse for Static`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Struct`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Struct`

##### `impl Default for Struct`

- `fn default() -> Self`

##### `impl Parse for Struct`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Super`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Super`

##### `impl Default for Super`

- `fn default() -> Self`

##### `impl Parse for Super`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Super`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Trait`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Trait`

##### `impl Default for Trait`

- `fn default() -> Self`

##### `impl Parse for Trait`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Try`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Try`

##### `impl Default for Try`

- `fn default() -> Self`

##### `impl Parse for Try`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Try`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Type`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Type`

##### `impl Default for Type`

- `fn default() -> Self`

##### `impl Parse for Type`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Type`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Typeof`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Typeof`

##### `impl Default for Typeof`

- `fn default() -> Self`

##### `impl Parse for Typeof`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Typeof`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Union`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Union`

##### `impl Default for Union`

- `fn default() -> Self`

##### `impl Parse for Union`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Union`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Unsafe`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Unsafe`

##### `impl Default for Unsafe`

- `fn default() -> Self`

##### `impl Parse for Unsafe`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Unsafe`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Unsized`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Unsized`

##### `impl Default for Unsized`

- `fn default() -> Self`

##### `impl Parse for Unsized`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Unsized`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Use`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Use`

##### `impl Default for Use`

- `fn default() -> Self`

##### `impl Parse for Use`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Virtual`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Virtual`

##### `impl Default for Virtual`

- `fn default() -> Self`

##### `impl Parse for Virtual`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Virtual`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Where`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Where`

##### `impl Default for Where`

- `fn default() -> Self`

##### `impl Parse for Where`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Where`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for While`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for While`

##### `impl Default for While`

- `fn default() -> Self`

##### `impl Parse for While`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for While`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Yield`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Yield`

##### `impl Default for Yield`

- `fn default() -> Self`

##### `impl Parse for Yield`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Yield`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for And`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for And`

##### `impl Default for And`

- `fn default() -> Self`

##### `impl Deref for And`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for And`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for And`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for AndAnd`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for AndAnd`

##### `impl Default for AndAnd`

- `fn default() -> Self`

##### `impl Parse for AndAnd`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for AndEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for AndEq`

##### `impl Default for AndEq`

- `fn default() -> Self`

##### `impl Parse for AndEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for AndEq`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for At`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for At`

##### `impl Default for At`

- `fn default() -> Self`

##### `impl Deref for At`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for At`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for At`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Caret`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Caret`

##### `impl Default for Caret`

- `fn default() -> Self`

##### `impl Deref for Caret`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Caret`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Caret`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for CaretEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for CaretEq`

##### `impl Default for CaretEq`

- `fn default() -> Self`

##### `impl Parse for CaretEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Colon`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Colon`

##### `impl Default for Colon`

- `fn default() -> Self`

##### `impl Deref for Colon`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Colon`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Colon`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Comma`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Comma`

##### `impl Default for Comma`

- `fn default() -> Self`

##### `impl Deref for Comma`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Comma`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Comma`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<P, T> Receiver for Comma`

- `type Target = T`

##### `impl<T> Sealed for Comma`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Dollar`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Dollar`

##### `impl Default for Dollar`

- `fn default() -> Self`

##### `impl Deref for Dollar`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Dollar`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Dollar`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Dot`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Dot`

##### `impl Default for Dot`

- `fn default() -> Self`

##### `impl Deref for Dot`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Dot`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Dot`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<P, T> Receiver for Dot`

- `type Target = T`

##### `impl<T> Sealed for Dot`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for DotDot`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for DotDot`

##### `impl Default for DotDot`

- `fn default() -> Self`

##### `impl Parse for DotDot`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for DotDotDot`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for DotDotDot`

##### `impl Default for DotDotDot`

- `fn default() -> Self`

##### `impl Parse for DotDotDot`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for DotDotEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for DotDotEq`

##### `impl Default for DotDotEq`

- `fn default() -> Self`

##### `impl Parse for DotDotEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Eq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Eq`

##### `impl Default for Eq`

- `fn default() -> Self`

##### `impl Deref for Eq`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Eq`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Eq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<P, T> Receiver for Eq`

- `type Target = T`

##### `impl Sealed for Eq`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for EqEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for EqEq`

##### `impl Default for EqEq`

- `fn default() -> Self`

##### `impl Parse for EqEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for EqEq`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for FatArrow`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for FatArrow`

##### `impl Default for FatArrow`

- `fn default() -> Self`

##### `impl Parse for FatArrow`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Ge`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Ge`

##### `impl Default for Ge`

- `fn default() -> Self`

##### `impl Parse for Ge`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for Ge`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Gt`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Gt`

##### `impl Default for Gt`

- `fn default() -> Self`

##### `impl Deref for Gt`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Gt`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Gt`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<P, T> Receiver for Gt`

- `type Target = T`

##### `impl<T> Sealed for Gt`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for LArrow`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for LArrow`

##### `impl Default for LArrow`

- `fn default() -> Self`

##### `impl Parse for LArrow`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Le`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Le`

##### `impl Default for Le`

- `fn default() -> Self`

##### `impl Parse for Le`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Lt`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Lt`

##### `impl Default for Lt`

- `fn default() -> Self`

##### `impl Deref for Lt`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Lt`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Lt`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Minus`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Minus`

##### `impl Default for Minus`

- `fn default() -> Self`

##### `impl Deref for Minus`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Minus`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Minus`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<P, T> Receiver for Minus`

- `type Target = T`

##### `impl Sealed for Minus`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for MinusEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for MinusEq`

##### `impl Default for MinusEq`

- `fn default() -> Self`

##### `impl Parse for MinusEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for MinusEq`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Ne`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Ne`

##### `impl Default for Ne`

- `fn default() -> Self`

##### `impl Parse for Ne`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Not`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Not`

##### `impl Default for Not`

- `fn default() -> Self`

##### `impl Deref for Not`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Not`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Not`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<P, T> Receiver for Not`

- `type Target = T`

##### `impl Sealed for Not`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Or`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Or`

##### `impl Default for Or`

- `fn default() -> Self`

##### `impl Deref for Or`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Or`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Or`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<P, T> Receiver for Or`

- `type Target = T`

##### `impl Sealed for Or`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for OrEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for OrEq`

##### `impl Default for OrEq`

- `fn default() -> Self`

##### `impl Parse for OrEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for OrEq`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for OrOr`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for OrOr`

##### `impl Default for OrOr`

- `fn default() -> Self`

##### `impl Parse for OrOr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for OrOr`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for PathSep`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for PathSep`

##### `impl Default for PathSep`

- `fn default() -> Self`

##### `impl Parse for PathSep`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Percent`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Percent`

##### `impl Default for Percent`

- `fn default() -> Self`

##### `impl Deref for Percent`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Percent`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Percent`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<P, T> Receiver for Percent`

- `type Target = T`

##### `impl Sealed for Percent`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for PercentEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for PercentEq`

##### `impl Default for PercentEq`

- `fn default() -> Self`

##### `impl Parse for PercentEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for PercentEq`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Plus`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Plus`

##### `impl Default for Plus`

- `fn default() -> Self`

##### `impl Deref for Plus`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Plus`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Plus`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for PlusEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for PlusEq`

##### `impl Default for PlusEq`

- `fn default() -> Self`

##### `impl Parse for PlusEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Pound`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Pound`

##### `impl Default for Pound`

- `fn default() -> Self`

##### `impl Deref for Pound`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Pound`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Pound`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<P, T> Receiver for Pound`

- `type Target = T`

##### `impl<T> Sealed for Pound`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Question`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Question`

##### `impl Default for Question`

- `fn default() -> Self`

##### `impl Deref for Question`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Question`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Question`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for RArrow`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for RArrow`

##### `impl Default for RArrow`

- `fn default() -> Self`

##### `impl Parse for RArrow`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Semi`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Semi`

##### `impl Default for Semi`

- `fn default() -> Self`

##### `impl Deref for Semi`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Semi`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Semi`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<P, T> Receiver for Semi`

- `type Target = T`

##### `impl<T> Sealed for Semi`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Shl`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Shl`

##### `impl Default for Shl`

- `fn default() -> Self`

##### `impl Parse for Shl`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for ShlEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for ShlEq`

##### `impl Default for ShlEq`

- `fn default() -> Self`

##### `impl Parse for ShlEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for ShlEq`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Shr`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Shr`

##### `impl Default for Shr`

- `fn default() -> Self`

##### `impl Parse for Shr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl Sealed for Shr`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for ShrEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for ShrEq`

##### `impl Default for ShrEq`

- `fn default() -> Self`

##### `impl Parse for ShrEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Slash`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Slash`

##### `impl Default for Slash`

- `fn default() -> Self`

##### `impl Deref for Slash`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Slash`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Slash`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for SlashEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for SlashEq`

##### `impl Default for SlashEq`

- `fn default() -> Self`

##### `impl Parse for SlashEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl<T> Sealed for SlashEq`

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Star`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Star`

##### `impl Default for Star`

- `fn default() -> Self`

##### `impl Deref for Star`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Star`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Star`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for StarEq`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for StarEq`

##### `impl Default for StarEq`

- `fn default() -> Self`

##### `impl Parse for StarEq`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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
[`Token!`](#token) macro instead.


#### Trait Implementations

##### `impl Clone for Tilde`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Tilde`

##### `impl Default for Tilde`

- `fn default() -> Self`

##### `impl Deref for Tilde`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for Tilde`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse for Tilde`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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

##### `impl Default for Brace`

- `fn default() -> Self`

##### `impl Sealed for Brace`

##### `impl Token for Brace`

### `Bracket`

```rust
struct Bracket {
    pub span: DelimSpan,
}
```

`[`&hellip;`](#hellip)`

#### Implementations

- `fn surround<F>(self: &Self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Clone for Bracket`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Bracket`

##### `impl Default for Bracket`

- `fn default() -> Self`

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

##### `impl Default for Paren`

- `fn default() -> Self`

##### `impl Sealed for Paren`

##### `impl Token for Paren`

## Traits

### `Token`

```rust
trait Token: private::Sealed { ... }
```

Marker trait for types that represent single tokens.

This trait is sealed and cannot be implemented for types outside of Syn.

