*[foldhash](../../index.md) / [seed](../index.md) / [global](index.md)*

---

# Module `global`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GlobalSeedStorage`](#globalseedstorage) | struct |  |
| [`GlobalSeed`](#globalseed) | struct | An object representing an initialized global seed. |
| [`generate_global_seed`](#generate_global_seed) | fn |  |
| [`UNINIT`](#uninit) | const |  |
| [`LOCKED`](#locked) | const |  |
| [`INIT`](#init) | const |  |

## Structs

### `GlobalSeedStorage`

```rust
struct GlobalSeedStorage {
    state: core::sync::atomic::AtomicU8,
    seed: core::cell::UnsafeCell<SharedSeed>,
}
```

#### Trait Implementations

##### `impl Sync for GlobalSeedStorage`

### `GlobalSeed`

```rust
struct GlobalSeed {
    _no_accidental_unsafe_init: (),
}
```

An object representing an initialized global seed.

Does not actually store the seed inside itself, it is a zero-sized type.
This prevents inflating the RandomState size and in turn HashMap's size.

#### Implementations

- <span id="globalseed-new"></span>`fn new() -> Self`

- <span id="globalseed-init-slow"></span>`fn init_slow()`

- <span id="globalseed-get"></span>`fn get(self) -> &'static SharedSeed` — [`SharedSeed`](../../index.md)

#### Trait Implementations

##### `impl Clone for GlobalSeed`

- <span id="globalseed-clone"></span>`fn clone(&self) -> GlobalSeed` — [`GlobalSeed`](#globalseed)

##### `impl Copy for GlobalSeed`

##### `impl Debug for GlobalSeed`

- <span id="globalseed-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `generate_global_seed`

```rust
fn generate_global_seed() -> SharedSeed
```

## Constants

### `UNINIT`

```rust
const UNINIT: u8 = 0u8;
```

### `LOCKED`

```rust
const LOCKED: u8 = 1u8;
```

### `INIT`

```rust
const INIT: u8 = 2u8;
```

