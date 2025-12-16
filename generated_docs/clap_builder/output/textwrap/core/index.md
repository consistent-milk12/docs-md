*[clap_builder](../../../index.md) / [output](../../index.md) / [textwrap](../index.md) / [core](index.md)*

---

# Module `core`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`display_width`](#display-width) | fn | Compute the display width of `text` |
| [`ch_width`](#ch-width) | fn |  |

## Functions

### `display_width`

```rust
fn display_width(text: &str) -> usize
```

*Defined in [`clap_builder-4.5.53/src/output/textwrap/core.rs:55-74`](../../../../../.source_1765900590/clap_builder-4.5.53/src/output/textwrap/core.rs#L55-L74)*

Compute the display width of `text`

# Examples

**Note:** When the `unicode` Cargo feature is disabled, all characters are presumed to take up
1 width.  With the feature enabled, function will correctly deal with [combining characters] in
their decomposed form (see [Unicode equivalence]).

An example of a decomposed character is “é”, which can be decomposed into: “e” followed by a
combining acute accent: “◌́”.  Without the `unicode` Cargo feature, every `char` has a width of
1. This includes the combining accent:

## Emojis and CJK Characters

Characters such as emojis and [CJK characters] used in the
Chinese, Japanese, and Korean languages are seen as double-width,
even if the `unicode-width` feature is disabled:

# Limitations

The displayed width of a string cannot always be computed from the
string alone. This is because the width depends on the rendering
engine used. This is particularly visible with [emoji modifier
sequences] where a base emoji is modified with, e.g., skin tone or
hair color modifiers. It is up to the rendering engine to detect
this and to produce a suitable emoji.

A simple example is “❤️”, which consists of “❤” (U+2764: Black
Heart Symbol) followed by U+FE0F (Variation Selector-16). By
itself, “❤” is a black heart, but if you follow it with the
variant selector, you may get a wider red heart.

A more complex example would be “👨‍🦰” which should depict a man
with red hair. Here the computed width is too large — and the
width differs depending on the use of the `unicode-width` feature:

This happens because the grapheme consists of three code points:
“👨” (U+1F468: Man), Zero Width Joiner (U+200D), and “🦰”
(U+1F9B0: Red Hair). You can see them above in the test. With
`unicode-width` enabled, the ZWJ is correctly seen as having zero
width, without it is counted as a double-width character.

## Terminal Support

Modern browsers typically do a great job at combining characters
as shown above, but terminals often struggle more. As an example,
Gnome Terminal version 3.38.1, shows “❤️” as a big red heart, but
shows "👨‍🦰" as “👨🦰”.





### `ch_width`

```rust
fn ch_width(_: char) -> usize
```

*Defined in [`clap_builder-4.5.53/src/output/textwrap/core.rs:82-84`](../../../../../.source_1765900590/clap_builder-4.5.53/src/output/textwrap/core.rs#L82-L84)*

