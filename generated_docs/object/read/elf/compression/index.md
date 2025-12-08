*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [compression](index.md)*

---

# Module `compression`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CompressionHeader`](#compressionheader) | trait | A trait for generic access to [`elf::CompressionHeader32`] and [`elf::CompressionHeader64`]. |

## Traits

### `CompressionHeader`

```rust
trait CompressionHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::CompressionHeader32`](../../../elf/index.md) and [`elf::CompressionHeader64`](../../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `fn ch_type(&self, endian: <Self as >::Endian) -> u32`

- `fn ch_size(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn ch_addralign(&self, endian: <Self as >::Endian) -> <Self as >::Word`

