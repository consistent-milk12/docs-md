# Crate `anstyle`

ANSI Text Styling

*A portmanteau of "ansi style"*

`anstyle` provides core types describing [ANSI styling escape
codes](https://en.wikipedia.org/wiki/ANSI_escape_code) for interoperability
between crates.

Example use cases:
- An argument parser allowing callers to define the colors used in the help-output without
  putting the text formatting crate in the public API
- A style description parser that can work with any text formatting crate

Priorities:
1. API stability
2. Low compile-time and binary-size overhead
3. `const` friendly API for callers to statically define their stylesheet

For integration with text styling crate, see:
- [anstyle-ansi-term](https://docs.rs/anstyle-ansi-term)
- [anstyle-crossterm](https://docs.rs/anstyle-crossterm)
- [anstyle-owo-colors](https://docs.rs/anstyle-owo-colors)
- [anstyle-termcolor](https://docs.rs/anstyle-termcolor)
- [anstyle-yansi](https://docs.rs/anstyle-yansi)

User-styling parsers:
- [anstyle-git](https://docs.rs/anstyle-git): Parse Git style descriptions
- [anstyle-ls](https://docs.rs/anstyle-ls): Parse `LS_COLORS` style descriptions

Convert to other formats
- [anstream](https://docs.rs/anstream): A simple cross platform library for writing colored text to a terminal
- [anstyle-roff](https://docs.rs/anstyle-roff): For converting to ROFF
- [anstyle-syntect](https://docs.rs/anstyle-syntect): For working with syntax highlighting

Utilities
- [anstyle-lossy](https://docs.rs/anstyle-lossy): Convert between `anstyle::Color` types
- [anstyle-parse](https://docs.rs/anstyle-parse): Parsing ANSI Style Escapes
- [anstyle-wincon](https://docs.rs/anstyle-wincon): Styling legacy Microsoft terminals

# Examples

The core type is [`Style`](#style):
```rust
let style = anstyle::Style::new().bold();
```

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`color`](#color)
  - [`effect`](#effect)
  - [`reset`](#reset)
  - [`style`](#style)
- [Structs](#structs)
  - [`Ansi256Color`](#ansi256color)
  - [`RgbColor`](#rgbcolor)
  - [`DisplayBuffer`](#displaybuffer)
  - [`NullFormatter`](#nullformatter)
  - [`Effects`](#effects)
  - [`Metadata`](#metadata)
  - [`EffectsDisplay`](#effectsdisplay)
  - [`EffectIter`](#effectiter)
  - [`EffectIndexIter`](#effectindexiter)
  - [`Reset`](#reset)
  - [`Style`](#style)
  - [`StyleDisplay`](#styledisplay)
- [Enums](#enums)
  - [`Color`](#color)
  - [`AnsiColor`](#ansicolor)
- [Constants](#constants)
  - [`DISPLAY_BUFFER_CAPACITY`](#display-buffer-capacity)
  - [`METADATA`](#metadata)
  - [`RESET`](#reset)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`color`](#color) | mod |  |
| [`effect`](#effect) | mod |  |
| [`reset`](#reset) | mod |  |
| [`style`](#style) | mod |  |
| [`Ansi256Color`](#ansi256color) | struct | 256 (8-bit) color support |
| [`RgbColor`](#rgbcolor) | struct | 24-bit ANSI RGB color codes |
| [`DisplayBuffer`](#displaybuffer) | struct |  |
| [`NullFormatter`](#nullformatter) | struct |  |
| [`Effects`](#effects) | struct | A set of text effects |
| [`Metadata`](#metadata) | struct |  |
| [`EffectsDisplay`](#effectsdisplay) | struct |  |
| [`EffectIter`](#effectiter) | struct | Enumerate each enabled value in [`Effects`] |
| [`EffectIndexIter`](#effectindexiter) | struct |  |
| [`Reset`](#reset) | struct | Reset terminal formatting |
| [`Style`](#style) | struct | ANSI Text styling |
| [`StyleDisplay`](#styledisplay) | struct |  |
| [`Color`](#color) | enum | Any ANSI color code scheme |
| [`AnsiColor`](#ansicolor) | enum | Available 4-bit ANSI color palette codes |
| [`DISPLAY_BUFFER_CAPACITY`](#display-buffer-capacity) | const |  |
| [`METADATA`](#metadata) | const |  |
| [`RESET`](#reset) | const |  |

## Modules

- [`macros`](macros/index.md)
- [`color`](color/index.md)
- [`effect`](effect/index.md)
- [`reset`](reset/index.md)
- [`style`](style/index.md)

## Structs

### `Ansi256Color`

```rust
struct Ansi256Color(u8);
```

*Defined in [`anstyle-1.0.13/src/color.rs:352`](../../.source_1765521767/anstyle-1.0.13/src/color.rs#L352)*

256 (8-bit) color support

- `0..16` are [`AnsiColor`](#ansicolor) palette codes
- `0..232` map to [`RgbColor`](#rgbcolor) color values
- `232..` map to [`RgbColor`](#rgbcolor) gray-scale values

#### Implementations

- <span id="ansi256color-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

  Create a `Style` with this as the foreground

- <span id="ansi256color-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](#style)

  Create a `Style` with this as the foreground

- <span id="ansi256color-index"></span>`const fn index(self) -> u8`

  Get the raw value

- <span id="ansi256color-into-ansi"></span>`const fn into_ansi(self) -> Option<AnsiColor>` — [`AnsiColor`](#ansicolor)

  Convert to [`AnsiColor`](#ansicolor) when there is a 1:1 mapping

- <span id="ansi256color-from-ansi"></span>`const fn from_ansi(color: AnsiColor) -> Self` — [`AnsiColor`](#ansicolor)

  Losslessly convert from [`AnsiColor`](#ansicolor)

- <span id="ansi256color-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code for a foreground color

- <span id="ansi256color-as-fg-buffer"></span>`fn as_fg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md#displaybuffer)

- <span id="ansi256color-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code for a background color

- <span id="ansi256color-as-bg-buffer"></span>`fn as_bg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md#displaybuffer)

- <span id="ansi256color-as-underline-buffer"></span>`fn as_underline_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md#displaybuffer)

#### Trait Implementations

##### `impl Any for Ansi256Color`

- <span id="ansi256color-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ansi256Color`

- <span id="ansi256color-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ansi256Color`

- <span id="ansi256color-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Ansi256Color`

- <span id="ansi256color-clone"></span>`fn clone(&self) -> Ansi256Color` — [`Ansi256Color`](#ansi256color)

##### `impl CloneToUninit for Ansi256Color`

- <span id="ansi256color-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Ansi256Color`

##### `impl Debug for Ansi256Color`

- <span id="ansi256color-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ansi256Color`

##### `impl<T> From for Ansi256Color`

- <span id="ansi256color-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Ansi256Color`

- <span id="ansi256color-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Ansi256Color`

- <span id="ansi256color-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Ansi256Color`

- <span id="ansi256color-ord-cmp"></span>`fn cmp(&self, other: &Ansi256Color) -> cmp::Ordering` — [`Ansi256Color`](#ansi256color)

##### `impl PartialEq for Ansi256Color`

- <span id="ansi256color-partialeq-eq"></span>`fn eq(&self, other: &Ansi256Color) -> bool` — [`Ansi256Color`](#ansi256color)

##### `impl PartialOrd for Ansi256Color`

- <span id="ansi256color-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Ansi256Color) -> option::Option<cmp::Ordering>` — [`Ansi256Color`](#ansi256color)

##### `impl StructuralPartialEq for Ansi256Color`

##### `impl ToOwned for Ansi256Color`

- <span id="ansi256color-toowned-type-owned"></span>`type Owned = T`

- <span id="ansi256color-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ansi256color-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Ansi256Color`

- <span id="ansi256color-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ansi256color-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ansi256Color`

- <span id="ansi256color-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ansi256color-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RgbColor`

```rust
struct RgbColor(u8, u8, u8);
```

*Defined in [`anstyle-1.0.13/src/color.rs:476`](../../.source_1765521767/anstyle-1.0.13/src/color.rs#L476)*

24-bit ANSI RGB color codes

#### Implementations

- <span id="rgbcolor-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

  Create a `Style` with this as the foreground

- <span id="rgbcolor-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](#style)

  Create a `Style` with this as the foreground

- <span id="rgbcolor-r"></span>`const fn r(self) -> u8`

  Red

- <span id="rgbcolor-g"></span>`const fn g(self) -> u8`

  Green

- <span id="rgbcolor-b"></span>`const fn b(self) -> u8`

  Blue

- <span id="rgbcolor-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code for a foreground color

- <span id="rgbcolor-as-fg-buffer"></span>`fn as_fg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md#displaybuffer)

- <span id="rgbcolor-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code for a background color

- <span id="rgbcolor-as-bg-buffer"></span>`fn as_bg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md#displaybuffer)

- <span id="rgbcolor-as-underline-buffer"></span>`fn as_underline_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md#displaybuffer)

#### Trait Implementations

##### `impl Any for RgbColor`

- <span id="rgbcolor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RgbColor`

- <span id="rgbcolor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RgbColor`

- <span id="rgbcolor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RgbColor`

- <span id="rgbcolor-clone"></span>`fn clone(&self) -> RgbColor` — [`RgbColor`](#rgbcolor)

##### `impl CloneToUninit for RgbColor`

- <span id="rgbcolor-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RgbColor`

##### `impl Debug for RgbColor`

- <span id="rgbcolor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RgbColor`

##### `impl<T> From for RgbColor`

- <span id="rgbcolor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RgbColor`

- <span id="rgbcolor-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RgbColor`

- <span id="rgbcolor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for RgbColor`

- <span id="rgbcolor-ord-cmp"></span>`fn cmp(&self, other: &RgbColor) -> cmp::Ordering` — [`RgbColor`](#rgbcolor)

##### `impl PartialEq for RgbColor`

- <span id="rgbcolor-partialeq-eq"></span>`fn eq(&self, other: &RgbColor) -> bool` — [`RgbColor`](#rgbcolor)

##### `impl PartialOrd for RgbColor`

- <span id="rgbcolor-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &RgbColor) -> option::Option<cmp::Ordering>` — [`RgbColor`](#rgbcolor)

##### `impl StructuralPartialEq for RgbColor`

##### `impl ToOwned for RgbColor`

- <span id="rgbcolor-toowned-type-owned"></span>`type Owned = T`

- <span id="rgbcolor-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rgbcolor-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RgbColor`

- <span id="rgbcolor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rgbcolor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RgbColor`

- <span id="rgbcolor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rgbcolor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DisplayBuffer`

```rust
struct DisplayBuffer {
    buffer: [u8; 19],
    len: usize,
}
```

*Defined in [`anstyle-1.0.13/src/color.rs:571-574`](../../.source_1765521767/anstyle-1.0.13/src/color.rs#L571-L574)*

#### Implementations

- <span id="displaybuffer-write-str"></span>`fn write_str(self, part: &'static str) -> Self`

- <span id="displaybuffer-write-code"></span>`fn write_code(self, code: u8) -> Self`

- <span id="displaybuffer-as-str"></span>`fn as_str(&self) -> &str`

- <span id="displaybuffer-write-to"></span>`fn write_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Any for DisplayBuffer`

- <span id="displaybuffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DisplayBuffer`

- <span id="displaybuffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DisplayBuffer`

- <span id="displaybuffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DisplayBuffer`

- <span id="displaybuffer-clone"></span>`fn clone(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md#displaybuffer)

##### `impl CloneToUninit for DisplayBuffer`

- <span id="displaybuffer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DisplayBuffer`

##### `impl Debug for DisplayBuffer`

- <span id="displaybuffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DisplayBuffer`

- <span id="displaybuffer-default"></span>`fn default() -> DisplayBuffer` — [`DisplayBuffer`](color/index.md#displaybuffer)

##### `impl Display for DisplayBuffer`

- <span id="displaybuffer-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for DisplayBuffer`

- <span id="displaybuffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DisplayBuffer`

- <span id="displaybuffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DisplayBuffer`

- <span id="displaybuffer-toowned-type-owned"></span>`type Owned = T`

- <span id="displaybuffer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="displaybuffer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for DisplayBuffer`

- <span id="displaybuffer-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for DisplayBuffer`

- <span id="displaybuffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="displaybuffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DisplayBuffer`

- <span id="displaybuffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="displaybuffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NullFormatter`

```rust
struct NullFormatter(&'static str);
```

*Defined in [`anstyle-1.0.13/src/color.rs:635`](../../.source_1765521767/anstyle-1.0.13/src/color.rs#L635)*

#### Trait Implementations

##### `impl Any for NullFormatter`

- <span id="nullformatter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NullFormatter`

- <span id="nullformatter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NullFormatter`

- <span id="nullformatter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NullFormatter`

- <span id="nullformatter-clone"></span>`fn clone(&self) -> NullFormatter` — [`NullFormatter`](color/index.md#nullformatter)

##### `impl CloneToUninit for NullFormatter`

- <span id="nullformatter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for NullFormatter`

##### `impl Debug for NullFormatter`

- <span id="nullformatter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NullFormatter`

- <span id="nullformatter-default"></span>`fn default() -> NullFormatter` — [`NullFormatter`](color/index.md#nullformatter)

##### `impl Display for NullFormatter`

- <span id="nullformatter-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for NullFormatter`

- <span id="nullformatter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NullFormatter`

- <span id="nullformatter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for NullFormatter`

- <span id="nullformatter-toowned-type-owned"></span>`type Owned = T`

- <span id="nullformatter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nullformatter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for NullFormatter`

- <span id="nullformatter-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for NullFormatter`

- <span id="nullformatter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nullformatter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NullFormatter`

- <span id="nullformatter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nullformatter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Effects`

```rust
struct Effects(u16);
```

*Defined in [`anstyle-1.0.13/src/effect.rs:9`](../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L9)*

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

- <span id="effects-contains"></span>`const fn contains(self, other: Effects) -> bool` — [`Effects`](#effects)

  Returns `true` if all of the effects in `other` are contained within `self`.

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;

  assert!(effects.contains(anstyle::Effects::BOLD));

  

  let effects = anstyle::Effects::new();

  assert!(!effects.contains(anstyle::Effects::BOLD));

  ```

- <span id="effects-insert"></span>`const fn insert(self, other: Effects) -> Self` — [`Effects`](#effects)

  Inserts the specified effects in-place.

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::new().insert(anstyle::Effects::new());

  assert!(effects.is_plain());

  

  let effects = anstyle::Effects::new().insert(anstyle::Effects::BOLD);

  assert!(effects.contains(anstyle::Effects::BOLD));

  ```

- <span id="effects-remove"></span>`const fn remove(self, other: Effects) -> Self` — [`Effects`](#effects)

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

- <span id="effects-iter"></span>`fn iter(self) -> EffectIter` — [`EffectIter`](#effectiter)

  Iterate over enabled effects

- <span id="effects-index-iter"></span>`fn index_iter(self) -> EffectIndexIter` — [`EffectIndexIter`](effect/index.md#effectindexiter)

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

- <span id="effects-clone"></span>`fn clone(&self) -> Effects` — [`Effects`](#effects)

##### `impl CloneToUninit for Effects`

- <span id="effects-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Effects`

##### `impl Debug for Effects`

- <span id="effects-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for Effects`

- <span id="effects-default"></span>`fn default() -> Effects` — [`Effects`](#effects)

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

- <span id="effects-ord-cmp"></span>`fn cmp(&self, other: &Effects) -> cmp::Ordering` — [`Effects`](#effects)

##### `impl PartialEq for Effects`

- <span id="effects-partialeq-eq"></span>`fn eq(&self, other: &Effects) -> bool` — [`Effects`](#effects)

##### `impl PartialOrd for Effects`

- <span id="effects-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Effects) -> option::Option<cmp::Ordering>` — [`Effects`](#effects)

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

*Defined in [`anstyle-1.0.13/src/effect.rs:263-266`](../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L263-L266)*

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

*Defined in [`anstyle-1.0.13/src/effect.rs:320`](../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L320)*

#### Trait Implementations

##### `impl Any for EffectsDisplay`

- <span id="effectsdisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EffectsDisplay`

- <span id="effectsdisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EffectsDisplay`

- <span id="effectsdisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for EffectsDisplay`

- <span id="effectsdisplay-clone"></span>`fn clone(&self) -> EffectsDisplay` — [`EffectsDisplay`](effect/index.md#effectsdisplay)

##### `impl CloneToUninit for EffectsDisplay`

- <span id="effectsdisplay-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for EffectsDisplay`

##### `impl Debug for EffectsDisplay`

- <span id="effectsdisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EffectsDisplay`

- <span id="effectsdisplay-default"></span>`fn default() -> EffectsDisplay` — [`EffectsDisplay`](effect/index.md#effectsdisplay)

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

*Defined in [`anstyle-1.0.13/src/effect.rs:334-337`](../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L334-L337)*

Enumerate each enabled value in [`Effects`](#effects)

#### Trait Implementations

##### `impl Any for EffectIter`

- <span id="effectiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EffectIter`

- <span id="effectiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EffectIter`

- <span id="effectiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for EffectIter`

- <span id="effectiter-clone"></span>`fn clone(&self) -> EffectIter` — [`EffectIter`](#effectiter)

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

- <span id="effectiter-partialeq-eq"></span>`fn eq(&self, other: &EffectIter) -> bool` — [`EffectIter`](#effectiter)

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

*Defined in [`anstyle-1.0.13/src/effect.rs:358-361`](../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L358-L361)*

#### Trait Implementations

##### `impl Any for EffectIndexIter`

- <span id="effectindexiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EffectIndexIter`

- <span id="effectindexiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EffectIndexIter`

- <span id="effectindexiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for EffectIndexIter`

- <span id="effectindexiter-clone"></span>`fn clone(&self) -> EffectIndexIter` — [`EffectIndexIter`](effect/index.md#effectindexiter)

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

- <span id="effectindexiter-partialeq-eq"></span>`fn eq(&self, other: &EffectIndexIter) -> bool` — [`EffectIndexIter`](effect/index.md#effectindexiter)

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

### `Reset`

```rust
struct Reset;
```

*Defined in [`anstyle-1.0.13/src/reset.rs:4`](../../.source_1765521767/anstyle-1.0.13/src/reset.rs#L4)*

Reset terminal formatting

#### Implementations

- <span id="reset-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code

  

  `Reset` also implements `Display` directly, so calling this method is optional.

#### Trait Implementations

##### `impl Any for Reset`

- <span id="reset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Reset`

- <span id="reset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Reset`

- <span id="reset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Reset`

- <span id="reset-clone"></span>`fn clone(&self) -> Reset` — [`Reset`](#reset)

##### `impl CloneToUninit for Reset`

- <span id="reset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Reset`

##### `impl Debug for Reset`

- <span id="reset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Reset`

- <span id="reset-default"></span>`fn default() -> Reset` — [`Reset`](#reset)

##### `impl Display for Reset`

- <span id="reset-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Reset`

##### `impl<T> From for Reset`

- <span id="reset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Reset`

- <span id="reset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Reset`

- <span id="reset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Reset`

- <span id="reset-ord-cmp"></span>`fn cmp(&self, other: &Reset) -> cmp::Ordering` — [`Reset`](#reset)

##### `impl PartialEq for Reset`

- <span id="reset-partialeq-eq"></span>`fn eq(&self, other: &Reset) -> bool` — [`Reset`](#reset)

##### `impl PartialOrd for Reset`

- <span id="reset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Reset) -> option::Option<cmp::Ordering>` — [`Reset`](#reset)

##### `impl StructuralPartialEq for Reset`

##### `impl ToOwned for Reset`

- <span id="reset-toowned-type-owned"></span>`type Owned = T`

- <span id="reset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="reset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Reset`

- <span id="reset-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Reset`

- <span id="reset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Reset`

- <span id="reset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Style`

```rust
struct Style {
    fg: Option<crate::Color>,
    bg: Option<crate::Color>,
    underline: Option<crate::Color>,
    effects: crate::Effects,
}
```

*Defined in [`anstyle-1.0.13/src/style.rs:18-23`](../../.source_1765521767/anstyle-1.0.13/src/style.rs#L18-L23)*

ANSI Text styling

You can print a `Style` to render the corresponding ANSI code.
Using the alternate flag `#` will render the ANSI reset code, if needed.
Together, this makes it convenient to render styles using inline format arguments.

# Examples

```rust
let style = anstyle::Style::new().bold();

let value = 42;
println!("{style}{value}{style:#}");
```

#### Implementations

- <span id="style-new"></span>`const fn new() -> Self`

  No effects enabled

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new();

  ```

- <span id="style-fg-color"></span>`const fn fg_color(self, fg: Option<crate::Color>) -> Self` — [`Color`](#color)

  Set foreground color

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into()));

  ```

- <span id="style-bg-color"></span>`const fn bg_color(self, bg: Option<crate::Color>) -> Self` — [`Color`](#color)

  Set background color

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().bg_color(Some(anstyle::AnsiColor::Red.into()));

  ```

- <span id="style-underline-color"></span>`const fn underline_color(self, underline: Option<crate::Color>) -> Self` — [`Color`](#color)

  Set underline color

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().underline_color(Some(anstyle::AnsiColor::Red.into()));

  ```

- <span id="style-effects"></span>`const fn effects(self, effects: crate::Effects) -> Self` — [`Effects`](#effects)

  Set text effects

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().effects(anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE);

  ```

- <span id="style-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code

  

  `Style` also implements `Display` directly, so calling this method is optional.

- <span id="style-fmt-to"></span>`fn fmt_to(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

- <span id="style-write-to"></span>`fn write_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

  Write the ANSI code

- <span id="style-render-reset"></span>`fn render_reset(self) -> impl core::fmt::Display + Copy`

  Renders the relevant `Reset` code

  

  Unlike `Reset::render`, this will elide the code if there is nothing to reset.

- <span id="style-write-reset-to"></span>`fn write_reset_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

  Write the relevant `Reset` code

  

  Unlike `Reset::render`, this will elide the code if there is nothing to reset.

#### Trait Implementations

##### `impl Any for Style`

- <span id="style-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl BitOr for Style`

- <span id="style-bitor-type-output"></span>`type Output = Style`

- <span id="style-bitor"></span>`fn bitor(self, rhs: crate::Effects) -> Self` — [`Effects`](#effects)

##### `impl BitOrAssign for Style`

- <span id="style-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: crate::Effects)` — [`Effects`](#effects)

##### `impl<T> Borrow for Style`

- <span id="style-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Style`

- <span id="style-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](#style)

##### `impl CloneToUninit for Style`

- <span id="style-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Style`

##### `impl Debug for Style`

- <span id="style-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Style` — [`Style`](#style)

##### `impl Display for Style`

- <span id="style-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Style`

##### `impl<T> From for Style`

- <span id="style-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Style`

- <span id="style-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Style`

- <span id="style-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Style`

- <span id="style-ord-cmp"></span>`fn cmp(&self, other: &Style) -> cmp::Ordering` — [`Style`](#style)

##### `impl PartialEq for Style`

- <span id="style-partialeq-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](#style)

##### `impl PartialOrd for Style`

- <span id="style-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Style) -> option::Option<cmp::Ordering>` — [`Style`](#style)

##### `impl StructuralPartialEq for Style`

##### `impl Sub for Style`

- <span id="style-sub-type-output"></span>`type Output = Style`

- <span id="style-sub"></span>`fn sub(self, other: crate::Effects) -> Self` — [`Effects`](#effects)

##### `impl SubAssign for Style`

- <span id="style-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: crate::Effects)` — [`Effects`](#effects)

##### `impl ToOwned for Style`

- <span id="style-toowned-type-owned"></span>`type Owned = T`

- <span id="style-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="style-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Style`

- <span id="style-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Style`

- <span id="style-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="style-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Style`

- <span id="style-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="style-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StyleDisplay`

```rust
struct StyleDisplay(Style);
```

*Defined in [`anstyle-1.0.13/src/style.rs:423`](../../.source_1765521767/anstyle-1.0.13/src/style.rs#L423)*

#### Trait Implementations

##### `impl Any for StyleDisplay`

- <span id="styledisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StyleDisplay`

- <span id="styledisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StyleDisplay`

- <span id="styledisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StyleDisplay`

- <span id="styledisplay-clone"></span>`fn clone(&self) -> StyleDisplay` — [`StyleDisplay`](style/index.md#styledisplay)

##### `impl CloneToUninit for StyleDisplay`

- <span id="styledisplay-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StyleDisplay`

##### `impl Debug for StyleDisplay`

- <span id="styledisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyleDisplay`

- <span id="styledisplay-default"></span>`fn default() -> StyleDisplay` — [`StyleDisplay`](style/index.md#styledisplay)

##### `impl Display for StyleDisplay`

- <span id="styledisplay-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for StyleDisplay`

- <span id="styledisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StyleDisplay`

- <span id="styledisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StyleDisplay`

- <span id="styledisplay-toowned-type-owned"></span>`type Owned = T`

- <span id="styledisplay-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="styledisplay-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for StyleDisplay`

- <span id="styledisplay-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for StyleDisplay`

- <span id="styledisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="styledisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StyleDisplay`

- <span id="styledisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="styledisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Color`

```rust
enum Color {
    Ansi(AnsiColor),
    Ansi256(Ansi256Color),
    Rgb(RgbColor),
}
```

*Defined in [`anstyle-1.0.13/src/color.rs:4-17`](../../.source_1765521767/anstyle-1.0.13/src/color.rs#L4-L17)*

Any ANSI color code scheme

#### Variants

- **`Ansi`**

  Available 4-bit ANSI color palette codes
  
  The user's terminal defines the meaning of the each palette code.

- **`Ansi256`**

  256 (8-bit) color support
  
  - `0..16` are [`AnsiColor`](#ansicolor) palette codes
  - `0..232` map to [`RgbColor`](#rgbcolor) color values
  - `232..` map to [`RgbColor`](#rgbcolor) gray-scale values

- **`Rgb`**

  24-bit ANSI RGB color codes

#### Implementations

- <span id="color-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

  Create a `Style` with this as the foreground

- <span id="color-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](#style)

  Create a `Style` with this as the foreground

- <span id="color-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code for a foreground color

- <span id="color-write-fg-to"></span>`fn write_fg_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

- <span id="color-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code for a background color

- <span id="color-write-bg-to"></span>`fn write_bg_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

- <span id="color-render-underline"></span>`fn render_underline(self) -> impl core::fmt::Display + Copy`

- <span id="color-write-underline-to"></span>`fn write_underline_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Any for Color`

- <span id="color-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Color`

- <span id="color-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Color`

- <span id="color-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Color`

- <span id="color-clone"></span>`fn clone(&self) -> Color` — [`Color`](#color)

##### `impl CloneToUninit for Color`

- <span id="color-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Color`

##### `impl Debug for Color`

- <span id="color-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Color`

##### `impl<T> From for Color`

- <span id="color-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Color`

- <span id="color-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Color`

- <span id="color-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Color`

- <span id="color-ord-cmp"></span>`fn cmp(&self, other: &Color) -> cmp::Ordering` — [`Color`](#color)

##### `impl PartialEq for Color`

- <span id="color-partialeq-eq"></span>`fn eq(&self, other: &Color) -> bool` — [`Color`](#color)

##### `impl PartialOrd for Color`

- <span id="color-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Color) -> option::Option<cmp::Ordering>` — [`Color`](#color)

##### `impl StructuralPartialEq for Color`

##### `impl ToOwned for Color`

- <span id="color-toowned-type-owned"></span>`type Owned = T`

- <span id="color-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="color-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Color`

- <span id="color-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="color-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Color`

- <span id="color-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="color-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AnsiColor`

```rust
enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
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

*Defined in [`anstyle-1.0.13/src/color.rs:138-186`](../../.source_1765521767/anstyle-1.0.13/src/color.rs#L138-L186)*

Available 4-bit ANSI color palette codes

The user's terminal defines the meaning of the each palette code.

#### Variants

- **`Black`**

  Black: #0 (foreground code `30`, background code `40`).

- **`Red`**

  Red: #1 (foreground code `31`, background code `41`).

- **`Green`**

  Green: #2 (foreground code `32`, background code `42`).

- **`Yellow`**

  Yellow: #3 (foreground code `33`, background code `43`).

- **`Blue`**

  Blue: #4 (foreground code `34`, background code `44`).

- **`Magenta`**

  Magenta: #5 (foreground code `35`, background code `45`).

- **`Cyan`**

  Cyan: #6 (foreground code `36`, background code `46`).

- **`White`**

  White: #7 (foreground code `37`, background code `47`).

- **`BrightBlack`**

  Bright black: #0 (foreground code `90`, background code `100`).

- **`BrightRed`**

  Bright red: #1 (foreground code `91`, background code `101`).

- **`BrightGreen`**

  Bright green: #2 (foreground code `92`, background code `102`).

- **`BrightYellow`**

  Bright yellow: #3 (foreground code `93`, background code `103`).

- **`BrightBlue`**

  Bright blue: #4 (foreground code `94`, background code `104`).

- **`BrightMagenta`**

  Bright magenta: #5 (foreground code `95`, background code `105`).

- **`BrightCyan`**

  Bright cyan: #6 (foreground code `96`, background code `106`).

- **`BrightWhite`**

  Bright white: #7 (foreground code `97`, background code `107`).

#### Implementations

- <span id="ansicolor-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

  Create a `Style` with this as the foreground

- <span id="ansicolor-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](#style)

  Create a `Style` with this as the foreground

- <span id="ansicolor-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code for a foreground color

- <span id="ansicolor-as-fg-str"></span>`fn as_fg_str(&self) -> &'static str`

- <span id="ansicolor-as-fg-buffer"></span>`fn as_fg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md#displaybuffer)

- <span id="ansicolor-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code for a background color

- <span id="ansicolor-as-bg-str"></span>`fn as_bg_str(&self) -> &'static str`

- <span id="ansicolor-as-bg-buffer"></span>`fn as_bg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md#displaybuffer)

- <span id="ansicolor-as-underline-buffer"></span>`fn as_underline_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md#displaybuffer)

- <span id="ansicolor-bright"></span>`fn bright(self, yes: bool) -> Self`

  Change the color to/from bright

- <span id="ansicolor-is-bright"></span>`fn is_bright(self) -> bool`

  Report whether the color is bright

#### Trait Implementations

##### `impl Any for AnsiColor`

- <span id="ansicolor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AnsiColor`

- <span id="ansicolor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AnsiColor`

- <span id="ansicolor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AnsiColor`

- <span id="ansicolor-clone"></span>`fn clone(&self) -> AnsiColor` — [`AnsiColor`](#ansicolor)

##### `impl CloneToUninit for AnsiColor`

- <span id="ansicolor-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AnsiColor`

##### `impl Debug for AnsiColor`

- <span id="ansicolor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AnsiColor`

##### `impl<T> From for AnsiColor`

- <span id="ansicolor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for AnsiColor`

- <span id="ansicolor-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for AnsiColor`

- <span id="ansicolor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for AnsiColor`

- <span id="ansicolor-ord-cmp"></span>`fn cmp(&self, other: &AnsiColor) -> cmp::Ordering` — [`AnsiColor`](#ansicolor)

##### `impl PartialEq for AnsiColor`

- <span id="ansicolor-partialeq-eq"></span>`fn eq(&self, other: &AnsiColor) -> bool` — [`AnsiColor`](#ansicolor)

##### `impl PartialOrd for AnsiColor`

- <span id="ansicolor-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &AnsiColor) -> option::Option<cmp::Ordering>` — [`AnsiColor`](#ansicolor)

##### `impl StructuralPartialEq for AnsiColor`

##### `impl ToOwned for AnsiColor`

- <span id="ansicolor-toowned-type-owned"></span>`type Owned = T`

- <span id="ansicolor-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ansicolor-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AnsiColor`

- <span id="ansicolor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ansicolor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AnsiColor`

- <span id="ansicolor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ansicolor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `DISPLAY_BUFFER_CAPACITY`
```rust
const DISPLAY_BUFFER_CAPACITY: usize = 19usize;
```

*Defined in [`anstyle-1.0.13/src/color.rs:568`](../../.source_1765521767/anstyle-1.0.13/src/color.rs#L568)*

### `METADATA`
```rust
const METADATA: [Metadata; 12];
```

*Defined in [`anstyle-1.0.13/src/effect.rs:268-317`](../../.source_1765521767/anstyle-1.0.13/src/effect.rs#L268-L317)*

### `RESET`
```rust
const RESET: &str;
```

*Defined in [`anstyle-1.0.13/src/reset.rs:22`](../../.source_1765521767/anstyle-1.0.13/src/reset.rs#L22)*

