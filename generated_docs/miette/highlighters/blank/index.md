*[miette](../../index.md) / [highlighters](../index.md) / [blank](index.md)*

---

# Module `blank`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BlankHighlighter`](#blankhighlighter) | struct | The default syntax highlighter. |
| [`BlankHighlighterState`](#blankhighlighterstate) | struct | The default highlighter state. |

## Structs

### `BlankHighlighter`

```rust
struct BlankHighlighter;
```

*Defined in [`miette-7.6.0/src/highlighters/blank.rs:10`](../../../../.source_1765210505/miette-7.6.0/src/highlighters/blank.rs#L10)*

The default syntax highlighter. It applies `Style::default()` to input text.
This is used by default when no syntax highlighting features are enabled.

#### Trait Implementations

##### `impl Clone for BlankHighlighter`

- <span id="blankhighlighter-clone"></span>`fn clone(&self) -> BlankHighlighter` — [`BlankHighlighter`](../index.md#blankhighlighter)

##### `impl Debug for BlankHighlighter`

- <span id="blankhighlighter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BlankHighlighter`

- <span id="blankhighlighter-default"></span>`fn default() -> Self`

##### `impl Highlighter for BlankHighlighter`

- <span id="blankhighlighter-start-highlighter-state"></span>`fn start_highlighter_state<'h>(self: &'h Self, _source: &dyn SpanContents<'_>) -> Box<dyn super::HighlighterState>` — [`SpanContents`](../../index.md#spancontents), [`HighlighterState`](../index.md#highlighterstate)

##### `impl OwoColorize for BlankHighlighter`

### `BlankHighlighterState`

```rust
struct BlankHighlighterState;
```

*Defined in [`miette-7.6.0/src/highlighters/blank.rs:30`](../../../../.source_1765210505/miette-7.6.0/src/highlighters/blank.rs#L30)*

The default highlighter state. It applies `Style::default()` to input text.
This is used by default when no syntax highlighting features are enabled.

#### Trait Implementations

##### `impl Clone for BlankHighlighterState`

- <span id="blankhighlighterstate-clone"></span>`fn clone(&self) -> BlankHighlighterState` — [`BlankHighlighterState`](../index.md#blankhighlighterstate)

##### `impl Debug for BlankHighlighterState`

- <span id="blankhighlighterstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl HighlighterState for BlankHighlighterState`

- <span id="blankhighlighterstate-highlight-line"></span>`fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<owo_colors::Styled<&'s str>>`

##### `impl OwoColorize for BlankHighlighterState`

