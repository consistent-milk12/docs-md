*[clap_derive](../../index.md) / [utils](../index.md) / [doc_comments](index.md)*

---

# Module `doc_comments`

The preprocessing we apply to doc comments.

#[derive(Parser)] works in terms of "paragraphs". Paragraph is a sequence of
non-empty adjacent lines, delimited by sequences of blank (whitespace only) lines.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`extract_doc_comment`](#extract_doc_comment) | fn |  |
| [`format_doc_comment`](#format_doc_comment) | fn |  |
| [`split_paragraphs`](#split_paragraphs) | fn |  |
| [`remove_period`](#remove_period) | fn |  |
| [`is_blank`](#is_blank) | fn |  |
| [`merge_lines`](#merge_lines) | fn |  |
| [`parse_markdown`](#parse_markdown) | fn |  |

## Functions

### `extract_doc_comment`

```rust
fn extract_doc_comment(attrs: &[syn::Attribute]) -> Vec<String>
```

*Defined in [`clap_derive-4.5.49/src/utils/doc_comments.rs:9-51`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/doc_comments.rs#L9-L51)*

### `format_doc_comment`

```rust
fn format_doc_comment(lines: &[String], preprocess: bool, force_long: bool) -> (Option<String>, Option<String>)
```

*Defined in [`clap_derive-4.5.49/src/utils/doc_comments.rs:53-74`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/doc_comments.rs#L53-L74)*

### `split_paragraphs`

```rust
fn split_paragraphs(lines: &[String]) -> Vec<String>
```

*Defined in [`clap_derive-4.5.49/src/utils/doc_comments.rs:77-100`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/doc_comments.rs#L77-L100)*

### `remove_period`

```rust
fn remove_period(s: String) -> String
```

*Defined in [`clap_derive-4.5.49/src/utils/doc_comments.rs:102-107`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/doc_comments.rs#L102-L107)*

### `is_blank`

```rust
fn is_blank(s: &str) -> bool
```

*Defined in [`clap_derive-4.5.49/src/utils/doc_comments.rs:109-111`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/doc_comments.rs#L109-L111)*

### `merge_lines`

```rust
fn merge_lines(lines: impl IntoIterator<Item = impl AsRef<str>>) -> String
```

*Defined in [`clap_derive-4.5.49/src/utils/doc_comments.rs:114-120`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/doc_comments.rs#L114-L120)*

### `parse_markdown`

```rust
fn parse_markdown(lines: &[String]) -> (String, Option<String>)
```

*Defined in [`clap_derive-4.5.49/src/utils/doc_comments.rs:123-133`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/doc_comments.rs#L123-L133)*

