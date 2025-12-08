*[thiserror_impl](../index.md) / [scan_expr](index.md)*

---

# Module `scan_expr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Input`](#input) | enum |  |
| [`Action`](#action) | enum |  |
| [`scan_expr`](#scan_expr) | fn |  |

## Enums

### `Input`

```rust
enum Input {
    Keyword(&'static str),
    Punct(&'static str),
    ConsumeAny,
    ConsumeBinOp,
    ConsumeBrace,
    ConsumeDelimiter,
    ConsumeIdent,
    ConsumeLifetime,
    ConsumeLiteral,
    ConsumeNestedBrace,
    ExpectPath,
    ExpectTurbofish,
    ExpectType,
    CanBeginExpr,
    Otherwise,
    Empty,
}
```

### `Action`

```rust
enum Action {
    SetState(&'static [(Input, Action)]),
    IncDepth,
    DecDepth,
    Finish,
}
```

## Functions

### `scan_expr`

```rust
fn scan_expr(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<()>
```

