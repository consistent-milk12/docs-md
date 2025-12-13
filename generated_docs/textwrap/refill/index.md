*[textwrap](../index.md) / [refill](index.md)*

---

# Module `refill`

Functionality for unfilling and refilling text.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unfill`](#unfill) | fn | Unpack a paragraph of already-wrapped text. |
| [`refill`](#refill) | fn | Refill a paragraph of wrapped text with a new width. |

## Functions

### `unfill`

```rust
fn unfill(text: &str) -> (String, crate::Options<'_>)
```

*Defined in [`textwrap-0.16.2/src/refill.rs:62-114`](../../../.source_1765633015/textwrap-0.16.2/src/refill.rs#L62-L114)*

Unpack a paragraph of already-wrapped text.

This function attempts to recover the original text from a single
paragraph of wrapped text, such as what [`fill()`](../fill/index.md) would produce.
This means that it turns

```text
textwrap: a small
library for
wrapping text.
```

back into

```text
textwrap: a small library for wrapping text.
```

In addition, it will recognize a common prefix and a common line
ending among the lines.

The prefix of the first line is returned in
`Options::initial_indent` and the prefix (if any) of the the
other lines is returned in `Options::subsequent_indent`.

Line ending is returned in `Options::line_ending`. If line ending
can not be confidently detected (mixed or no line endings in the
input), [`LineEnding::LF`](../index.md) will be returned.

In addition to `' '`, the prefixes can consist of characters used
for unordered lists (`'-'`, `'+'`, and `'*'`) and block quotes
(`'>'`) in Markdown as well as characters often used for inline
comments (`'#'` and `'/'`).

The text must come from a single wrapped paragraph. This means
that there can be no empty lines (`"\n\n"` or `"\r\n\r\n"`) within
the text. It is unspecified what happens if `unfill` is called on
more than one paragraph of text.

# Examples

```rust
use textwrap::{LineEnding, unfill};

let (text, options) = unfill("\
* This is an
  example of
  a list item.
");

assert_eq!(text, "This is an example of a list item.\n");
assert_eq!(options.initial_indent, "* ");
assert_eq!(options.subsequent_indent, "  ");
assert_eq!(options.line_ending, LineEnding::LF);
```

### `refill`

```rust
fn refill<'a, Opt>(filled_text: &str, new_width_or_options: Opt) -> String
where
    Opt: Into<crate::Options<'a>>
```

*Defined in [`textwrap-0.16.2/src/refill.rs:169-188`](../../../.source_1765633015/textwrap-0.16.2/src/refill.rs#L169-L188)*

Refill a paragraph of wrapped text with a new width.

This function will first use [`unfill()`](#unfill) to remove newlines from
the text. Afterwards the text is filled again using [`fill()`](../fill/index.md).

The `new_width_or_options` argument specify the new width and can
specify other options as well — except for
`Options::initial_indent` and `Options::subsequent_indent`,
which are deduced from `filled_text`.

# Examples

```rust
use textwrap::refill;

// Some loosely wrapped text. The "> " prefix is recognized automatically.
let text = "\
> Memory
> safety without garbage
> collection.
";

assert_eq!(refill(text, 20), "\
> Memory safety
> without garbage
> collection.
");

assert_eq!(refill(text, 40), "\
> Memory safety without garbage
> collection.
");

assert_eq!(refill(text, 60), "\
> Memory safety without garbage collection.
");
```

You can also reshape bullet points:

```rust
use textwrap::refill;

let text = "\
- This is my
  list item.
";

assert_eq!(refill(text, 20), "\
- This is my list
  item.
");
```

