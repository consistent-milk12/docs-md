*[thiserror_impl](../index.md) / [scan_expr](index.md)*

---

# Module `scan_expr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Input`](#input) | enum |  |
| [`Action`](#action) | enum |  |
| [`scan_expr`](#scan-expr) | fn |  |

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

*Defined in [`thiserror-impl-2.0.17/src/scan_expr.rs:6-23`](../../../.source_1765210505/thiserror-impl-2.0.17/src/scan_expr.rs#L6-L23)*

### `Action`

```rust
enum Action {
    SetState(&'static [(Input, Action)]),
    IncDepth,
    DecDepth,
    Finish,
}
```

*Defined in [`thiserror-impl-2.0.17/src/scan_expr.rs:25-30`](../../../.source_1765210505/thiserror-impl-2.0.17/src/scan_expr.rs#L25-L30)*

## Functions

### `scan_expr`

```rust
fn scan_expr(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<()>
```

*Defined in [`thiserror-impl-2.0.17/src/scan_expr.rs:192-264`](../../../.source_1765210505/thiserror-impl-2.0.17/src/scan_expr.rs#L192-L264)*

