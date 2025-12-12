# Crate `thiserror_impl`

## Contents

- [Modules](#modules)
  - [`ast`](#ast)
  - [`attr`](#attr)
  - [`expand`](#expand)
  - [`fallback`](#fallback)
  - [`fmt`](#fmt)
  - [`generics`](#generics)
  - [`prop`](#prop)
  - [`scan_expr`](#scan-expr)
  - [`unraw`](#unraw)
  - [`valid`](#valid)
- [Structs](#structs)
  - [`private`](#private)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ast`](#ast) | mod |  |
| [`attr`](#attr) | mod |  |
| [`expand`](#expand) | mod |  |
| [`fallback`](#fallback) | mod |  |
| [`fmt`](#fmt) | mod |  |
| [`generics`](#generics) | mod |  |
| [`prop`](#prop) | mod |  |
| [`scan_expr`](#scan-expr) | mod |  |
| [`unraw`](#unraw) | mod |  |
| [`valid`](#valid) | mod |  |
| [`private`](#private) | struct |  |

## Modules

- [`ast`](ast/index.md)
- [`attr`](attr/index.md)
- [`expand`](expand/index.md)
- [`fallback`](fallback/index.md)
- [`fmt`](fmt/index.md)
- [`generics`](generics/index.md)
- [`prop`](prop/index.md)
- [`scan_expr`](scan_expr/index.md)
- [`unraw`](unraw/index.md)
- [`valid`](valid/index.md)

## Structs

### `private`

```rust
struct private;
```

*Defined in [`thiserror-impl-2.0.17/src/lib.rs:45`](../../.source_1765210505/thiserror-impl-2.0.17/src/lib.rs#L45)*

#### Trait Implementations

##### `impl Spanned for private`

- <span id="private-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for private`

- <span id="private-to-tokens"></span>`fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream)`

