*[miette](../../index.md) / [handlers](../index.md) / [theme](index.md)*

---

# Module `theme`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GraphicalTheme`](#graphicaltheme) | struct | Theme used by [`GraphicalReportHandler`](crate::GraphicalReportHandler) to render fancy [`Diagnostic`](crate::Diagnostic) reports. |
| [`ThemeStyles`](#themestyles) | struct | Styles for various parts of graphical rendering for the [`GraphicalReportHandler`](crate::GraphicalReportHandler). |
| [`ThemeCharacters`](#themecharacters) | struct | Characters to be used when drawing when using [`GraphicalReportHandler`](crate::GraphicalReportHandler). |
| [`style`](#style) | fn |  |

## Structs

### `GraphicalTheme`

```rust
struct GraphicalTheme {
    pub characters: ThemeCharacters,
    pub styles: ThemeStyles,
}
```

*Defined in [`miette-7.6.0/src/handlers/theme.rs:17-22`](../../../../.source_1765633015/miette-7.6.0/src/handlers/theme.rs#L17-L22)*

Theme used by [`GraphicalReportHandler`](crate::GraphicalReportHandler) to
render fancy [`Diagnostic`](crate::Diagnostic) reports.

A theme consists of two things: the set of characters to be used for drawing,
and the
[`owo_colors::Style`](https://docs.rs/owo-colors/latest/owo_colors/struct.Style.html)s to be used to paint various items.

You can create your own custom graphical theme using this type, or you can use
one of the predefined ones using the methods below.

#### Fields

- **`characters`**: `ThemeCharacters`

  Characters to be used for drawing.

- **`styles`**: `ThemeStyles`

  Styles to be used for painting.

#### Implementations

- <span id="graphicaltheme-ascii"></span>`fn ascii() -> Self`

  ASCII-art-based graphical drawing, with ANSI styling.

- <span id="graphicaltheme-unicode"></span>`fn unicode() -> Self`

  Graphical theme that draws using both ansi colors and unicode

  characters.

  

  Note that full rgb colors aren't enabled by default because they're

  an accessibility hazard, especially in the context of terminal themes

  that can change the background color and make hardcoded colors illegible.

  Such themes typically remap ansi codes properly, treating them more

  like CSS classes than specific colors.

- <span id="graphicaltheme-unicode-nocolor"></span>`fn unicode_nocolor() -> Self`

  Graphical theme that draws in monochrome, while still using unicode

  characters.

- <span id="graphicaltheme-none"></span>`fn none() -> Self`

  A "basic" graphical theme that skips colors and unicode characters and

  just does monochrome ascii art. If you want a completely non-graphical

  rendering of your [`Diagnostic`](crate::Diagnostic)s, check out

  [`NarratableReportHandler`](crate::NarratableReportHandler), or write

  your own [`ReportHandler`](crate::ReportHandler)

#### Trait Implementations

##### `impl Any for GraphicalTheme`

- <span id="graphicaltheme-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GraphicalTheme`

- <span id="graphicaltheme-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GraphicalTheme`

- <span id="graphicaltheme-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for GraphicalTheme`

- <span id="graphicaltheme-clone"></span>`fn clone(&self) -> GraphicalTheme` — [`GraphicalTheme`](../index.md#graphicaltheme)

##### `impl CloneToUninit for GraphicalTheme`

- <span id="graphicaltheme-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for GraphicalTheme`

- <span id="graphicaltheme-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GraphicalTheme`

- <span id="graphicaltheme-default"></span>`fn default() -> Self`

##### `impl<T> From for GraphicalTheme`

- <span id="graphicaltheme-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GraphicalTheme`

- <span id="graphicaltheme-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for GraphicalTheme`

##### `impl ToOwned for GraphicalTheme`

- <span id="graphicaltheme-toowned-type-owned"></span>`type Owned = T`

- <span id="graphicaltheme-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="graphicaltheme-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GraphicalTheme`

- <span id="graphicaltheme-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="graphicaltheme-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GraphicalTheme`

- <span id="graphicaltheme-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="graphicaltheme-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ThemeStyles`

```rust
struct ThemeStyles {
    pub error: owo_colors::Style,
    pub warning: owo_colors::Style,
    pub advice: owo_colors::Style,
    pub help: owo_colors::Style,
    pub link: owo_colors::Style,
    pub linum: owo_colors::Style,
    pub highlights: Vec<owo_colors::Style>,
}
```

*Defined in [`miette-7.6.0/src/handlers/theme.rs:87-103`](../../../../.source_1765633015/miette-7.6.0/src/handlers/theme.rs#L87-L103)*

Styles for various parts of graphical rendering for the
[`GraphicalReportHandler`](crate::GraphicalReportHandler).

#### Fields

- **`error`**: `owo_colors::Style`

  Style to apply to things highlighted as "error".

- **`warning`**: `owo_colors::Style`

  Style to apply to things highlighted as "warning".

- **`advice`**: `owo_colors::Style`

  Style to apply to things highlighted as "advice".

- **`help`**: `owo_colors::Style`

  Style to apply to the help text.

- **`link`**: `owo_colors::Style`

  Style to apply to filenames/links/URLs.

- **`linum`**: `owo_colors::Style`

  Style to apply to line numbers.

- **`highlights`**: `Vec<owo_colors::Style>`

  Styles to cycle through (using `.iter().cycle()`), to render the lines
  and text for diagnostic highlights.

#### Implementations

- <span id="themestyles-rgb"></span>`fn rgb() -> Self`

  Nice RGB colors.

  [Credit](http://terminal.sexy/#FRUV0NDQFRUVrEFCkKlZ9L91ap-1qnWfdbWq0NDQUFBQrEFCkKlZ9L91ap-1qnWfdbWq9fX1).

- <span id="themestyles-ansi"></span>`fn ansi() -> Self`

  ANSI color-based styles.

- <span id="themestyles-none"></span>`fn none() -> Self`

  No styling. Just regular ol' monochrome.

#### Trait Implementations

##### `impl Any for ThemeStyles`

- <span id="themestyles-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThemeStyles`

- <span id="themestyles-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThemeStyles`

- <span id="themestyles-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ThemeStyles`

- <span id="themestyles-clone"></span>`fn clone(&self) -> ThemeStyles` — [`ThemeStyles`](../index.md#themestyles)

##### `impl CloneToUninit for ThemeStyles`

- <span id="themestyles-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ThemeStyles`

- <span id="themestyles-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ThemeStyles`

- <span id="themestyles-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThemeStyles`

- <span id="themestyles-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ThemeStyles`

##### `impl ToOwned for ThemeStyles`

- <span id="themestyles-toowned-type-owned"></span>`type Owned = T`

- <span id="themestyles-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="themestyles-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ThemeStyles`

- <span id="themestyles-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="themestyles-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThemeStyles`

- <span id="themestyles-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="themestyles-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ThemeCharacters`

```rust
struct ThemeCharacters {
    pub hbar: char,
    pub vbar: char,
    pub xbar: char,
    pub vbar_break: char,
    pub uarrow: char,
    pub rarrow: char,
    pub ltop: char,
    pub mtop: char,
    pub rtop: char,
    pub lbot: char,
    pub rbot: char,
    pub mbot: char,
    pub lbox: char,
    pub rbox: char,
    pub lcross: char,
    pub rcross: char,
    pub underbar: char,
    pub underline: char,
    pub error: String,
    pub warning: String,
    pub advice: String,
}
```

*Defined in [`miette-7.6.0/src/handlers/theme.rs:167-195`](../../../../.source_1765633015/miette-7.6.0/src/handlers/theme.rs#L167-L195)*

Characters to be used when drawing when using
[`GraphicalReportHandler`](crate::GraphicalReportHandler).

#### Implementations

- <span id="themecharacters-unicode"></span>`fn unicode() -> Self`

  Fancy unicode-based graphical elements.

- <span id="themecharacters-emoji"></span>`fn emoji() -> Self`

  Emoji-heavy unicode characters.

- <span id="themecharacters-ascii"></span>`fn ascii() -> Self`

  ASCII-art-based graphical elements. Works well on older terminals.

#### Trait Implementations

##### `impl Any for ThemeCharacters`

- <span id="themecharacters-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThemeCharacters`

- <span id="themecharacters-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThemeCharacters`

- <span id="themecharacters-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ThemeCharacters`

- <span id="themecharacters-clone"></span>`fn clone(&self) -> ThemeCharacters` — [`ThemeCharacters`](../index.md#themecharacters)

##### `impl CloneToUninit for ThemeCharacters`

- <span id="themecharacters-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ThemeCharacters`

- <span id="themecharacters-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ThemeCharacters`

##### `impl<T> From for ThemeCharacters`

- <span id="themecharacters-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThemeCharacters`

- <span id="themecharacters-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ThemeCharacters`

##### `impl PartialEq for ThemeCharacters`

- <span id="themecharacters-partialeq-eq"></span>`fn eq(&self, other: &ThemeCharacters) -> bool` — [`ThemeCharacters`](../index.md#themecharacters)

##### `impl StructuralPartialEq for ThemeCharacters`

##### `impl ToOwned for ThemeCharacters`

- <span id="themecharacters-toowned-type-owned"></span>`type Owned = T`

- <span id="themecharacters-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="themecharacters-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ThemeCharacters`

- <span id="themecharacters-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="themecharacters-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThemeCharacters`

- <span id="themecharacters-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="themecharacters-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `style`

```rust
fn style() -> owo_colors::Style
```

*Defined in [`miette-7.6.0/src/handlers/theme.rs:105-107`](../../../../.source_1765633015/miette-7.6.0/src/handlers/theme.rs#L105-L107)*

