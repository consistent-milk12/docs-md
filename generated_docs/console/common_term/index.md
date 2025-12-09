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

*Defined in [`console-0.16.1/src/common_term.rs:5-11`](../../../.source_1765210505/console-0.16.1/src/common_term.rs#L5-L11)*

### `move_cursor_up`

```rust
fn move_cursor_up(out: &crate::term::Term, n: usize) -> io::Result<()>
```

*Defined in [`console-0.16.1/src/common_term.rs:13-19`](../../../.source_1765210505/console-0.16.1/src/common_term.rs#L13-L19)*

### `move_cursor_left`

```rust
fn move_cursor_left(out: &crate::term::Term, n: usize) -> io::Result<()>
```

*Defined in [`console-0.16.1/src/common_term.rs:20-26`](../../../.source_1765210505/console-0.16.1/src/common_term.rs#L20-L26)*

### `move_cursor_right`

```rust
fn move_cursor_right(out: &crate::term::Term, n: usize) -> io::Result<()>
```

*Defined in [`console-0.16.1/src/common_term.rs:28-34`](../../../.source_1765210505/console-0.16.1/src/common_term.rs#L28-L34)*

### `move_cursor_to`

```rust
fn move_cursor_to(out: &crate::term::Term, x: usize, y: usize) -> io::Result<()>
```

*Defined in [`console-0.16.1/src/common_term.rs:37-39`](../../../.source_1765210505/console-0.16.1/src/common_term.rs#L37-L39)*

### `clear_chars`

```rust
fn clear_chars(out: &crate::term::Term, n: usize) -> io::Result<()>
```

*Defined in [`console-0.16.1/src/common_term.rs:41-47`](../../../.source_1765210505/console-0.16.1/src/common_term.rs#L41-L47)*

### `clear_line`

```rust
fn clear_line(out: &crate::term::Term) -> io::Result<()>
```

*Defined in [`console-0.16.1/src/common_term.rs:50-52`](../../../.source_1765210505/console-0.16.1/src/common_term.rs#L50-L52)*

### `clear_screen`

```rust
fn clear_screen(out: &crate::term::Term) -> io::Result<()>
```

*Defined in [`console-0.16.1/src/common_term.rs:55-57`](../../../.source_1765210505/console-0.16.1/src/common_term.rs#L55-L57)*

### `clear_to_end_of_screen`

```rust
fn clear_to_end_of_screen(out: &crate::term::Term) -> io::Result<()>
```

*Defined in [`console-0.16.1/src/common_term.rs:60-62`](../../../.source_1765210505/console-0.16.1/src/common_term.rs#L60-L62)*

### `show_cursor`

```rust
fn show_cursor(out: &crate::term::Term) -> io::Result<()>
```

*Defined in [`console-0.16.1/src/common_term.rs:65-67`](../../../.source_1765210505/console-0.16.1/src/common_term.rs#L65-L67)*

### `hide_cursor`

```rust
fn hide_cursor(out: &crate::term::Term) -> io::Result<()>
```

*Defined in [`console-0.16.1/src/common_term.rs:70-72`](../../../.source_1765210505/console-0.16.1/src/common_term.rs#L70-L72)*

