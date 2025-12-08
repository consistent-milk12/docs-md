*[rayon](../index.md) / [math](index.md)*

---

# Module `math`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`simplify_range`](#simplify_range) | fn | Normalize arbitrary `RangeBounds` to a `Range` |

## Functions

### `simplify_range`

```rust
fn simplify_range(range: impl RangeBounds<usize>, len: usize) -> std::ops::Range<usize>
```

Normalize arbitrary `RangeBounds` to a `Range`

