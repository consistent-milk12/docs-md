*[serde_derive](../../index.md) / [internals](../index.md) / [receiver](index.md)*

---

# Module `receiver`

## Structs

### `ReplaceReceiver<'a>`

```rust
struct ReplaceReceiver<'a>(&'a syn::TypePath);
```

#### Implementations

- `fn self_ty(self: &Self, span: Span) -> TypePath`

- `fn self_to_qself(self: &Self, qself: &mut Option<QSelf>, path: &mut Path)`

- `fn self_to_expr_path(self: &Self, path: &mut Path)`

## Functions

### `replace_receiver`

```rust
fn replace_receiver(input: &mut syn::DeriveInput)
```

