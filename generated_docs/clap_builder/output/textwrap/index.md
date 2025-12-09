*[clap_builder](../../index.md) / [output](../index.md) / [textwrap](index.md)*

---

# Module `textwrap`

Fork of `textwrap` crate

Benefits of forking:
- Pull in only what we need rather than relying on the compiler to remove what we don't need
- `LineWrapper` is able to incrementally wrap which will help with `StyledStr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`core`](#core) | mod |  |
| [`wrap`](#wrap) | fn |  |

## Modules

- [`core`](core/index.md)

## Functions

### `wrap`

```rust
fn wrap(content: &str, _hard_width: usize) -> String
```

