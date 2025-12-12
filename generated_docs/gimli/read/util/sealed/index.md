*[gimli](../../../index.md) / [read](../../index.md) / [util](../index.md) / [sealed](index.md)*

---

# Module `sealed`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CapacityFull`](#capacityfull) | struct |  |
| [`Sealed`](#sealed) | trait | # Safety Implementer must not modify the content in storage. |

## Structs

### `CapacityFull`

```rust
struct CapacityFull;
```

*Defined in [`gimli-0.32.3/src/read/util.rs:25`](../../../../../.source_1765521767/gimli-0.32.3/src/read/util.rs#L25)*

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

*Defined in [`gimli-0.32.3/src/read/util.rs:14-22`](../../../../../.source_1765521767/gimli-0.32.3/src/read/util.rs#L14-L22)*

# Safety
Implementer must not modify the content in storage.

#### Associated Types

- `type Storage`

#### Required Methods

- `fn new_storage() -> <Self as >::Storage`

#### Provided Methods

- `fn grow(_storage: &mut <Self as >::Storage, _additional: usize) -> Result<(), CapacityFull>`

#### Implementors

- `[T; N]`
- `alloc::boxed::Box<[T; N]>`
- `alloc::vec::Vec<T>`

