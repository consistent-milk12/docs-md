*[foldhash](../index.md) / [seed](index.md)*

---

# Module `seed`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`global`](#global) | mod |  |
| [`SharedSeed`](#sharedseed) | struct | A random seed intended to be shared by many different foldhash instances. |
| [`gen_per_hasher_seed`](#gen-per-hasher-seed) | fn |  |
| [`FIXED_GLOBAL_SEED`](#fixed-global-seed) | const | Used for FixedState, and RandomState if atomics for dynamic init are unavailable. |

## Modules

- [`global`](global/index.md)

## Structs

### `SharedSeed`

```rust
struct SharedSeed {
    seeds: [u64; 6],
}
```

*Defined in [`foldhash-0.2.0/src/seed.rs:78-80`](../../../.source_1765521767/foldhash-0.2.0/src/seed.rs#L78-L80)*

A random seed intended to be shared by many different foldhash instances.

This seed is consumed by [`FoldHasher::with_seed`](crate::fast::FoldHasher::with_seed),
and [`SeedableRandomState::with_seed`](crate::fast::SeedableRandomState::with_seed).

#### Implementations

- <span id="sharedseed-global-random"></span>`fn global_random() -> &'static SharedSeed` — [`SharedSeed`](#sharedseed)

  Returns the globally shared randomly initialized [`SharedSeed`](#sharedseed) as used

  by [`RandomState`](crate::fast::RandomState).

- <span id="sharedseed-global-fixed"></span>`const fn global_fixed() -> &'static SharedSeed` — [`SharedSeed`](#sharedseed)

  Returns the globally shared fixed [`SharedSeed`](#sharedseed) as used

  by [`FixedState`](crate::fast::FixedState).

- <span id="sharedseed-from-u64"></span>`const fn from_u64(seed: u64) -> Self`

  Generates a new [`SharedSeed`](#sharedseed) from a single 64-bit seed.

  

  Note that this is somewhat expensive so it is suggested to re-use the

  [`SharedSeed`](#sharedseed) as much as possible, using the per-hasher seed to

  differentiate between hash instances.

#### Trait Implementations

##### `impl Any for SharedSeed`

- <span id="sharedseed-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SharedSeed`

- <span id="sharedseed-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SharedSeed`

- <span id="sharedseed-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SharedSeed`

- <span id="sharedseed-clone"></span>`fn clone(&self) -> SharedSeed` — [`SharedSeed`](#sharedseed)

##### `impl CloneToUninit for SharedSeed`

- <span id="sharedseed-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SharedSeed`

- <span id="sharedseed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SharedSeed`

- <span id="sharedseed-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SharedSeed`

- <span id="sharedseed-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SharedSeed`

- <span id="sharedseed-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sharedseed-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SharedSeed`

- <span id="sharedseed-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sharedseed-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `gen_per_hasher_seed`

```rust
fn gen_per_hasher_seed() -> u64
```

*Defined in [`foldhash-0.2.0/src/seed.rs:22-71`](../../../.source_1765521767/foldhash-0.2.0/src/seed.rs#L22-L71)*

## Constants

### `FIXED_GLOBAL_SEED`
```rust
const FIXED_GLOBAL_SEED: SharedSeed;
```

*Defined in [`foldhash-0.2.0/src/seed.rs:11-20`](../../../.source_1765521767/foldhash-0.2.0/src/seed.rs#L11-L20)*

Used for FixedState, and RandomState if atomics for dynamic init are unavailable.

