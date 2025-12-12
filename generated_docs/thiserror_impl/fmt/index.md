*[thiserror_impl](../index.md) / [fmt](index.md)*

---

# Module `fmt`

## Contents

- [Structs](#structs)
  - [`FmtArguments`](#fmtarguments)
- [Functions](#functions)
  - [`explicit_named_args`](#explicit-named-args)
  - [`try_explicit_named_args`](#try-explicit-named-args)
  - [`fallback_explicit_named_args`](#fallback-explicit-named-args)
  - [`is_syn_full`](#is-syn-full)
  - [`take_int`](#take-int)
  - [`take_ident`](#take-ident)
  - [`between`](#between)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FmtArguments`](#fmtarguments) | struct |  |
| [`explicit_named_args`](#explicit-named-args) | fn |  |
| [`try_explicit_named_args`](#try-explicit-named-args) | fn |  |
| [`fallback_explicit_named_args`](#fallback-explicit-named-args) | fn |  |
| [`is_syn_full`](#is-syn-full) | fn |  |
| [`take_int`](#take-int) | fn |  |
| [`take_ident`](#take-ident) | fn |  |
| [`between`](#between) | fn |  |

## Structs

### `FmtArguments`

```rust
struct FmtArguments {
    named: std::collections::BTreeSet<crate::unraw::IdentUnraw>,
    first_unnamed: Option<proc_macro2::TokenStream>,
}
```

*Defined in [`thiserror-impl-2.0.17/src/fmt.rs:166-169`](../../../.source_1765210505/thiserror-impl-2.0.17/src/fmt.rs#L166-L169)*

## Functions

### `explicit_named_args`

```rust
fn explicit_named_args(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<FmtArguments>
```

*Defined in [`thiserror-impl-2.0.17/src/fmt.rs:172-190`](../../../.source_1765210505/thiserror-impl-2.0.17/src/fmt.rs#L172-L190)*

### `try_explicit_named_args`

```rust
fn try_explicit_named_args(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<FmtArguments>
```

*Defined in [`thiserror-impl-2.0.17/src/fmt.rs:192-229`](../../../.source_1765210505/thiserror-impl-2.0.17/src/fmt.rs#L192-L229)*

### `fallback_explicit_named_args`

```rust
fn fallback_explicit_named_args(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<FmtArguments>
```

*Defined in [`thiserror-impl-2.0.17/src/fmt.rs:231-253`](../../../.source_1765210505/thiserror-impl-2.0.17/src/fmt.rs#L231-L253)*

### `is_syn_full`

```rust
fn is_syn_full() -> bool
```

*Defined in [`thiserror-impl-2.0.17/src/fmt.rs:255-271`](../../../.source_1765210505/thiserror-impl-2.0.17/src/fmt.rs#L255-L271)*

### `take_int`

```rust
fn take_int<'a>(read: &mut &'a str) -> &'a str
```

*Defined in [`thiserror-impl-2.0.17/src/fmt.rs:273-284`](../../../.source_1765210505/thiserror-impl-2.0.17/src/fmt.rs#L273-L284)*

### `take_ident`

```rust
fn take_ident<'a>(read: &mut &'a str) -> &'a str
```

*Defined in [`thiserror-impl-2.0.17/src/fmt.rs:286-297`](../../../.source_1765210505/thiserror-impl-2.0.17/src/fmt.rs#L286-L297)*

### `between`

```rust
fn between<'a>(begin: syn::parse::ParseStream<'a>, end: syn::parse::ParseStream<'a>) -> proc_macro2::TokenStream
```

*Defined in [`thiserror-impl-2.0.17/src/fmt.rs:299-323`](../../../.source_1765210505/thiserror-impl-2.0.17/src/fmt.rs#L299-L323)*

