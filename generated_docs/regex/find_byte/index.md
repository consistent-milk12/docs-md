*[regex](../index.md) / [find_byte](index.md)*

---

# Module `find_byte`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`find_byte`](#find_byte) | fn | Searches for the given needle in the given haystack. |

## Functions

### `find_byte`

```rust
fn find_byte(needle: u8, haystack: &[u8]) -> Option<usize>
```

Searches for the given needle in the given haystack.

If the perf-literal feature is enabled, then this uses the super optimized
memchr crate. Otherwise, it uses the naive byte-at-a-time implementation.

