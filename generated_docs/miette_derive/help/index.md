*[miette_derive](../index.md) / [help](index.md)*

---

# Module `help`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Help`](#help) | enum |  |

## Enums

### `Help`

```rust
enum Help {
    Display(crate::fmt::Display),
    Field(syn::Member, Box<syn::Type>),
}
```

*Defined in [`miette-derive-7.6.0/src/help.rs:19-22`](../../../.source_1765210505/miette-derive-7.6.0/src/help.rs#L19-L22)*

#### Implementations

- <span id="help-from-fields"></span>`fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- <span id="help-from-fields-vec"></span>`fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- <span id="help-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md#diagnosticdef)

- <span id="help-gen-struct"></span>`fn gen_struct(&self, fields: &Fields) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Parse for Help`

- <span id="help-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

