*[unicode_normalization](../index.md) / [char](index.md)*

---

# Module `char`

Methods for composing and decomposing characters.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`compose`](#compose) | fn |  |
| [`decompose_canonical`](#decompose-canonical) | fn |  |
| [`decompose_cjk_compat_variants`](#decompose-cjk-compat-variants) | fn |  |
| [`decompose_compatible`](#decompose-compatible) | fn |  |
| [`canonical_combining_class`](#canonical-combining-class) | fn |  |
| [`is_combining_mark`](#is-combining-mark) | fn |  |
| [`is_public_assigned`](#is-public-assigned) | fn | Return whether the given character is assigned (`General_Category` != `Unassigned`) and not Private-Use (`General_Category` != `Private_Use`), in the supported version of Unicode. |

## Functions

### `compose`

```rust
fn compose(a: char, b: char) -> Option<char>
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:110-112`](../../../.source_1765210505/unicode-normalization-0.1.25/src/normalize.rs#L110-L112)*

Compose two characters into a single character, if possible.
See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
for more information.

### `decompose_canonical`

```rust
fn decompose_canonical<F>(c: char, emit_char: F)
where
    F: FnMut(char)
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:23-28`](../../../.source_1765210505/unicode-normalization-0.1.25/src/normalize.rs#L23-L28)*

Compute canonical Unicode decomposition for character.
See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
for more information.

### `decompose_cjk_compat_variants`

```rust
fn decompose_cjk_compat_variants<F>(c: char, emit_char: F)
where
    F: FnMut(char)
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:51-72`](../../../.source_1765210505/unicode-normalization-0.1.25/src/normalize.rs#L51-L72)*

Compute standard-variation decomposition for character.

[Standardized Variation Sequences] are used instead of the standard canonical
decompositions, notably for CJK codepoints with singleton canonical decompositions,
to avoid losing information. See the [Unicode Variation Sequence FAQ] and the
"Other Enhancements" section of the [Unicode 6.3 Release Summary] for more information.




### `decompose_compatible`

```rust
fn decompose_compatible<F: FnMut(char)>(c: char, emit_char: F)
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:34-38`](../../../.source_1765210505/unicode-normalization-0.1.25/src/normalize.rs#L34-L38)*

Compute canonical or compatible Unicode decomposition for character.
See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
for more information.

### `canonical_combining_class`

```rust
fn canonical_combining_class(c: char) -> u8
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:19-28`](../../../.source_1765210505/unicode-normalization-0.1.25/src/lookups.rs#L19-L28)*

Look up the canonical combining class for a codepoint.

The value returned is as defined in the Unicode Character Database.

### `is_combining_mark`

```rust
fn is_combining_mark(c: char) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:82-91`](../../../.source_1765210505/unicode-normalization-0.1.25/src/lookups.rs#L82-L91)*

Return whether the given character is a combining mark (`General_Category=Mark`)

### `is_public_assigned`

```rust
fn is_public_assigned(c: char) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:20435-21172`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L20435-L21172)*

