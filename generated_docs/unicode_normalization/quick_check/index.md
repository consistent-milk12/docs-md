*[unicode_normalization](../index.md) / [quick_check](index.md)*

---

# Module `quick_check`

## Contents

- [Enums](#enums)
  - [`IsNormalized`](#isnormalized)
- [Functions](#functions)
  - [`quick_check`](#quick-check)
  - [`is_nfc_quick`](#is-nfc-quick)
  - [`is_nfkc_quick`](#is-nfkc-quick)
  - [`is_nfd_quick`](#is-nfd-quick)
  - [`is_nfkd_quick`](#is-nfkd-quick)
  - [`is_nfc_stream_safe_quick`](#is-nfc-stream-safe-quick)
  - [`is_nfd_stream_safe_quick`](#is-nfd-stream-safe-quick)
  - [`is_nfc`](#is-nfc)
  - [`is_nfkc`](#is-nfkc)
  - [`is_nfd`](#is-nfd)
  - [`is_nfkd`](#is-nfkd)
  - [`is_nfc_stream_safe`](#is-nfc-stream-safe)
  - [`is_nfd_stream_safe`](#is-nfd-stream-safe)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IsNormalized`](#isnormalized) | enum | QuickCheck quickly determines if a string is normalized, it can return `Maybe` |
| [`quick_check`](#quick-check) | fn |  |
| [`is_nfc_quick`](#is-nfc-quick) | fn | Quickly check if a string is in NFC, potentially returning `IsNormalized::Maybe` if further checks are necessary. |
| [`is_nfkc_quick`](#is-nfkc-quick) | fn | Quickly check if a string is in NFKC. |
| [`is_nfd_quick`](#is-nfd-quick) | fn | Quickly check if a string is in NFD. |
| [`is_nfkd_quick`](#is-nfkd-quick) | fn | Quickly check if a string is in NFKD. |
| [`is_nfc_stream_safe_quick`](#is-nfc-stream-safe-quick) | fn | Quickly check if a string is Stream-Safe NFC. |
| [`is_nfd_stream_safe_quick`](#is-nfd-stream-safe-quick) | fn | Quickly check if a string is Stream-Safe NFD. |
| [`is_nfc`](#is-nfc) | fn | Authoritatively check if a string is in NFC. |
| [`is_nfkc`](#is-nfkc) | fn | Authoritatively check if a string is in NFKC. |
| [`is_nfd`](#is-nfd) | fn | Authoritatively check if a string is in NFD. |
| [`is_nfkd`](#is-nfkd) | fn | Authoritatively check if a string is in NFKD. |
| [`is_nfc_stream_safe`](#is-nfc-stream-safe) | fn | Authoritatively check if a string is Stream-Safe NFC. |
| [`is_nfd_stream_safe`](#is-nfd-stream-safe) | fn | Authoritatively check if a string is Stream-Safe NFD. |

## Enums

### `IsNormalized`

```rust
enum IsNormalized {
    Yes,
    No,
    Maybe,
}
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:13-20`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L13-L20)*

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

- <span id="isnormalized-eq"></span>`fn eq(&self, other: &IsNormalized) -> bool` — [`IsNormalized`](#isnormalized)

##### `impl StructuralPartialEq for IsNormalized`

## Functions

### `quick_check`

```rust
fn quick_check<F, I>(s: I, is_allowed: F, stream_safe: bool) -> IsNormalized
where
    I: Iterator<Item = char>,
    F: Fn(char) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:24-69`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L24-L69)*

### `is_nfc_quick`

```rust
fn is_nfc_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:75-77`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L75-L77)*

Quickly check if a string is in NFC, potentially returning
`IsNormalized::Maybe` if further checks are necessary.  In this case a check
like `s.chars().nfc().eq(s.chars())` should suffice.

### `is_nfkc_quick`

```rust
fn is_nfkc_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:81-83`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L81-L83)*

Quickly check if a string is in NFKC.

### `is_nfd_quick`

```rust
fn is_nfd_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:87-89`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L87-L89)*

Quickly check if a string is in NFD.

### `is_nfkd_quick`

```rust
fn is_nfkd_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:93-95`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L93-L95)*

Quickly check if a string is in NFKD.

### `is_nfc_stream_safe_quick`

```rust
fn is_nfc_stream_safe_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:99-101`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L99-L101)*

Quickly check if a string is Stream-Safe NFC.

### `is_nfd_stream_safe_quick`

```rust
fn is_nfd_stream_safe_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:105-107`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L105-L107)*

Quickly check if a string is Stream-Safe NFD.

### `is_nfc`

```rust
fn is_nfc(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:111-117`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L111-L117)*

Authoritatively check if a string is in NFC.

### `is_nfkc`

```rust
fn is_nfkc(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:121-127`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L121-L127)*

Authoritatively check if a string is in NFKC.

### `is_nfd`

```rust
fn is_nfd(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:131-137`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L131-L137)*

Authoritatively check if a string is in NFD.

### `is_nfkd`

```rust
fn is_nfkd(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:141-147`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L141-L147)*

Authoritatively check if a string is in NFKD.

### `is_nfc_stream_safe`

```rust
fn is_nfc_stream_safe(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:151-157`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L151-L157)*

Authoritatively check if a string is Stream-Safe NFC.

### `is_nfd_stream_safe`

```rust
fn is_nfd_stream_safe(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:161-167`](../../../.source_1765210505/unicode-normalization-0.1.25/src/quick_check.rs#L161-L167)*

Authoritatively check if a string is Stream-Safe NFD.

