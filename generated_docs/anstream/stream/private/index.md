*[anstream](../../index.md) / [stream](../index.md) / [private](index.md)*

---

# Module `private`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Sealed`](#sealed) | trait |  |

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

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

