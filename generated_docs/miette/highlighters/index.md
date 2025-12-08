*[miette](../index.md) / [highlighters](index.md)*

---

# Module `highlighters`

This module provides a trait for creating custom syntax highlighters that
highlight [`Diagnostic`](crate::Diagnostic) source code with ANSI escape
sequences when rendering with the [`GraphicalReportHighlighter`](crate::GraphicalReportHandler).

It also provides built-in highlighter implementations that you can use out of the box.
By default, there are no syntax highlighters exported by miette
(except for the no-op [`BlankHighlighter`](#blankhighlighter)).
To enable support for specific highlighters, you should enable their associated feature flag.

Currently supported syntax highlighters and their feature flags:
* `syntect-highlighter` - Enables [`syntect`](https://docs.rs/syntect/latest/syntect/) syntax highlighting support via the `SyntectHighlighter`


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`blank`](#blank) | mod |  |
| [`MietteHighlighter`](#miettehighlighter) | struct | Arcified trait object for Highlighter. |
| [`BlankHighlighter`](#blankhighlighter) | struct | The default syntax highlighter. |
| [`BlankHighlighterState`](#blankhighlighterstate) | struct | The default highlighter state. |
| [`Highlighter`](#highlighter) | trait | A syntax highlighter for highlighting miette [`SourceCode`](crate::SourceCode) snippets. |
| [`HighlighterState`](#highlighterstate) | trait | A stateful highlighter that incrementally highlights lines of a particular |

## Modules

- [`blank`](blank/index.md) - 

## Structs

### `MietteHighlighter`

```rust
struct MietteHighlighter(std::sync::Arc<dyn Highlighter + Send + Sync>);
```

Arcified trait object for Highlighter. Used internally by [`GraphicalReportHandler`](../handlers/index.md)

Wrapping the trait object in this way allows us to implement `Debug` and `Clone`.

#### Implementations

- <span id="miettehighlighter-nocolor"></span>`fn nocolor() -> Self`

#### Trait Implementations

##### `impl Clone for MietteHighlighter`

- <span id="miettehighlighter-clone"></span>`fn clone(&self) -> MietteHighlighter` — [`MietteHighlighter`](#miettehighlighter)

##### `impl Debug for MietteHighlighter`

- <span id="miettehighlighter-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for MietteHighlighter`

- <span id="miettehighlighter-default"></span>`fn default() -> Self`

##### `impl Deref for MietteHighlighter`

- <span id="miettehighlighter-target"></span>`type Target = dyn Highlighter + Send + Sync`

- <span id="miettehighlighter-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<D> OwoColorize for MietteHighlighter`

##### `impl<P, T> Receiver for MietteHighlighter`

- <span id="miettehighlighter-target"></span>`type Target = T`

### `BlankHighlighter`

```rust
struct BlankHighlighter;
```

The default syntax highlighter. It applies `Style::default()` to input text.
This is used by default when no syntax highlighting features are enabled.

#### Trait Implementations

##### `impl Clone for BlankHighlighter`

- <span id="blankhighlighter-clone"></span>`fn clone(&self) -> BlankHighlighter` — [`BlankHighlighter`](#blankhighlighter)

##### `impl Debug for BlankHighlighter`

- <span id="blankhighlighter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BlankHighlighter`

- <span id="blankhighlighter-default"></span>`fn default() -> Self`

##### `impl Highlighter for BlankHighlighter`

- <span id="blankhighlighter-start-highlighter-state"></span>`fn start_highlighter_state<'h>(self: &'h Self, _source: &dyn SpanContents<'_>) -> Box<dyn super::HighlighterState>` — [`SpanContents`](../index.md), [`HighlighterState`](#highlighterstate)

##### `impl<D> OwoColorize for BlankHighlighter`

### `BlankHighlighterState`

```rust
struct BlankHighlighterState;
```

The default highlighter state. It applies `Style::default()` to input text.
This is used by default when no syntax highlighting features are enabled.

#### Trait Implementations

##### `impl Clone for BlankHighlighterState`

- <span id="blankhighlighterstate-clone"></span>`fn clone(&self) -> BlankHighlighterState` — [`BlankHighlighterState`](#blankhighlighterstate)

##### `impl Debug for BlankHighlighterState`

- <span id="blankhighlighterstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl HighlighterState for BlankHighlighterState`

- <span id="blankhighlighterstate-highlight-line"></span>`fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<owo_colors::Styled<&'s str>>`

##### `impl<D> OwoColorize for BlankHighlighterState`

## Traits

### `Highlighter`

```rust
trait Highlighter { ... }
```

A syntax highlighter for highlighting miette [`SourceCode`](crate::SourceCode) snippets.

#### Required Methods

- `fn start_highlighter_state<'h>(self: &'h Self, source: &dyn SpanContents<'_>) -> Box<dyn HighlighterState>`

   Creates a new [`HighlighterState`](#highlighterstate) to begin parsing and highlighting

### `HighlighterState`

```rust
trait HighlighterState { ... }
```

A stateful highlighter that incrementally highlights lines of a particular
source code.

The [`GraphicalReportHandler`](crate::GraphicalReportHandler)
will create a highlighter state by calling
[`start_highlighter_state`](Highlighter::start_highlighter_state) at the
start of rendering, then it will iteratively call
[`highlight_line`](HighlighterState::highlight_line) to render individual
highlighted lines. This allows [`Highlighter`](#highlighter) implementations to maintain
mutable parsing and highlighting state.

#### Required Methods

- `fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<Styled<&'s str>>`

  Highlight an individual line from the source code by returning a vector of [Styled]

