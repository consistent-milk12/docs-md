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

- [`private`](private/index.md)

## Traits

### `RawStream`

```rust
trait RawStream: std::io::Write + IsTerminal + private::Sealed { ... }
```

*Defined in [`anstream-0.6.21/src/stream.rs:5`](../../../.source_1765210505/anstream-0.6.21/src/stream.rs#L5)*

Required functionality for underlying [`std::io::Write`](../../fs_err/index.md) for adaptation

#### Implementors

- `&mut T`
- `Box<T>`
- `Vec<u8>`
- `dyn std::io::Write + Send + Sync`
- `dyn std::io::Write + Send`
- `dyn std::io::Write`
- `std::fs::File`
- `std::io::StderrLock<'_>`
- `std::io::Stderr`
- `std::io::StdoutLock<'_>`
- `std::io::Stdout`

### `IsTerminal`

```rust
trait IsTerminal: private::Sealed { ... }
```

*Defined in [`anstream-0.6.21/src/stream.rs:37-40`](../../../.source_1765210505/anstream-0.6.21/src/stream.rs#L37-L40)*

Trait to determine if a descriptor/handle refers to a terminal/tty.

#### Required Methods

- `fn is_terminal(&self) -> bool`

  Returns `true` if the descriptor/handle refers to a terminal/tty.

#### Implementors

- `&T`
- `&mut T`
- `Box<T>`
- `Vec<u8>`
- `dyn std::io::Write + Send + Sync`
- `dyn std::io::Write + Send`
- `dyn std::io::Write`
- `std::fs::File`
- `std::io::StderrLock<'_>`
- `std::io::Stderr`
- `std::io::StdoutLock<'_>`
- `std::io::Stdout`

### `AsLockedWrite`

```rust
trait AsLockedWrite: private::Sealed { ... }
```

*Defined in [`anstream-0.6.21/src/stream.rs:135-143`](../../../.source_1765210505/anstream-0.6.21/src/stream.rs#L135-L143)*

Lock a stream

#### Associated Types

- `type Write: 2`

#### Required Methods

- `fn as_locked_write(&mut self) -> <Self as >::Write`

  Lock a stream

#### Implementors

- `&mut T`
- `Box<T>`
- `Vec<u8>`
- `dyn std::io::Write + Send + Sync`
- `dyn std::io::Write + Send`
- `dyn std::io::Write`
- `std::fs::File`
- `std::io::StderrLock<'static>`
- `std::io::Stderr`
- `std::io::StdoutLock<'static>`
- `std::io::Stdout`

