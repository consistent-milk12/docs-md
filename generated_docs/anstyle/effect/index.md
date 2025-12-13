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

*Defined in [`anstyle-1.0.13/src/effect.rs:9`](../../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L9)*

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

  No effects enabled

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::new();

  ```

- <span id="effects-is-plain"></span>`const fn is_plain(self) -> bool`

  Check if no effects are enabled

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::new();

  assert!(effects.is_plain());

  

  let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;

  assert!(!effects.is_plain());

  ```

- <span id="effects-contains"></span>`const fn contains(self, other: Effects) -> bool` — [`Effects`](../index.md#effects)

  Returns `true` if all of the effects in `other` are contained within `self`.

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;

  assert!(effects.contains(anstyle::Effects::BOLD));

  

  let effects = anstyle::Effects::new();

  assert!(!effects.contains(anstyle::Effects::BOLD));

  ```

- <span id="effects-insert"></span>`const fn insert(self, other: Effects) -> Self` — [`Effects`](../index.md#effects)

  Inserts the specified effects in-place.

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::new().insert(anstyle::Effects::new());

  assert!(effects.is_plain());

  

  let effects = anstyle::Effects::new().insert(anstyle::Effects::BOLD);

  assert!(effects.contains(anstyle::Effects::BOLD));

  ```

- <span id="effects-remove"></span>`const fn remove(self, other: Effects) -> Self` — [`Effects`](../index.md#effects)

  Removes the specified effects in-place.

  

  # Examples

  

  ```rust

  let effects = (anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE).remove(anstyle::Effects::BOLD);

  assert!(!effects.contains(anstyle::Effects::BOLD));

  assert!(effects.contains(anstyle::Effects::UNDERLINE));

  ```

- <span id="effects-clear"></span>`const fn clear(self) -> Self`

  Reset all effects in-place

  ```rust

  let effects = (anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE).clear();

  assert!(!effects.contains(anstyle::Effects::BOLD));

  assert!(!effects.contains(anstyle::Effects::UNDERLINE));

  ```

- <span id="effects-set"></span>`const fn set(self, other: Self, enable: bool) -> Self`

  Enable or disable the specified effects depending on the passed value.

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::new().set(anstyle::Effects::BOLD, true);

  assert!(effects.contains(anstyle::Effects::BOLD));

  ```

- <span id="effects-iter"></span>`fn iter(self) -> EffectIter` — [`EffectIter`](../index.md#effectiter)

  Iterate over enabled effects

- <span id="effects-index-iter"></span>`fn index_iter(self) -> EffectIndexIter` — [`EffectIndexIter`](#effectindexiter)

  Iterate over enabled effect indices

- <span id="effects-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code

- <span id="effects-write-to"></span>`fn write_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Any for Effects`

- <span id="effects-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl BitOr for Effects`

- <span id="effects-bitor-type-output"></span>`type Output = Effects`

- <span id="effects-bitor"></span>`fn bitor(self, rhs: Self) -> Self`

##### `impl BitOrAssign for Effects`

- <span id="effects-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl<T> Borrow for Effects`

- <span id="effects-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Effects`

- <span id="effects-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Effects`

- <span id="effects-clone"></span>`fn clone(&self) -> Effects` — [`Effects`](../index.md#effects)

##### `impl CloneToUninit for Effects`

- <span id="effects-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Effects`

##### `impl Debug for Effects`

- <span id="effects-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for Effects`

- <span id="effects-default"></span>`fn default() -> Effects` — [`Effects`](../index.md#effects)

##### `impl Eq for Effects`

##### `impl<T> From for Effects`

- <span id="effects-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Effects`

- <span id="effects-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Effects`

- <span id="effects-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Effects`

- <span id="effects-ord-cmp"></span>`fn cmp(&self, other: &Effects) -> cmp::Ordering` — [`Effects`](../index.md#effects)

##### `impl PartialEq for Effects`

- <span id="effects-partialeq-eq"></span>`fn eq(&self, other: &Effects) -> bool` — [`Effects`](../index.md#effects)

##### `impl PartialOrd for Effects`

- <span id="effects-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Effects) -> option::Option<cmp::Ordering>` — [`Effects`](../index.md#effects)

##### `impl StructuralPartialEq for Effects`

##### `impl Sub for Effects`

- <span id="effects-sub-type-output"></span>`type Output = Effects`

- <span id="effects-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for Effects`

- <span id="effects-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

##### `impl ToOwned for Effects`

- <span id="effects-toowned-type-owned"></span>`type Owned = T`

- <span id="effects-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="effects-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Effects`

- <span id="effects-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="effects-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Effects`

- <span id="effects-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="effects-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Metadata`

```rust
struct Metadata {
    name: &'static str,
    escape: &'static str,
}
```

*Defined in [`anstyle-1.0.13/src/effect.rs:263-266`](../../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L263-L266)*

#### Trait Implementations

##### `impl Any for Metadata`

- <span id="metadata-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Metadata`

- <span id="metadata-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Metadata`

- <span id="metadata-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Metadata`

- <span id="metadata-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Metadata`

- <span id="metadata-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Metadata`

- <span id="metadata-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="metadata-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Metadata`

- <span id="metadata-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="metadata-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EffectsDisplay`

```rust
struct EffectsDisplay(Effects);
```

*Defined in [`anstyle-1.0.13/src/effect.rs:320`](../../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L320)*

#### Trait Implementations

##### `impl Any for EffectsDisplay`

- <span id="effectsdisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EffectsDisplay`

- <span id="effectsdisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EffectsDisplay`

- <span id="effectsdisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for EffectsDisplay`

- <span id="effectsdisplay-clone"></span>`fn clone(&self) -> EffectsDisplay` — [`EffectsDisplay`](#effectsdisplay)

##### `impl CloneToUninit for EffectsDisplay`

- <span id="effectsdisplay-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for EffectsDisplay`

##### `impl Debug for EffectsDisplay`

- <span id="effectsdisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EffectsDisplay`

- <span id="effectsdisplay-default"></span>`fn default() -> EffectsDisplay` — [`EffectsDisplay`](#effectsdisplay)

##### `impl Display for EffectsDisplay`

- <span id="effectsdisplay-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for EffectsDisplay`

- <span id="effectsdisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EffectsDisplay`

- <span id="effectsdisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for EffectsDisplay`

- <span id="effectsdisplay-toowned-type-owned"></span>`type Owned = T`

- <span id="effectsdisplay-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="effectsdisplay-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for EffectsDisplay`

- <span id="effectsdisplay-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for EffectsDisplay`

- <span id="effectsdisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="effectsdisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EffectsDisplay`

- <span id="effectsdisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="effectsdisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EffectIter`

```rust
struct EffectIter {
    index: usize,
    effects: Effects,
}
```

*Defined in [`anstyle-1.0.13/src/effect.rs:334-337`](../../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L334-L337)*

Enumerate each enabled value in [`Effects`](../index.md)

#### Trait Implementations

##### `impl Any for EffectIter`

- <span id="effectiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EffectIter`

- <span id="effectiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EffectIter`

- <span id="effectiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for EffectIter`

- <span id="effectiter-clone"></span>`fn clone(&self) -> EffectIter` — [`EffectIter`](../index.md#effectiter)

##### `impl CloneToUninit for EffectIter`

- <span id="effectiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for EffectIter`

- <span id="effectiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for EffectIter`

##### `impl<T> From for EffectIter`

- <span id="effectiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EffectIter`

- <span id="effectiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for EffectIter`

- <span id="effectiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="effectiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="effectiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for EffectIter`

- <span id="effectiter-iterator-type-item"></span>`type Item = Effects`

- <span id="effectiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIter`

- <span id="effectiter-partialeq-eq"></span>`fn eq(&self, other: &EffectIter) -> bool` — [`EffectIter`](../index.md#effectiter)

##### `impl StructuralPartialEq for EffectIter`

##### `impl ToOwned for EffectIter`

- <span id="effectiter-toowned-type-owned"></span>`type Owned = T`

- <span id="effectiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="effectiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EffectIter`

- <span id="effectiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="effectiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EffectIter`

- <span id="effectiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="effectiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EffectIndexIter`

```rust
struct EffectIndexIter {
    index: usize,
    effects: Effects,
}
```

*Defined in [`anstyle-1.0.13/src/effect.rs:358-361`](../../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L358-L361)*

#### Trait Implementations

##### `impl Any for EffectIndexIter`

- <span id="effectindexiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EffectIndexIter`

- <span id="effectindexiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EffectIndexIter`

- <span id="effectindexiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for EffectIndexIter`

- <span id="effectindexiter-clone"></span>`fn clone(&self) -> EffectIndexIter` — [`EffectIndexIter`](#effectindexiter)

##### `impl CloneToUninit for EffectIndexIter`

- <span id="effectindexiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for EffectIndexIter`

- <span id="effectindexiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for EffectIndexIter`

##### `impl<T> From for EffectIndexIter`

- <span id="effectindexiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EffectIndexIter`

- <span id="effectindexiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for EffectIndexIter`

- <span id="effectindexiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="effectindexiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="effectindexiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for EffectIndexIter`

- <span id="effectindexiter-iterator-type-item"></span>`type Item = usize`

- <span id="effectindexiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIndexIter`

- <span id="effectindexiter-partialeq-eq"></span>`fn eq(&self, other: &EffectIndexIter) -> bool` — [`EffectIndexIter`](#effectindexiter)

##### `impl StructuralPartialEq for EffectIndexIter`

##### `impl ToOwned for EffectIndexIter`

- <span id="effectindexiter-toowned-type-owned"></span>`type Owned = T`

- <span id="effectindexiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="effectindexiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EffectIndexIter`

- <span id="effectindexiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="effectindexiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EffectIndexIter`

- <span id="effectindexiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="effectindexiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `METADATA`
```rust
const METADATA: [Metadata; 12];
```

*Defined in [`anstyle-1.0.13/src/effect.rs:268-317`](../../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L268-L317)*

