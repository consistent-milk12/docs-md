*[unicode_normalization](../index.md) / [quick_check](index.md)*

---

# Module `quick_check`

## Enums

### `IsNormalized`

```rust
enum IsNormalized {
    Yes,
    No,
    Maybe,
}
```

QuickCheck quickly determines if a string is normalized, it can return
`Maybe`

The QuickCheck algorithm can quickly determine if a text is or isn't
normalized without any allocations in many cases, but it has to be able to
return `Maybe` when a full decomposition and recomposition is necessary.

#### Variants

- **`Yes`**

  The text is definitely normalized.

- **`No`**

  The text is definitely not normalized.

- **`Maybe`**

  The text may be normalized.

#### Trait Implementations

##### `impl Debug for IsNormalized`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for IsNormalized`

##### `impl PartialEq for IsNormalized`

- `fn eq(self: &Self, other: &IsNormalized) -> bool` — [`IsNormalized`](#isnormalized)

##### `impl StructuralPartialEq for IsNormalized`

## Functions

### `quick_check`

```rust
fn quick_check<F, I>(s: I, is_allowed: F, stream_safe: bool) -> IsNormalized
where
    I: Iterator<Item = char>,
    F: Fn(char) -> IsNormalized
```

### `is_nfc_quick`

```rust
fn is_nfc_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

Quickly check if a string is in NFC, potentially returning
`IsNormalized::Maybe` if further checks are necessary.  In this case a check
like `s.chars().nfc().eq(s.chars())` should suffice.

### `is_nfkc_quick`

```rust
fn is_nfkc_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

Quickly check if a string is in NFKC.

### `is_nfd_quick`

```rust
fn is_nfd_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

Quickly check if a string is in NFD.

### `is_nfkd_quick`

```rust
fn is_nfkd_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

Quickly check if a string is in NFKD.

### `is_nfc_stream_safe_quick`

```rust
fn is_nfc_stream_safe_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

Quickly check if a string is Stream-Safe NFC.

### `is_nfd_stream_safe_quick`

```rust
fn is_nfd_stream_safe_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

Quickly check if a string is Stream-Safe NFD.

### `is_nfc`

```rust
fn is_nfc(s: &str) -> bool
```

Authoritatively check if a string is in NFC.

### `is_nfkc`

```rust
fn is_nfkc(s: &str) -> bool
```

Authoritatively check if a string is in NFKC.

### `is_nfd`

```rust
fn is_nfd(s: &str) -> bool
```

Authoritatively check if a string is in NFD.

### `is_nfkd`

```rust
fn is_nfkd(s: &str) -> bool
```

Authoritatively check if a string is in NFKD.

### `is_nfc_stream_safe`

```rust
fn is_nfc_stream_safe(s: &str) -> bool
```

Authoritatively check if a string is Stream-Safe NFC.

### `is_nfd_stream_safe`

```rust
fn is_nfd_stream_safe(s: &str) -> bool
```

Authoritatively check if a string is Stream-Safe NFD.

