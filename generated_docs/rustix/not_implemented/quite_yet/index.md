*[rustix](../../index.md) / [not_implemented](../index.md) / [quite_yet](index.md)*

---

# Module `quite_yet`

These functions are not quite yet finished in rustix.

Rustix's codebase includes experimental implementations of these functions,
however they are not yet publicly exposed because their API might need more
work and/or they don't yet have a libc backend implementation yet.

See [#1314] for more information, and please leave comments if there are
specific functions you're interested in.


## Functions

### `_exit`

```rust
fn _exit()
```

See the [module comment](self).

### `_Exit`

```rust
fn _Exit()
```

See the [module comment](self).

### `exit_group`

```rust
fn exit_group()
```

See the [module comment](self).

### `sigpending`

```rust
fn sigpending()
```

See the [module comment](self).

### `sigsuspend`

```rust
fn sigsuspend()
```

See the [module comment](self).

### `execveat`

```rust
fn execveat()
```

See the [module comment](self).

### `execve`

```rust
fn execve()
```

See the [module comment](self).

### `gethostname`

```rust
fn gethostname()
```

For now, use `rustix::process::uname().nodename()` instead.

See also the [module comment](self).

