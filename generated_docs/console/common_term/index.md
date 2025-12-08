*[console](../index.md) / [common_term](index.md)*

---

# Module `common_term`

## Contents

- [Functions](#functions)
  - [`move_cursor_down`](#move_cursor_down)
  - [`move_cursor_up`](#move_cursor_up)
  - [`move_cursor_left`](#move_cursor_left)
  - [`move_cursor_right`](#move_cursor_right)
  - [`move_cursor_to`](#move_cursor_to)
  - [`clear_chars`](#clear_chars)
  - [`clear_line`](#clear_line)
  - [`clear_screen`](#clear_screen)
  - [`clear_to_end_of_screen`](#clear_to_end_of_screen)
  - [`show_cursor`](#show_cursor)
  - [`hide_cursor`](#hide_cursor)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`move_cursor_down`](#move_cursor_down) | fn |  |
| [`move_cursor_up`](#move_cursor_up) | fn |  |
| [`move_cursor_left`](#move_cursor_left) | fn |  |
| [`move_cursor_right`](#move_cursor_right) | fn |  |
| [`move_cursor_to`](#move_cursor_to) | fn |  |
| [`clear_chars`](#clear_chars) | fn |  |
| [`clear_line`](#clear_line) | fn |  |
| [`clear_screen`](#clear_screen) | fn |  |
| [`clear_to_end_of_screen`](#clear_to_end_of_screen) | fn |  |
| [`show_cursor`](#show_cursor) | fn |  |
| [`hide_cursor`](#hide_cursor) | fn |  |

## Functions

### `move_cursor_down`

```rust
fn move_cursor_down(out: &crate::term::Term, n: usize) -> io::Result<()>
```

### `move_cursor_up`

```rust
fn move_cursor_up(out: &crate::term::Term, n: usize) -> io::Result<()>
```

### `move_cursor_left`

```rust
fn move_cursor_left(out: &crate::term::Term, n: usize) -> io::Result<()>
```

### `move_cursor_right`

```rust
fn move_cursor_right(out: &crate::term::Term, n: usize) -> io::Result<()>
```

### `move_cursor_to`

```rust
fn move_cursor_to(out: &crate::term::Term, x: usize, y: usize) -> io::Result<()>
```

### `clear_chars`

```rust
fn clear_chars(out: &crate::term::Term, n: usize) -> io::Result<()>
```

### `clear_line`

```rust
fn clear_line(out: &crate::term::Term) -> io::Result<()>
```

### `clear_screen`

```rust
fn clear_screen(out: &crate::term::Term) -> io::Result<()>
```

### `clear_to_end_of_screen`

```rust
fn clear_to_end_of_screen(out: &crate::term::Term) -> io::Result<()>
```

### `show_cursor`

```rust
fn show_cursor(out: &crate::term::Term) -> io::Result<()>
```

### `hide_cursor`

```rust
fn hide_cursor(out: &crate::term::Term) -> io::Result<()>
```

