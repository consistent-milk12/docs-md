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

#### Associated Types

- `type Val`

#### Required Methods

- `fn load_consume(&self) -> <Self as >::Val`

  Loads a value from the atomic using a "consume" memory ordering.

#### Implementors

- `core::sync::atomic::AtomicBool`
- `core::sync::atomic::AtomicI16`
- `core::sync::atomic::AtomicI32`
- `core::sync::atomic::AtomicI64`
- `core::sync::atomic::AtomicI8`
- `core::sync::atomic::AtomicIsize`
- `core::sync::atomic::AtomicPtr<T>`
- `core::sync::atomic::AtomicU16`
- `core::sync::atomic::AtomicU32`
- `core::sync::atomic::AtomicU64`
- `core::sync::atomic::AtomicU8`
- `core::sync::atomic::AtomicUsize`

## Macros

### `impl_consume!`

### `impl_atomic!`

