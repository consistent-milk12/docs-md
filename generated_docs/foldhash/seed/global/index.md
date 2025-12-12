*[foldhash](../../index.md) / [seed](../index.md) / [global](index.md)*

---

# Module `global`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GlobalSeedStorage`](#globalseedstorage) | struct |  |
| [`GlobalSeed`](#globalseed) | struct | An object representing an initialized global seed. |
| [`generate_global_seed`](#generate-global-seed) | fn |  |
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

*Defined in [`foldhash-0.2.0/src/seed.rs:181-184`](../../../../.source_1765210505/foldhash-0.2.0/src/seed.rs#L181-L184)*

#### Trait Implementations

##### `impl Sync for GlobalSeedStorage`

### `GlobalSeed`

```rust
struct GlobalSeed {
    _no_accidental_unsafe_init: (),
}
```

*Defined in [`foldhash-0.2.0/src/seed.rs:205-208`](../../../../.source_1765210505/foldhash-0.2.0/src/seed.rs#L205-L208)*

An object representing an initialized global seed.

Does not actually store the seed inside itself, it is a zero-sized type.
This prevents inflating the RandomState size and in turn HashMap's size.

#### Implementations

- <span id="globalseed-new"></span>`fn new() -> Self`

- <span id="globalseed-init-slow"></span>`fn init_slow()`

- <span id="globalseed-get"></span>`fn get(self) -> &'static SharedSeed` — [`SharedSeed`](../index.md#sharedseed)

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

*Defined in [`foldhash-0.2.0/src/seed.rs:140-174`](../../../../.source_1765210505/foldhash-0.2.0/src/seed.rs#L140-L174)*

## Constants

### `UNINIT`
```rust
const UNINIT: u8 = 0u8;
```

*Defined in [`foldhash-0.2.0/src/seed.rs:186`](../../../../.source_1765210505/foldhash-0.2.0/src/seed.rs#L186)*

### `LOCKED`
```rust
const LOCKED: u8 = 1u8;
```

*Defined in [`foldhash-0.2.0/src/seed.rs:187`](../../../../.source_1765210505/foldhash-0.2.0/src/seed.rs#L187)*

### `INIT`
```rust
const INIT: u8 = 2u8;
```

*Defined in [`foldhash-0.2.0/src/seed.rs:188`](../../../../.source_1765210505/foldhash-0.2.0/src/seed.rs#L188)*

