*[miette_derive](../index.md) / [severity](index.md)*

---

# Module `severity`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Severity`](#severity) | struct |  |
| [`get_severity`](#get_severity) | fn |  |

## Structs

### `Severity`

```rust
struct Severity(syn::Ident);
```

*Defined in [`miette-derive-7.6.0/src/severity.rs:15`](../../../.source_1765210505/miette-derive-7.6.0/src/severity.rs#L15)*

#### Implementations

- <span id="severity-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

- <span id="severity-gen-struct"></span>`fn gen_struct(&self) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Parse for Severity`

- <span id="severity-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

## Functions

### `get_severity`

```rust
fn get_severity(input: &str, span: proc_macro2::Span) -> syn::Result<String>
```

*Defined in [`miette-derive-7.6.0/src/severity.rs:50-60`](../../../.source_1765210505/miette-derive-7.6.0/src/severity.rs#L50-L60)*

