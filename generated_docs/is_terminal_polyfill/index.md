# Crate `is_terminal_polyfill`

> Polyfill for `is_terminal` stdlib feature for use with older MSRVs

## Modules

- [`sealed`](sealed/index.md) - 

## Traits

### `IsTerminal`

```rust
trait IsTerminal: sealed::Sealed { ... }
```

Trait to determine if a descriptor/handle refers to a terminal/tty.

#### Required Methods

- `fn is_terminal(self: &Self) -> bool`

  Returns `true` if the descriptor/handle refers to a terminal/tty.

## Macros

### `impl_is_terminal!`

