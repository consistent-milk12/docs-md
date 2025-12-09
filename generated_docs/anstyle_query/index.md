# Crate `anstyle_query`

Low level terminal capability lookups

## Contents

- [Modules](#modules)
  - [`windows`](#windows)
- [Functions](#functions)
  - [`clicolor`](#clicolor)
  - [`clicolor_force`](#clicolor_force)
  - [`no_color`](#no_color)
  - [`term_supports_color`](#term_supports_color)
  - [`term_supports_ansi_color`](#term_supports_ansi_color)
  - [`truecolor`](#truecolor)
  - [`is_ci`](#is_ci)
  - [`non_empty`](#non_empty)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`windows`](#windows) | mod | Windows-specific style queries |
| [`clicolor`](#clicolor) | fn | Check [CLICOLOR] status |
| [`clicolor_force`](#clicolor_force) | fn | Check [CLICOLOR_FORCE] status |
| [`no_color`](#no_color) | fn | Check [NO_COLOR] status |
| [`term_supports_color`](#term_supports_color) | fn | Check `TERM` for color support |
| [`term_supports_ansi_color`](#term_supports_ansi_color) | fn | Check `TERM` for ANSI color support |
| [`truecolor`](#truecolor) | fn | Check [COLORTERM] for truecolor support |
| [`is_ci`](#is_ci) | fn | Report whether this is running in CI |
| [`non_empty`](#non_empty) | fn |  |

## Modules

- [`windows`](windows/index.md) — Windows-specific style queries

## Functions

### `clicolor`

```rust
fn clicolor() -> Option<bool>
```

*Defined in [`anstyle-query-1.1.5/src/lib.rs:23-26`](../../.source_1765210505/anstyle-query-1.1.5/src/lib.rs#L23-L26)*

Check [CLICOLOR] status

- When `true`, ANSI colors are supported and should be used when the program isn't piped,
  similar to [`term_supports_color`](#term-supports-color)
- When `false`, don’t output ANSI color escape codes, similar to [`no_color`](#no-color)

See also:
- [terminfo](https://crates.io/crates/terminfo) or [term](https://crates.io/crates/term) for
  checking termcaps
- [termbg](https://crates.io/crates/termbg) for detecting background color


### `clicolor_force`

```rust
fn clicolor_force() -> bool
```

*Defined in [`anstyle-query-1.1.5/src/lib.rs:34-36`](../../.source_1765210505/anstyle-query-1.1.5/src/lib.rs#L34-L36)*

Check [CLICOLOR_FORCE] status

ANSI colors should be enabled no matter what.


### `no_color`

```rust
fn no_color() -> bool
```

*Defined in [`anstyle-query-1.1.5/src/lib.rs:49-51`](../../.source_1765210505/anstyle-query-1.1.5/src/lib.rs#L49-L51)*

Check [NO_COLOR] status

When `true`, should prevent the addition of ANSI color.

User-level configuration files and per-instance command-line arguments should override
[NO_COLOR]. A user should be able to export `$NO_COLOR` in their shell configuration file as a
default, but configure a specific program in its configuration file to specifically enable
color.


### `term_supports_color`

```rust
fn term_supports_color() -> bool
```

*Defined in [`anstyle-query-1.1.5/src/lib.rs:55-82`](../../.source_1765210505/anstyle-query-1.1.5/src/lib.rs#L55-L82)*

Check `TERM` for color support

### `term_supports_ansi_color`

```rust
fn term_supports_ansi_color() -> bool
```

*Defined in [`anstyle-query-1.1.5/src/lib.rs:89-109`](../../.source_1765210505/anstyle-query-1.1.5/src/lib.rs#L89-L109)*

Check `TERM` for ANSI color support

On Windows, you might need to also check [`windows::enable_ansi_colors`](windows/index.md) as ANSI color support
is opt-in, rather than assumed.

### `truecolor`

```rust
fn truecolor() -> bool
```

*Defined in [`anstyle-query-1.1.5/src/lib.rs:115-119`](../../.source_1765210505/anstyle-query-1.1.5/src/lib.rs#L115-L119)*

Check [COLORTERM] for truecolor support


### `is_ci`

```rust
fn is_ci() -> bool
```

*Defined in [`anstyle-query-1.1.5/src/lib.rs:128-135`](../../.source_1765210505/anstyle-query-1.1.5/src/lib.rs#L128-L135)*

Report whether this is running in CI

CI is a common environment where, despite being piped, ansi color codes are supported

This is not as exhaustive as you'd find in a crate like `is_ci` but it should work in enough
cases.

### `non_empty`

```rust
fn non_empty(var: Option<&std::ffi::OsStr>) -> bool
```

*Defined in [`anstyle-query-1.1.5/src/lib.rs:137-139`](../../.source_1765210505/anstyle-query-1.1.5/src/lib.rs#L137-L139)*

