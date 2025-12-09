*[memchr](../../index.md) / [arch](../index.md) / [all](index.md)*

---

# Module `all`

Contains architecture independent routines.

These routines are often used as a "fallback" implementation when the more
specialized architecture dependent routines are unavailable.

## Contents

- [Modules](#modules)
  - [`memchr`](#memchr)
  - [`packedpair`](#packedpair)
  - [`rabinkarp`](#rabinkarp)
  - [`shiftor`](#shiftor)
  - [`twoway`](#twoway)
- [Functions](#functions)
  - [`is_prefix`](#is_prefix)
  - [`is_suffix`](#is_suffix)
  - [`is_equal`](#is_equal)
  - [`is_equal_raw`](#is_equal_raw)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`memchr`](#memchr) | mod | Provides architecture independent implementations of `memchr` and friends. |
| [`packedpair`](#packedpair) | mod | Provides an architecture independent implementation of the "packed pair" algorithm. |
| [`rabinkarp`](#rabinkarp) | mod | An implementation of the [Rabin-Karp substring search algorithm][rabinkarp]. |
| [`shiftor`](#shiftor) | mod | An implementation of the [Shift-Or substring search algorithm][shiftor]. |
| [`twoway`](#twoway) | mod | An implementation of the [Two-Way substring search algorithm][two-way]. |
| [`is_prefix`](#is_prefix) | fn | Returns true if and only if `needle` is a prefix of `haystack`. |
| [`is_suffix`](#is_suffix) | fn | Returns true if and only if `needle` is a suffix of `haystack`. |
| [`is_equal`](#is_equal) | fn | Compare corresponding bytes in `x` and `y` for equality. |
| [`is_equal_raw`](#is_equal_raw) | fn | Compare `n` bytes at the given pointers for equality. |

## Modules

- [`memchr`](memchr/index.md) — Provides architecture independent implementations of `memchr` and friends.
- [`packedpair`](packedpair/index.md) — Provides an architecture independent implementation of the "packed pair"
- [`rabinkarp`](rabinkarp/index.md) — An implementation of the [Rabin-Karp substring search algorithm][rabinkarp].
- [`shiftor`](shiftor/index.md) — An implementation of the [Shift-Or substring search algorithm][shiftor].
- [`twoway`](twoway/index.md) — An implementation of the [Two-Way substring search algorithm][two-way].

## Functions

### `is_prefix`

```rust
fn is_prefix(haystack: &[u8], needle: &[u8]) -> bool
```

*Defined in [`memchr-2.7.6/src/arch/all/mod.rs:26-29`](../../../../.source_1765210505/memchr-2.7.6/src/arch/all/mod.rs#L26-L29)*

Returns true if and only if `needle` is a prefix of `haystack`.

This uses a latency optimized variant of `memcmp` internally which *might*
make this faster for very short strings.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

### `is_suffix`

```rust
fn is_suffix(haystack: &[u8], needle: &[u8]) -> bool
```

*Defined in [`memchr-2.7.6/src/arch/all/mod.rs:42-45`](../../../../.source_1765210505/memchr-2.7.6/src/arch/all/mod.rs#L42-L45)*

Returns true if and only if `needle` is a suffix of `haystack`.

This uses a latency optimized variant of `memcmp` internally which *might*
make this faster for very short strings.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

### `is_equal`

```rust
fn is_equal(x: &[u8], y: &[u8]) -> bool
```

*Defined in [`memchr-2.7.6/src/arch/all/mod.rs:65-73`](../../../../.source_1765210505/memchr-2.7.6/src/arch/all/mod.rs#L65-L73)*

Compare corresponding bytes in `x` and `y` for equality.

That is, this returns true if and only if `x.len() == y.len()` and
`x[i] == y[i]` for all `0 <= i < x.len()`.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

# Motivation

Why not use slice equality instead? Well, slice equality usually results in
a call out to the current platform's `libc` which might not be inlineable
or have other overhead. This routine isn't guaranteed to be a win, but it
might be in some cases.

### `is_equal_raw`

```rust
unsafe fn is_equal_raw(x: *const u8, y: *const u8, n: usize) -> bool
```

*Defined in [`memchr-2.7.6/src/arch/all/mod.rs:108-158`](../../../../.source_1765210505/memchr-2.7.6/src/arch/all/mod.rs#L108-L158)*

Compare `n` bytes at the given pointers for equality.

This returns true if and only if `*x.add(i) == *y.add(i)` for all
`0 <= i < n`.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

# Motivation

Why not use slice equality instead? Well, slice equality usually results in
a call out to the current platform's `libc` which might not be inlineable
or have other overhead. This routine isn't guaranteed to be a win, but it
might be in some cases.

# Safety

* Both `x` and `y` must be valid for reads of up to `n` bytes.
* Both `x` and `y` must point to an initialized value.
* Both `x` and `y` must each point to an allocated object and
must either be in bounds or at most one byte past the end of the
allocated object. `x` and `y` do not need to point to the same allocated
object, but they may.
* Both `x` and `y` must be _derived from_ a pointer to their respective
allocated objects.
* The distance between `x` and `x+n` must not overflow `isize`. Similarly
for `y` and `y+n`.
* The distance being in bounds must not rely on "wrapping around" the
address space.

