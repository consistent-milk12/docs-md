*[portable_atomic](../../index.md) / [imp](../index.md) / [fallback](index.md)*

---

# Module `fallback`

## Contents

- [Modules](#modules)
  - [`utils`](#utils)
  - [`seq_lock`](#seq_lock)
- [Structs](#structs)
  - [`AtomicI128`](#atomici128)
  - [`AtomicU128`](#atomicu128)
- [Functions](#functions)
  - [`lock`](#lock)
- [Macros](#macros)
  - [`atomic!`](#atomic)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`utils`](#utils) | mod |  |
| [`seq_lock`](#seq_lock) | mod |  |
| [`AtomicI128`](#atomici128) | struct |  |
| [`AtomicU128`](#atomicu128) | struct |  |
| [`lock`](#lock) | fn |  |
| [`atomic!`](#atomic) | macro |  |

## Modules

- [`utils`](utils/index.md)
- [`seq_lock`](seq_lock/index.md)

## Structs

### `AtomicI128`

```rust
struct AtomicI128 {
    v: UnsafeCell<i128>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/mod.rs:451`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/fallback/mod.rs#L451)*

#### Implementations

- <span id="atomici128-const-len"></span>`const LEN: usize`

- <span id="atomici128-chunks"></span>`unsafe fn chunks(&self) -> &[core::sync::atomic::AtomicUsize; 2]`

- <span id="atomici128-optimistic-read"></span>`fn optimistic_read(&self) -> i128`

- <span id="atomici128-read"></span>`fn read(&self, _guard: &SeqLockWriteGuard<'static>) -> i128` — [`SeqLockWriteGuard`](seq_lock/index.md#seqlockwriteguard)

- <span id="atomici128-write"></span>`fn write(&self, val: i128, _guard: &SeqLockWriteGuard<'static>)` — [`SeqLockWriteGuard`](seq_lock/index.md#seqlockwriteguard)

#### Trait Implementations

##### `impl Sync for AtomicI128`

### `AtomicU128`

```rust
struct AtomicU128 {
    v: UnsafeCell<u128>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/mod.rs:452`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/fallback/mod.rs#L452)*

#### Implementations

- <span id="atomicu128-const-len"></span>`const LEN: usize`

- <span id="atomicu128-chunks"></span>`unsafe fn chunks(&self) -> &[core::sync::atomic::AtomicUsize; 2]`

- <span id="atomicu128-optimistic-read"></span>`fn optimistic_read(&self) -> u128`

- <span id="atomicu128-read"></span>`fn read(&self, _guard: &SeqLockWriteGuard<'static>) -> u128` — [`SeqLockWriteGuard`](seq_lock/index.md#seqlockwriteguard)

- <span id="atomicu128-write"></span>`fn write(&self, val: u128, _guard: &SeqLockWriteGuard<'static>)` — [`SeqLockWriteGuard`](seq_lock/index.md#seqlockwriteguard)

#### Trait Implementations

##### `impl Sync for AtomicU128`

## Functions

### `lock`

```rust
fn lock(addr: usize) -> &'static self::seq_lock::SeqLock
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/mod.rs:138-155`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/fallback/mod.rs#L138-L155)*

## Macros

### `atomic!`

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/mod.rs:157-414`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/fallback/mod.rs#L157-L414)*

