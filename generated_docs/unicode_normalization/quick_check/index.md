*[unicode_normalization](../index.md) / [quick_check](index.md)*

---

# Module `quick_check`

## Contents

- [Enums](#enums)
  - [`IsNormalized`](#isnormalized)
- [Functions](#functions)
  - [`quick_check`](#quick_check)
  - [`is_nfc_quick`](#is_nfc_quick)
  - [`is_nfkc_quick`](#is_nfkc_quick)
  - [`is_nfd_quick`](#is_nfd_quick)
  - [`is_nfkd_quick`](#is_nfkd_quick)
  - [`is_nfc_stream_safe_quick`](#is_nfc_stream_safe_quick)
  - [`is_nfd_stream_safe_quick`](#is_nfd_stream_safe_quick)
  - [`is_nfc`](#is_nfc)
  - [`is_nfkc`](#is_nfkc)
  - [`is_nfd`](#is_nfd)
  - [`is_nfkd`](#is_nfkd)
  - [`is_nfc_stream_safe`](#is_nfc_stream_safe)
  - [`is_nfd_stream_safe`](#is_nfd_stream_safe)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IsNormalized`](#isnormalized) | enum | QuickCheck quickly determines if a string is normalized, it can return |
| [`quick_check`](#quick_check) | fn |  |
| [`is_nfc_quick`](#is_nfc_quick) | fn | Quickly check if a string is in NFC, potentially returning |
| [`is_nfkc_quick`](#is_nfkc_quick) | fn | Quickly check if a string is in NFKC. |
| [`is_nfd_quick`](#is_nfd_quick) | fn | Quickly check if a string is in NFD. |
| [`is_nfkd_quick`](#is_nfkd_quick) | fn | Quickly check if a string is in NFKD. |
| [`is_nfc_stream_safe_quick`](#is_nfc_stream_safe_quick) | fn | Quickly check if a string is Stream-Safe NFC. |
| [`is_nfd_stream_safe_quick`](#is_nfd_stream_safe_quick) | fn | Quickly check if a string is Stream-Safe NFD. |
| [`is_nfc`](#is_nfc) | fn | Authoritatively check if a string is in NFC. |
| [`is_nfkc`](#is_nfkc) | fn | Authoritatively check if a string is in NFKC. |
| [`is_nfd`](#is_nfd) | fn | Authoritatively check if a string is in NFD. |
| [`is_nfkd`](#is_nfkd) | fn | Authoritatively check if a string is in NFKD. |
| [`is_nfc_stream_safe`](#is_nfc_stream_safe) | fn | Authoritatively check if a string is Stream-Safe NFC. |
| [`is_nfd_stream_safe`](#is_nfd_stream_safe) | fn | Authoritatively check if a string is Stream-Safe NFD. |

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

- <span id="isnormalized-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IsNormalized`

##### `impl PartialEq for IsNormalized`

- <span id="isnormalized-eq"></span>`fn eq(&self, other: &IsNormalized) -> bool` — [`IsNormalized`](../index.md)

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

