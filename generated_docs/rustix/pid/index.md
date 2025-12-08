*[rustix](../index.md) / [pid](index.md)*

---

# Module `pid`

The `Pid` type.

## Structs

### `Pid`

```rust
struct Pid(core::num::NonZeroI32);
```

`pid_t`—A non-zero Unix process ID.

This is a pid, and not a pidfd. It is not a file descriptor, and the
process it refers to could disappear at any time and be replaced by
another, unrelated, process.

On Linux, `Pid` values are also used to identify threads.

#### Implementations

- `const INIT: Self`

- `const fn from_raw(raw: i32) -> Option<Self>`

- `const unsafe fn from_raw_unchecked(raw: i32) -> Self`

- `fn from_child(child: &std::process::Child) -> Self`

- `const fn as_raw_nonzero(self: Self) -> NonZeroI32`

- `const fn as_raw_pid(self: Self) -> i32`

- `const fn as_raw(pid: Option<Self>) -> i32`

- `const fn is_init(self: Self) -> bool`

#### Trait Implementations

##### `impl Binary for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Clone for Pid`

- `fn clone(self: &Self) -> Pid` — [`Pid`](#pid)

##### `impl Copy for Pid`

##### `impl Debug for Pid`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Pid`

##### `impl Hash for Pid`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl LowerExp for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Pid`

- `fn eq(self: &Self, other: &Pid) -> bool` — [`Pid`](#pid)

##### `impl StructuralPartialEq for Pid`

##### `impl<T> ToString for Pid`

- `fn to_string(self: &Self) -> String`

##### `impl UpperExp for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `RawPid`

```rust
type RawPid = i32;
```

A process identifier as a raw integer.

