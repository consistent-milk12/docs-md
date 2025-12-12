*[proc_macro2](../index.md) / [location](index.md)*

---

# Module `location`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LineColumn`](#linecolumn) | struct | A line-column pair representing the start or end of a `Span`. |

## Structs

### `LineColumn`

```rust
struct LineColumn {
    pub line: usize,
    pub column: usize,
}
```

*Defined in [`proc-macro2-1.0.103/src/location.rs:8-15`](../../../.source_1765521767/proc-macro2-1.0.103/src/location.rs#L8-L15)*

A line-column pair representing the start or end of a `Span`.

This type is semver exempt and not exposed by default.

#### Fields

- **`line`**: `usize`

  The 1-indexed line in the source file on which the span starts or ends
  (inclusive).

- **`column`**: `usize`

  The 0-indexed column (in UTF-8 characters) in the source file on which
  the span starts or ends (inclusive).

#### Trait Implementations

##### `impl Clone for LineColumn`

- <span id="linecolumn-clone"></span>`fn clone(&self) -> LineColumn` — [`LineColumn`](#linecolumn)

##### `impl Copy for LineColumn`

##### `impl Debug for LineColumn`

- <span id="linecolumn-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LineColumn`

##### `impl Hash for LineColumn`

- <span id="linecolumn-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for LineColumn`

- <span id="linecolumn-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl PartialEq for LineColumn`

- <span id="linecolumn-eq"></span>`fn eq(&self, other: &LineColumn) -> bool` — [`LineColumn`](#linecolumn)

##### `impl PartialOrd for LineColumn`

- <span id="linecolumn-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl StructuralPartialEq for LineColumn`

