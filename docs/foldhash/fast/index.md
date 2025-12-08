*[foldhash](../index.md) / [fast](index.md)*

---

# Module `fast`

The foldhash implementation optimized for speed.

## Structs

### `FoldHasher<'a>`

```rust
struct FoldHasher<'a> {
    accumulator: u64,
    sponge: u128,
    sponge_len: u8,
    seeds: &'a [u64; 6],
}
```

A `Hasher` instance implementing foldhash, optimized for speed.

While you can create one directly with `FoldHasher::with_seed`, you
most likely want to use [`RandomState`](#randomstate), [`SeedableRandomState`](#seedablerandomstate) or
[`FixedState`](#fixedstate) to create [`FoldHasher`](#foldhasher)s.

#### Implementations

- `const fn with_seed(per_hasher_seed: u64, shared_seed: &'a SharedSeed) -> FoldHasher<'a>` — [`SharedSeed`](../seed/index.md), [`FoldHasher`](#foldhasher)

- `fn write_num<T: Into<u128>>(self: &mut Self, x: T)`

#### Trait Implementations

##### `impl<'a> Clone for FoldHasher<'a>`

- `fn clone(self: &Self) -> FoldHasher<'a>` — [`FoldHasher`](#foldhasher)

##### `impl<'a> Hasher for FoldHasher<'a>`

- `fn write(self: &mut Self, bytes: &[u8])`

- `fn write_u8(self: &mut Self, i: u8)`

- `fn write_u16(self: &mut Self, i: u16)`

- `fn write_u32(self: &mut Self, i: u32)`

- `fn write_u64(self: &mut Self, i: u64)`

- `fn write_u128(self: &mut Self, i: u128)`

- `fn write_usize(self: &mut Self, i: usize)`

- `fn finish(self: &Self) -> u64`

### `RandomState`

```rust
struct RandomState {
    per_hasher_seed: u64,
    global_seed: global::GlobalSeed,
}
```

A [`BuildHasher`](../../serde_core/lib/index.md) for [`fast::FoldHasher`](FoldHasher) that is randomly initialized.

#### Trait Implementations

##### `impl BuildHasher for RandomState`

- `type Hasher = FoldHasher<'static>`

- `fn build_hasher(self: &Self) -> FoldHasher<'static>` — [`FoldHasher`](#foldhasher)

##### `impl Clone for RandomState`

- `fn clone(self: &Self) -> RandomState` — [`RandomState`](#randomstate)

##### `impl Debug for RandomState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for RandomState`

- `fn default() -> Self`

### `SeedableRandomState`

```rust
struct SeedableRandomState {
    per_hasher_seed: u64,
    shared_seed: &'static crate::seed::SharedSeed,
}
```

A [`BuildHasher`](../../serde_core/lib/index.md) for [`fast::FoldHasher`](FoldHasher) that is randomly
initialized by default, but can also be initialized with a specific seed.

This can be useful for e.g. testing, but the downside is that this type
has a size of 16 bytes rather than the 8 bytes [`RandomState`](#randomstate) is.

#### Implementations

- `fn random() -> Self`

- `fn fixed() -> Self`

- `fn with_seed(per_hasher_seed: u64, shared_seed: &'static SharedSeed) -> Self` — [`SharedSeed`](../seed/index.md)

#### Trait Implementations

##### `impl BuildHasher for SeedableRandomState`

- `type Hasher = FoldHasher<'static>`

- `fn build_hasher(self: &Self) -> FoldHasher<'static>` — [`FoldHasher`](#foldhasher)

##### `impl Clone for SeedableRandomState`

- `fn clone(self: &Self) -> SeedableRandomState` — [`SeedableRandomState`](#seedablerandomstate)

##### `impl Debug for SeedableRandomState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for SeedableRandomState`

- `fn default() -> Self`

### `FixedState`

```rust
struct FixedState {
    per_hasher_seed: u64,
}
```

A [`BuildHasher`](../../serde_core/lib/index.md) for [`fast::FoldHasher`](FoldHasher) that always has the same fixed seed.

Not recommended unless you absolutely need determinism.

#### Implementations

- `const fn with_seed(per_hasher_seed: u64) -> Self`

#### Trait Implementations

##### `impl BuildHasher for FixedState`

- `type Hasher = FoldHasher<'static>`

- `fn build_hasher(self: &Self) -> FoldHasher<'static>` — [`FoldHasher`](#foldhasher)

##### `impl Clone for FixedState`

- `fn clone(self: &Self) -> FixedState` — [`FixedState`](#fixedstate)

##### `impl Debug for FixedState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for FixedState`

- `fn default() -> Self`

