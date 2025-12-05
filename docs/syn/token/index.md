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

[Peeking]: crate::parse::ParseBuffer::peek()
[Parsing]: crate::parse::ParseBuffer::parse()
[Printing]: https://docs.rs/quote/1.0/quote/trait.ToTokens.html


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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Sealed`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = WithSpan`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sealed`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Sealed`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Sealed`

##### `impl Token`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl Copy`

##### `impl Default`

- `fn default() -> Self`

##### `impl Sealed`

##### `impl Token`

## Traits

### `Token`

```rust
trait Token: private::Sealed { ... }
```

Marker trait for types that represent single tokens.

This trait is sealed and cannot be implemented for types outside of Syn.

