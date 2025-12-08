*[hashbrown](../../index.md) / [control](../index.md) / [tag](index.md)*

---

# Module `tag`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Tag`](#tag) | struct | Single tag in a control group. |
| [`TagSliceExt`](#tagsliceext) | trait | Extension trait for slices of tags. |

## Structs

### `Tag`

```rust
struct Tag(u8);
```

Single tag in a control group.

#### Implementations

- <span id="tag-empty"></span>`const EMPTY: Tag`

- <span id="tag-deleted"></span>`const DELETED: Tag`

- <span id="tag-is-full"></span>`const fn is_full(self) -> bool`

- <span id="tag-is-special"></span>`const fn is_special(self) -> bool`

- <span id="tag-special-is-empty"></span>`const fn special_is_empty(self) -> bool`

- <span id="tag-full"></span>`const fn full(hash: u64) -> Tag` — [`Tag`](#tag)

#### Trait Implementations

##### `impl Clone for Tag`

- <span id="tag-clone"></span>`fn clone(&self) -> Tag` — [`Tag`](#tag)

##### `impl Copy for Tag`

##### `impl Debug for Tag`

- <span id="tag-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Tag`

##### `impl<Q, K> Equivalent for Tag`

- <span id="tag-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for Tag`

- <span id="tag-eq"></span>`fn eq(&self, other: &Tag) -> bool` — [`Tag`](#tag)

##### `impl StructuralPartialEq for Tag`

## Traits

### `TagSliceExt`

```rust
trait TagSliceExt { ... }
```

Extension trait for slices of tags.

#### Required Methods

- `fn fill_tag(&mut self, tag: Tag)`

  Fills the control with the given tag.

- `fn fill_empty(&mut self)`

  Clears out the control.

