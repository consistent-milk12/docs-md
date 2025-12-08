*[textwrap](../index.md) / [wrap](index.md)*

---

# Module `wrap`

Functions for wrapping text.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`wrap`](#wrap) | fn | Wrap a line of text at a given width. |
| [`wrap_single_line`](#wrap_single_line) | fn |  |
| [`wrap_single_line_slow_path`](#wrap_single_line_slow_path) | fn | Wrap a single line of text. |

## Functions

### `wrap`

```rust
fn wrap<'a, Opt>(text: &str, width_or_options: Opt) -> Vec<std::borrow::Cow<'_, str>>
where
    Opt: Into<crate::Options<'a>>
```

Wrap a line of text at a given width.

The result is a vector of lines, each line is of type [`Cow<'_,
str>`](Cow), which means that the line will borrow from the input
`&str` if possible. The lines do not have trailing whitespace,
including a final `'\n'`. Please use [`fill()`](crate::fill()) if
you need a [`String`](../../clap_builder/index.md) instead.

The easiest way to use this function is to pass an integer for
`width_or_options`:

```rust
use textwrap::wrap;

let lines = wrap("Memory safety without garbage collection.", 15);
assert_eq!(lines, &[
    "Memory safety",
    "without garbage",
    "collection.",
]);
```

If you need to customize the wrapping, you can pass an [`Options`](../index.md)
instead of an `usize`:

```rust
use textwrap::{wrap, Options};

let options = Options::new(15)
    .initial_indent("- ")
    .subsequent_indent("  ");
let lines = wrap("Memory safety without garbage collection.", &options);
assert_eq!(lines, &[
    "- Memory safety",
    "  without",
    "  garbage",
    "  collection.",
]);
```

# Optimal-Fit Wrapping

By default, `wrap` will try to ensure an even right margin by
finding breaks which avoid short lines. We call this an
“optimal-fit algorithm” since the line breaks are computed by
considering all possible line breaks. The alternative is a
“first-fit algorithm” which simply accumulates words until they no
longer fit on the line.

As an example, using the first-fit algorithm to wrap the famous
Hamlet quote “To be, or not to be: that is the question” in a
narrow column with room for only 10 characters looks like this:

```rust
use textwrap::{WrapAlgorithm::FirstFit, Options, wrap};

let lines = wrap("To be, or not to be: that is the question",
                 Options::new(10).wrap_algorithm(FirstFit));
assert_eq!(lines.join("\n") + "\n", "\
To be, or
not to be:
that is
the
question
");
```

Notice how the second to last line is quite narrow because
“question” was too large to fit? The greedy first-fit algorithm
doesn’t look ahead, so it has no other option than to put
“question” onto its own line.

With the optimal-fit wrapping algorithm, the previous lines are
shortened slightly in order to make the word “is” go into the
second last line:

```rust
#[cfg(feature = "smawk")] {
use textwrap::{Options, WrapAlgorithm, wrap};

let lines = wrap(
    "To be, or not to be: that is the question",
    Options::new(10).wrap_algorithm(WrapAlgorithm::new_optimal_fit())
);
assert_eq!(lines.join("\n") + "\n", "\
To be,
or not to
be: that
is the
question
"); }
```

Please see [`WrapAlgorithm`](crate::WrapAlgorithm) for details on
the choices.

# Examples

The returned iterator yields lines of type `Cow<'_, str>`. If
possible, the wrapped lines will borrow from the input string. As
an example, a hanging indentation, the first line can borrow from
the input, but the subsequent lines become owned strings:

```rust
use std::borrow::Cow::{Borrowed, Owned};
use textwrap::{wrap, Options};

let options = Options::new(15).subsequent_indent("....");
let lines = wrap("Wrapping text all day long.", &options);
let annotated = lines
    .iter()
    .map(|line| match line {
        Borrowed(text) => format!("[Borrowed] {}", text),
        Owned(text) => format!("[Owned]    {}", text),
    })
    .collect::<Vec<_>>();
assert_eq!(
    annotated,
    &[
        "[Borrowed] Wrapping text",
        "[Owned]    ....all day",
        "[Owned]    ....long.",
    ]
);
```

## Leading and Trailing Whitespace

As a rule, leading whitespace (indentation) is preserved and
trailing whitespace is discarded.

In more details, when wrapping words into lines, words are found
by splitting the input text on space characters. One or more
spaces (shown here as “␣”) are attached to the end of each word:

```text
"Foo␣␣␣bar␣baz" -> ["Foo␣␣␣", "bar␣", "baz"]
```

These words are then put into lines. The interword whitespace is
preserved, unless the lines are wrapped so that the `"Foo␣␣␣"`
word falls at the end of a line:

```rust
use textwrap::wrap;

assert_eq!(wrap("Foo   bar baz", 10), vec!["Foo   bar", "baz"]);
assert_eq!(wrap("Foo   bar baz", 8), vec!["Foo", "bar baz"]);
```

Notice how the trailing whitespace is removed in both case: in the
first example, `"bar␣"` becomes `"bar"` and in the second case
`"Foo␣␣␣"` becomes `"Foo"`.

Leading whitespace is preserved when the following word fits on
the first line. To understand this, consider how words are found
in a text with leading spaces:

```text
"␣␣foo␣bar" -> ["␣␣", "foo␣", "bar"]
```

When put into lines, the indentation is preserved if `"foo"` fits
on the first line, otherwise you end up with an empty line:

```rust
use textwrap::wrap;

assert_eq!(wrap("  foo bar", 8), vec!["  foo", "bar"]);
assert_eq!(wrap("  foo bar", 4), vec!["", "foo", "bar"]);
```

### `wrap_single_line`

```rust
fn wrap_single_line<'a>(line: &'a str, options: &crate::Options<'_>, lines: &mut Vec<std::borrow::Cow<'a, str>>)
```

### `wrap_single_line_slow_path`

```rust
fn wrap_single_line_slow_path<'a>(line: &'a str, options: &crate::Options<'_>, lines: &mut Vec<std::borrow::Cow<'a, str>>)
```

Wrap a single line of text.

This is taken when `line` is longer than `options.width`.

