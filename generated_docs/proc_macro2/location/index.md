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

*Defined in [`proc-macro2-1.0.103/src/location.rs:8-15`](../../../.source_1765633015/proc-macro2-1.0.103/src/location.rs#L8-L15)*

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

##### `impl Any for LineColumn`

- <span id="linecolumn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineColumn`

- <span id="linecolumn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineColumn`

- <span id="linecolumn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LineColumn`

- <span id="linecolumn-clone"></span>`fn clone(&self) -> LineColumn` — [`LineColumn`](#linecolumn)

##### `impl CloneToUninit for LineColumn`

- <span id="linecolumn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LineColumn`

##### `impl Debug for LineColumn`

- <span id="linecolumn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LineColumn`

##### `impl<T> From for LineColumn`

- <span id="linecolumn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LineColumn`

- <span id="linecolumn-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for LineColumn`

- <span id="linecolumn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for LineColumn`

- <span id="linecolumn-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl PartialEq for LineColumn`

- <span id="linecolumn-partialeq-eq"></span>`fn eq(&self, other: &LineColumn) -> bool` — [`LineColumn`](#linecolumn)

##### `impl PartialOrd for LineColumn`

- <span id="linecolumn-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl StructuralPartialEq for LineColumn`

##### `impl ToOwned for LineColumn`

- <span id="linecolumn-toowned-type-owned"></span>`type Owned = T`

- <span id="linecolumn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="linecolumn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LineColumn`

- <span id="linecolumn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linecolumn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineColumn`

- <span id="linecolumn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linecolumn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

