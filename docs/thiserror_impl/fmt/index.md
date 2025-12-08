*[thiserror_impl](../index.md) / [fmt](index.md)*

---

# Module `fmt`

## Structs

### `FmtArguments`

```rust
struct FmtArguments {
    named: std::collections::BTreeSet<crate::unraw::IdentUnraw>,
    first_unnamed: Option<proc_macro2::TokenStream>,
}
```

## Functions

### `explicit_named_args`

```rust
fn explicit_named_args(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<FmtArguments>
```

### `try_explicit_named_args`

```rust
fn try_explicit_named_args(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<FmtArguments>
```

### `fallback_explicit_named_args`

```rust
fn fallback_explicit_named_args(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<FmtArguments>
```

### `is_syn_full`

```rust
fn is_syn_full() -> bool
```

### `take_int`

```rust
fn take_int<'a>(read: &mut &'a str) -> &'a str
```

### `take_ident`

```rust
fn take_ident<'a>(read: &mut &'a str) -> &'a str
```

### `between`

```rust
fn between<'a>(begin: syn::parse::ParseStream<'a>, end: syn::parse::ParseStream<'a>) -> proc_macro2::TokenStream
```

