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
  - [`write_ansi_range`](#write_ansi_range)

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
| [`write_ansi_range`](#write_ansi_range) | fn | Write the visible text between start and end. |

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

*Defined in [`indicatif-0.18.3/src/style.rs:23-31`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L23-L31)*

#### Implementations

- <span id="progressstyle-default-bar"></span>`fn default_bar() -> Self`

- <span id="progressstyle-default-spinner"></span>`fn default_spinner() -> Self`

- <span id="progressstyle-with-template"></span>`fn with_template(template: &str) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

- <span id="progressstyle-set-tab-width"></span>`fn set_tab_width(&mut self, new_tab_width: usize)`

- <span id="progressstyle-set-for-stderr"></span>`fn set_for_stderr(&mut self)`

- <span id="progressstyle-new"></span>`fn new(template: Template) -> Self` — [`Template`](#template)

- <span id="progressstyle-tick-chars"></span>`fn tick_chars(self, s: &str) -> Self`

- <span id="progressstyle-tick-strings"></span>`fn tick_strings(self, s: &[&str]) -> Self`

- <span id="progressstyle-progress-chars"></span>`fn progress_chars(self, s: &str) -> Self`

- <span id="progressstyle-with-key"></span>`fn with_key<S: ProgressTracker + 'static>(self, key: &'static str, f: S) -> Self`

- <span id="progressstyle-template"></span>`fn template(self, s: &str) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

- <span id="progressstyle-current-tick-str"></span>`fn current_tick_str(&self, state: &ProgressState) -> &str` — [`ProgressState`](../state/index.md)

- <span id="progressstyle-get-tick-str"></span>`fn get_tick_str(&self, idx: u64) -> &str`

- <span id="progressstyle-get-final-tick-str"></span>`fn get_final_tick_str(&self) -> &str`

- <span id="progressstyle-format-bar"></span>`fn format_bar(&self, fract: f32, width: usize, alt_style: Option<&Style>) -> BarDisplay<'_>` — [`BarDisplay`](#bardisplay)

- <span id="progressstyle-format-state"></span>`fn format_state(&self, state: &ProgressState, lines: &mut Vec<LineType>, target_width: u16)` — [`ProgressState`](../state/index.md), [`LineType`](../draw_target/index.md)

- <span id="progressstyle-push-line"></span>`fn push_line(&self, lines: &mut Vec<LineType>, cur: &mut String, state: &ProgressState, buf: &mut String, target_width: u16, wide: &Option<WideElement<'_>>)` — [`LineType`](../draw_target/index.md), [`ProgressState`](../state/index.md), [`WideElement`](#wideelement)

#### Trait Implementations

##### `impl Clone for ProgressStyle`

- <span id="progressstyle-clone"></span>`fn clone(&self) -> ProgressStyle` — [`ProgressStyle`](#progressstyle)

### `TabRewriter<'a>`

```rust
struct TabRewriter<'a>(&'a mut dyn fmt::Write, usize);
```

*Defined in [`indicatif-0.18.3/src/style.rs:444`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L444)*

#### Trait Implementations

##### `impl Write for TabRewriter<'_>`

- <span id="tabrewriter-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

### `Template`

```rust
struct Template {
    parts: Vec<TemplatePart>,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:506-508`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L506-L508)*

#### Implementations

- <span id="template-from-str-with-tab-width"></span>`fn from_str_with_tab_width(s: &str, tab_width: usize) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

- <span id="template-from-str"></span>`fn from_str(s: &str) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

- <span id="template-set-tab-width"></span>`fn set_tab_width(&mut self, new_tab_width: usize)`

#### Trait Implementations

##### `impl Clone for Template`

- <span id="template-clone"></span>`fn clone(&self) -> Template` — [`Template`](#template)

##### `impl Debug for Template`

- <span id="template-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TemplateError`

```rust
struct TemplateError {
    state: State,
    next: char,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:656-659`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L656-L659)*

#### Trait Implementations

##### `impl Debug for TemplateError`

- <span id="templateerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TemplateError`

- <span id="templateerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for TemplateError`

##### `impl ToString for TemplateError`

- <span id="templateerror-to-string"></span>`fn to_string(&self) -> String`

### `BarDisplay<'a>`

```rust
struct BarDisplay<'a> {
    chars: &'a [Box<str>],
    filled: usize,
    cur: Option<usize>,
    rest: console::StyledObject<RepeatedStringDisplay<'a>>,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:699-704`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L699-L704)*

#### Trait Implementations

##### `impl Display for BarDisplay<'_>`

- <span id="bardisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for BarDisplay<'a>`

- <span id="bardisplay-to-string"></span>`fn to_string(&self) -> String`

### `RepeatedStringDisplay<'a>`

```rust
struct RepeatedStringDisplay<'a> {
    str: &'a str,
    num: usize,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:718-721`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L718-L721)*

#### Trait Implementations

##### `impl Display for RepeatedStringDisplay<'_>`

- <span id="repeatedstringdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for RepeatedStringDisplay<'a>`

- <span id="repeatedstringdisplay-to-string"></span>`fn to_string(&self) -> String`

### `PaddedStringDisplay<'a>`

```rust
struct PaddedStringDisplay<'a> {
    str: &'a str,
    width: usize,
    align: Alignment,
    truncate: bool,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:732-737`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L732-L737)*

