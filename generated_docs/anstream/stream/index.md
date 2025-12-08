*[anstream](../index.md) / [stream](index.md)*

---

# Module `stream`

Higher-level traits to describe writeable streams

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`RawStream`](#rawstream) | trait | Required functionality for underlying [`std::io::Write`] for adaptation |
| [`IsTerminal`](#isterminal) | trait | Trait to determine if a descriptor/handle refers to a terminal/tty. |
| [`AsLockedWrite`](#aslockedwrite) | trait | Lock a stream |

## Modules

- [`private`](private/index.md) - 

## Traits

### `RawStream`

```rust
trait RawStream: std::io::Write + IsTerminal + private::Sealed { ... }
```

Required functionality for underlying [`std::io::Write`](../../fs_err/index.md) for adaptation

### `IsTerminal`

```rust
trait IsTerminal: private::Sealed { ... }
```

Trait to determine if a descriptor/handle refers to a terminal/tty.

#### Required Methods

- `fn is_terminal(&self) -> bool`

  Returns `true` if the descriptor/handle refers to a terminal/tty.

### `AsLockedWrite`

```rust
trait AsLockedWrite: private::Sealed { ... }
```

Lock a stream

#### Required Methods

- `type Write: 2`

- `fn as_locked_write(&mut self) -> <Self as >::Write`

  Lock a stream

