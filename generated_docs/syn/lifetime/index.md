*[syn](../index.md) / [lifetime](index.md)*

---

# Module `lifetime`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `Lifetime`

```rust
struct Lifetime {
    pub apostrophe: proc_macro2::Span,
    pub ident: proc_macro2::Ident,
}
```

A Rust lifetime: `'a`.

Lifetime names must conform to the following rules:

- Must start with an apostrophe.
- Must not consist of just an apostrophe: `'`.
- Character after the apostrophe must be `_` or a Unicode code point with
  the XID_Start property.
- All following characters must be Unicode code points with the XID_Continue
  property.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for Lifetime`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Lifetime`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Lifetime`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Lifetime`

##### `impl Hash for Lifetime`

- `fn hash<H: Hasher>(self: &Self, h: &mut H)`

##### `impl Ord for Lifetime`

- `fn cmp(self: &Self, other: &Lifetime) -> Ordering` — [`Lifetime`](../index.md)

##### `impl Parse for crate::lifetime::Lifetime`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Lifetime`

- `fn eq(self: &Self, other: &Lifetime) -> bool` — [`Lifetime`](../index.md)

##### `impl PartialOrd for Lifetime`

- `fn partial_cmp(self: &Self, other: &Lifetime) -> Option<Ordering>` — [`Lifetime`](../index.md)

##### `impl<T> Sealed for Lifetime`

##### `impl<T> Spanned for Lifetime`

- `fn span(self: &Self) -> Span`

##### `impl<T> ToString for Lifetime`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for crate::lifetime::Lifetime`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lifetime::Lifetime`

