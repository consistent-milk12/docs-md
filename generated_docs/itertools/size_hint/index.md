*[itertools](../index.md) / [size_hint](index.md)*

---

# Module `size_hint`

Arithmetic on `Iterator.size_hint()` values.


## Contents

- [Functions](#functions)
  - [`add`](#add)
  - [`add_scalar`](#add-scalar)
  - [`sub_scalar`](#sub-scalar)
  - [`mul`](#mul)
  - [`mul_scalar`](#mul-scalar)
  - [`max`](#max)
  - [`min`](#min)
- [Type Aliases](#type-aliases)
  - [`SizeHint`](#sizehint)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`add`](#add) | fn | Add `SizeHint` correctly. |
| [`add_scalar`](#add-scalar) | fn | Add `x` correctly to a `SizeHint`. |
| [`sub_scalar`](#sub-scalar) | fn | Subtract `x` correctly from a `SizeHint`. |
| [`mul`](#mul) | fn | Multiply `SizeHint` correctly |
| [`mul_scalar`](#mul-scalar) | fn | Multiply `x` correctly with a `SizeHint`. |
| [`max`](#max) | fn | Return the maximum |
| [`min`](#min) | fn | Return the minimum |
| [`SizeHint`](#sizehint) | type | `SizeHint` is the return type of `Iterator::size_hint()`. |

## Functions

### `add`

```rust
fn add(a: (usize, Option<usize>), b: (usize, Option<usize>)) -> (usize, Option<usize>)
```

*Defined in [`itertools-0.14.0/src/size_hint.rs:11-19`](../../../.source_1765900590/itertools-0.14.0/src/size_hint.rs#L11-L19)*

Add `SizeHint` correctly.

### `add_scalar`

```rust
fn add_scalar(sh: (usize, Option<usize>), x: usize) -> (usize, Option<usize>)
```

*Defined in [`itertools-0.14.0/src/size_hint.rs:23-28`](../../../.source_1765900590/itertools-0.14.0/src/size_hint.rs#L23-L28)*

Add `x` correctly to a `SizeHint`.

### `sub_scalar`

```rust
fn sub_scalar(sh: (usize, Option<usize>), x: usize) -> (usize, Option<usize>)
```

*Defined in [`itertools-0.14.0/src/size_hint.rs:32-37`](../../../.source_1765900590/itertools-0.14.0/src/size_hint.rs#L32-L37)*

Subtract `x` correctly from a `SizeHint`.

### `mul`

```rust
fn mul(a: (usize, Option<usize>), b: (usize, Option<usize>)) -> (usize, Option<usize>)
```

*Defined in [`itertools-0.14.0/src/size_hint.rs:41-49`](../../../.source_1765900590/itertools-0.14.0/src/size_hint.rs#L41-L49)*

Multiply `SizeHint` correctly

### `mul_scalar`

```rust
fn mul_scalar(sh: (usize, Option<usize>), x: usize) -> (usize, Option<usize>)
```

*Defined in [`itertools-0.14.0/src/size_hint.rs:53-58`](../../../.source_1765900590/itertools-0.14.0/src/size_hint.rs#L53-L58)*

Multiply `x` correctly with a `SizeHint`.

### `max`

```rust
fn max(a: (usize, Option<usize>), b: (usize, Option<usize>)) -> (usize, Option<usize>)
```

*Defined in [`itertools-0.14.0/src/size_hint.rs:62-74`](../../../.source_1765900590/itertools-0.14.0/src/size_hint.rs#L62-L74)*

Return the maximum

### `min`

```rust
fn min(a: (usize, Option<usize>), b: (usize, Option<usize>)) -> (usize, Option<usize>)
```

*Defined in [`itertools-0.14.0/src/size_hint.rs:78-87`](../../../.source_1765900590/itertools-0.14.0/src/size_hint.rs#L78-L87)*

Return the minimum

## Type Aliases

### `SizeHint`

```rust
type SizeHint = (usize, Option<usize>);
```

*Defined in [`itertools-0.14.0/src/size_hint.rs:7`](../../../.source_1765900590/itertools-0.14.0/src/size_hint.rs#L7)*

`SizeHint` is the return type of `Iterator::size_hint()`.

