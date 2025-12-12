*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [dynamic](index.md)*

---

# Module `dynamic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Dyn`](#dyn) | trait | A trait for generic access to [`elf::Dyn32`] and [`elf::Dyn64`]. |

## Traits

### `Dyn`

```rust
trait Dyn: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/elf/dynamic.rs:11-87`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/dynamic.rs#L11-L87)*

A trait for generic access to [`elf::Dyn32`](../../../elf/index.md) and [`elf::Dyn64`](../../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn d_tag(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn d_val(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Provided Methods

- `fn tag32(&self, endian: <Self as >::Endian) -> Option<u32>`

  Try to convert the tag to a `u32`.

- `fn val32(&self, endian: <Self as >::Endian) -> Option<u32>`

  Try to convert the value to a `u32`.

- `fn is_string(&self, endian: <Self as >::Endian) -> bool`

  Return true if the value is an offset in the dynamic string table.

- `fn string<'data>(&self, endian: <Self as >::Endian, strings: StringTable<'data>) -> Result<&'data [u8]>`

  Use the value to get a string in a string table.

- `fn is_address(&self, endian: <Self as >::Endian) -> bool`

  Return true if the value is an address.

#### Implementors

- [`Dyn32`](../../../elf/index.md#dyn32)
- [`Dyn64`](../../../elf/index.md#dyn64)

