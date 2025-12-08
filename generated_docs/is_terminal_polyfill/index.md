# Crate `is_terminal_polyfill`

> Polyfill for `is_terminal` stdlib feature for use with older MSRVs

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sealed`](#sealed) | mod |  |
| [`IsTerminal`](#isterminal) | trait | Trait to determine if a descriptor/handle refers to a terminal/tty. |
| [`impl_is_terminal!`](#impl_is_terminal) | macro |  |

## Modules

- [`sealed`](sealed/index.md) - 

## Traits

### `IsTerminal`

```rust
trait IsTerminal: sealed::Sealed { ... }
```

Trait to determine if a descriptor/handle refers to a terminal/tty.

#### Required Methods

- `fn is_terminal(&self) -> bool`

  Returns `true` if the descriptor/handle refers to a terminal/tty.

## Macros

### `impl_is_terminal!`

