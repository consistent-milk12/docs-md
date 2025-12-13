*[owo_colors](../index.md) / [dyn_colors](index.md)*

---

# Module `dyn_colors`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParseColorError`](#parsecolorerror) | struct | An error for when the color can not be parsed from a string at runtime |
| [`DynColors`](#dyncolors) | enum | An enum describing runtime-configurable colors |

## Structs

### `ParseColorError`

```rust
struct ParseColorError;
```

*Defined in [`owo-colors-4.2.3/src/dyn_colors.rs:72`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_colors.rs#L72)*

An error for when the color can not be parsed from a string at runtime

#### Trait Implementations

##### `impl Any for ParseColorError`

- <span id="parsecolorerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseColorError`

- <span id="parsecolorerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseColorError`

- <span id="parsecolorerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ParseColorError`

- <span id="parsecolorerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ParseColorError`

- <span id="parsecolorerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseColorError`

- <span id="parsecolorerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ParseColorError`

##### `impl<U> TryFrom for ParseColorError`

- <span id="parsecolorerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parsecolorerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseColorError`

- <span id="parsecolorerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parsecolorerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `DynColors`

```rust
enum DynColors {
    Ansi(crate::AnsiColors),
    Css(crate::CssColors),
    Xterm(crate::XtermColors),
    Rgb(u8, u8, u8),
}
```

*Defined in [`owo-colors-4.2.3/src/dyn_colors.rs:13-18`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_colors.rs#L13-L18)*

An enum describing runtime-configurable colors

This can be displayed using [`FgDynColorDisplay`](FgDynColorDisplay) or [`BgDynColorDisplay`](BgDynColorDisplay),
allowing for multiple types of colors to be used at runtime.

#### Trait Implementations

##### `impl Any for DynColors`

- <span id="dyncolors-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DynColors`

- <span id="dyncolors-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DynColors`

- <span id="dyncolors-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DynColors`

- <span id="dyncolors-clone"></span>`fn clone(&self) -> DynColors` — [`DynColors`](../index.md#dyncolors)

##### `impl CloneToUninit for DynColors`

- <span id="dyncolors-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DynColors`

##### `impl Debug for DynColors`

- <span id="dyncolors-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for DynColors`

- <span id="dyncolors-dyncolor-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-dyncolor-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-dyncolor-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-dyncolor-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DynColors`

##### `impl<T> From for DynColors`

- <span id="dyncolors-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for DynColors`

- <span id="dyncolors-fromstr-type-err"></span>`type Err = ParseColorError`

- <span id="dyncolors-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl<U> Into for DynColors`

- <span id="dyncolors-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DynColors`

##### `impl PartialEq for DynColors`

- <span id="dyncolors-partialeq-eq"></span>`fn eq(&self, other: &DynColors) -> bool` — [`DynColors`](../index.md#dyncolors)

##### `impl StructuralPartialEq for DynColors`

##### `impl<U> TryFrom for DynColors`

- <span id="dyncolors-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyncolors-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DynColors`

- <span id="dyncolors-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyncolors-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

