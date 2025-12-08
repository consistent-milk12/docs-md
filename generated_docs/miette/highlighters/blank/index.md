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

The default syntax highlighter. It applies `Style::default()` to input text.
This is used by default when no syntax highlighting features are enabled.

#### Trait Implementations

##### `impl Clone for BlankHighlighter`

- <span id="blankhighlighter-clone"></span>`fn clone(&self) -> BlankHighlighter` — [`BlankHighlighter`](../index.md)

##### `impl Debug for BlankHighlighter`

- <span id="blankhighlighter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BlankHighlighter`

- <span id="blankhighlighter-default"></span>`fn default() -> Self`

##### `impl Highlighter for BlankHighlighter`

- <span id="blankhighlighter-start-highlighter-state"></span>`fn start_highlighter_state<'h>(self: &'h Self, _source: &dyn SpanContents<'_>) -> Box<dyn super::HighlighterState>` — [`SpanContents`](../../index.md), [`HighlighterState`](../index.md)

##### `impl<D> OwoColorize for BlankHighlighter`

### `BlankHighlighterState`

```rust
struct BlankHighlighterState;
```

The default highlighter state. It applies `Style::default()` to input text.
This is used by default when no syntax highlighting features are enabled.

#### Trait Implementations

##### `impl Clone for BlankHighlighterState`

- <span id="blankhighlighterstate-clone"></span>`fn clone(&self) -> BlankHighlighterState` — [`BlankHighlighterState`](../index.md)

##### `impl Debug for BlankHighlighterState`

- <span id="blankhighlighterstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl HighlighterState for BlankHighlighterState`

- <span id="blankhighlighterstate-highlight-line"></span>`fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<owo_colors::Styled<&'s str>>`

##### `impl<D> OwoColorize for BlankHighlighterState`

