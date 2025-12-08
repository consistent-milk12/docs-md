*[miette_derive](../index.md) / [fmt](index.md)*

---

# Module `fmt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Display`](#display) | struct |  |
| [`explicit_named_args`](#explicit_named_args) | fn |  |
| [`take_int`](#take_int) | fn |  |
| [`take_ident`](#take_ident) | fn |  |
| [`parse_token_expr`](#parse_token_expr) | fn |  |

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

- <span id="display-expand-shorthand"></span>`fn expand_shorthand(&mut self, members: &Set<Member>)`

#### Trait Implementations

##### `impl Clone for Display`

- <span id="display-clone"></span>`fn clone(&self) -> Display` — [`Display`](#display)

##### `impl<T> Spanned for Display`

- <span id="display-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Display`

- <span id="display-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

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

