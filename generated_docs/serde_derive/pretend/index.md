*[serde_derive](../index.md) / [pretend](index.md)*

---

# Module `pretend`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pretend_used`](#pretend-used) | fn |  |
| [`pretend_fields_used`](#pretend-fields-used) | fn |  |
| [`pretend_fields_used_struct`](#pretend-fields-used-struct) | fn |  |
| [`pretend_fields_used_struct_packed`](#pretend-fields-used-struct-packed) | fn |  |
| [`pretend_fields_used_enum`](#pretend-fields-used-enum) | fn |  |
| [`pretend_variants_used`](#pretend-variants-used) | fn |  |

## Functions

### `pretend_used`

```rust
fn pretend_used(cont: &crate::internals::ast::Container<'_>, is_packed: bool) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/pretend.rs:23-31`](../../../.source_1765210505/serde_derive-1.0.228/src/pretend.rs#L23-L31)*

### `pretend_fields_used`

```rust
fn pretend_fields_used(cont: &crate::internals::ast::Container<'_>, is_packed: bool) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/pretend.rs:65-77`](../../../.source_1765210505/serde_derive-1.0.228/src/pretend.rs#L65-L77)*

### `pretend_fields_used_struct`

```rust
fn pretend_fields_used_struct(cont: &crate::internals::ast::Container<'_>, fields: &[crate::internals::ast::Field<'_>]) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/pretend.rs:79-92`](../../../.source_1765210505/serde_derive-1.0.228/src/pretend.rs#L79-L92)*

### `pretend_fields_used_struct_packed`

```rust
fn pretend_fields_used_struct_packed(cont: &crate::internals::ast::Container<'_>, fields: &[crate::internals::ast::Field<'_>]) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/pretend.rs:94-111`](../../../.source_1765210505/serde_derive-1.0.228/src/pretend.rs#L94-L111)*

### `pretend_fields_used_enum`

```rust
fn pretend_fields_used_enum(cont: &crate::internals::ast::Container<'_>, variants: &[crate::internals::ast::Variant<'_>]) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/pretend.rs:113-139`](../../../.source_1765210505/serde_derive-1.0.228/src/pretend.rs#L113-L139)*

### `pretend_variants_used`

```rust
fn pretend_variants_used(cont: &crate::internals::ast::Container<'_>) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/pretend.rs:150-188`](../../../.source_1765210505/serde_derive-1.0.228/src/pretend.rs#L150-L188)*

