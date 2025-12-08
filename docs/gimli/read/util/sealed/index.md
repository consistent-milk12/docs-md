*[gimli](../../../index.md) / [read](../../index.md) / [util](../index.md) / [sealed](index.md)*

---

# Module `sealed`

## Structs

### `CapacityFull`

```rust
struct CapacityFull;
```

#### Trait Implementations

##### `impl Clone for CapacityFull`

- `fn clone(self: &Self) -> CapacityFull` — [`CapacityFull`](#capacityfull)

##### `impl Copy for CapacityFull`

##### `impl Debug for CapacityFull`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

