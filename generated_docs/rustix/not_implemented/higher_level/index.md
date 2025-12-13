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
  - [`getpwuid_r`](#getpwuid-r)
  - [`getpwnam_r`](#getpwnam-r)
  - [`gethostbyname`](#gethostbyname)
  - [`execv`](#execv)
  - [`execvp`](#execvp)
  - [`execvpe`](#execvpe)
  - [`wordexp`](#wordexp)
  - [`localtime`](#localtime)
  - [`localtime_r`](#localtime-r)
  - [`gmtime`](#gmtime)
  - [`gmtime_r`](#gmtime-r)
  - [`ctime`](#ctime)
  - [`ctime_r`](#ctime-r)
  - [`asctime`](#asctime)
  - [`asctime_r`](#asctime-r)
  - [`mktime`](#mktime)
  - [`getifaddrs`](#getifaddrs)
  - [`closefrom`](#closefrom)
  - [`login_tty`](#login-tty)
  - [`openpty`](#openpty)
  - [`isatty`](#isatty)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`getpwent`](#getpwent) | fn | See the [module comment](self). |
| [`getpwuid`](#getpwuid) | fn | See the [module comment](self). |
| [`getpwnam`](#getpwnam) | fn | See the [module comment](self). |
| [`getpwuid_r`](#getpwuid-r) | fn | See the [module comment](self). |
| [`getpwnam_r`](#getpwnam-r) | fn | See the [module comment](self). |
| [`gethostbyname`](#gethostbyname) | fn | See the [module comment](self). |
| [`execv`](#execv) | fn | See the [module comment](self). |
| [`execvp`](#execvp) | fn | See the [module comment](self). |
| [`execvpe`](#execvpe) | fn | See the [module comment](self). |
| [`wordexp`](#wordexp) | fn | See the [module comment](self). |
| [`localtime`](#localtime) | fn | See the [module comment](self). |
| [`localtime_r`](#localtime-r) | fn | See the [module comment](self). |
| [`gmtime`](#gmtime) | fn | See the [module comment](self). |
| [`gmtime_r`](#gmtime-r) | fn | See the [module comment](self). |
| [`ctime`](#ctime) | fn | See the [module comment](self). |
| [`ctime_r`](#ctime-r) | fn | See the [module comment](self). |
| [`asctime`](#asctime) | fn | See the [module comment](self). |
| [`asctime_r`](#asctime-r) | fn | See the [module comment](self). |
| [`mktime`](#mktime) | fn | See the [module comment](self). |
| [`getifaddrs`](#getifaddrs) | fn | See the [module comment](self). |
| [`closefrom`](#closefrom) | fn | See [rustix-openpty](https://crates.io/crates/rustix-openpty). |
| [`login_tty`](#login-tty) | fn | See [rustix-openpty](https://crates.io/crates/rustix-openpty). |
| [`openpty`](#openpty) | fn | See [rustix-openpty](https://crates.io/crates/rustix-openpty). |
| [`isatty`](#isatty) | fn | See [`std::io::IsTerminal`]. |

## Functions

### `getpwent`

```rust
fn getpwent()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:226`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L226)*

See the [module comment](self).

### `getpwuid`

```rust
fn getpwuid()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:227`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L227)*

See the [module comment](self).

### `getpwnam`

```rust
fn getpwnam()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:228`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L228)*

See the [module comment](self).

### `getpwuid_r`

```rust
fn getpwuid_r()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:229`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L229)*

See the [module comment](self).

### `getpwnam_r`

```rust
fn getpwnam_r()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:230`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L230)*

See the [module comment](self).

### `gethostbyname`

```rust
fn gethostbyname()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:231`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L231)*

See the [module comment](self).

### `execv`

```rust
fn execv()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:232`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L232)*

See the [module comment](self).

### `execvp`

```rust
fn execvp()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:233`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L233)*

See the [module comment](self).

### `execvpe`

```rust
fn execvpe()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:234`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L234)*

See the [module comment](self).

### `wordexp`

```rust
fn wordexp()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:235`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L235)*

See the [module comment](self).

### `localtime`

```rust
fn localtime()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:236`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L236)*

See the [module comment](self).

### `localtime_r`

```rust
fn localtime_r()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:237`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L237)*

See the [module comment](self).

### `gmtime`

```rust
fn gmtime()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:238`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L238)*

See the [module comment](self).

### `gmtime_r`

```rust
fn gmtime_r()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:239`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L239)*

See the [module comment](self).

### `ctime`

```rust
fn ctime()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:240`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L240)*

See the [module comment](self).

### `ctime_r`

```rust
fn ctime_r()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:241`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L241)*

See the [module comment](self).

### `asctime`

```rust
fn asctime()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:242`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L242)*

See the [module comment](self).

### `asctime_r`

```rust
fn asctime_r()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:243`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L243)*

See the [module comment](self).

### `mktime`

```rust
fn mktime()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:244`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L244)*

See the [module comment](self).

### `getifaddrs`

```rust
fn getifaddrs()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:245`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L245)*

See the [module comment](self).

### `closefrom`

```rust
fn closefrom()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:248-250`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L248-L250)*

See [rustix-openpty](https://crates.io/crates/rustix-openpty).

### `login_tty`

```rust
fn login_tty()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:252-254`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L252-L254)*

See [rustix-openpty](https://crates.io/crates/rustix-openpty).

### `openpty`

```rust
fn openpty()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:256-258`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L256-L258)*

See [rustix-openpty](https://crates.io/crates/rustix-openpty).

### `isatty`

```rust
fn isatty()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:268-270`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L268-L270)*

See [`std::io::IsTerminal`](../../../anstream/stream/index.md).

For Rust < 1.70, see [is-terminal]. For a rustix-based implementation,
see [rustix-is-terminal].




