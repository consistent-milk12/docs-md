*[textwrap](../index.md) / [columns](index.md)*

---

# Module `columns`

Functionality for wrapping text into columns.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`wrap_columns`](#wrap_columns) | fn | Wrap text into columns with a given total width. |

## Functions

### `wrap_columns`

```rust
fn wrap_columns<'a, Opt>(text: &str, columns: usize, total_width_or_options: Opt, left_gap: &str, middle_gap: &str, right_gap: &str) -> Vec<String>
where
    Opt: Into<crate::Options<'a>>
```

*Defined in [`textwrap-0.16.2/src/columns.rs:63-114`](../../../.source_1765210505/textwrap-0.16.2/src/columns.rs#L63-L114)*

Wrap text into columns with a given total width.

The `left_gap`, `middle_gap` and `right_gap` arguments specify the
strings to insert before, between, and after the columns. The
total width of all columns and all gaps is specified using the
`total_width_or_options` argument. This argument can simply be an
integer if you want to use default settings when wrapping, or it
can be a [`Options`](../options/index.md) value if you want to customize the wrapping.

If the columns are narrow, it is recommended to set
`Options::break_words` to `true` to prevent words from
protruding into the margins.

The per-column width is computed like this:

```rust
let (left_gap, middle_gap, right_gap) = ("", "", "");
let columns = 2;
let options = textwrap::Options::new(80);
let inner_width = options.width
    - textwrap::core::display_width(left_gap)
    - textwrap::core::display_width(right_gap)
    - textwrap::core::display_width(middle_gap) * (columns - 1);
let column_width = inner_width / columns;
```

The `text` is wrapped using [`wrap()`](../wrap/index.md) and the given `options`
argument, but the width is overwritten to the computed
`column_width`.

# Panics

Panics if `columns` is zero.

# Examples

```rust
use textwrap::wrap_columns;

let text = "\
This is an example text, which is wrapped into three columns. \
Notice how the final column can be shorter than the others.";

#[cfg(feature = "smawk")]
assert_eq!(wrap_columns(text, 3, 50, "| ", " | ", " |"),
           vec!["| This is       | into three    | column can be  |",
                "| an example    | columns.      | shorter than   |",
                "| text, which   | Notice how    | the others.    |",
                "| is wrapped    | the final     |                |"]);

// Without the `smawk` feature, the middle column is a little more uneven:
#[cfg(not(feature = "smawk"))]
assert_eq!(wrap_columns(text, 3, 50, "| ", " | ", " |"),
           vec!["| This is an    | three         | column can be  |",
                "| example text, | columns.      | shorter than   |",
                "| which is      | Notice how    | the others.    |",
                "| wrapped into  | the final     |                |"]);

