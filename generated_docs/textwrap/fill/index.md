*[textwrap](../index.md) / [fill](index.md)*

---

# Module `fill`

Functions for filling text.

## Functions

### `fill`

```rust
fn fill<'a, Opt>(text: &str, width_or_options: Opt) -> String
where
    Opt: Into<crate::Options<'a>>
```

Fill a line of text at a given width.

The result is a [`String`](../../clap_builder/index.md), complete with newlines between each
line. Use [`wrap()`](../index.md) if you need access to the individual lines.

The easiest way to use this function is to pass an integer for
`width_or_options`:

```rust
use textwrap::fill;

assert_eq!(
    fill("Memory safety without garbage collection.", 15),
    "Memory safety\nwithout garbage\ncollection."
);
```

If you need to customize the wrapping, you can pass an [`Options`](../index.md)
instead of an `usize`:

```rust
use textwrap::{fill, Options};

let options = Options::new(15)
    .initial_indent("- ")
    .subsequent_indent("  ");
assert_eq!(
    fill("Memory safety without garbage collection.", &options),
    "- Memory safety\n  without\n  garbage\n  collection."
);
```

### `fill_slow_path`

```rust
fn fill_slow_path(text: &str, options: crate::Options<'_>) -> String
```

Slow path for fill.

This is taken when `text` is longer than `options.width`.

### `fill_inplace`

```rust
fn fill_inplace(text: &mut String, width: usize)
```

Fill `text` in-place without reallocating the input string.

This function works by modifying the input string: some `' '`
characters will be replaced by `'\n'` characters. The rest of the
text remains untouched.

Since we can only replace existing whitespace in the input with
`'\n'` (there is no space for `"\r\n"`), we cannot do hyphenation
nor can we split words longer than the line width. We also need to
use `AsciiSpace` as the word separator since we need `' '`
characters between words in order to replace some of them with a
`'\n'`. Indentation is also ruled out. In other words,
`fill_inplace(width)` behaves as if you had called [`fill()`](../index.md) with
these options:

```rust
use textwrap::{core, LineEnding, Options, WordSplitter, WordSeparator, WrapAlgorithm};
let width = 80;
Options::new(width)
    .break_words(false)
    .line_ending(LineEnding::LF)
    .word_separator(WordSeparator::AsciiSpace)
    .wrap_algorithm(WrapAlgorithm::FirstFit)
    .word_splitter(WordSplitter::NoHyphenation);
```

The wrap algorithm is
[`WrapAlgorithm::FirstFit`](crate::WrapAlgorithm::FirstFit) since
this is the fastest algorithm — and the main reason to use
`fill_inplace` is to get the string broken into newlines as fast
as possible.

A last difference is that (unlike [`fill()`](../index.md)) `fill_inplace` can
leave trailing whitespace on lines. This is because we wrap by
inserting a `'\n'` at the final whitespace in the input string:

```rust
let mut text = String::from("Hello   World!");
textwrap::fill_inplace(&mut text, 10);
assert_eq!(text, "Hello  \nWorld!");
```

If we didn't do this, the word `World!` would end up being
indented. You can avoid this if you make sure that your input text
has no double spaces.

# Performance

In benchmarks, `fill_inplace` is about twice as fast as
[`fill()`](../index.md). Please see the [`linear`
benchmark](https://github.com/mgeisler/textwrap/blob/master/benchmarks/linear.rs)
for details.

