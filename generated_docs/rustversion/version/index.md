*[rustversion](../index.md) / [version](index.md)*

---

# Module `version`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Version`](#version) | struct |  |
| [`Channel`](#channel) | enum |  |

## Structs

### `Version`

```rust
struct Version {
    pub minor: u16,
    pub patch: u16,
    pub channel: Channel,
}
```

*Defined in [`rustversion-1.0.22/src/version.rs:6-10`](../../../.source_1765521767/rustversion-1.0.22/src/version.rs#L6-L10)*

#### Trait Implementations

##### `impl Clone for Version`

- <span id="version-clone"></span>`fn clone(&self) -> Version` — [`Version`](#version)

##### `impl Copy for Version`

##### `impl Debug for Version`

- <span id="version-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for crate::version::Version`

- <span id="crateversionversion-eq"></span>`fn eq(&self, rhs: &Bound) -> bool` — [`Bound`](../bound/index.md#bound)

##### `impl PartialOrd for crate::version::Version`

- <span id="crateversionversion-partial-cmp"></span>`fn partial_cmp(&self, rhs: &Bound) -> Option<Ordering>` — [`Bound`](../bound/index.md#bound)

##### `impl StructuralPartialEq for Version`

## Enums

### `Channel`

```rust
enum Channel {
    Stable,
    Beta,
    Nightly(crate::date::Date),
    Dev,
}
```

*Defined in [`rustversion-1.0.22/src/version.rs:13-18`](../../../.source_1765521767/rustversion-1.0.22/src/version.rs#L13-L18)*

#### Trait Implementations

##### `impl Clone for Channel`

- <span id="channel-clone"></span>`fn clone(&self) -> Channel` — [`Channel`](#channel)

##### `impl Copy for Channel`

##### `impl Debug for Channel`

- <span id="channel-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Channel`

- <span id="channel-eq"></span>`fn eq(&self, other: &Channel) -> bool` — [`Channel`](#channel)

##### `impl StructuralPartialEq for Channel`

