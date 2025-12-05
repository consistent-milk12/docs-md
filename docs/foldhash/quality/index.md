*[foldhash](../index.md) / [quality](index.md)*

---

# Module `quality`

The foldhash implementation optimized for quality.

## Structs

### `FoldHasher<'a>`

```rust
struct FoldHasher<'a> {
    inner: fast::FoldHasher<'a>,
}
```

A [`Hasher`](#hasher) instance implementing foldhash, optimized for quality.

While you can create one directly with `FoldHasher::with_seed`, you
most likely want to use [`RandomState`](#randomstate), [`SeedableRandomState`](../fast/index.md) or
[`FixedState`](#fixedstate) to create [`FoldHasher`](../fast/index.md)s.

#### Implementations

- `const fn with_seed(per_hasher_seed: u64, shared_seed: &'a SharedSeed) -> FoldHasher<'a>` — [`SharedSeed`](../../seed/index.md), [`FoldHasher`](../../quality/index.md)

#### Trait Implementations

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> FoldHasher<'a>` — [`FoldHasher`](../../quality/index.md)

##### `impl Hasher<'a>`

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
    inner: fast::RandomState,
}
```

A [`BuildHasher`](#buildhasher) for [`quality::FoldHasher`](FoldHasher) that is randomly initialized.

#### Trait Implementations

##### `impl BuildHasher`

- `type Hasher = FoldHasher<'static>`

- `fn build_hasher(self: &Self) -> FoldHasher<'static>` — [`FoldHasher`](../../quality/index.md)

##### `impl Clone`

- `fn clone(self: &Self) -> RandomState` — [`RandomState`](../../quality/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> RandomState` — [`RandomState`](../../quality/index.md)

### `SeedableRandomState`

```rust
struct SeedableRandomState {
    inner: fast::SeedableRandomState,
}
```

A [`BuildHasher`](#buildhasher) for [`quality::FoldHasher`](FoldHasher) that is randomly
initialized by default, but can also be initialized with a specific seed.

This can be useful for e.g. testing, but the downside is that this type
has a size of 16 bytes rather than the 8 bytes [`RandomState`](#randomstate) is.

#### Implementations

- `fn random() -> Self`

- `fn fixed() -> Self`

- `fn with_seed(per_hasher_seed: u64, shared_seed: &'static SharedSeed) -> Self` — [`SharedSeed`](../../seed/index.md)

#### Trait Implementations

##### `impl BuildHasher`

- `type Hasher = FoldHasher<'static>`

- `fn build_hasher(self: &Self) -> FoldHasher<'static>` — [`FoldHasher`](../../quality/index.md)

##### `impl Clone`

- `fn clone(self: &Self) -> SeedableRandomState` — [`SeedableRandomState`](../../quality/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> SeedableRandomState` — [`SeedableRandomState`](../../quality/index.md)

### `FixedState`

```rust
struct FixedState {
    inner: fast::FixedState,
}
```

A [`BuildHasher`](#buildhasher) for [`quality::FoldHasher`](FoldHasher) that always has the same fixed seed.

Not recommended unless you absolutely need determinism.

#### Implementations

- `const fn with_seed(per_hasher_seed: u64) -> Self`

#### Trait Implementations

##### `impl BuildHasher`

- `type Hasher = FoldHasher<'static>`

- `fn build_hasher(self: &Self) -> FoldHasher<'static>` — [`FoldHasher`](../../quality/index.md)

##### `impl Clone`

- `fn clone(self: &Self) -> FixedState` — [`FixedState`](../../quality/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> FixedState` — [`FixedState`](../../quality/index.md)

