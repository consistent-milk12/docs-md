*[object](../../index.md) / [read](../index.md) / [gnu_compression](index.md)*

---

# Module `gnu_compression`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`compressed_file_range`](#compressed-file-range) | fn |  |

## Functions

### `compressed_file_range`

```rust
fn compressed_file_range<'data, R: ReadRef<'data>>(file_data: R, section_offset: u64, section_size: u64) -> read::Result<crate::CompressedFileRange>
```

*Defined in [`object-0.37.3/src/read/gnu_compression.rs:7-36`](../../../../.source_1765521767/object-0.37.3/src/read/gnu_compression.rs#L7-L36)*

