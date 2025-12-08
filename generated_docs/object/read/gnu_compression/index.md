*[object](../../index.md) / [read](../index.md) / [gnu_compression](index.md)*

---

# Module `gnu_compression`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`compressed_file_range`](#compressed_file_range) | fn |  |

## Functions

### `compressed_file_range`

```rust
fn compressed_file_range<'data, R: ReadRef<'data>>(file_data: R, section_offset: u64, section_size: u64) -> read::Result<crate::CompressedFileRange>
```

