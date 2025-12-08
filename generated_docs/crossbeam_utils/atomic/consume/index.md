*[crossbeam_utils](../../index.md) / [atomic](../index.md) / [consume](index.md)*

---

# Module `consume`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AtomicConsume`](#atomicconsume) | trait | Trait which allows reading from primitive atomic types with "consume" ordering. |
| [`impl_consume!`](#impl_consume) | macro |  |
| [`impl_atomic!`](#impl_atomic) | macro |  |

## Traits

### `AtomicConsume`

```rust
trait AtomicConsume { ... }
```

Trait which allows reading from primitive atomic types with "consume" ordering.

#### Required Methods

- `type Val`

- `fn load_consume(&self) -> <Self as >::Val`

  Loads a value from the atomic using a "consume" memory ordering.

## Macros

### `impl_consume!`

### `impl_atomic!`

