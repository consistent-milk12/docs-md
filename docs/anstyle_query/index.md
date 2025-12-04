# Crate `anstyle_query`

Low level terminal capability lookups

## Modules

- [`windows`](windows/index.md) - Windows-specific style queries

## Functions

### `clicolor`

```rust
fn clicolor() -> Option<bool>
```

Check [CLICOLOR] status

- When `true`, ANSI colors are supported and should be used when the program isn't piped,
  similar to [`term_supports_color`](#term-supports-color)
- When `false`, don’t output ANSI color escape codes, similar to [`no_color`](#no-color)

See also:
- [terminfo](https://crates.io/crates/terminfo) or [term](https://crates.io/crates/term) for
  checking termcaps
- [termbg](https://crates.io/crates/termbg) for detecting background color

[CLICOLOR]: https://bixense.com/clicolors/

### `clicolor_force`

```rust
fn clicolor_force() -> bool
```

Check [CLICOLOR_FORCE] status

ANSI colors should be enabled no matter what.

[CLICOLOR_FORCE]: https://bixense.com/clicolors/

### `no_color`

```rust
fn no_color() -> bool
```

Check [NO_COLOR] status

When `true`, should prevent the addition of ANSI color.

User-level configuration files and per-instance command-line arguments should override
[NO_COLOR]. A user should be able to export `$NO_COLOR` in their shell configuration file as a
default, but configure a specific program in its configuration file to specifically enable
color.

[NO_COLOR]: https://no-color.org/

### `term_supports_color`

```rust
fn term_supports_color() -> bool
```

Check `TERM` for color support

### `term_supports_ansi_color`

```rust
fn term_supports_ansi_color() -> bool
```

Check `TERM` for ANSI color support

On Windows, you might need to also check `windows::enable_ansi_colors` as ANSI color support
is opt-in, rather than assumed.

### `truecolor`

```rust
fn truecolor() -> bool
```

Check [COLORTERM] for truecolor support

[COLORTERM]: https://github.com/termstandard/colors

### `is_ci`

```rust
fn is_ci() -> bool
```

Report whether this is running in CI

CI is a common environment where, despite being piped, ansi color codes are supported

This is not as exhaustive as you'd find in a crate like `is_ci` but it should work in enough
cases.

