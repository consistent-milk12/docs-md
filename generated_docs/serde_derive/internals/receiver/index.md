*[serde_derive](../../index.md) / [internals](../index.md) / [receiver](index.md)*

---

# Module `receiver`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReplaceReceiver`](#replacereceiver) | struct |  |
| [`replace_receiver`](#replace-receiver) | fn |  |

## Structs

### `ReplaceReceiver<'a>`

```rust
struct ReplaceReceiver<'a>(&'a syn::TypePath);
```

*Defined in [`serde_derive-1.0.228/src/internals/receiver.rs:21`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/receiver.rs#L21)*

#### Implementations

- <span id="replacereceiver-self-ty"></span>`fn self_ty(&self, span: Span) -> TypePath`

- <span id="replacereceiver-self-to-qself"></span>`fn self_to_qself(&self, qself: &mut Option<QSelf>, path: &mut Path)`

- <span id="replacereceiver-self-to-expr-path"></span>`fn self_to_expr_path(&self, path: &mut Path)`

## Functions

### `replace_receiver`

```rust
fn replace_receiver(input: &mut syn::DeriveInput)
```

*Defined in [`serde_derive-1.0.228/src/internals/receiver.rs:10-19`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/receiver.rs#L10-L19)*

