*[rustix](../index.md) / [pid](index.md)*

---

# Module `pid`

The `Pid` type.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pid`](#pid) | struct | `pid_t`—A non-zero Unix process ID. |
| [`RawPid`](#rawpid) | type | A process identifier as a raw integer. |

## Structs

### `Pid`

```rust
struct Pid(core::num::NonZeroI32);
```

*Defined in [`rustix-1.1.2/src/pid.rs:19`](../../../.source_1765521767/rustix-1.1.2/src/pid.rs#L19)*

`pid_t`—A non-zero Unix process ID.

This is a pid, and not a pidfd. It is not a file descriptor, and the
process it refers to could disappear at any time and be replaced by
another, unrelated, process.

On Linux, `Pid` values are also used to identify threads.

#### Implementations

- <span id="pid-const-init"></span>`const INIT: Self`

- <span id="pid-from-raw"></span>`const fn from_raw(raw: i32) -> Option<Self>`

- <span id="pid-from-raw-unchecked"></span>`const unsafe fn from_raw_unchecked(raw: i32) -> Self`

- <span id="pid-from-child"></span>`fn from_child(child: &std::process::Child) -> Self`

- <span id="pid-as-raw-nonzero"></span>`const fn as_raw_nonzero(self) -> NonZeroI32`

- <span id="pid-as-raw-pid"></span>`const fn as_raw_pid(self) -> i32`

- <span id="pid-as-raw"></span>`const fn as_raw(pid: Option<Self>) -> i32`

- <span id="pid-is-init"></span>`const fn is_init(self) -> bool`

#### Trait Implementations

##### `impl Binary for Pid`

- <span id="pid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Clone for Pid`

- <span id="pid-clone"></span>`fn clone(&self) -> Pid` — [`Pid`](#pid)

##### `impl Copy for Pid`

##### `impl Debug for Pid`

- <span id="pid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Pid`

- <span id="pid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Pid`

##### `impl Hash for Pid`

- <span id="pid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl LowerExp for Pid`

- <span id="pid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex for Pid`

- <span id="pid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal for Pid`

- <span id="pid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Pid`

- <span id="pid-eq"></span>`fn eq(&self, other: &Pid) -> bool` — [`Pid`](#pid)

##### `impl StructuralPartialEq for Pid`

##### `impl ToString for Pid`

- <span id="pid-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for Pid`

- <span id="pid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex for Pid`

- <span id="pid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `RawPid`

```rust
type RawPid = i32;
```

*Defined in [`rustix-1.1.2/src/pid.rs:8`](../../../.source_1765521767/rustix-1.1.2/src/pid.rs#L8)*

A process identifier as a raw integer.

