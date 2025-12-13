*[regex](../index.md) / [find_byte](index.md)*

---

# Module `find_byte`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`find_byte`](#find-byte) | fn | Searches for the given needle in the given haystack. |

## Functions

### `find_byte`

```rust
fn find_byte(needle: u8, haystack: &[u8]) -> Option<usize>
```

*Defined in [`regex-1.12.2/src/find_byte.rs:5-17`](../../../.source_1765633015/regex-1.12.2/src/find_byte.rs#L5-L17)*

Searches for the given needle in the given haystack.

If the perf-literal feature is enabled, then this uses the super optimized
memchr crate. Otherwise, it uses the naive byte-at-a-time implementation.

