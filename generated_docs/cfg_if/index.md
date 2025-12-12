# Crate `cfg_if`

A macro for defining `#[cfg]` if-else statements.

The macro provided by this crate, `cfg_if`, is similar to the `if/elif` C
preprocessor macro by allowing definition of a cascade of `#[cfg]` cases,
emitting the implementation which matches first.

This allows you to conveniently provide a long list `#[cfg]`'d blocks of code
without having to rewrite each clause multiple times.

# Example

```rust
cfg_if::cfg_if! {
    if #[cfg(unix)] {
        fn foo() { /* unix specific functionality */ }
    } else if #[cfg(target_pointer_width = "32")] {
        fn foo() { /* non-unix, 32-bit functionality */ }
    } else {
        fn foo() { /* fallback implementation */ }
    }
}

fn main() {}
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`cfg_if!`](#cfg-if) | macro | The main macro provided by this crate. |

## Macros

### `cfg_if!`

*Defined in [`cfg-if-1.0.4/src/lib.rs:34-96`](../../.source_1765210505/cfg-if-1.0.4/src/lib.rs#L34-L96)*

The main macro provided by this crate. See crate documentation for more
information.

