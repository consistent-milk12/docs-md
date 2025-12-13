*[owo_colors](../../index.md) / [colors](../index.md) / [ansi_colors](index.md)*

---

# Module `ansi_colors`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AnsiColors`](#ansicolors) | enum | Available standard ANSI colors for use with [`OwoColorize::color`](OwoColorize::color) or [`OwoColorize::on_color`](OwoColorize::on_color) |

## Enums

### `AnsiColors`

```rust
enum AnsiColors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

Available standard ANSI colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Any for AnsiColors`

- <span id="ansicolors-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AnsiColors`

- <span id="ansicolors-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AnsiColors`

- <span id="ansicolors-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AnsiColors`

- <span id="ansicolors-clone"></span>`fn clone(&self) -> AnsiColors` — [`AnsiColors`](#ansicolors)

##### `impl CloneToUninit for AnsiColors`

- <span id="ansicolors-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AnsiColors`

##### `impl Debug for AnsiColors`

- <span id="ansicolors-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for AnsiColors`

- <span id="ansicolors-dyncolor-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-dyncolor-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-dyncolor-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-dyncolor-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AnsiColors`

##### `impl<T> From for AnsiColors`

- <span id="ansicolors-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AnsiColors`

- <span id="ansicolors-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for AnsiColors`

##### `impl PartialEq for AnsiColors`

- <span id="ansicolors-partialeq-eq"></span>`fn eq(&self, other: &AnsiColors) -> bool` — [`AnsiColors`](#ansicolors)

##### `impl StructuralPartialEq for AnsiColors`

##### `impl<U> TryFrom for AnsiColors`

- <span id="ansicolors-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ansicolors-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AnsiColors`

- <span id="ansicolors-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ansicolors-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

