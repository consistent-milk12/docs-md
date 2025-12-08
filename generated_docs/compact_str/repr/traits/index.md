*[compact_str](../../index.md) / [repr](../index.md) / [traits](index.md)*

---

# Module `traits`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoRepr`](#intorepr) | trait | Defines how to _efficiently_ create a [`Repr`] from `self` |
| [`FALSE`](#false) | const |  |
| [`TRUE`](#true) | const |  |

## Traits

### `IntoRepr`

```rust
trait IntoRepr { ... }
```

Defines how to _efficiently_ create a [`Repr`](../index.md) from `self`

#### Required Methods

- `fn into_repr(self) -> Result<Repr, ToCompactStringError>`

## Constants

### `FALSE`

```rust
const FALSE: super::Repr;
```

### `TRUE`

```rust
const TRUE: super::Repr;
```

