*[thiserror_impl](../index.md) / [unraw](index.md)*

---

# Module `unraw`

## Structs

### `IdentUnraw`

```rust
struct IdentUnraw(proc_macro2::Ident);
```

#### Implementations

- `fn new(ident: Ident) -> Self`

- `fn to_local(self: &Self) -> Ident`

- `fn set_span(self: &mut Self, span: Span)`

#### Trait Implementations

##### `impl Clone for IdentUnraw`

- `fn clone(self: &Self) -> IdentUnraw` — [`IdentUnraw`](#identunraw)

##### `impl Display for IdentUnraw`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IdentUnraw`

##### `impl Ord for IdentUnraw`

- `fn cmp(self: &Self, other: &Self) -> Ordering`

##### `impl Parse for IdentUnraw`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl PartialEq for IdentUnraw`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialOrd for IdentUnraw`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

##### `impl<T> Spanned for IdentUnraw`

- `fn span(self: &Self) -> Span`

##### `impl<T> ToString for IdentUnraw`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for IdentUnraw`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Enums

### `MemberUnraw`

```rust
enum MemberUnraw {
    Named(IdentUnraw),
    Unnamed(syn::Index),
}
```

#### Implementations

- `fn span(self: &Self) -> Span`

#### Trait Implementations

##### `impl Clone for MemberUnraw`

- `fn clone(self: &Self) -> MemberUnraw` — [`MemberUnraw`](#memberunraw)

##### `impl Display for MemberUnraw`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MemberUnraw`

##### `impl Hash for MemberUnraw`

- `fn hash<H: Hasher>(self: &Self, hasher: &mut H)`

##### `impl PartialEq for MemberUnraw`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl<T> Spanned for MemberUnraw`

- `fn span(self: &Self) -> Span`

##### `impl<T> ToString for MemberUnraw`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for MemberUnraw`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

