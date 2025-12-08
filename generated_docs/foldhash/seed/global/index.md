*[foldhash](../../index.md) / [seed](../index.md) / [global](index.md)*

---

# Module `global`

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

- `fn new() -> Self`

- `fn init_slow()`

- `fn get(self: Self) -> &'static SharedSeed` — [`SharedSeed`](../../index.md)

#### Trait Implementations

##### `impl Clone for GlobalSeed`

- `fn clone(self: &Self) -> GlobalSeed` — [`GlobalSeed`](#globalseed)

##### `impl Copy for GlobalSeed`

##### `impl Debug for GlobalSeed`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

