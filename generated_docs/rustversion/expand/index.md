*[rustversion](../index.md) / [expand](index.md)*

---

# Module `expand`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`cfg`](#cfg) | fn |  |
| [`try_cfg`](#try-cfg) | fn |  |
| [`try_attr`](#try-attr) | fn |  |
| [`allow_incompatible_msrv`](#allow-incompatible-msrv) | fn |  |

## Functions

### `cfg`

```rust
fn cfg(introducer: &str, args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream
```

*Defined in [`rustversion-1.0.22/src/expand.rs:7-9`](../../../.source_1765633015/rustversion-1.0.22/src/expand.rs#L7-L9)*

### `try_cfg`

```rust
fn try_cfg(introducer: &str, args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> std::result::Result<proc_macro::TokenStream, Error>
```

*Defined in [`rustversion-1.0.22/src/expand.rs:11-31`](../../../.source_1765633015/rustversion-1.0.22/src/expand.rs#L11-L31)*

### `try_attr`

```rust
fn try_attr(args: attr::Args, input: proc_macro::TokenStream) -> std::result::Result<proc_macro::TokenStream, Error>
```

*Defined in [`rustversion-1.0.22/src/expand.rs:33-74`](../../../.source_1765633015/rustversion-1.0.22/src/expand.rs#L33-L74)*

### `allow_incompatible_msrv`

```rust
fn allow_incompatible_msrv(input: proc_macro::TokenStream) -> proc_macro::TokenStream
```

*Defined in [`rustversion-1.0.22/src/expand.rs:76-100`](../../../.source_1765633015/rustversion-1.0.22/src/expand.rs#L76-L100)*

