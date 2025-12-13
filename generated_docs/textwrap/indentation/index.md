*[textwrap](../index.md) / [indentation](index.md)*

---

# Module `indentation`

Functions related to adding and removing indentation from lines of
text.

The functions here can be used to uniformly indent or dedent
(unindent) word wrapped lines of text.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`indent`](#indent) | fn | Indent each line by the given prefix. |
| [`dedent`](#dedent) | fn | Removes common leading whitespace from each line. |

## Functions

### `indent`

```rust
fn indent(s: &str, prefix: &str) -> String
```

*Defined in [`textwrap-0.16.2/src/indentation.rs:52-75`](../../../.source_1765633015/textwrap-0.16.2/src/indentation.rs#L52-L75)*

Indent each line by the given prefix.

# Examples

```rust
use textwrap::indent;

assert_eq!(indent("First line.\nSecond line.\n", "  "),
           "  First line.\n  Second line.\n");
```

When indenting, trailing whitespace is stripped from the prefix.
This means that empty lines remain empty afterwards:

```rust
use textwrap::indent;

assert_eq!(indent("First line.\n\n\nSecond line.\n", "  "),
           "  First line.\n\n\n  Second line.\n");
```

Notice how `"\n\n\n"` remained as `"\n\n\n"`.

This feature is useful when you want to indent text and have a
space between your prefix and the text. In this case, you _don't_
want a trailing space on empty lines:

```rust
use textwrap::indent;

assert_eq!(indent("foo = 123\n\nprint(foo)\n", "# "),
           "# foo = 123\n#\n# print(foo)\n");
```

Notice how `"\n\n"` became `"\n#\n"` instead of `"\n# \n"` which
would have trailing whitespace.

Leading and trailing whitespace coming from the text itself is
kept unchanged:

```rust
use textwrap::indent;

assert_eq!(indent(" \t  Foo   ", "->"), "-> \t  Foo   ");
```

### `dedent`

```rust
fn dedent(s: &str) -> String
```

*Defined in [`textwrap-0.16.2/src/indentation.rs:95-150`](../../../.source_1765633015/textwrap-0.16.2/src/indentation.rs#L95-L150)*

Removes common leading whitespace from each line.

This function will look at each non-empty line and determine the
maximum amount of whitespace that can be removed from all lines:

```rust
use textwrap::dedent;

assert_eq!(dedent("
    1st line
      2nd line
    3rd line
"), "
1st line
  2nd line
3rd line
");
```

