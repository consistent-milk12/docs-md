*[semver](../index.md) / [display](index.md)*

---

# Module `display`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pad`](#pad) | fn |  |
| [`digits`](#digits) | fn |  |

## Functions

### `pad`

```rust
fn pad(formatter: &mut fmt::Formatter<'_>, do_display: impl FnOnce(&mut fmt::Formatter<'_>) -> fmt::Result, do_len: impl FnOnce() -> usize) -> fmt::Result
```

*Defined in [`semver-1.0.27/src/display.rs:120-155`](../../../.source_1765633015/semver-1.0.27/src/display.rs#L120-L155)*

### `digits`

```rust
fn digits(val: u64) -> usize
```

*Defined in [`semver-1.0.27/src/display.rs:157-163`](../../../.source_1765633015/semver-1.0.27/src/display.rs#L157-L163)*

