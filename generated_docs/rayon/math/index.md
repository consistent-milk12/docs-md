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

*Defined in [`rayon-1.11.0/src/math.rs:4-25`](../../../.source_1765210505/rayon-1.11.0/src/math.rs#L4-L25)*

Normalize arbitrary `RangeBounds` to a `Range`

