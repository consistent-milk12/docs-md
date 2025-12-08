# Crate `is_ci`

## Functions

### `is_ci`

```rust
fn is_ci() -> bool
```

Returns true if the current environment is found to probably be a CI
environment or service. That's it, that's all it does.

### `cached`

```rust
fn cached() -> bool
```

Returns true if the current environment is found to probably be a CI
environment or service, and caches the result for future calls. If you
expect the environment to change, use [uncached](#uncached).

### `uncached`

```rust
fn uncached() -> bool
```

Returns true if the current environment is found to probably be a CI
environment or service. If you expect to call this multiple times without
the environment changing, use [cached](#cached).

### `check`

```rust
fn check(name: &str) -> bool
```

