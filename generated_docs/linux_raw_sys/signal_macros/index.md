*[linux_raw_sys](../index.md) / [signal_macros](index.md)*

---

# Module `signal_macros`

## Functions

### `sig_ign`

```rust
const fn sig_ign() -> super::general::__kernel_sighandler_t
```

Rust doesn't currently permit us to use `transmute` to convert the
`SIG_IGN` value into a function pointer in a `const` initializer, so
we make it a function instead.


## Constants

### `SIG_DFL`

```rust
const SIG_DFL: super::general::__kernel_sighandler_t;
```

