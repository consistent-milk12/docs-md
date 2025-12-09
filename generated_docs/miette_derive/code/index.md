*[miette_derive](../index.md) / [code](index.md)*

---

# Module `code`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Code`](#code) | struct |  |

## Structs

### `Code`

```rust
struct Code(String);
```

*Defined in [`miette-derive-7.6.0/src/code.rs:16`](../../../.source_1765210505/miette-derive-7.6.0/src/code.rs#L16)*

#### Implementations

- <span id="code-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

- <span id="code-gen-struct"></span>`fn gen_struct(&self) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Debug for Code`

- <span id="code-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for Code`

- <span id="code-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

