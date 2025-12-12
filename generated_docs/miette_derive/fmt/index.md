*[miette_derive](../index.md) / [fmt](index.md)*

---

# Module `fmt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Display`](#display) | struct |  |
| [`explicit_named_args`](#explicit-named-args) | fn |  |
| [`take_int`](#take-int) | fn |  |
| [`take_ident`](#take-ident) | fn |  |
| [`parse_token_expr`](#parse-token-expr) | fn |  |

## Structs

### `Display`

```rust
struct Display {
    pub fmt: syn::LitStr,
    pub args: proc_macro2::TokenStream,
    pub has_bonus_display: bool,
}
```

*Defined in [`miette-derive-7.6.0/src/fmt.rs:12-16`](../../../.source_1765521767/miette-derive-7.6.0/src/fmt.rs#L12-L16)*

#### Implementations

- <span id="display-expand-shorthand"></span>`fn expand_shorthand(&mut self, members: &Set<Member>)`

#### Trait Implementations

##### `impl Clone for Display`

- <span id="display-clone"></span>`fn clone(&self) -> Display` — [`Display`](#display)

##### `impl Spanned for Display`

- <span id="display-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Display`

- <span id="display-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Functions

### `explicit_named_args`

```rust
fn explicit_named_args(input: syn::parse::ParseStream<'_>) -> syn::Result<std::collections::HashSet<syn::Ident>>
```

*Defined in [`miette-derive-7.6.0/src/fmt.rs:116-131`](../../../.source_1765521767/miette-derive-7.6.0/src/fmt.rs#L116-L131)*

### `take_int`

```rust
fn take_int(read: &mut &str) -> String
```

*Defined in [`miette-derive-7.6.0/src/fmt.rs:133-145`](../../../.source_1765521767/miette-derive-7.6.0/src/fmt.rs#L133-L145)*

### `take_ident`

```rust
fn take_ident(read: &mut &str) -> syn::Ident
```

*Defined in [`miette-derive-7.6.0/src/fmt.rs:147-164`](../../../.source_1765521767/miette-derive-7.6.0/src/fmt.rs#L147-L164)*

### `parse_token_expr`

```rust
fn parse_token_expr(input: syn::parse::ParseStream<'_>, begin_expr: bool) -> syn::Result<proc_macro2::TokenStream>
```

*Defined in [`miette-derive-7.6.0/src/fmt.rs:166-235`](../../../.source_1765521767/miette-derive-7.6.0/src/fmt.rs#L166-L235)*

