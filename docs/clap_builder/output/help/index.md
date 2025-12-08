*[clap_builder](../../index.md) / [output](../index.md) / [help](index.md)*

---

# Module `help`

## Functions

### `write_help`

```rust
fn write_help(writer: &mut crate::builder::StyledStr, cmd: &crate::builder::Command, usage: &self::usage::Usage<'_>, use_long: bool)
```

Writes the parser help to the wrapped stream.

