*[unicode_normalization](../index.md) / [normalize](index.md)*

---

# Module `normalize`

Functions for computing canonical and compatible decompositions for Unicode characters.

## Contents

- [Functions](#functions)
  - [`decompose_canonical`](#decompose_canonical)
  - [`decompose_compatible`](#decompose_compatible)
  - [`decompose_cjk_compat_variants`](#decompose_cjk_compat_variants)
  - [`decompose`](#decompose)
  - [`compose`](#compose)
  - [`is_hangul_syllable`](#is_hangul_syllable)
  - [`decompose_hangul`](#decompose_hangul)
  - [`hangul_decomposition_length`](#hangul_decomposition_length)
  - [`compose_hangul`](#compose_hangul)
- [Constants](#constants)
  - [`S_BASE`](#s_base)
  - [`L_BASE`](#l_base)
  - [`V_BASE`](#v_base)
  - [`T_BASE`](#t_base)
  - [`L_COUNT`](#l_count)
  - [`V_COUNT`](#v_count)
  - [`T_COUNT`](#t_count)
  - [`N_COUNT`](#n_count)
  - [`S_COUNT`](#s_count)
  - [`S_LAST`](#s_last)
  - [`L_LAST`](#l_last)
  - [`V_LAST`](#v_last)
  - [`T_LAST`](#t_last)
  - [`T_FIRST`](#t_first)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`decompose_canonical`](#decompose_canonical) | fn | Compute canonical Unicode decomposition for character. |
| [`decompose_compatible`](#decompose_compatible) | fn | Compute canonical or compatible Unicode decomposition for character. |
| [`decompose_cjk_compat_variants`](#decompose_cjk_compat_variants) | fn | Compute standard-variation decomposition for character. |
| [`decompose`](#decompose) | fn |  |
| [`compose`](#compose) | fn | Compose two characters into a single character, if possible. |
| [`is_hangul_syllable`](#is_hangul_syllable) | fn |  |
| [`decompose_hangul`](#decompose_hangul) | fn |  |
| [`hangul_decomposition_length`](#hangul_decomposition_length) | fn |  |
| [`compose_hangul`](#compose_hangul) | fn |  |
| [`S_BASE`](#s_base) | const |  |
| [`L_BASE`](#l_base) | const |  |
| [`V_BASE`](#v_base) | const |  |
| [`T_BASE`](#t_base) | const |  |
| [`L_COUNT`](#l_count) | const |  |
| [`V_COUNT`](#v_count) | const |  |
| [`T_COUNT`](#t_count) | const |  |
| [`N_COUNT`](#n_count) | const |  |
| [`S_COUNT`](#s_count) | const |  |
| [`S_LAST`](#s_last) | const |  |
| [`L_LAST`](#l_last) | const |  |
| [`V_LAST`](#v_last) | const |  |
| [`T_LAST`](#t_last) | const |  |
| [`T_FIRST`](#t_first) | const |  |

## Functions

### `decompose_canonical`

```rust
fn decompose_canonical<F>(c: char, emit_char: F)
where
    F: FnMut(char)
```

Compute canonical Unicode decomposition for character.
See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
for more information.

### `decompose_compatible`

```rust
fn decompose_compatible<F: FnMut(char)>(c: char, emit_char: F)
```

Compute canonical or compatible Unicode decomposition for character.
See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
for more information.

### `decompose_cjk_compat_variants`

```rust
fn decompose_cjk_compat_variants<F>(c: char, emit_char: F)
where
    F: FnMut(char)
```

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

### `compose`

```rust
fn compose(a: char, b: char) -> Option<char>
```

Compose two characters into a single character, if possible.
See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
for more information.

### `is_hangul_syllable`

```rust
fn is_hangul_syllable(c: char) -> bool
```

### `decompose_hangul`

```rust
unsafe fn decompose_hangul<F>(s: char, emit_char: F)
where
    F: FnMut(char)
```

### `hangul_decomposition_length`

```rust
fn hangul_decomposition_length(s: char) -> usize
```

### `compose_hangul`

```rust
fn compose_hangul(a: char, b: char) -> Option<char>
```

## Constants

### `S_BASE`

```rust
const S_BASE: u32 = 44_032u32;
```

### `L_BASE`

```rust
const L_BASE: u32 = 4_352u32;
```

### `V_BASE`

```rust
const V_BASE: u32 = 4_449u32;
```

### `T_BASE`

```rust
const T_BASE: u32 = 4_519u32;
```

### `L_COUNT`

```rust
const L_COUNT: u32 = 19u32;
```

### `V_COUNT`

```rust
const V_COUNT: u32 = 21u32;
```

### `T_COUNT`

```rust
const T_COUNT: u32 = 28u32;
```

### `N_COUNT`

```rust
const N_COUNT: u32 = 588u32;
```

### `S_COUNT`

```rust
const S_COUNT: u32 = 11_172u32;
```

### `S_LAST`

```rust
const S_LAST: u32 = 55_203u32;
```

### `L_LAST`

```rust
const L_LAST: u32 = 4_370u32;
```

### `V_LAST`

```rust
const V_LAST: u32 = 4_469u32;
```

### `T_LAST`

```rust
const T_LAST: u32 = 4_546u32;
```

### `T_FIRST`

```rust
const T_FIRST: u32 = 4_520u32;
```

