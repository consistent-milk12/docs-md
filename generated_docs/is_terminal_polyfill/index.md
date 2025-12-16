# Crate `is_terminal_polyfill`

> Polyfill for `is_terminal` stdlib feature for use with older MSRVs

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sealed`](#sealed) | mod |  |
| [`IsTerminal`](#isterminal) | trait | Trait to determine if a descriptor/handle refers to a terminal/tty. |
| [`impl_is_terminal!`](#impl-is-terminal) | macro |  |

## Modules

- [`sealed`](sealed/index.md)

## Traits

### `IsTerminal`

```rust
trait IsTerminal: sealed::Sealed { ... }
```

*Defined in [`is_terminal_polyfill-1.70.2/src/lib.rs:7-23`](../../.source_1765900590/is_terminal_polyfill-1.70.2/src/lib.rs#L7-L23)*

Trait to determine if a descriptor/handle refers to a terminal/tty.

#### Required Methods

- `fn IsTerminal::is_terminal(&self) -> bool`

  Returns `true` if the descriptor/handle refers to a terminal/tty.
  
  On platforms where Rust does not know how to detect a terminal yet, this will return
  `false`. This will also return `false` if an unexpected error occurred, such as from
  passing an invalid file descriptor.
  
  ##### Platform-specific behavior
  
  On Windows, in addition to detecting consoles, this currently uses some heuristics to
  detect older msys/cygwin/mingw pseudo-terminals based on device name: devices with names
  starting with `msys-` or `cygwin-` and ending in `-pty` will be considered terminals.
  Note that this [may change in the future][changes].

#### Implementors

- `std::fs::File`
- `std::io::StderrLock<'_>`
- `std::io::Stderr`
- `std::io::StdinLock<'_>`
- `std::io::Stdin`
- `std::io::StdoutLock<'_>`
- `std::io::Stdout`

## Macros

### `impl_is_terminal!`

*Defined in [`is_terminal_polyfill-1.70.2/src/lib.rs:29-40`](../../.source_1765900590/is_terminal_polyfill-1.70.2/src/lib.rs#L29-L40)*

