*[clap_builder](../../index.md) / [output](../index.md) / [help](index.md)*

---

# Module `help`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`write_help`](#write_help) | fn | Writes the parser help to the wrapped stream. |

## Functions

### `write_help`

```rust
fn write_help(writer: &mut crate::builder::StyledStr, cmd: &crate::builder::Command, usage: &self::usage::Usage<'_>, use_long: bool)
```

*Defined in [`clap_builder-4.5.53/src/output/help.rs:9-39`](../../../../.source_1765210505/clap_builder-4.5.53/src/output/help.rs#L9-L39)*

Writes the parser help to the wrapped stream.

