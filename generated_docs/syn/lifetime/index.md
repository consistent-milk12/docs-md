*[syn](../index.md) / [lifetime](index.md)*

---

# Module `lifetime`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Lifetime`](#lifetime) | struct | A Rust lifetime: `'a`. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

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

- <span id="cratelifetime-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for Lifetime`

- <span id="lifetime-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Lifetime`

- <span id="cratelifetime-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Lifetime`

- <span id="lifetime-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Lifetime`

##### `impl Hash for Lifetime`

- <span id="lifetime-hash"></span>`fn hash<H: Hasher>(&self, h: &mut H)`

##### `impl Ord for Lifetime`

- <span id="lifetime-cmp"></span>`fn cmp(&self, other: &Lifetime) -> Ordering` — [`Lifetime`](../index.md)

##### `impl Parse for crate::lifetime::Lifetime`

- <span id="cratelifetimelifetime-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for Lifetime`

- <span id="lifetime-eq"></span>`fn eq(&self, other: &Lifetime) -> bool` — [`Lifetime`](../index.md)

##### `impl PartialOrd for Lifetime`

- <span id="lifetime-partial-cmp"></span>`fn partial_cmp(&self, other: &Lifetime) -> Option<Ordering>` — [`Lifetime`](../index.md)

##### `impl Sealed for crate::lifetime::Lifetime`

##### `impl<T> Spanned for Lifetime`

- <span id="lifetime-span"></span>`fn span(&self) -> Span`

##### `impl<T> ToString for Lifetime`

- <span id="lifetime-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lifetime::Lifetime`

- <span id="cratelifetimelifetime-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lifetime::Lifetime`

