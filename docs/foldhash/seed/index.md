*[foldhash](../index.md) / [seed](index.md)*

---

# Module `seed`

## Modules

- [`global`](global/index.md) - 

## Structs

### `SharedSeed`

```rust
struct SharedSeed {
    seeds: [u64; 6],
}
```

A random seed intended to be shared by many different foldhash instances.

This seed is consumed by [`FoldHasher::with_seed`](crate::fast::FoldHasher::with_seed),
and [`SeedableRandomState::with_seed`](crate::fast::SeedableRandomState::with_seed).

#### Implementations

- `fn global_random() -> &'static SharedSeed` — [`SharedSeed`](#sharedseed)

- `const fn global_fixed() -> &'static SharedSeed` — [`SharedSeed`](#sharedseed)

- `const fn from_u64(seed: u64) -> Self`

#### Trait Implementations

##### `impl Clone for SharedSeed`

- `fn clone(self: &Self) -> SharedSeed` — [`SharedSeed`](#sharedseed)

##### `impl Debug for SharedSeed`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `gen_per_hasher_seed`

```rust
fn gen_per_hasher_seed() -> u64
```

## Constants

### `FIXED_GLOBAL_SEED`

```rust
const FIXED_GLOBAL_SEED: SharedSeed;
```

Used for FixedState, and RandomState if atomics for dynamic init are unavailable.

