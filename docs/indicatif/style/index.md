*[indicatif](../index.md) / [style](index.md)*

---

# Module `style`

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

#### Implementations

- `fn default_bar() -> Self`

- `fn default_spinner() -> Self`

- `fn with_template(template: &str) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

- `fn set_tab_width(self: &mut Self, new_tab_width: usize)`

- `fn set_for_stderr(self: &mut Self)`

- `fn new(template: Template) -> Self` — [`Template`](#template)

- `fn tick_chars(self: Self, s: &str) -> Self`

- `fn tick_strings(self: Self, s: &[&str]) -> Self`

- `fn progress_chars(self: Self, s: &str) -> Self`

- `fn with_key<S: ProgressTracker + 'static>(self: Self, key: &'static str, f: S) -> Self`

- `fn template(self: Self, s: &str) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

- `fn current_tick_str(self: &Self, state: &ProgressState) -> &str` — [`ProgressState`](../state/index.md)

- `fn get_tick_str(self: &Self, idx: u64) -> &str`

- `fn get_final_tick_str(self: &Self) -> &str`

- `fn format_bar(self: &Self, fract: f32, width: usize, alt_style: Option<&Style>) -> BarDisplay<'_>` — [`BarDisplay`](#bardisplay)

- `fn format_state(self: &Self, state: &ProgressState, lines: &mut Vec<LineType>, target_width: u16)` — [`ProgressState`](../state/index.md), [`LineType`](../draw_target/index.md)

- `fn push_line(self: &Self, lines: &mut Vec<LineType>, cur: &mut String, state: &ProgressState, buf: &mut String, target_width: u16, wide: &Option<WideElement<'_>>)` — [`LineType`](../draw_target/index.md), [`ProgressState`](../state/index.md), [`WideElement`](#wideelement)

#### Trait Implementations

##### `impl Clone for ProgressStyle`

- `fn clone(self: &Self) -> ProgressStyle` — [`ProgressStyle`](#progressstyle)

### `TabRewriter<'a>`

```rust
struct TabRewriter<'a>(&'a mut dyn fmt::Write, usize);
```

#### Trait Implementations

##### `impl Write for TabRewriter<'_>`

- `fn write_str(self: &mut Self, s: &str) -> fmt::Result`

### `Template`

```rust
struct Template {
    parts: Vec<TemplatePart>,
}
```

#### Implementations

- `fn from_str_with_tab_width(s: &str, tab_width: usize) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

- `fn from_str(s: &str) -> Result<Self, TemplateError>` — [`TemplateError`](#templateerror)

- `fn set_tab_width(self: &mut Self, new_tab_width: usize)`

#### Trait Implementations

##### `impl Clone for Template`

- `fn clone(self: &Self) -> Template` — [`Template`](#template)

##### `impl Debug for Template`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `TemplateError`

```rust
struct TemplateError {
    state: State,
    next: char,
}
```

#### Trait Implementations

##### `impl Debug for TemplateError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for TemplateError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for TemplateError`

##### `impl<T> ToString for TemplateError`

- `fn to_string(self: &Self) -> String`

### `BarDisplay<'a>`

```rust
struct BarDisplay<'a> {
    chars: &'a [Box<str>],
    filled: usize,
    cur: Option<usize>,
    rest: console::StyledObject<RepeatedStringDisplay<'a>>,
}
```

#### Trait Implementations

##### `impl Display for BarDisplay<'_>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for BarDisplay<'a>`

- `fn to_string(self: &Self) -> String`

### `RepeatedStringDisplay<'a>`

```rust
struct RepeatedStringDisplay<'a> {
    str: &'a str,
    num: usize,
}
```

#### Trait Implementations

##### `impl Display for RepeatedStringDisplay<'_>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for RepeatedStringDisplay<'a>`

- `fn to_string(self: &Self) -> String`

### `PaddedStringDisplay<'a>`

```rust
struct PaddedStringDisplay<'a> {
    str: &'a str,
    width: usize,
    align: Alignment,
    truncate: bool,
}
```

#### Trait Implementations

##### `impl Display for PaddedStringDisplay<'_>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for PaddedStringDisplay<'a>`

- `fn to_string(self: &Self) -> String`

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

#### Implementations

- `fn expand(self: Self, cur: String, style: &ProgressStyle, state: &ProgressState, buf: &mut String, width: u16) -> String` — [`ProgressStyle`](#progressstyle), [`ProgressState`](../state/index.md)

#### Trait Implementations

##### `impl<'a> Clone for WideElement<'a>`

- `fn clone(self: &Self) -> WideElement<'a>` — [`WideElement`](#wideelement)

##### `impl<'a> Copy for WideElement<'a>`

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

#### Trait Implementations

##### `impl Clone for TemplatePart`

- `fn clone(self: &Self) -> TemplatePart` — [`TemplatePart`](#templatepart)

##### `impl Debug for TemplatePart`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for TemplatePart`

##### `impl PartialEq for TemplatePart`

- `fn eq(self: &Self, other: &TemplatePart) -> bool` — [`TemplatePart`](#templatepart)

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

#### Trait Implementations

##### `impl Clone for State`

- `fn clone(self: &Self) -> State` — [`State`](#state)

##### `impl Copy for State`

##### `impl Debug for State`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for State`

##### `impl PartialEq for State`

- `fn eq(self: &Self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`

### `Alignment`

```rust
enum Alignment {
    Left,
    Center,
    Right,
}
```

#### Trait Implementations

##### `impl Clone for Alignment`

- `fn clone(self: &Self) -> Alignment` — [`Alignment`](#alignment)

##### `impl Copy for Alignment`

##### `impl Debug for Alignment`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Alignment`

##### `impl PartialEq for Alignment`

- `fn eq(self: &Self, other: &Alignment) -> bool` — [`Alignment`](#alignment)

##### `impl StructuralPartialEq for Alignment`

## Traits

### `ProgressTracker`

```rust
trait ProgressTracker: Send + Sync { ... }
```

Trait for defining stateful or stateless formatters

#### Required Methods

- `fn clone_box(self: &Self) -> Box<dyn ProgressTracker>`

  Creates a new instance of the progress tracker

- `fn tick(self: &mut Self, state: &ProgressState, now: Instant)`

  Notifies the progress tracker of a tick event

- `fn reset(self: &mut Self, state: &ProgressState, now: Instant)`

  Notifies the progress tracker of a reset event

- `fn write(self: &Self, state: &ProgressState, w: &mut dyn fmt::Write)`

  Provides access to the progress bar display buffer for custom messages

## Functions

### `segment`

```rust
fn segment(s: &str) -> Vec<Box<str>>
```

### `measure`

```rust
fn measure(s: &str) -> usize
```

### `width`

```rust
fn width(c: &[Box<str>]) -> usize
```

finds the unicode-aware width of the passed grapheme cluters
panics on an empty parameter, or if the characters are not equal-width

### `write_ansi_range`

```rust
fn write_ansi_range(formatter: &mut std::fmt::Formatter<'_>, text: &str, start: usize, end: usize) -> fmt::Result
```

Write the visible text between start and end. The ansi escape
sequences are written unchanged.

