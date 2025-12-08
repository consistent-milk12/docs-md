*[rustix](../../index.md) / [ioctl](../index.md) / [linux](index.md)*

---

# Module `linux`

`ioctl` opcode behavior for Linux platforms.

## Contents

- [Modules](#modules)
  - [`consts`](#consts)
- [Functions](#functions)
  - [`compose_opcode`](#compose_opcode)
- [Constants](#constants)
  - [`NUM_BITS`](#num_bits)
  - [`GROUP_BITS`](#group_bits)
  - [`NUM_SHIFT`](#num_shift)
  - [`GROUP_SHIFT`](#group_shift)
  - [`SIZE_SHIFT`](#size_shift)
  - [`DIR_SHIFT`](#dir_shift)
  - [`NUM_MASK`](#num_mask)
  - [`GROUP_MASK`](#group_mask)
  - [`SIZE_MASK`](#size_mask)
  - [`DIR_MASK`](#dir_mask)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`consts`](#consts) | mod |  |
| [`compose_opcode`](#compose_opcode) | fn | Compose an opcode from its component parts. |
| [`NUM_BITS`](#num_bits) | const |  |
| [`GROUP_BITS`](#group_bits) | const |  |
| [`NUM_SHIFT`](#num_shift) | const |  |
| [`GROUP_SHIFT`](#group_shift) | const |  |
| [`SIZE_SHIFT`](#size_shift) | const |  |
| [`DIR_SHIFT`](#dir_shift) | const |  |
| [`NUM_MASK`](#num_mask) | const |  |
| [`GROUP_MASK`](#group_mask) | const |  |
| [`SIZE_MASK`](#size_mask) | const |  |
| [`DIR_MASK`](#dir_mask) | const |  |

## Modules

- [`consts`](consts/index.md) - 

## Functions

### `compose_opcode`

```rust
const fn compose_opcode(dir: super::Direction, group: super::Opcode, num: super::Opcode, size: super::Opcode) -> super::Opcode
```

Compose an opcode from its component parts.

## Constants

### `NUM_BITS`

```rust
const NUM_BITS: super::Opcode = 8u32;
```

### `GROUP_BITS`

```rust
const GROUP_BITS: super::Opcode = 8u32;
```

### `NUM_SHIFT`

```rust
const NUM_SHIFT: super::Opcode = 0u32;
```

### `GROUP_SHIFT`

```rust
const GROUP_SHIFT: super::Opcode = 8u32;
```

### `SIZE_SHIFT`

```rust
const SIZE_SHIFT: super::Opcode = 16u32;
```

### `DIR_SHIFT`

```rust
const DIR_SHIFT: super::Opcode = 30u32;
```

### `NUM_MASK`

```rust
const NUM_MASK: super::Opcode = 255u32;
```

### `GROUP_MASK`

```rust
const GROUP_MASK: super::Opcode = 255u32;
```

### `SIZE_MASK`

```rust
const SIZE_MASK: super::Opcode = 16_383u32;
```

### `DIR_MASK`

```rust
const DIR_MASK: super::Opcode = 3u32;
```

