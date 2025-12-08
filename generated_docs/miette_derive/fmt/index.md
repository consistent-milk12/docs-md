*[miette_derive](../index.md) / [fmt](index.md)*

---

# Module `fmt`

## Structs

### `Display`

```rust
struct Display {
    pub fmt: syn::LitStr,
    pub args: proc_macro2::TokenStream,
    pub has_bonus_display: bool,
}
```

#### Implementations

- `fn expand_shorthand(self: &mut Self, members: &Set<Member>)`

#### Trait Implementations

##### `impl Clone for Display`

- `fn clone(self: &Self) -> Display` — [`Display`](#display)

##### `impl<T> Spanned for Display`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Display`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Functions

### `explicit_named_args`

```rust
fn explicit_named_args(input: syn::parse::ParseStream<'_>) -> syn::Result<std::collections::HashSet<syn::Ident>>
```

### `take_int`

```rust
fn take_int(read: &mut &str) -> String
```

### `take_ident`

```rust
fn take_ident(read: &mut &str) -> syn::Ident
```

### `parse_token_expr`

```rust
fn parse_token_expr(input: syn::parse::ParseStream<'_>, begin_expr: bool) -> syn::Result<proc_macro2::TokenStream>
```

