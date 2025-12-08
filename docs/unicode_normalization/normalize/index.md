*[unicode_normalization](../index.md) / [normalize](index.md)*

---

# Module `normalize`

Functions for computing canonical and compatible decompositions for Unicode characters.

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

