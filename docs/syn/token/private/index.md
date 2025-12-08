*[syn](../../index.md) / [token](../index.md) / [private](index.md)*

---

# Module `private`

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

