*[rustix](../../index.md) / [ioctl](../index.md) / [opcode](index.md)*

---

# Module `opcode`

Const functions for computing opcode values.

Linux's headers define macros such as `_IO`, `_IOR`, `_IOW`, and `_IOWR`
for defining ioctl values in a structured way that encode whether they
are reading and/or writing, and other information about the ioctl. The
functions in this module correspond to those macros.

If you're writing a driver and defining your own ioctl numbers, it's
recommended to use these functions to compute them.

## Functions

### `from_components`

```rust
const fn from_components(direction: Direction, group: u8, number: u8, data_size: usize) -> Opcode
```

Create a new opcode from a direction, group, number, and size.

This corresponds to the C macro `_IOC(direction, group, number, size)`

### `none`

```rust
const fn none(group: u8, number: u8) -> Opcode
```

Create a new opcode from a group, a number, that uses no data.

This corresponds to the C macro `_IO(group, number)`.

### `read`

```rust
const fn read<T>(group: u8, number: u8) -> Opcode
```

Create a new reading opcode from a group, a number and the type of
data.

This corresponds to the C macro `_IOR(group, number, T)`.

### `write`

```rust
const fn write<T>(group: u8, number: u8) -> Opcode
```

Create a new writing opcode from a group, a number and the type of
data.

This corresponds to the C macro `_IOW(group, number, T)`.

### `read_write`

```rust
const fn read_write<T>(group: u8, number: u8) -> Opcode
```

Create a new reading and writing opcode from a group, a number and the
type of data.

This corresponds to the C macro `_IOWR(group, number, T)`.

