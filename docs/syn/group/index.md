*[syn](../index.md) / [group](index.md)*

---

# Module `group`

## Functions

### `parse_group`

```rust
fn parse_group<'a>(input: &crate::parse::ParseBuffer<'a>) -> crate::error::Result<Group<'a>>
```

### `parse_delimited`

```rust
fn parse_delimited<'a>(input: &crate::parse::ParseBuffer<'a>, delimiter: proc_macro2::Delimiter) -> crate::error::Result<(proc_macro2::extra::DelimSpan, crate::parse::ParseBuffer<'a>)>
```

