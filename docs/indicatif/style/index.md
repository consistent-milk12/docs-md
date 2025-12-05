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

- `fn with_template(template: &str) -> Result<Self, TemplateError>` — [`TemplateError`](../../style/index.md)

- `fn set_tab_width(self: &mut Self, new_tab_width: usize)`

- `fn set_for_stderr(self: &mut Self)`

- `fn new(template: Template) -> Self` — [`Template`](../../style/index.md)

- `fn tick_chars(self: Self, s: &str) -> Self`

- `fn tick_strings(self: Self, s: &[&str]) -> Self`

- `fn progress_chars(self: Self, s: &str) -> Self`

- `fn with_key<S: ProgressTracker + 'static>(self: Self, key: &'static str, f: S) -> Self`

- `fn template(self: Self, s: &str) -> Result<Self, TemplateError>` — [`TemplateError`](../../style/index.md)

- `fn current_tick_str(self: &Self, state: &ProgressState) -> &str` — [`ProgressState`](../../state/index.md)

- `fn get_tick_str(self: &Self, idx: u64) -> &str`

- `fn get_final_tick_str(self: &Self) -> &str`

- `fn format_bar(self: &Self, fract: f32, width: usize, alt_style: Option<&Style>) -> BarDisplay<'_>` — [`BarDisplay`](../../style/index.md)

- `fn format_state(self: &Self, state: &ProgressState, lines: &mut Vec<LineType>, target_width: u16)` — [`ProgressState`](../../state/index.md), [`LineType`](../../draw_target/index.md)

- `fn push_line(self: &Self, lines: &mut Vec<LineType>, cur: &mut String, state: &ProgressState, buf: &mut String, target_width: u16, wide: &Option<WideElement<'_>>)` — [`LineType`](../../draw_target/index.md), [`ProgressState`](../../state/index.md), [`WideElement`](../../style/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ProgressStyle` — [`ProgressStyle`](../../style/index.md)

### `TemplateError`

```rust
struct TemplateError {
    state: State,
    next: char,
}
```

#### Trait Implementations

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

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

### `write_ansi_range`

```rust
fn write_ansi_range(formatter: &mut std::fmt::Formatter<'_>, text: &str, start: usize, end: usize) -> fmt::Result
```

Write the visible text between start and end. The ansi escape
sequences are written unchanged.

