*[unicode_normalization](../index.md) / [normalize](index.md)*

---

# Module `normalize`

Functions for computing canonical and compatible decompositions for Unicode characters.

## Contents

- [Functions](#functions)
  - [`decompose_canonical`](#decompose-canonical)
  - [`decompose_compatible`](#decompose-compatible)
  - [`decompose_cjk_compat_variants`](#decompose-cjk-compat-variants)
  - [`decompose`](#decompose)
  - [`compose`](#compose)
  - [`is_hangul_syllable`](#is-hangul-syllable)
  - [`decompose_hangul`](#decompose-hangul)
  - [`hangul_decomposition_length`](#hangul-decomposition-length)
  - [`compose_hangul`](#compose-hangul)
- [Constants](#constants)
  - [`S_BASE`](#s-base)
  - [`L_BASE`](#l-base)
  - [`V_BASE`](#v-base)
  - [`T_BASE`](#t-base)
  - [`L_COUNT`](#l-count)
  - [`V_COUNT`](#v-count)
  - [`T_COUNT`](#t-count)
  - [`N_COUNT`](#n-count)
  - [`S_COUNT`](#s-count)
  - [`S_LAST`](#s-last)
  - [`L_LAST`](#l-last)
  - [`V_LAST`](#v-last)
  - [`T_LAST`](#t-last)
  - [`T_FIRST`](#t-first)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`decompose_canonical`](#decompose-canonical) | fn | Compute canonical Unicode decomposition for character. |
| [`decompose_compatible`](#decompose-compatible) | fn | Compute canonical or compatible Unicode decomposition for character. |
| [`decompose_cjk_compat_variants`](#decompose-cjk-compat-variants) | fn | Compute standard-variation decomposition for character. |
| [`decompose`](#decompose) | fn |  |
| [`compose`](#compose) | fn | Compose two characters into a single character, if possible. |
| [`is_hangul_syllable`](#is-hangul-syllable) | fn |  |
| [`decompose_hangul`](#decompose-hangul) | fn |  |
| [`hangul_decomposition_length`](#hangul-decomposition-length) | fn |  |
| [`compose_hangul`](#compose-hangul) | fn |  |
| [`S_BASE`](#s-base) | const |  |
| [`L_BASE`](#l-base) | const |  |
| [`V_BASE`](#v-base) | const |  |
| [`T_BASE`](#t-base) | const |  |
| [`L_COUNT`](#l-count) | const |  |
| [`V_COUNT`](#v-count) | const |  |
| [`T_COUNT`](#t-count) | const |  |
| [`N_COUNT`](#n-count) | const |  |
| [`S_COUNT`](#s-count) | const |  |
| [`S_LAST`](#s-last) | const |  |
| [`L_LAST`](#l-last) | const |  |
| [`V_LAST`](#v-last) | const |  |
| [`T_LAST`](#t-last) | const |  |
| [`T_FIRST`](#t-first) | const |  |

## Functions

### `decompose_canonical`

```rust
fn decompose_canonical<F>(c: char, emit_char: F)
where
    F: FnMut(char)
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:23-28`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L23-L28)*

Compute canonical Unicode decomposition for character.
See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
for more information.

### `decompose_compatible`

```rust
fn decompose_compatible<F: FnMut(char)>(c: char, emit_char: F)
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:34-38`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L34-L38)*

Compute canonical or compatible Unicode decomposition for character.
See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
for more information.

### `decompose_cjk_compat_variants`

```rust
fn decompose_cjk_compat_variants<F>(c: char, emit_char: F)
where
    F: FnMut(char)
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:51-72`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L51-L72)*

Compute standard-variation decomposition for character.

[Standardized Variation Sequences] are used instead of the standard canonical
decompositions, notably for CJK codepoints with singleton canonical decompositions,
to avoid losing information. See the [Unicode Variation Sequence FAQ] and the
"Other Enhancements" section of the [Unicode 6.3 Release Summary] for more information.




### `decompose`

```rust
fn decompose<D, F>(c: char, decompose_char: D, emit_char: F)
where
    D: Fn(char) -> Option<&'static [char]>,
    F: FnMut(char)
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:76-105`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L76-L105)*

### `compose`

```rust
fn compose(a: char, b: char) -> Option<char>
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:110-112`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L110-L112)*

Compose two characters into a single character, if possible.
See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
for more information.

### `is_hangul_syllable`

```rust
fn is_hangul_syllable(c: char) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:136-139`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L136-L139)*

### `decompose_hangul`

```rust
unsafe fn decompose_hangul<F>(s: char, emit_char: F)
where
    F: FnMut(char)
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:145-169`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L145-L169)*

### `hangul_decomposition_length`

```rust
fn hangul_decomposition_length(s: char) -> usize
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:172-180`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L172-L180)*

### `compose_hangul`

```rust
fn compose_hangul(a: char, b: char) -> Option<char>
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:186-212`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L186-L212)*

## Constants

### `S_BASE`
```rust
const S_BASE: u32 = 44_032u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:116`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L116)*

### `L_BASE`
```rust
const L_BASE: u32 = 4_352u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:117`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L117)*

### `V_BASE`
```rust
const V_BASE: u32 = 4_449u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:118`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L118)*

### `T_BASE`
```rust
const T_BASE: u32 = 4_519u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:119`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L119)*

### `L_COUNT`
```rust
const L_COUNT: u32 = 19u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:120`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L120)*

### `V_COUNT`
```rust
const V_COUNT: u32 = 21u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:121`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L121)*

### `T_COUNT`
```rust
const T_COUNT: u32 = 28u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:122`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L122)*

### `N_COUNT`
```rust
const N_COUNT: u32 = 588u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:123`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L123)*

### `S_COUNT`
```rust
const S_COUNT: u32 = 11_172u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:124`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L124)*

### `S_LAST`
```rust
const S_LAST: u32 = 55_203u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:126`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L126)*

### `L_LAST`
```rust
const L_LAST: u32 = 4_370u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:127`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L127)*

### `V_LAST`
```rust
const V_LAST: u32 = 4_469u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:128`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L128)*

### `T_LAST`
```rust
const T_LAST: u32 = 4_546u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:129`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L129)*

### `T_FIRST`
```rust
const T_FIRST: u32 = 4_520u32;
```

*Defined in [`unicode-normalization-0.1.25/src/normalize.rs:133`](../../../.source_1765633015/unicode-normalization-0.1.25/src/normalize.rs#L133)*

