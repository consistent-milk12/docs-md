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

*Defined in [`hashbrown-0.16.1/src/control/tag.rs:6`](../../../../.source_1765210505/hashbrown-0.16.1/src/control/tag.rs#L6)*

Single tag in a control group.

#### Implementations

- <span id="tag-const-empty"></span>`const EMPTY: Tag`

- <span id="tag-const-deleted"></span>`const DELETED: Tag`

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

##### `impl<K> Equivalent for Tag`

- <span id="tag-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for Tag`

- <span id="tag-eq"></span>`fn eq(&self, other: &Tag) -> bool` — [`Tag`](#tag)

##### `impl StructuralPartialEq for Tag`

## Traits

### `TagSliceExt`

```rust
trait TagSliceExt { ... }
```

*Defined in [`hashbrown-0.16.1/src/control/tag.rs:67-76`](../../../../.source_1765210505/hashbrown-0.16.1/src/control/tag.rs#L67-L76)*

Extension trait for slices of tags.

#### Required Methods

- `fn fill_tag(&mut self, tag: Tag)`

  Fills the control with the given tag.

#### Provided Methods

- `fn fill_empty(&mut self)`

  Clears out the control.

#### Implementors

- `[Tag]`

