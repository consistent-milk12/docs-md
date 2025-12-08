*[rustix](../../index.md) / [not_implemented](../index.md) / [higher_level](index.md)*

---

# Module `higher_level`

Functions which provide higher-level functionality are out of scope for
rustix.

These functions are provided by typical libc implementations, but are
higher-level than the simple syscall-like functions that rustix focuses on.
They could be implemented as a separate library built on top of rustix,
rather than being part of rustix itself. This module contains an incomplete
list of such functions.

## Contents

- [Functions](#functions)
  - [`getpwent`](#getpwent)
  - [`getpwuid`](#getpwuid)
  - [`getpwnam`](#getpwnam)
  - [`getpwuid_r`](#getpwuid_r)
  - [`getpwnam_r`](#getpwnam_r)
  - [`gethostbyname`](#gethostbyname)
  - [`execv`](#execv)
  - [`execvp`](#execvp)
  - [`execvpe`](#execvpe)
  - [`wordexp`](#wordexp)
  - [`localtime`](#localtime)
  - [`localtime_r`](#localtime_r)
  - [`gmtime`](#gmtime)
  - [`gmtime_r`](#gmtime_r)
  - [`ctime`](#ctime)
  - [`ctime_r`](#ctime_r)
  - [`asctime`](#asctime)
  - [`asctime_r`](#asctime_r)
  - [`mktime`](#mktime)
  - [`getifaddrs`](#getifaddrs)
  - [`closefrom`](#closefrom)
  - [`login_tty`](#login_tty)
  - [`openpty`](#openpty)
  - [`isatty`](#isatty)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`getpwent`](#getpwent) | fn | See the [module comment](self). |
| [`getpwuid`](#getpwuid) | fn | See the [module comment](self). |
| [`getpwnam`](#getpwnam) | fn | See the [module comment](self). |
| [`getpwuid_r`](#getpwuid_r) | fn | See the [module comment](self). |
| [`getpwnam_r`](#getpwnam_r) | fn | See the [module comment](self). |
| [`gethostbyname`](#gethostbyname) | fn | See the [module comment](self). |
| [`execv`](#execv) | fn | See the [module comment](self). |
| [`execvp`](#execvp) | fn | See the [module comment](self). |
| [`execvpe`](#execvpe) | fn | See the [module comment](self). |
| [`wordexp`](#wordexp) | fn | See the [module comment](self). |
| [`localtime`](#localtime) | fn | See the [module comment](self). |
| [`localtime_r`](#localtime_r) | fn | See the [module comment](self). |
| [`gmtime`](#gmtime) | fn | See the [module comment](self). |
| [`gmtime_r`](#gmtime_r) | fn | See the [module comment](self). |
| [`ctime`](#ctime) | fn | See the [module comment](self). |
| [`ctime_r`](#ctime_r) | fn | See the [module comment](self). |
| [`asctime`](#asctime) | fn | See the [module comment](self). |
| [`asctime_r`](#asctime_r) | fn | See the [module comment](self). |
| [`mktime`](#mktime) | fn | See the [module comment](self). |
| [`getifaddrs`](#getifaddrs) | fn | See the [module comment](self). |
| [`closefrom`](#closefrom) | fn | See [rustix-openpty](https://crates.io/crates/rustix-openpty). |
| [`login_tty`](#login_tty) | fn | See [rustix-openpty](https://crates.io/crates/rustix-openpty). |
| [`openpty`](#openpty) | fn | See [rustix-openpty](https://crates.io/crates/rustix-openpty). |
| [`isatty`](#isatty) | fn | See [`std::io::IsTerminal`]. |

## Functions

### `getpwent`

```rust
fn getpwent()
```

See the [module comment](self).

### `getpwuid`

```rust
fn getpwuid()
```

See the [module comment](self).

### `getpwnam`

```rust
fn getpwnam()
```

See the [module comment](self).

### `getpwuid_r`

```rust
fn getpwuid_r()
```

See the [module comment](self).

### `getpwnam_r`

```rust
fn getpwnam_r()
```

See the [module comment](self).

### `gethostbyname`

```rust
fn gethostbyname()
```

See the [module comment](self).

### `execv`

```rust
fn execv()
```

See the [module comment](self).

### `execvp`

```rust
fn execvp()
```

See the [module comment](self).

### `execvpe`

```rust
fn execvpe()
```

See the [module comment](self).

### `wordexp`

```rust
fn wordexp()
```

See the [module comment](self).

### `localtime`

```rust
fn localtime()
```

See the [module comment](self).

### `localtime_r`

```rust
fn localtime_r()
```

See the [module comment](self).

### `gmtime`

```rust
fn gmtime()
```

See the [module comment](self).

### `gmtime_r`

```rust
fn gmtime_r()
```

See the [module comment](self).

### `ctime`

```rust
fn ctime()
```

See the [module comment](self).

### `ctime_r`

```rust
fn ctime_r()
```

See the [module comment](self).

### `asctime`

```rust
fn asctime()
```

See the [module comment](self).

### `asctime_r`

```rust
fn asctime_r()
```

See the [module comment](self).

### `mktime`

```rust
fn mktime()
```

See the [module comment](self).

### `getifaddrs`

```rust
fn getifaddrs()
```

See the [module comment](self).

### `closefrom`

```rust
fn closefrom()
```

See [rustix-openpty](https://crates.io/crates/rustix-openpty).

### `login_tty`

```rust
fn login_tty()
```

See [rustix-openpty](https://crates.io/crates/rustix-openpty).

### `openpty`

```rust
fn openpty()
```

See [rustix-openpty](https://crates.io/crates/rustix-openpty).

### `isatty`

```rust
fn isatty()
```

See [`std::io::IsTerminal`](../../../anstream/stream/index.md).

For Rust < 1.70, see [is-terminal]. For a rustix-based implementation,
see [rustix-is-terminal].




