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

#### Implementors

- `bool`
- `char`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `num::NonZeroI128`
- `num::NonZeroI16`
- `num::NonZeroI32`
- `num::NonZeroI64`
- `num::NonZeroI8`
- `num::NonZeroIsize`
- `num::NonZeroU128`
- `num::NonZeroU16`
- `num::NonZeroU32`
- `num::NonZeroU64`
- `num::NonZeroU8`
- `num::NonZeroUsize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Constants

### `FALSE`

```rust
const FALSE: super::Repr;
```

### `TRUE`

```rust
const TRUE: super::Repr;
```

