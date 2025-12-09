# Crate `is_ci`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`is_ci`](#is_ci) | fn | Returns true if the current environment is found to probably be a CI environment or service. |
| [`cached`](#cached) | fn | Returns true if the current environment is found to probably be a CI environment or service, and caches the result for future calls. |
| [`uncached`](#uncached) | fn | Returns true if the current environment is found to probably be a CI environment or service. |
| [`check`](#check) | fn |  |

## Functions

### `is_ci`

```rust
fn is_ci() -> bool
```

*Defined in [`is_ci-1.2.0/src/lib.rs:10-12`](../../.source_1765210505/is_ci-1.2.0/src/lib.rs#L10-L12)*

Returns true if the current environment is found to probably be a CI
environment or service. That's it, that's all it does.

### `cached`

```rust
fn cached() -> bool
```

*Defined in [`is_ci-1.2.0/src/lib.rs:17-20`](../../.source_1765210505/is_ci-1.2.0/src/lib.rs#L17-L20)*

Returns true if the current environment is found to probably be a CI
environment or service, and caches the result for future calls. If you
expect the environment to change, use [uncached](#uncached).

### `uncached`

```rust
fn uncached() -> bool
```

*Defined in [`is_ci-1.2.0/src/lib.rs:25-66`](../../.source_1765210505/is_ci-1.2.0/src/lib.rs#L25-L66)*

Returns true if the current environment is found to probably be a CI
environment or service. If you expect to call this multiple times without
the environment changing, use [cached](#cached).

### `check`

```rust
fn check(name: &str) -> bool
```

*Defined in [`is_ci-1.2.0/src/lib.rs:68-70`](../../.source_1765210505/is_ci-1.2.0/src/lib.rs#L68-L70)*

