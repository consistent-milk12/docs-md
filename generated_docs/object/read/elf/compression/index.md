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

*Defined in [`object-0.37.3/src/read/elf/compression.rs:9-16`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/compression.rs#L9-L16)*

A trait for generic access to [`elf::CompressionHeader32`](../../../elf/index.md) and [`elf::CompressionHeader64`](../../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn ch_type(&self, endian: <Self as >::Endian) -> u32`

- `fn ch_size(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn ch_addralign(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Implementors

- [`CompressionHeader32`](../../../elf/index.md#compressionheader32)
- [`CompressionHeader64`](../../../elf/index.md#compressionheader64)

