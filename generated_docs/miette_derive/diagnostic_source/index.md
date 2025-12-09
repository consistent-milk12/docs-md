*[miette_derive](../index.md) / [diagnostic_source](index.md)*

---

# Module `diagnostic_source`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DiagnosticSource`](#diagnosticsource) | struct |  |

## Structs

### `DiagnosticSource`

```rust
struct DiagnosticSource(syn::Member);
```

*Defined in [`miette-derive-7.6.0/src/diagnostic_source.rs:11`](../../../.source_1765210505/miette-derive-7.6.0/src/diagnostic_source.rs#L11)*

#### Implementations

- <span id="diagnosticsource-from-fields"></span>`fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- <span id="diagnosticsource-from-fields-vec"></span>`fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- <span id="diagnosticsource-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

- <span id="diagnosticsource-gen-struct"></span>`fn gen_struct(&self) -> Option<TokenStream>`

