*[clap_derive](../../index.md) / [utils](../index.md) / [doc_comments](index.md)*

---

# Module `doc_comments`

The preprocessing we apply to doc comments.

#[derive(Parser)] works in terms of "paragraphs". Paragraph is a sequence of
non-empty adjacent lines, delimited by sequences of blank (whitespace only) lines.

## Functions

### `extract_doc_comment`

```rust
fn extract_doc_comment(attrs: &[syn::Attribute]) -> Vec<String>
```

### `format_doc_comment`

```rust
fn format_doc_comment(lines: &[String], preprocess: bool, force_long: bool) -> (Option<String>, Option<String>)
```

### `split_paragraphs`

```rust
fn split_paragraphs(lines: &[String]) -> Vec<String>
```

### `remove_period`

```rust
fn remove_period(s: String) -> String
```

### `is_blank`

```rust
fn is_blank(s: &str) -> bool
```

### `merge_lines`

```rust
fn merge_lines(lines: impl IntoIterator<Item = impl AsRef<str>>) -> String
```

### `parse_markdown`

```rust
fn parse_markdown(lines: &[String]) -> (String, Option<String>)
```

