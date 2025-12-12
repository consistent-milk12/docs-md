*[anstyle](../index.md) / [effect](index.md)*

---

# Module `effect`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Effects`](#effects) | struct | A set of text effects |
| [`Metadata`](#metadata) | struct |  |
| [`EffectsDisplay`](#effectsdisplay) | struct |  |
| [`EffectIter`](#effectiter) | struct | Enumerate each enabled value in [`Effects`] |
| [`EffectIndexIter`](#effectindexiter) | struct |  |
| [`METADATA`](#metadata) | const |  |

## Structs

### `Effects`

```rust
struct Effects(u16);
```

*Defined in [`anstyle-1.0.13/src/effect.rs:9`](../../../.source_1765210505/anstyle-1.0.13/src/effect.rs#L9)*

A set of text effects

# Examples

```rust
let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
```

#### Implementations

- <span id="effects-const-plain"></span>`const PLAIN: Self`

- <span id="effects-const-bold"></span>`const BOLD: Self`

- <span id="effects-const-dimmed"></span>`const DIMMED: Self`

- <span id="effects-const-italic"></span>`const ITALIC: Self`

- <span id="effects-const-underline"></span>`const UNDERLINE: Self`

- <span id="effects-const-double-underline"></span>`const DOUBLE_UNDERLINE: Self`

- <span id="effects-const-curly-underline"></span>`const CURLY_UNDERLINE: Self`

- <span id="effects-const-dotted-underline"></span>`const DOTTED_UNDERLINE: Self`

- <span id="effects-const-dashed-underline"></span>`const DASHED_UNDERLINE: Self`

- <span id="effects-const-blink"></span>`const BLINK: Self`

- <span id="effects-const-invert"></span>`const INVERT: Self`

- <span id="effects-const-hidden"></span>`const HIDDEN: Self`

- <span id="effects-const-strikethrough"></span>`const STRIKETHROUGH: Self`

- <span id="effects-new"></span>`const fn new() -> Self`

- <span id="effects-is-plain"></span>`const fn is_plain(self) -> bool`

- <span id="effects-contains"></span>`const fn contains(self, other: Effects) -> bool` — [`Effects`](../index.md#effects)

- <span id="effects-insert"></span>`const fn insert(self, other: Effects) -> Self` — [`Effects`](../index.md#effects)

- <span id="effects-remove"></span>`const fn remove(self, other: Effects) -> Self` — [`Effects`](../index.md#effects)

- <span id="effects-clear"></span>`const fn clear(self) -> Self`

- <span id="effects-set"></span>`const fn set(self, other: Self, enable: bool) -> Self`

- <span id="effects-iter"></span>`fn iter(self) -> EffectIter` — [`EffectIter`](../index.md#effectiter)

- <span id="effects-index-iter"></span>`fn index_iter(self) -> EffectIndexIter` — [`EffectIndexIter`](#effectindexiter)

- <span id="effects-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

- <span id="effects-write-to"></span>`fn write_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl BitOr for Effects`

- <span id="effects-bitor-type-output"></span>`type Output = Effects`

- <span id="effects-bitor"></span>`fn bitor(self, rhs: Self) -> Self`

##### `impl BitOrAssign for Effects`

- <span id="effects-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl Clone for Effects`

- <span id="effects-clone"></span>`fn clone(&self) -> Effects` — [`Effects`](../index.md#effects)

##### `impl Copy for Effects`

##### `impl Debug for Effects`

- <span id="effects-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for Effects`

- <span id="effects-default"></span>`fn default() -> Effects` — [`Effects`](../index.md#effects)

##### `impl Eq for Effects`

##### `impl Hash for Effects`

- <span id="effects-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Effects`

- <span id="effects-cmp"></span>`fn cmp(&self, other: &Effects) -> cmp::Ordering` — [`Effects`](../index.md#effects)

##### `impl PartialEq for Effects`

- <span id="effects-eq"></span>`fn eq(&self, other: &Effects) -> bool` — [`Effects`](../index.md#effects)

##### `impl PartialOrd for Effects`

- <span id="effects-partial-cmp"></span>`fn partial_cmp(&self, other: &Effects) -> option::Option<cmp::Ordering>` — [`Effects`](../index.md#effects)

##### `impl StructuralPartialEq for Effects`

##### `impl Sub for Effects`

- <span id="effects-sub-type-output"></span>`type Output = Effects`

- <span id="effects-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for Effects`

- <span id="effects-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

### `Metadata`

```rust
struct Metadata {
    name: &'static str,
    escape: &'static str,
}
```

*Defined in [`anstyle-1.0.13/src/effect.rs:263-266`](../../../.source_1765210505/anstyle-1.0.13/src/effect.rs#L263-L266)*

### `EffectsDisplay`

```rust
struct EffectsDisplay(Effects);
```

*Defined in [`anstyle-1.0.13/src/effect.rs:320`](../../../.source_1765210505/anstyle-1.0.13/src/effect.rs#L320)*

#### Trait Implementations

##### `impl Clone for EffectsDisplay`

- <span id="effectsdisplay-clone"></span>`fn clone(&self) -> EffectsDisplay` — [`EffectsDisplay`](#effectsdisplay)

##### `impl Copy for EffectsDisplay`

##### `impl Debug for EffectsDisplay`

- <span id="effectsdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EffectsDisplay`

- <span id="effectsdisplay-default"></span>`fn default() -> EffectsDisplay` — [`EffectsDisplay`](#effectsdisplay)

##### `impl Display for EffectsDisplay`

- <span id="effectsdisplay-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl ToString for EffectsDisplay`

- <span id="effectsdisplay-to-string"></span>`fn to_string(&self) -> String`

### `EffectIter`

```rust
struct EffectIter {
    index: usize,
    effects: Effects,
}
```

*Defined in [`anstyle-1.0.13/src/effect.rs:334-337`](../../../.source_1765210505/anstyle-1.0.13/src/effect.rs#L334-L337)*

Enumerate each enabled value in [`Effects`](../index.md)

#### Trait Implementations

##### `impl Clone for EffectIter`

- <span id="effectiter-clone"></span>`fn clone(&self) -> EffectIter` — [`EffectIter`](../index.md#effectiter)

##### `impl Debug for EffectIter`

- <span id="effectiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for EffectIter`

##### `impl IntoIterator for EffectIter`

- <span id="effectiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="effectiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="effectiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for EffectIter`

- <span id="effectiter-iterator-type-item"></span>`type Item = Effects`

- <span id="effectiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIter`

- <span id="effectiter-eq"></span>`fn eq(&self, other: &EffectIter) -> bool` — [`EffectIter`](../index.md#effectiter)

##### `impl StructuralPartialEq for EffectIter`

### `EffectIndexIter`

```rust
struct EffectIndexIter {
    index: usize,
    effects: Effects,
}
```

*Defined in [`anstyle-1.0.13/src/effect.rs:358-361`](../../../.source_1765210505/anstyle-1.0.13/src/effect.rs#L358-L361)*

#### Trait Implementations

##### `impl Clone for EffectIndexIter`

- <span id="effectindexiter-clone"></span>`fn clone(&self) -> EffectIndexIter` — [`EffectIndexIter`](#effectindexiter)

##### `impl Debug for EffectIndexIter`

- <span id="effectindexiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for EffectIndexIter`

##### `impl IntoIterator for EffectIndexIter`

- <span id="effectindexiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="effectindexiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="effectindexiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for EffectIndexIter`

- <span id="effectindexiter-iterator-type-item"></span>`type Item = usize`

- <span id="effectindexiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIndexIter`

- <span id="effectindexiter-eq"></span>`fn eq(&self, other: &EffectIndexIter) -> bool` — [`EffectIndexIter`](#effectindexiter)

##### `impl StructuralPartialEq for EffectIndexIter`

## Constants

### `METADATA`
```rust
const METADATA: [Metadata; 12];
```

*Defined in [`anstyle-1.0.13/src/effect.rs:268-317`](../../../.source_1765210505/anstyle-1.0.13/src/effect.rs#L268-L317)*

