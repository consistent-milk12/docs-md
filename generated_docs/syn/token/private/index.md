*[syn](../../index.md) / [token](../index.md) / [private](index.md)*

---

# Module `private`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WithSpan`](#withspan) | struct | Support writing `token.span` rather than `token.spans[0]` on tokens that |
| [`Sealed`](#sealed) | trait |  |

## Structs

### `WithSpan`

```rust
struct WithSpan {
    pub span: proc_macro2::Span,
}
```

Support writing `token.span` rather than `token.spans[0]` on tokens that
hold a single span.

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

