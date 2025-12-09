*[miette_derive](../index.md) / [source_code](index.md)*

---

# Module `source_code`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SourceCode`](#sourcecode) | struct |  |

## Structs

### `SourceCode`

```rust
struct SourceCode {
    source_code: syn::Member,
    is_option: bool,
}
```

*Defined in [`miette-derive-7.6.0/src/source_code.rs:11-14`](../../../.source_1765210505/miette-derive-7.6.0/src/source_code.rs#L11-L14)*

#### Implementations

- <span id="sourcecode-from-fields"></span>`fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- <span id="sourcecode-from-fields-vec"></span>`fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- <span id="sourcecode-gen-struct"></span>`fn gen_struct(&self, fields: &syn::Fields) -> Option<TokenStream>`

- <span id="sourcecode-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

