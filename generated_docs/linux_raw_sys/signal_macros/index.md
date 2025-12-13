*[linux_raw_sys](../index.md) / [signal_macros](index.md)*

---

# Module `signal_macros`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sig_ign`](#sig-ign) | fn | Rust doesn't currently permit us to use `transmute` to convert the `SIG_IGN` value into a function pointer in a `const` initializer, so we make it a function instead. |
| [`SIG_DFL`](#sig-dfl) | const |  |

## Functions

### `sig_ign`

```rust
const fn sig_ign() -> super::general::__kernel_sighandler_t
```

*Defined in [`linux-raw-sys-0.11.0/src/lib.rs:207-213`](../../../.source_1765633015/linux-raw-sys-0.11.0/src/lib.rs#L207-L213)*

Rust doesn't currently permit us to use `transmute` to convert the
`SIG_IGN` value into a function pointer in a `const` initializer, so
we make it a function instead.


## Constants

### `SIG_DFL`
```rust
const SIG_DFL: super::general::__kernel_sighandler_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/lib.rs:200`](../../../.source_1765633015/linux-raw-sys-0.11.0/src/lib.rs#L200)*

