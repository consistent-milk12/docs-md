*[gimli](../../../index.md) / [read](../../index.md) / [util](../index.md) / [sealed](index.md)*

---

# Module `sealed`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CapacityFull`](#capacityfull) | struct |  |
| [`Sealed`](#sealed) | trait | # Safety |

## Structs

### `CapacityFull`

```rust
struct CapacityFull;
```

#### Trait Implementations

##### `impl Clone for CapacityFull`

- <span id="capacityfull-clone"></span>`fn clone(&self) -> CapacityFull` — [`CapacityFull`](#capacityfull)

##### `impl Copy for CapacityFull`

##### `impl Debug for CapacityFull`

- <span id="capacityfull-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

# Safety
Implementer must not modify the content in storage.

#### Required Methods

- `type Storage`

- `fn new_storage() -> <Self as >::Storage`

- `fn grow(_storage: &mut <Self as >::Storage, _additional: usize) -> Result<(), CapacityFull>`

