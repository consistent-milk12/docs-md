*[indicatif](../index.md) / [term_like](index.md)*

---

# Module `term_like`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TermLike`](#termlike) | trait | A trait for minimal terminal-like behavior. |

## Traits

### `TermLike`

```rust
trait TermLike: Debug + Send + Sync { ... }
```

*Defined in [`indicatif-0.18.3/src/term_like.rs:11-37`](../../../.source_1765900590/indicatif-0.18.3/src/term_like.rs#L11-L37)*

A trait for minimal terminal-like behavior.

Anything that implements this trait can be used a draw target via `ProgressDrawTarget::term_like`.


<details>
<summary><strong>Methods (10)</strong> - click to expand</summary>

**Required:**
- [`TermLike::width`](#fn-termlikewidth)
- [`TermLike::move_cursor_up`](#fn-termlikemove-cursor-up)
- [`TermLike::move_cursor_down`](#fn-termlikemove-cursor-down)
- [`TermLike::move_cursor_right`](#fn-termlikemove-cursor-right)
- [`TermLike::move_cursor_left`](#fn-termlikemove-cursor-left)
- [`TermLike::write_line`](#fn-termlikewrite-line)
- [`TermLike::write_str`](#fn-termlikewrite-str)
- [`TermLike::clear_line`](#fn-termlikeclear-line)
- [`TermLike::flush`](#fn-termlikeflush)

**Provided:**
- [`TermLike::height`](#fn-termlikeheight)

</details>

#### Required Methods

- `fn TermLike::width(&self) -> u16`

  Return the terminal width

- `fn TermLike::move_cursor_up(&self, n: usize) -> io::Result<()>`

  Move the cursor up by `n` lines

- `fn TermLike::move_cursor_down(&self, n: usize) -> io::Result<()>`

  Move the cursor down by `n` lines

- `fn TermLike::move_cursor_right(&self, n: usize) -> io::Result<()>`

  Move the cursor right by `n` chars

- `fn TermLike::move_cursor_left(&self, n: usize) -> io::Result<()>`

  Move the cursor left by `n` chars

- `fn TermLike::write_line(&self, s: &str) -> io::Result<()>`

  Write a string and add a newline.

- `fn TermLike::write_str(&self, s: &str) -> io::Result<()>`

  Write a string

- `fn TermLike::clear_line(&self) -> io::Result<()>`

  Clear the current line and reset the cursor to beginning of the line

- `fn TermLike::flush(&self) -> io::Result<()>`

#### Provided Methods

- `fn TermLike::height(&self) -> u16`

  Return the terminal height

#### Implementors

- `console::Term`

