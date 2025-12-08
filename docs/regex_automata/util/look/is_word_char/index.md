*[regex_automata](../../../index.md) / [util](../../index.md) / [look](../index.md) / [is_word_char](index.md)*

---

# Module `is_word_char`

A module that looks for word codepoints using regex-syntax's data tables.

## Functions

### `check`

```rust
fn check() -> Result<(), super::UnicodeWordBoundaryError>
```

### `fwd`

```rust
fn fwd(haystack: &[u8], at: usize) -> Result<bool, super::UnicodeWordBoundaryError>
```

### `rev`

```rust
fn rev(haystack: &[u8], at: usize) -> Result<bool, super::UnicodeWordBoundaryError>
```

