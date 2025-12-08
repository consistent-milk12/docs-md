*[hashbrown](../../index.md) / [control](../index.md) / [tag](index.md)*

---

# Module `tag`

## Structs

### `Tag`

```rust
struct Tag(u8);
```

Single tag in a control group.

#### Implementations

- `const EMPTY: Tag`

- `const DELETED: Tag`

- `const fn is_full(self: Self) -> bool`

- `const fn is_special(self: Self) -> bool`

- `const fn special_is_empty(self: Self) -> bool`

- `const fn full(hash: u64) -> Tag` — [`Tag`](#tag)

#### Trait Implementations

##### `impl Clone for Tag`

- `fn clone(self: &Self) -> Tag` — [`Tag`](#tag)

##### `impl Copy for Tag`

##### `impl Debug for Tag`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Tag`

##### `impl<Q, K> Equivalent for Tag`

- `fn equivalent(self: &Self, key: &K) -> bool`

##### `impl PartialEq for Tag`

- `fn eq(self: &Self, other: &Tag) -> bool` — [`Tag`](#tag)

##### `impl StructuralPartialEq for Tag`

## Traits

### `TagSliceExt`

```rust
trait TagSliceExt { ... }
```

Extension trait for slices of tags.

#### Required Methods

- `fn fill_tag(self: &mut Self, tag: Tag)`

  Fills the control with the given tag.

- `fn fill_empty(self: &mut Self)`

  Clears out the control.

