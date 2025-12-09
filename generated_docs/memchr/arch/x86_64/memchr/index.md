*[memchr](../../../index.md) / [arch](../../index.md) / [x86_64](../index.md) / [memchr](index.md)*

---

# Module `memchr`

Wrapper routines for `memchr` and friends.

These routines efficiently dispatch to the best implementation based on what
the CPU supports.

## Contents

- [Functions](#functions)
  - [`memchr_raw`](#memchr_raw)
  - [`memrchr_raw`](#memrchr_raw)
  - [`memchr2_raw`](#memchr2_raw)
  - [`memrchr2_raw`](#memrchr2_raw)
  - [`memchr3_raw`](#memchr3_raw)
  - [`memrchr3_raw`](#memrchr3_raw)
  - [`count_raw`](#count_raw)
- [Macros](#macros)
  - [`unsafe_ifunc!`](#unsafe_ifunc)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`memchr_raw`](#memchr_raw) | fn | memchr, but using raw pointers to represent the haystack. |
| [`memrchr_raw`](#memrchr_raw) | fn | memrchr, but using raw pointers to represent the haystack. |
| [`memchr2_raw`](#memchr2_raw) | fn | memchr2, but using raw pointers to represent the haystack. |
| [`memrchr2_raw`](#memrchr2_raw) | fn | memrchr2, but using raw pointers to represent the haystack. |
| [`memchr3_raw`](#memchr3_raw) | fn | memchr3, but using raw pointers to represent the haystack. |
| [`memrchr3_raw`](#memrchr3_raw) | fn | memrchr3, but using raw pointers to represent the haystack. |
| [`count_raw`](#count_raw) | fn | Count all matching bytes, but using raw pointers to represent the haystack. |
| [`unsafe_ifunc!`](#unsafe_ifunc) | macro | Provides a way to run a memchr-like function while amortizing the cost of runtime CPU feature detection. |

## Functions

### `memchr_raw`

```rust
fn memchr_raw(n1: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/arch/x86_64/memchr.rs:174-189`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/x86_64/memchr.rs#L174-L189)*

memchr, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `One::find_raw`.

### `memrchr_raw`

```rust
fn memrchr_raw(n1: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/arch/x86_64/memchr.rs:197-212`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/x86_64/memchr.rs#L197-L212)*

memrchr, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `One::rfind_raw`.

### `memchr2_raw`

```rust
fn memchr2_raw(n1: u8, n2: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/arch/x86_64/memchr.rs:220-237`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/x86_64/memchr.rs#L220-L237)*

memchr2, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Two::find_raw`.

### `memrchr2_raw`

```rust
fn memrchr2_raw(n1: u8, n2: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/arch/x86_64/memchr.rs:245-262`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/x86_64/memchr.rs#L245-L262)*

memrchr2, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Two::rfind_raw`.

### `memchr3_raw`

```rust
fn memchr3_raw(n1: u8, n2: u8, n3: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/arch/x86_64/memchr.rs:270-289`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/x86_64/memchr.rs#L270-L289)*

memchr3, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Three::find_raw`.

### `memrchr3_raw`

```rust
fn memrchr3_raw(n1: u8, n2: u8, n3: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/arch/x86_64/memchr.rs:297-316`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/x86_64/memchr.rs#L297-L316)*

memrchr3, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Three::rfind_raw`.

### `count_raw`

```rust
fn count_raw(n1: u8, start: *const u8, end: *const u8) -> usize
```

*Defined in [`memchr-2.7.6/src/arch/x86_64/memchr.rs:324-335`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/x86_64/memchr.rs#L324-L335)*

Count all matching bytes, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `One::count_raw`.

## Macros

### `unsafe_ifunc!`

*Defined in [`memchr-2.7.6/src/arch/x86_64/memchr.rs:58-160`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/x86_64/memchr.rs#L58-L160)*

Provides a way to run a memchr-like function while amortizing the cost of
runtime CPU feature detection.

This works by loading a function pointer from an atomic global. Initially,
this global is set to a function that does CPU feature detection. For
example, if AVX2 is enabled, then the AVX2 implementation is used.
Otherwise, at least on x86_64, the SSE2 implementation is used. (And
in some niche cases, if SSE2 isn't available, then the architecture
independent fallback implementation is used.)

After the first call to this function, the atomic global is replaced with
the specific AVX2, SSE2 or fallback routine chosen. Subsequent calls then
will directly call the chosen routine instead of needing to go through the
CPU feature detection branching again.

This particular macro is specifically written to provide the implementation
of functions with the following signature:

```ignore
fn memchr(needle1: u8, start: *const u8, end: *const u8) -> Option<usize>;
```

Where you can also have `memchr2` and `memchr3`, but with `needle2` and
`needle3`, respectively. The `start` and `end` parameters correspond to the
start and end of the haystack, respectively.

We use raw pointers here instead of the more obvious `haystack: &[u8]` so
that the function is compatible with our lower level iterator logic that
operates on raw pointers. We use this macro to implement "raw" memchr
routines with the signature above, and then define memchr routines using
regular slices on top of them.

Note that we use `#[cfg(target_feature = "sse2")]` below even though
it shouldn't be strictly necessary because without it, it seems to
cause the compiler to blow up. I guess it can't handle a function
pointer being created with a sse target feature? Dunno. See the
`build-for-x86-64-but-non-sse-target` CI job if you want to experiment with
this.

# Safety

Primarily callers must ensure that `$fnty` is a correct function pointer
type and not something else.

Callers must also ensure that `$memchrty::$memchrfind` corresponds to a
routine that returns a valid function pointer when a match is found. That
is, a pointer that is `>= start` and `< end`.

Callers must also ensure that the `$hay_start` and `$hay_end` identifiers
correspond to valid pointers.

