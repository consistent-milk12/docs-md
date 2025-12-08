*[indicatif](../index.md) / [term_like](index.md)*

---

# Module `term_like`

## Traits

### `TermLike`

```rust
trait TermLike: Debug + Send + Sync { ... }
```

A trait for minimal terminal-like behavior.

Anything that implements this trait can be used a draw target via `ProgressDrawTarget::term_like`.


#### Required Methods

- `fn width(self: &Self) -> u16`

  Return the terminal width

- `fn height(self: &Self) -> u16`

  Return the terminal height

- `fn move_cursor_up(self: &Self, n: usize) -> io::Result<()>`

  Move the cursor up by `n` lines

- `fn move_cursor_down(self: &Self, n: usize) -> io::Result<()>`

  Move the cursor down by `n` lines

- `fn move_cursor_right(self: &Self, n: usize) -> io::Result<()>`

  Move the cursor right by `n` chars

- `fn move_cursor_left(self: &Self, n: usize) -> io::Result<()>`

  Move the cursor left by `n` chars

- `fn write_line(self: &Self, s: &str) -> io::Result<()>`

  Write a string and add a newline.

- `fn write_str(self: &Self, s: &str) -> io::Result<()>`

  Write a string

- `fn clear_line(self: &Self) -> io::Result<()>`

  Clear the current line and reset the cursor to beginning of the line

- `fn flush(self: &Self) -> io::Result<()>`

