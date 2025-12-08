*[rustversion](../index.md) / [expand](index.md)*

---

# Module `expand`

## Functions

### `cfg`

```rust
fn cfg(introducer: &str, args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream
```

### `try_cfg`

```rust
fn try_cfg(introducer: &str, args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> std::result::Result<proc_macro::TokenStream, Error>
```

### `try_attr`

```rust
fn try_attr(args: attr::Args, input: proc_macro::TokenStream) -> std::result::Result<proc_macro::TokenStream, Error>
```

### `allow_incompatible_msrv`

```rust
fn allow_incompatible_msrv(input: proc_macro::TokenStream) -> proc_macro::TokenStream
```

