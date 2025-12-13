*[indicatif](../index.md) / [style](index.md)*

---

# Module `style`

## Contents

- [Structs](#structs)
  - [`ProgressStyle`](#progressstyle)
  - [`TabRewriter`](#tabrewriter)
  - [`Template`](#template)
  - [`TemplateError`](#templateerror)
  - [`BarDisplay`](#bardisplay)
  - [`RepeatedStringDisplay`](#repeatedstringdisplay)
  - [`PaddedStringDisplay`](#paddedstringdisplay)
- [Enums](#enums)
  - [`WideElement`](#wideelement)
  - [`TemplatePart`](#templatepart)
  - [`State`](#state)
  - [`Alignment`](#alignment)
- [Traits](#traits)
  - [`ProgressTracker`](#progresstracker)
- [Functions](#functions)
  - [`segment`](#segment)
  - [`measure`](#measure)
  - [`width`](#width)
  - [`write_ansi_range`](#write-ansi-range)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ProgressStyle`](#progressstyle) | struct |  |
| [`TabRewriter`](#tabrewriter) | struct |  |
| [`Template`](#template) | struct |  |
| [`TemplateError`](#templateerror) | struct |  |
| [`BarDisplay`](#bardisplay) | struct |  |
| [`RepeatedStringDisplay`](#repeatedstringdisplay) | struct |  |
| [`PaddedStringDisplay`](#paddedstringdisplay) | struct |  |
| [`WideElement`](#wideelement) | enum |  |
| [`TemplatePart`](#templatepart) | enum |  |
| [`State`](#state) | enum |  |
| [`Alignment`](#alignment) | enum |  |
| [`ProgressTracker`](#progresstracker) | trait | Trait for defining stateful or stateless formatters |
| [`segment`](#segment) | fn |  |
| [`measure`](#measure) | fn |  |
| [`width`](#width) | fn | finds the unicode-aware width of the passed grapheme cluters panics on an empty parameter, or if the characters are not equal-width |
| [`write_ansi_range`](#write-ansi-range) | fn | Write the visible text between start and end. |

## Structs

### `ProgressStyle`

```rust
struct ProgressStyle {
    tick_strings: Vec<Box<str>>,
    progress_chars: Vec<Box<str>>,
    template: Template,
    char_width: usize,
    tab_width: usize,
    format_map: std::collections::HashMap<&'static str, Box<dyn ProgressTracker>>,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:23-31`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L23-L31)*

#### Implementations

- <span id="progressstyle-default-bar"></span>`fn default_bar() -> Self`

  Returns the default progress bar style for bars

- <span id="progressstyle-default-spinner"></span>`fn default_spinner() -> Self`

  Returns the default progress bar style for spinners

- <span id="progressstyle-with-template"></span>`fn with_template(template: &str) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

  Sets the template string for the progress bar

  

  Review the [list of template keys](../index.html#templates) for more information.

- <span id="progressstyle-set-tab-width"></span>`fn set_tab_width(&mut self, new_tab_width: usize)`

- <span id="progressstyle-set-for-stderr"></span>`fn set_for_stderr(&mut self)`

  Specifies that the progress bar is intended to be printed to stderr

  

  The progress bar will determine whether to enable/disable colors based on stderr

  instead of stdout. Under the hood, this uses [`console::colors_enabled_stderr`](../../console/utils/index.md).

- <span id="progressstyle-new"></span>`fn new(template: Template) -> Self` — [`Template`](#template)

- <span id="progressstyle-tick-chars"></span>`fn tick_chars(self, s: &str) -> Self`

  Sets the tick character sequence for spinners

  

  Note that the last character is used as the [final tick string][Self::get_final_tick_str()].

  At least two characters are required to provide a non-final and final state.

- <span id="progressstyle-tick-strings"></span>`fn tick_strings(self, s: &[&str]) -> Self`

  Sets the tick string sequence for spinners

  

  Note that the last string is used as the [final tick string][Self::get_final_tick_str()].

  At least two strings are required to provide a non-final and final state.

- <span id="progressstyle-progress-chars"></span>`fn progress_chars(self, s: &str) -> Self`

  Sets the progress characters `(filled, current, to do)`

  

  You can pass more than three for a more detailed display.

  All passed grapheme clusters need to be of equal width.

- <span id="progressstyle-with-key"></span>`fn with_key<S: ProgressTracker + 'static>(self, key: &'static str, f: S) -> Self`

  Adds a custom key that owns a [`ProgressTracker`](#progresstracker) to the template

- <span id="progressstyle-template"></span>`fn template(self, s: &str) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

  Sets the template string for the progress bar

  

  Review the [list of template keys](../index.html#templates) for more information.

- <span id="progressstyle-current-tick-str"></span>`fn current_tick_str(&self, state: &ProgressState) -> &str` — [`ProgressState`](../state/index.md#progressstate)

- <span id="progressstyle-get-tick-str"></span>`fn get_tick_str(&self, idx: u64) -> &str`

  Returns the tick string for a given number

- <span id="progressstyle-get-final-tick-str"></span>`fn get_final_tick_str(&self) -> &str`

  Returns the tick string for the finished state

- <span id="progressstyle-format-bar"></span>`fn format_bar(&self, fract: f32, width: usize, alt_style: Option<&Style>) -> BarDisplay<'_>` — [`BarDisplay`](#bardisplay)

- <span id="progressstyle-format-state"></span>`fn format_state(&self, state: &ProgressState, lines: &mut Vec<LineType>, target_width: u16)` — [`ProgressState`](../state/index.md#progressstate), [`LineType`](../draw_target/index.md#linetype)

- <span id="progressstyle-push-line"></span>`fn push_line(&self, lines: &mut Vec<LineType>, cur: &mut String, state: &ProgressState, buf: &mut String, target_width: u16, wide: &Option<WideElement<'_>>)` — [`LineType`](../draw_target/index.md#linetype), [`ProgressState`](../state/index.md#progressstate), [`WideElement`](#wideelement)

  This is used exclusively to add the bars built above to the lines to print

#### Trait Implementations

##### `impl Any for ProgressStyle`

- <span id="progressstyle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProgressStyle`

- <span id="progressstyle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProgressStyle`

- <span id="progressstyle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ProgressStyle`

- <span id="progressstyle-clone"></span>`fn clone(&self) -> ProgressStyle` — [`ProgressStyle`](#progressstyle)

##### `impl CloneToUninit for ProgressStyle`

- <span id="progressstyle-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for ProgressStyle`

- <span id="progressstyle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProgressStyle`

- <span id="progressstyle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ProgressStyle`

- <span id="progressstyle-toowned-type-owned"></span>`type Owned = T`

- <span id="progressstyle-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="progressstyle-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ProgressStyle`

- <span id="progressstyle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="progressstyle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProgressStyle`

- <span id="progressstyle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="progressstyle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TabRewriter<'a>`

```rust
struct TabRewriter<'a>(&'a mut dyn fmt::Write, usize);
```

*Defined in [`indicatif-0.18.3/src/style.rs:444`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L444)*

#### Trait Implementations

##### `impl Any for TabRewriter<'a>`

- <span id="tabrewriter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TabRewriter<'a>`

- <span id="tabrewriter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TabRewriter<'a>`

- <span id="tabrewriter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for TabRewriter<'a>`

- <span id="tabrewriter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TabRewriter<'a>`

- <span id="tabrewriter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for TabRewriter<'a>`

- <span id="tabrewriter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tabrewriter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TabRewriter<'a>`

- <span id="tabrewriter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tabrewriter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write for TabRewriter<'_>`

- <span id="tabrewriter-write-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

### `Template`

```rust
struct Template {
    parts: Vec<TemplatePart>,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:506-508`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L506-L508)*

#### Implementations

- <span id="template-from-str-with-tab-width"></span>`fn from_str_with_tab_width(s: &str, tab_width: usize) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

- <span id="template-from-str"></span>`fn from_str(s: &str) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

- <span id="template-set-tab-width"></span>`fn set_tab_width(&mut self, new_tab_width: usize)`

#### Trait Implementations

##### `impl Any for Template`

- <span id="template-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Template`

- <span id="template-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Template`

- <span id="template-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Template`

- <span id="template-clone"></span>`fn clone(&self) -> Template` — [`Template`](#template)

##### `impl CloneToUninit for Template`

- <span id="template-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Template`

- <span id="template-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Template`

- <span id="template-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Template`

- <span id="template-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Template`

- <span id="template-toowned-type-owned"></span>`type Owned = T`

- <span id="template-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="template-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Template`

- <span id="template-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="template-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Template`

- <span id="template-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="template-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TemplateError`

```rust
struct TemplateError {
    state: State,
    next: char,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:656-659`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L656-L659)*

#### Trait Implementations

##### `impl Any for TemplateError`

- <span id="templateerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TemplateError`

- <span id="templateerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TemplateError`

- <span id="templateerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for TemplateError`

- <span id="templateerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TemplateError`

- <span id="templateerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for TemplateError`

##### `impl<T> From for TemplateError`

- <span id="templateerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TemplateError`

- <span id="templateerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for TemplateError`

- <span id="templateerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for TemplateError`

- <span id="templateerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="templateerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TemplateError`

- <span id="templateerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="templateerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BarDisplay<'a>`

```rust
struct BarDisplay<'a> {
    chars: &'a [Box<str>],
    filled: usize,
    cur: Option<usize>,
    rest: console::StyledObject<RepeatedStringDisplay<'a>>,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:699-704`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L699-L704)*

#### Trait Implementations

##### `impl Any for BarDisplay<'a>`

- <span id="bardisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BarDisplay<'a>`

- <span id="bardisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BarDisplay<'a>`

- <span id="bardisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for BarDisplay<'_>`

- <span id="bardisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BarDisplay<'a>`

- <span id="bardisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BarDisplay<'a>`

- <span id="bardisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for BarDisplay<'a>`

- <span id="bardisplay-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for BarDisplay<'a>`

- <span id="bardisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bardisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BarDisplay<'a>`

- <span id="bardisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bardisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RepeatedStringDisplay<'a>`

```rust
struct RepeatedStringDisplay<'a> {
    str: &'a str,
    num: usize,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:718-721`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L718-L721)*

#### Trait Implementations

##### `impl Any for RepeatedStringDisplay<'a>`

- <span id="repeatedstringdisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RepeatedStringDisplay<'a>`

- <span id="repeatedstringdisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RepeatedStringDisplay<'a>`

- <span id="repeatedstringdisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for RepeatedStringDisplay<'_>`

- <span id="repeatedstringdisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RepeatedStringDisplay<'a>`

- <span id="repeatedstringdisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RepeatedStringDisplay<'a>`

- <span id="repeatedstringdisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for RepeatedStringDisplay<'a>`

- <span id="repeatedstringdisplay-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for RepeatedStringDisplay<'a>`

- <span id="repeatedstringdisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repeatedstringdisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RepeatedStringDisplay<'a>`

- <span id="repeatedstringdisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repeatedstringdisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PaddedStringDisplay<'a>`

```rust
struct PaddedStringDisplay<'a> {
    str: &'a str,
    width: usize,
    align: Alignment,
    truncate: bool,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:732-737`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L732-L737)*

#### Trait Implementations

##### `impl Any for PaddedStringDisplay<'a>`

- <span id="paddedstringdisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PaddedStringDisplay<'a>`

- <span id="paddedstringdisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PaddedStringDisplay<'a>`

- <span id="paddedstringdisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for PaddedStringDisplay<'_>`

- <span id="paddedstringdisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PaddedStringDisplay<'a>`

- <span id="paddedstringdisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PaddedStringDisplay<'a>`

- <span id="paddedstringdisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for PaddedStringDisplay<'a>`

- <span id="paddedstringdisplay-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for PaddedStringDisplay<'a>`

- <span id="paddedstringdisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="paddedstringdisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PaddedStringDisplay<'a>`

- <span id="paddedstringdisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="paddedstringdisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `WideElement<'a>`

```rust
enum WideElement<'a> {
    Bar {
        alt_style: &'a Option<console::Style>,
    },
    Message {
        align: &'a Alignment,
    },
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:454-457`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L454-L457)*

#### Implementations

- <span id="wideelement-expand"></span>`fn expand(self, cur: String, style: &ProgressStyle, state: &ProgressState, buf: &mut String, width: u16) -> String` — [`ProgressStyle`](#progressstyle), [`ProgressState`](../state/index.md#progressstate)

#### Trait Implementations

##### `impl Any for WideElement<'a>`

- <span id="wideelement-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WideElement<'a>`

- <span id="wideelement-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WideElement<'a>`

- <span id="wideelement-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WideElement<'a>`

- <span id="wideelement-clone"></span>`fn clone(&self) -> WideElement<'a>` — [`WideElement`](#wideelement)

##### `impl CloneToUninit for WideElement<'a>`

- <span id="wideelement-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for WideElement<'a>`

##### `impl<T> From for WideElement<'a>`

- <span id="wideelement-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WideElement<'a>`

- <span id="wideelement-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for WideElement<'a>`

- <span id="wideelement-toowned-type-owned"></span>`type Owned = T`

- <span id="wideelement-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="wideelement-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WideElement<'a>`

- <span id="wideelement-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="wideelement-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WideElement<'a>`

- <span id="wideelement-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="wideelement-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TemplatePart`

```rust
enum TemplatePart {
    Literal(crate::state::TabExpandedString),
    Placeholder {
        key: String,
        align: Alignment,
        width: Option<u16>,
        truncate: bool,
        style: Option<console::Style>,
        alt_style: Option<console::Style>,
    },
    NewLine,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:674-685`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L674-L685)*

#### Trait Implementations

##### `impl Any for TemplatePart`

- <span id="templatepart-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TemplatePart`

- <span id="templatepart-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TemplatePart`

- <span id="templatepart-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TemplatePart`

- <span id="templatepart-clone"></span>`fn clone(&self) -> TemplatePart` — [`TemplatePart`](#templatepart)

##### `impl CloneToUninit for TemplatePart`

- <span id="templatepart-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TemplatePart`

- <span id="templatepart-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TemplatePart`

##### `impl<T> From for TemplatePart`

- <span id="templatepart-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TemplatePart`

- <span id="templatepart-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TemplatePart`

- <span id="templatepart-partialeq-eq"></span>`fn eq(&self, other: &TemplatePart) -> bool` — [`TemplatePart`](#templatepart)

##### `impl StructuralPartialEq for TemplatePart`

##### `impl ToOwned for TemplatePart`

- <span id="templatepart-toowned-type-owned"></span>`type Owned = T`

- <span id="templatepart-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="templatepart-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TemplatePart`

- <span id="templatepart-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="templatepart-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TemplatePart`

- <span id="templatepart-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="templatepart-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `State`

```rust
enum State {
    Literal,
    MaybeOpen,
    DoubleClose,
    Key,
    Align,
    Width,
    FirstStyle,
    AltStyle,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:688-697`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L688-L697)*

#### Trait Implementations

##### `impl Any for State`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl CloneToUninit for State`

- <span id="state-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for State`

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for State`

##### `impl<T> From for State`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for State`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for State`

- <span id="state-partialeq-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`

##### `impl ToOwned for State`

- <span id="state-toowned-type-owned"></span>`type Owned = T`

- <span id="state-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="state-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for State`

- <span id="state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for State`

- <span id="state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Alignment`

```rust
enum Alignment {
    Left,
    Center,
    Right,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:808-812`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L808-L812)*

#### Trait Implementations

##### `impl Any for Alignment`

- <span id="alignment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Alignment`

- <span id="alignment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Alignment`

- <span id="alignment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Alignment`

- <span id="alignment-clone"></span>`fn clone(&self) -> Alignment` — [`Alignment`](#alignment)

##### `impl CloneToUninit for Alignment`

- <span id="alignment-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Alignment`

##### `impl Debug for Alignment`

- <span id="alignment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Alignment`

##### `impl<T> From for Alignment`

- <span id="alignment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Alignment`

- <span id="alignment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Alignment`

- <span id="alignment-partialeq-eq"></span>`fn eq(&self, other: &Alignment) -> bool` — [`Alignment`](#alignment)

##### `impl StructuralPartialEq for Alignment`

##### `impl ToOwned for Alignment`

- <span id="alignment-toowned-type-owned"></span>`type Owned = T`

- <span id="alignment-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="alignment-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Alignment`

- <span id="alignment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="alignment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Alignment`

- <span id="alignment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="alignment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ProgressTracker`

```rust
trait ProgressTracker: Send + Sync { ... }
```

*Defined in [`indicatif-0.18.3/src/style.rs:815-824`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L815-L824)*

Trait for defining stateful or stateless formatters

#### Required Methods

- `fn clone_box(&self) -> Box<dyn ProgressTracker>`

  Creates a new instance of the progress tracker

- `fn tick(&mut self, state: &ProgressState, now: Instant)`

  Notifies the progress tracker of a tick event

- `fn reset(&mut self, state: &ProgressState, now: Instant)`

  Notifies the progress tracker of a reset event

- `fn write(&self, state: &ProgressState, w: &mut dyn fmt::Write)`

  Provides access to the progress bar display buffer for custom messages

#### Implementors

- `F`

## Functions

### `segment`

```rust
fn segment(s: &str) -> Vec<Box<str>>
```

*Defined in [`indicatif-0.18.3/src/style.rs:41-43`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L41-L43)*

### `measure`

```rust
fn measure(s: &str) -> usize
```

*Defined in [`indicatif-0.18.3/src/style.rs:46-48`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L46-L48)*

### `width`

```rust
fn width(c: &[Box<str>]) -> usize
```

*Defined in [`indicatif-0.18.3/src/style.rs:57-68`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L57-L68)*

finds the unicode-aware width of the passed grapheme cluters
panics on an empty parameter, or if the characters are not equal-width

### `write_ansi_range`

```rust
fn write_ansi_range(formatter: &mut std::fmt::Formatter<'_>, text: &str, start: usize, end: usize) -> fmt::Result
```

*Defined in [`indicatif-0.18.3/src/style.rs:774-805`](../../../.source_1765633015/indicatif-0.18.3/src/style.rs#L774-L805)*

Write the visible text between start and end. The ansi escape
sequences are written unchanged.

