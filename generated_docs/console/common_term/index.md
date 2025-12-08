*[console](../index.md) / [common_term](index.md)*

---

# Module `common_term`

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

