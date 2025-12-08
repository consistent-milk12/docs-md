*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [dynamic](index.md)*

---

# Module `dynamic`

## Traits

### `Dyn`

```rust
trait Dyn: Debug + Pod { ... }
```

A trait for generic access to [`elf::Dyn32`](../../../elf/index.md) and [`elf::Dyn64`](../../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `fn d_tag(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn d_val(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn tag32(self: &Self, endian: <Self as >::Endian) -> Option<u32>`

  Try to convert the tag to a `u32`.

- `fn val32(self: &Self, endian: <Self as >::Endian) -> Option<u32>`

  Try to convert the value to a `u32`.

- `fn is_string(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the value is an offset in the dynamic string table.

- `fn string<'data>(self: &Self, endian: <Self as >::Endian, strings: StringTable<'data>) -> Result<&'data [u8]>`

  Use the value to get a string in a string table.

- `fn is_address(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the value is an address.

