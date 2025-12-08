*[rustix](../../index.md) / [not_implemented](../index.md) / [yet](index.md)*

---

# Module `yet`

These functions are not yet implemented in rustix, but probably could be.

These are functions that users have asked about, and which probably are in
scope for rustix, but are not yet implemented. This module contains an
incomplete list of such functions.

## Contents

- [Functions](#functions)
  - [`tgkill`](#tgkill)
  - [`raise`](#raise)
  - [`sysctl`](#sysctl)
  - [`mq_open`](#mq_open)
  - [`mq_send`](#mq_send)
  - [`mq_unlink`](#mq_unlink)
  - [`recvmmsg`](#recvmmsg)
  - [`cachestat`](#cachestat)
  - [`fanotify_init`](#fanotify_init)
  - [`fanotify_mark`](#fanotify_mark)
  - [`getifaddrs`](#getifaddrs)
  - [`signalfd`](#signalfd)
  - [`mount_setattr`](#mount_setattr)
  - [`extattr_delete_fd`](#extattr_delete_fd)
  - [`extattr_delete_link`](#extattr_delete_link)
  - [`extattr_get_fd`](#extattr_get_fd)
  - [`extattr_get_link`](#extattr_get_link)
  - [`extattr_list_fd`](#extattr_list_fd)
  - [`extattr_list_link`](#extattr_list_link)
  - [`extattr_set_fd`](#extattr_set_fd)
  - [`extattr_set_link`](#extattr_set_link)
  - [`get_mempolicy`](#get_mempolicy)
  - [`mbind`](#mbind)
  - [`set_mempolicy`](#set_mempolicy)
  - [`migrate_pages`](#migrate_pages)
  - [`move_pages`](#move_pages)
  - [`fchmodat2`](#fchmodat2)
  - [`shmat`](#shmat)
  - [`shmdt`](#shmdt)
  - [`shmget`](#shmget)
  - [`shmctl`](#shmctl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`tgkill`](#tgkill) | fn | See the [module comment](self). |
| [`raise`](#raise) | fn | See the [module comment](self). |
| [`sysctl`](#sysctl) | fn | See the [module comment](self). |
| [`mq_open`](#mq_open) | fn | See the [module comment](self). |
| [`mq_send`](#mq_send) | fn | See the [module comment](self). |
| [`mq_unlink`](#mq_unlink) | fn | See the [module comment](self). |
| [`recvmmsg`](#recvmmsg) | fn | See the [module comment](self). |
| [`cachestat`](#cachestat) | fn | See the [module comment](self). |
| [`fanotify_init`](#fanotify_init) | fn | See the [module comment](self). |
| [`fanotify_mark`](#fanotify_mark) | fn | See the [module comment](self). |
| [`getifaddrs`](#getifaddrs) | fn | See the [module comment](self). |
| [`signalfd`](#signalfd) | fn | See the [module comment](self). |
| [`mount_setattr`](#mount_setattr) | fn | See the [module comment](self). |
| [`extattr_delete_fd`](#extattr_delete_fd) | fn | See the [module comment](self). |
| [`extattr_delete_link`](#extattr_delete_link) | fn | See the [module comment](self). |
| [`extattr_get_fd`](#extattr_get_fd) | fn | See the [module comment](self). |
| [`extattr_get_link`](#extattr_get_link) | fn | See the [module comment](self). |
| [`extattr_list_fd`](#extattr_list_fd) | fn | See the [module comment](self). |
| [`extattr_list_link`](#extattr_list_link) | fn | See the [module comment](self). |
| [`extattr_set_fd`](#extattr_set_fd) | fn | See the [module comment](self). |
| [`extattr_set_link`](#extattr_set_link) | fn | See the [module comment](self). |
| [`get_mempolicy`](#get_mempolicy) | fn | See the [module comment](self). |
| [`mbind`](#mbind) | fn | See the [module comment](self). |
| [`set_mempolicy`](#set_mempolicy) | fn | See the [module comment](self). |
| [`migrate_pages`](#migrate_pages) | fn | See the [module comment](self). |
| [`move_pages`](#move_pages) | fn | See the [module comment](self). |
| [`fchmodat2`](#fchmodat2) | fn | See the [module comment](self). |
| [`shmat`](#shmat) | fn | See the [module comment](self). |
| [`shmdt`](#shmdt) | fn | See the [module comment](self). |
| [`shmget`](#shmget) | fn | See the [module comment](self). |
| [`shmctl`](#shmctl) | fn | See the [module comment](self). |

## Functions

### `tgkill`

```rust
fn tgkill()
```

See the [module comment](self).

### `raise`

```rust
fn raise()
```

See the [module comment](self).

### `sysctl`

```rust
fn sysctl()
```

See the [module comment](self).

### `mq_open`

```rust
fn mq_open()
```

See the [module comment](self).

### `mq_send`

```rust
fn mq_send()
```

See the [module comment](self).

### `mq_unlink`

```rust
fn mq_unlink()
```

See the [module comment](self).

### `recvmmsg`

```rust
fn recvmmsg()
```

See the [module comment](self).

### `cachestat`

```rust
fn cachestat()
```

See the [module comment](self).

### `fanotify_init`

```rust
fn fanotify_init()
```

See the [module comment](self).

### `fanotify_mark`

```rust
fn fanotify_mark()
```

See the [module comment](self).

### `getifaddrs`

```rust
fn getifaddrs()
```

See the [module comment](self).

### `signalfd`

```rust
fn signalfd()
```

See the [module comment](self).

### `mount_setattr`

```rust
fn mount_setattr()
```

See the [module comment](self).

### `extattr_delete_fd`

```rust
fn extattr_delete_fd()
```

See the [module comment](self).

### `extattr_delete_link`

```rust
fn extattr_delete_link()
```

See the [module comment](self).

### `extattr_get_fd`

```rust
fn extattr_get_fd()
```

See the [module comment](self).

### `extattr_get_link`

```rust
fn extattr_get_link()
```

See the [module comment](self).

### `extattr_list_fd`

```rust
fn extattr_list_fd()
```

See the [module comment](self).

### `extattr_list_link`

```rust
fn extattr_list_link()
```

See the [module comment](self).

### `extattr_set_fd`

```rust
fn extattr_set_fd()
```

See the [module comment](self).

### `extattr_set_link`

```rust
fn extattr_set_link()
```

See the [module comment](self).

### `get_mempolicy`

```rust
fn get_mempolicy()
```

See the [module comment](self).

### `mbind`

```rust
fn mbind()
```

See the [module comment](self).

### `set_mempolicy`

```rust
fn set_mempolicy()
```

See the [module comment](self).

### `migrate_pages`

```rust
fn migrate_pages()
```

See the [module comment](self).

### `move_pages`

```rust
fn move_pages()
```

See the [module comment](self).

### `fchmodat2`

```rust
fn fchmodat2()
```

See the [module comment](self).

### `shmat`

```rust
fn shmat()
```

See the [module comment](self).

### `shmdt`

```rust
fn shmdt()
```

See the [module comment](self).

### `shmget`

```rust
fn shmget()
```

See the [module comment](self).

### `shmctl`

```rust
fn shmctl()
```

See the [module comment](self).

