*[miette](../index.md) / [handler](index.md)*

---

# Module `handler`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`syscall`](#syscall) | mod |  |
| [`MietteHandlerOpts`](#miettehandleropts) | struct | Create a custom [`MietteHandler`] from options. |
| [`MietteHandler`](#miettehandler) | struct | A [`ReportHandler`] that displays a given [`Report`](crate::Report) in a quasi-graphical way, using terminal colors, unicode drawing characters, and other such things. |
| [`RgbColors`](#rgbcolors) | enum | Settings to control the color format used for graphical rendering. |
| [`HighlighterOption`](#highlighteroption) | enum |  |

## Modules

- [`syscall`](syscall/index.md)

## Structs

### `MietteHandlerOpts`

```rust
struct MietteHandlerOpts {
    linkify: Option<bool>,
    width: Option<usize>,
    theme: Option<crate::GraphicalTheme>,
    force_graphical: Option<bool>,
    force_narrated: Option<bool>,
    rgb_colors: RgbColors,
    color: Option<bool>,
    unicode: Option<bool>,
    footer: Option<String>,
    context_lines: Option<usize>,
    tab_width: Option<usize>,
    with_cause_chain: Option<bool>,
    break_words: Option<bool>,
    wrap_lines: Option<bool>,
    word_separator: Option<textwrap::WordSeparator>,
    word_splitter: Option<textwrap::WordSplitter>,
    highlighter: Option<crate::highlighters::MietteHighlighter>,
    show_related_as_nested: Option<bool>,
}
```

*Defined in [`miette-7.6.0/src/handler.rs:42-61`](../../../.source_1765521767/miette-7.6.0/src/handler.rs#L42-L61)*

Create a custom [`MietteHandler`](../index.md) from options.

## Example

```no_run
miette::set_hook(Box::new(|_| {
    Box::new(miette::MietteHandlerOpts::new()
        .terminal_links(true)
        .unicode(false)
        .context_lines(3)
        .build())
}))
.unwrap();
```

#### Implementations

- <span id="miettehandleropts-new"></span>`fn new() -> Self`

  Create a new `MietteHandlerOpts`.

- <span id="miettehandleropts-terminal-links"></span>`fn terminal_links(self, linkify: bool) -> Self`

  If true, specify whether the graphical handler will make codes be

  clickable links in supported terminals. Defaults to auto-detection

  based on known supported terminals.

- <span id="miettehandleropts-graphical-theme"></span>`fn graphical_theme(self, theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](../handlers/index.md#graphicaltheme)

  Set a graphical theme for the handler when rendering in graphical mode.

  Use [`force_graphical()`](`MietteHandlerOpts::force_graphical) to force

  graphical mode. This option overrides

  [`color()`](`MietteHandlerOpts::color).

- <span id="miettehandleropts-with-syntax-highlighting"></span>`fn with_syntax_highlighting(self, highlighter: impl Highlighter + Send + Sync + 'static) -> Self` — [`Highlighter`](../highlighters/index.md#highlighter)

  Set a syntax highlighter when rendering in graphical mode.

  Use [`force_graphical()`](MietteHandlerOpts::force_graphical()) to

  force graphical mode.

  

  Syntax highlighting is disabled by default unless the

  `syntect-highlighter` feature is enabled. Call this method

  to override the default and use a custom highlighter

  implementation instead.

  

  Use

  [`without_syntax_highlighting()`](MietteHandlerOpts::without_syntax_highlighting())

  To disable highlighting completely.

  

  Setting this option will not force color output. In all cases, the

  current color configuration via

  [`color()`](MietteHandlerOpts::color()) takes precedence over

  highlighter configuration. However, this option does take precedence over

  [`rgb_colors()`](MietteHandlerOpts::rgb_colors()) (meaning syntax highlighting will be

  enabled regardless of the value of `MietteHandlerOpts::rgb_colors`).

- <span id="miettehandleropts-without-syntax-highlighting"></span>`fn without_syntax_highlighting(self) -> Self`

  Disables syntax highlighting when rendering in graphical mode.

  Use [`force_graphical()`](MietteHandlerOpts::force_graphical()) to

  force graphical mode.

  

  Syntax highlighting is disabled by default unless the

  `syntect-highlighter` feature is enabled. Call this method if you want

  to disable highlighting when building with this feature.

- <span id="miettehandleropts-width"></span>`fn width(self, width: usize) -> Self`

  Sets the width to wrap the report at. Defaults to 80.

- <span id="miettehandleropts-wrap-lines"></span>`fn wrap_lines(self, wrap_lines: bool) -> Self`

  If true, long lines can be wrapped.

  

  If false, long lines will not be broken when they exceed the width.

  

  Defaults to true.

- <span id="miettehandleropts-break-words"></span>`fn break_words(self, break_words: bool) -> Self`

  If true, long words can be broken when wrapping.

  

  If false, long words will not be broken when they exceed the width.

  

  Defaults to true.

- <span id="miettehandleropts-word-separator"></span>`fn word_separator(self, word_separator: textwrap::WordSeparator) -> Self`

  Sets the `textwrap::WordSeparator` to use when determining wrap points.

- <span id="miettehandleropts-word-splitter"></span>`fn word_splitter(self, word_splitter: textwrap::WordSplitter) -> Self`

  Sets the `textwrap::WordSplitter` to use when determining wrap points.

- <span id="miettehandleropts-with-cause-chain"></span>`fn with_cause_chain(self) -> Self`

  Include the cause chain of the top-level error in the report.

- <span id="miettehandleropts-without-cause-chain"></span>`fn without_cause_chain(self) -> Self`

  Do not include the cause chain of the top-level error in the report.

- <span id="miettehandleropts-show-related-errors-as-siblings"></span>`fn show_related_errors_as_siblings(self) -> Self`

  Show related errors as siblings.

- <span id="miettehandleropts-show-related-errors-as-nested"></span>`fn show_related_errors_as_nested(self) -> Self`

  Show related errors as nested errors.

- <span id="miettehandleropts-color"></span>`fn color(self, color: bool) -> Self`

  If true, colors will be used during graphical rendering, regardless

  of whether or not the terminal supports them.

  

  If false, colors will never be used.

  

  If unspecified, colors will be used only if the terminal supports them.

  

  The actual format depends on the value of

  `MietteHandlerOpts::rgb_colors`.

- <span id="miettehandleropts-rgb-colors"></span>`fn rgb_colors(self, color: RgbColors) -> Self` — [`RgbColors`](../index.md#rgbcolors)

  Controls which color format to use if colors are used in graphical

  rendering.

  

  The default is `Never`.

  

  This value does not control whether or not colors are being used in the

  first place. That is handled by the `MietteHandlerOpts::color`

  setting. If colors are not being used, the value of `rgb_colors` has

  no effect.

  

  It also does not control colors when a syntax highlighter is in use.

- <span id="miettehandleropts-unicode"></span>`fn unicode(self, unicode: bool) -> Self`

  If true, forces unicode display for graphical output. If set to false,

  forces ASCII art display.

- <span id="miettehandleropts-force-graphical"></span>`fn force_graphical(self, force: bool) -> Self`

  If true, graphical rendering will be used regardless of terminal

  detection.

- <span id="miettehandleropts-force-narrated"></span>`fn force_narrated(self, force: bool) -> Self`

  If true, forces use of the narrated renderer.

- <span id="miettehandleropts-footer"></span>`fn footer(self, footer: String) -> Self`

  Set a footer to be displayed at the bottom of the report.

- <span id="miettehandleropts-context-lines"></span>`fn context_lines(self, context_lines: usize) -> Self`

  Sets the number of context lines before and after a span to display.

- <span id="miettehandleropts-tab-width"></span>`fn tab_width(self, width: usize) -> Self`

  Set the displayed tab width in spaces.

- <span id="miettehandleropts-build"></span>`fn build(self) -> MietteHandler` — [`MietteHandler`](../index.md#miettehandler)

  Builds a [`MietteHandler`](../index.md) from this builder.

- <span id="miettehandleropts-is-graphical"></span>`fn is_graphical(&self) -> bool`

- <span id="miettehandleropts-use-links"></span>`fn use_links(&self) -> bool`

- <span id="miettehandleropts-get-width"></span>`fn get_width(&self) -> usize`

#### Trait Implementations

##### `impl Any for MietteHandlerOpts`

- <span id="miettehandleropts-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MietteHandlerOpts`

- <span id="miettehandleropts-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MietteHandlerOpts`

- <span id="miettehandleropts-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MietteHandlerOpts`

- <span id="miettehandleropts-clone"></span>`fn clone(&self) -> MietteHandlerOpts` — [`MietteHandlerOpts`](../index.md#miettehandleropts)

##### `impl CloneToUninit for MietteHandlerOpts`

- <span id="miettehandleropts-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MietteHandlerOpts`

- <span id="miettehandleropts-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MietteHandlerOpts`

- <span id="miettehandleropts-default"></span>`fn default() -> MietteHandlerOpts` — [`MietteHandlerOpts`](../index.md#miettehandleropts)

##### `impl<T> From for MietteHandlerOpts`

- <span id="miettehandleropts-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MietteHandlerOpts`

- <span id="miettehandleropts-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MietteHandlerOpts`

##### `impl ToOwned for MietteHandlerOpts`

- <span id="miettehandleropts-toowned-type-owned"></span>`type Owned = T`

- <span id="miettehandleropts-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="miettehandleropts-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MietteHandlerOpts`

- <span id="miettehandleropts-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="miettehandleropts-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MietteHandlerOpts`

- <span id="miettehandleropts-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="miettehandleropts-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MietteHandler`

```rust
struct MietteHandler {
    inner: Box<dyn ReportHandler + Send + Sync>,
}
```

*Defined in [`miette-7.6.0/src/handler.rs:387-389`](../../../.source_1765521767/miette-7.6.0/src/handler.rs#L387-L389)*

A [`ReportHandler`](../index.md) that displays a given [`Report`](crate::Report) in a
quasi-graphical way, using terminal colors, unicode drawing characters, and
other such things.

This is the default reporter bundled with `miette`.

This printer can be customized by using
`GraphicalReportHandler::new_themed()` and handing it a [`GraphicalTheme`](../handlers/index.md) of
your own creation (or using one of its own defaults).

See [`set_hook`](crate::set_hook) for more details on customizing your global
printer.

#### Implementations

- <span id="miettehandler-new"></span>`fn new() -> Self`

  Creates a new [`MietteHandler`](../index.md) with default settings.

#### Trait Implementations

##### `impl Any for MietteHandler`

- <span id="miettehandler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MietteHandler`

- <span id="miettehandler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MietteHandler`

- <span id="miettehandler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Default for MietteHandler`

- <span id="miettehandler-default"></span>`fn default() -> Self`

##### `impl<T> From for MietteHandler`

- <span id="miettehandler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MietteHandler`

- <span id="miettehandler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MietteHandler`

##### `impl ReportHandler for MietteHandler`

- <span id="miettehandler-reporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md#diagnostic)

##### `impl<U> TryFrom for MietteHandler`

- <span id="miettehandler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="miettehandler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MietteHandler`

- <span id="miettehandler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="miettehandler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `RgbColors`

```rust
enum RgbColors {
    Always,
    Preferred,
    Never,
}
```

*Defined in [`miette-7.6.0/src/handler.rs:15-23`](../../../.source_1765521767/miette-7.6.0/src/handler.rs#L15-L23)*

Settings to control the color format used for graphical rendering.

#### Variants

- **`Always`**

  Use RGB colors even if the terminal does not support them

- **`Preferred`**

  Use RGB colors instead of ANSI if the terminal supports RGB

- **`Never`**

  Always use ANSI, regardless of terminal support for RGB

#### Trait Implementations

##### `impl Any for RgbColors`

- <span id="rgbcolors-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RgbColors`

- <span id="rgbcolors-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RgbColors`

- <span id="rgbcolors-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RgbColors`

- <span id="rgbcolors-clone"></span>`fn clone(&self) -> RgbColors` — [`RgbColors`](../index.md#rgbcolors)

##### `impl CloneToUninit for RgbColors`

- <span id="rgbcolors-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RgbColors`

##### `impl Debug for RgbColors`

- <span id="rgbcolors-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RgbColors`

- <span id="rgbcolors-default"></span>`fn default() -> RgbColors` — [`RgbColors`](../index.md#rgbcolors)

##### `impl Eq for RgbColors`

##### `impl<T> From for RgbColors`

- <span id="rgbcolors-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RgbColors`

- <span id="rgbcolors-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for RgbColors`

##### `impl PartialEq for RgbColors`

- <span id="rgbcolors-partialeq-eq"></span>`fn eq(&self, other: &RgbColors) -> bool` — [`RgbColors`](../index.md#rgbcolors)

##### `impl StructuralPartialEq for RgbColors`

##### `impl ToOwned for RgbColors`

- <span id="rgbcolors-toowned-type-owned"></span>`type Owned = T`

- <span id="rgbcolors-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rgbcolors-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RgbColors`

- <span id="rgbcolors-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rgbcolors-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RgbColors`

- <span id="rgbcolors-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rgbcolors-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HighlighterOption`

```rust
enum HighlighterOption {
    Disable,
    EnableCustom(crate::highlighters::MietteHighlighter),
}
```

*Defined in [`miette-7.6.0/src/handler.rs:414-419`](../../../.source_1765521767/miette-7.6.0/src/handler.rs#L414-L419)*

#### Implementations

- <span id="highlighteroption-select"></span>`fn select(color: Option<bool>, highlighter: Option<MietteHighlighter>, supports_color: bool) -> HighlighterOption` — [`MietteHighlighter`](../highlighters/index.md#miettehighlighter), [`HighlighterOption`](#highlighteroption)

#### Trait Implementations

##### `impl Any for HighlighterOption`

- <span id="highlighteroption-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HighlighterOption`

- <span id="highlighteroption-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HighlighterOption`

- <span id="highlighteroption-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Default for HighlighterOption`

- <span id="highlighteroption-default"></span>`fn default() -> Self`

##### `impl<T> From for HighlighterOption`

- <span id="highlighteroption-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HighlighterOption`

- <span id="highlighteroption-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for HighlighterOption`

##### `impl<U> TryFrom for HighlighterOption`

- <span id="highlighteroption-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="highlighteroption-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HighlighterOption`

- <span id="highlighteroption-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="highlighteroption-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

