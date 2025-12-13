*[semver](../index.md) / [eval](index.md)*

---

# Module `eval`

## Contents

- [Functions](#functions)
  - [`matches_req`](#matches-req)
  - [`matches_comparator`](#matches-comparator)
  - [`matches_impl`](#matches-impl)
  - [`matches_exact`](#matches-exact)
  - [`matches_greater`](#matches-greater)
  - [`matches_less`](#matches-less)
  - [`matches_tilde`](#matches-tilde)
  - [`matches_caret`](#matches-caret)
  - [`pre_is_compatible`](#pre-is-compatible)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`matches_req`](#matches-req) | fn |  |
| [`matches_comparator`](#matches-comparator) | fn |  |
| [`matches_impl`](#matches-impl) | fn |  |
| [`matches_exact`](#matches-exact) | fn |  |
| [`matches_greater`](#matches-greater) | fn |  |
| [`matches_less`](#matches-less) | fn |  |
| [`matches_tilde`](#matches-tilde) | fn |  |
| [`matches_caret`](#matches-caret) | fn |  |
| [`pre_is_compatible`](#pre-is-compatible) | fn |  |

## Functions

### `matches_req`

```rust
fn matches_req(req: &crate::VersionReq, ver: &crate::Version) -> bool
```

*Defined in [`semver-1.0.27/src/eval.rs:3-24`](../../../.source_1765633015/semver-1.0.27/src/eval.rs#L3-L24)*

### `matches_comparator`

```rust
fn matches_comparator(cmp: &crate::Comparator, ver: &crate::Version) -> bool
```

*Defined in [`semver-1.0.27/src/eval.rs:26-28`](../../../.source_1765633015/semver-1.0.27/src/eval.rs#L26-L28)*

### `matches_impl`

```rust
fn matches_impl(cmp: &crate::Comparator, ver: &crate::Version) -> bool
```

*Defined in [`semver-1.0.27/src/eval.rs:30-40`](../../../.source_1765633015/semver-1.0.27/src/eval.rs#L30-L40)*

### `matches_exact`

```rust
fn matches_exact(cmp: &crate::Comparator, ver: &crate::Version) -> bool
```

*Defined in [`semver-1.0.27/src/eval.rs:42-60`](../../../.source_1765633015/semver-1.0.27/src/eval.rs#L42-L60)*

### `matches_greater`

```rust
fn matches_greater(cmp: &crate::Comparator, ver: &crate::Version) -> bool
```

*Defined in [`semver-1.0.27/src/eval.rs:62-86`](../../../.source_1765633015/semver-1.0.27/src/eval.rs#L62-L86)*

### `matches_less`

```rust
fn matches_less(cmp: &crate::Comparator, ver: &crate::Version) -> bool
```

*Defined in [`semver-1.0.27/src/eval.rs:88-112`](../../../.source_1765633015/semver-1.0.27/src/eval.rs#L88-L112)*

### `matches_tilde`

```rust
fn matches_tilde(cmp: &crate::Comparator, ver: &crate::Version) -> bool
```

*Defined in [`semver-1.0.27/src/eval.rs:114-132`](../../../.source_1765633015/semver-1.0.27/src/eval.rs#L114-L132)*

### `matches_caret`

```rust
fn matches_caret(cmp: &crate::Comparator, ver: &crate::Version) -> bool
```

*Defined in [`semver-1.0.27/src/eval.rs:134-172`](../../../.source_1765633015/semver-1.0.27/src/eval.rs#L134-L172)*

### `pre_is_compatible`

```rust
fn pre_is_compatible(cmp: &crate::Comparator, ver: &crate::Version) -> bool
```

*Defined in [`semver-1.0.27/src/eval.rs:174-179`](../../../.source_1765633015/semver-1.0.27/src/eval.rs#L174-L179)*

