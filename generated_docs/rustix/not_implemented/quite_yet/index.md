*[rustix](../../index.md) / [not_implemented](../index.md) / [quite_yet](index.md)*

---

# Module `quite_yet`

These functions are not quite yet finished in rustix.

Rustix's codebase includes experimental implementations of these functions,
however they are not yet publicly exposed because their API might need more
work and/or they don't yet have a libc backend implementation yet.

See [#1314] for more information, and please leave comments if there are
specific functions you're interested in.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`_exit`](#exit) | fn | See the [module comment](self). |
| [`_Exit`](#exit) | fn | See the [module comment](self). |
| [`exit_group`](#exit-group) | fn | See the [module comment](self). |
| [`sigpending`](#sigpending) | fn | See the [module comment](self). |
| [`sigsuspend`](#sigsuspend) | fn | See the [module comment](self). |
| [`execveat`](#execveat) | fn | See the [module comment](self). |
| [`execve`](#execve) | fn | See the [module comment](self). |
| [`gethostname`](#gethostname) | fn | For now, use `rustix::process::uname().nodename()` instead. |

## Functions

### `_exit`

```rust
fn _exit()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:334`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L334)*

See the [module comment](self).

### `_Exit`

```rust
fn _Exit()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:335`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L335)*

See the [module comment](self).

### `exit_group`

```rust
fn exit_group()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:336`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L336)*

See the [module comment](self).

### `sigpending`

```rust
fn sigpending()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:337`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L337)*

See the [module comment](self).

### `sigsuspend`

```rust
fn sigsuspend()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:338`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L338)*

See the [module comment](self).

### `execveat`

```rust
fn execveat()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:339`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L339)*

See the [module comment](self).

### `execve`

```rust
fn execve()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:340`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L340)*

See the [module comment](self).

### `gethostname`

```rust
fn gethostname()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:345-347`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L345-L347)*

For now, use `rustix::process::uname().nodename()` instead.

See also the [module comment](self).