#### Trait Implementations

##### `impl Display for PaddedStringDisplay<'_>`

- <span id="paddedstringdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for PaddedStringDisplay<'a>`

- <span id="paddedstringdisplay-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`indicatif-0.18.3/src/style.rs:454-457`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L454-L457)*

#### Implementations

- <span id="wideelement-expand"></span>`fn expand(self, cur: String, style: &ProgressStyle, state: &ProgressState, buf: &mut String, width: u16) -> String` — [`ProgressStyle`](#progressstyle), [`ProgressState`](../state/index.md)

#### Trait Implementations

##### `impl Clone for WideElement<'a>`

- <span id="wideelement-clone"></span>`fn clone(&self) -> WideElement<'a>` — [`WideElement`](#wideelement)

##### `impl Copy for WideElement<'a>`

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

*Defined in [`indicatif-0.18.3/src/style.rs:674-685`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L674-L685)*

#### Trait Implementations

##### `impl Clone for TemplatePart`

- <span id="templatepart-clone"></span>`fn clone(&self) -> TemplatePart` — [`TemplatePart`](#templatepart)

##### `impl Debug for TemplatePart`

- <span id="templatepart-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TemplatePart`

##### `impl PartialEq for TemplatePart`

- <span id="templatepart-eq"></span>`fn eq(&self, other: &TemplatePart) -> bool` — [`TemplatePart`](#templatepart)

##### `impl StructuralPartialEq for TemplatePart`

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

*Defined in [`indicatif-0.18.3/src/style.rs:688-697`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L688-L697)*

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl Copy for State`

##### `impl Debug for State`

- <span id="state-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for State`

##### `impl PartialEq for State`

- <span id="state-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`

### `Alignment`

```rust
enum Alignment {
    Left,
    Center,
    Right,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:808-812`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L808-L812)*

#### Trait Implementations

##### `impl Clone for Alignment`

- <span id="alignment-clone"></span>`fn clone(&self) -> Alignment` — [`Alignment`](#alignment)

##### `impl Copy for Alignment`

##### `impl Debug for Alignment`

- <span id="alignment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Alignment`

##### `impl PartialEq for Alignment`

- <span id="alignment-eq"></span>`fn eq(&self, other: &Alignment) -> bool` — [`Alignment`](#alignment)

##### `impl StructuralPartialEq for Alignment`

## Traits

### `ProgressTracker`

```rust
trait ProgressTracker: Send + Sync { ... }
```

*Defined in [`indicatif-0.18.3/src/style.rs:815-824`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L815-L824)*

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

*Defined in [`indicatif-0.18.3/src/style.rs:41-43`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L41-L43)*

### `measure`

```rust
fn measure(s: &str) -> usize
```

*Defined in [`indicatif-0.18.3/src/style.rs:46-48`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L46-L48)*

### `width`

```rust
fn width(c: &[Box<str>]) -> usize
```

*Defined in [`indicatif-0.18.3/src/style.rs:57-68`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L57-L68)*

finds the unicode-aware width of the passed grapheme cluters
panics on an empty parameter, or if the characters are not equal-width

### `write_ansi_range`

```rust
fn write_ansi_range(formatter: &mut std::fmt::Formatter<'_>, text: &str, start: usize, end: usize) -> fmt::Result
```

*Defined in [`indicatif-0.18.3/src/style.rs:774-805`](../../../.source_1765210505/indicatif-0.18.3/src/style.rs#L774-L805)*

Write the visible text between start and end. The ansi escape
sequences are written unchanged.

