*[anstyle](../index.md) / [effect](index.md)*

---

# Module `effect`

## Structs

### `Effects`

```rust
struct Effects(u16);
```

A set of text effects

# Examples

```rust
let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
```

#### Implementations

- `const PLAIN: Self`

- `const BOLD: Self`

- `const DIMMED: Self`

- `const ITALIC: Self`

- `const UNDERLINE: Self`

- `const DOUBLE_UNDERLINE: Self`

- `const CURLY_UNDERLINE: Self`

- `const DOTTED_UNDERLINE: Self`

- `const DASHED_UNDERLINE: Self`

- `const BLINK: Self`

- `const INVERT: Self`

- `const HIDDEN: Self`

- `const STRIKETHROUGH: Self`

- `const fn new() -> Self`

- `const fn is_plain(self: Self) -> bool`

- `const fn contains(self: Self, other: Effects) -> bool` — [`Effects`](../index.md)

- `const fn insert(self: Self, other: Effects) -> Self` — [`Effects`](../index.md)

- `const fn remove(self: Self, other: Effects) -> Self` — [`Effects`](../index.md)

- `const fn clear(self: Self) -> Self`

- `const fn set(self: Self, other: Self, enable: bool) -> Self`

- `fn iter(self: Self) -> EffectIter` — [`EffectIter`](../index.md)

- `fn index_iter(self: Self) -> EffectIndexIter` — [`EffectIndexIter`](#effectindexiter)

- `fn render(self: Self) -> impl core::fmt::Display + Copy`

- `fn write_to(self: Self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl BitOr for Effects`

- `type Output = Effects`

- `fn bitor(self: Self, rhs: Self) -> Self`

##### `impl BitOrAssign for Effects`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl Clone for Effects`

- `fn clone(self: &Self) -> Effects` — [`Effects`](../index.md)

##### `impl Copy for Effects`

##### `impl Debug for Effects`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for Effects`

- `fn default() -> Effects` — [`Effects`](../index.md)

##### `impl Eq for Effects`

##### `impl Hash for Effects`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Effects`

- `fn cmp(self: &Self, other: &Effects) -> $crate::cmp::Ordering` — [`Effects`](../index.md)

##### `impl PartialEq for Effects`

- `fn eq(self: &Self, other: &Effects) -> bool` — [`Effects`](../index.md)

##### `impl PartialOrd for Effects`

- `fn partial_cmp(self: &Self, other: &Effects) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Effects`](../index.md)

##### `impl StructuralPartialEq for Effects`

##### `impl Sub for Effects`

- `type Output = Effects`

- `fn sub(self: Self, other: Self) -> Self`

##### `impl SubAssign for Effects`

- `fn sub_assign(self: &mut Self, other: Self)`

### `Metadata`

```rust
struct Metadata {
    name: &'static str,
    escape: &'static str,
}
```

### `EffectsDisplay`

```rust
struct EffectsDisplay(Effects);
```

#### Trait Implementations

##### `impl Clone for EffectsDisplay`

- `fn clone(self: &Self) -> EffectsDisplay` — [`EffectsDisplay`](#effectsdisplay)

##### `impl Copy for EffectsDisplay`

##### `impl Debug for EffectsDisplay`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for EffectsDisplay`

- `fn default() -> EffectsDisplay` — [`EffectsDisplay`](#effectsdisplay)

##### `impl Display for EffectsDisplay`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for EffectsDisplay`

- `fn to_string(self: &Self) -> String`

### `EffectIter`

```rust
struct EffectIter {
    index: usize,
    effects: Effects,
}
```

Enumerate each enabled value in [`Effects`](../index.md)

#### Trait Implementations

##### `impl Clone for EffectIter`

- `fn clone(self: &Self) -> EffectIter` — [`EffectIter`](../index.md)

##### `impl Debug for EffectIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for EffectIter`

##### `impl<I> IntoIterator for EffectIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for EffectIter`

- `type Item = Effects`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIter`

- `fn eq(self: &Self, other: &EffectIter) -> bool` — [`EffectIter`](../index.md)

##### `impl StructuralPartialEq for EffectIter`

### `EffectIndexIter`

```rust
struct EffectIndexIter {
    index: usize,
    effects: Effects,
}
```

#### Trait Implementations

##### `impl Clone for EffectIndexIter`

- `fn clone(self: &Self) -> EffectIndexIter` — [`EffectIndexIter`](#effectindexiter)

##### `impl Debug for EffectIndexIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for EffectIndexIter`

##### `impl<I> IntoIterator for EffectIndexIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for EffectIndexIter`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIndexIter`

- `fn eq(self: &Self, other: &EffectIndexIter) -> bool` — [`EffectIndexIter`](#effectindexiter)

##### `impl StructuralPartialEq for EffectIndexIter`

## Constants

### `METADATA`

```rust
const METADATA: [Metadata; 12];
```

