*[regex_automata](../../../index.md) / [util](../../index.md) / [look](../index.md) / [is_word_char](index.md)*

---

# Module `is_word_char`

A module that looks for word codepoints using regex-syntax's data tables.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`check`](#check) | fn |  |
| [`fwd`](#fwd) | fn |  |
| [`rev`](#rev) | fn |  |

## Functions

### `check`

```rust
fn check() -> Result<(), super::UnicodeWordBoundaryError>
```

*Defined in [`regex-automata-0.4.13/src/util/look.rs:1577-1579`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/look.rs#L1577-L1579)*

### `fwd`

```rust
fn fwd(haystack: &[u8], at: usize) -> Result<bool, super::UnicodeWordBoundaryError>
```

*Defined in [`regex-automata-0.4.13/src/util/look.rs:1582-1594`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/look.rs#L1582-L1594)*

### `rev`

```rust
fn rev(haystack: &[u8], at: usize) -> Result<bool, super::UnicodeWordBoundaryError>
```

*Defined in [`regex-automata-0.4.13/src/util/look.rs:1597-1609`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/look.rs#L1597-L1609)*

