*[rustix](../../../index.md) / [termios](../../index.md) / [types](../index.md) / [speed](index.md)*

---

# Module `speed`

Speeds for use with `Termios::set_speed`, `Termios::set_input_speed`,
and `Termios::set_output_speed`.

Unlike in some platforms' libc APIs, these always have the same numerical
value as their names; for example, `B50` has the value `50`, and so on.
Consequently, it's not necessary to use them. They are provided here
because they help identify speeds which are likely to be supported, on
platforms and devices which don't support arbitrary speeds.

## Contents

- [Functions](#functions)
  - [`decode`](#decode)
  - [`encode`](#encode)
- [Constants](#constants)
  - [`B0`](#b0)
  - [`B50`](#b50)
  - [`B75`](#b75)
  - [`B110`](#b110)
  - [`B134`](#b134)
  - [`B150`](#b150)
  - [`B200`](#b200)
  - [`B300`](#b300)
  - [`B600`](#b600)
  - [`B1200`](#b1200)
  - [`B1800`](#b1800)
  - [`B2400`](#b2400)
  - [`B4800`](#b4800)
  - [`B9600`](#b9600)
  - [`B19200`](#b19200)
  - [`B38400`](#b38400)
  - [`B57600`](#b57600)
  - [`B115200`](#b115200)
  - [`B230400`](#b230400)
  - [`B460800`](#b460800)
  - [`B500000`](#b500000)
  - [`B576000`](#b576000)
  - [`B921600`](#b921600)
  - [`B1000000`](#b1000000)
  - [`B1152000`](#b1152000)
  - [`B1500000`](#b1500000)
  - [`B2000000`](#b2000000)
  - [`B2500000`](#b2500000)
  - [`B3000000`](#b3000000)
  - [`B3500000`](#b3500000)
  - [`B4000000`](#b4000000)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`decode`](#decode) | fn | Translate from a `c::speed_t` code to an arbitrary integer speed value `u32`. |
| [`encode`](#encode) | fn | Translate from an arbitrary `u32` arbitrary integer speed value to a `c::speed_t` code. |
| [`B0`](#b0) | const | `B0` |
| [`B50`](#b50) | const | `B50` |
| [`B75`](#b75) | const | `B75` |
| [`B110`](#b110) | const | `B110` |
| [`B134`](#b134) | const | `B134` |
| [`B150`](#b150) | const | `B150` |
| [`B200`](#b200) | const | `B200` |
| [`B300`](#b300) | const | `B300` |
| [`B600`](#b600) | const | `B600` |
| [`B1200`](#b1200) | const | `B1200` |
| [`B1800`](#b1800) | const | `B1800` |
| [`B2400`](#b2400) | const | `B2400` |
| [`B4800`](#b4800) | const | `B4800` |
| [`B9600`](#b9600) | const | `B9600` |
| [`B19200`](#b19200) | const | `B19200` |
| [`B38400`](#b38400) | const | `B38400` |
| [`B57600`](#b57600) | const | `B57600` |
| [`B115200`](#b115200) | const | `B115200` |
| [`B230400`](#b230400) | const | `B230400` |
| [`B460800`](#b460800) | const | `B460800` |
| [`B500000`](#b500000) | const | `B500000` |
| [`B576000`](#b576000) | const | `B576000` |
| [`B921600`](#b921600) | const | `B921600` |
| [`B1000000`](#b1000000) | const | `B1000000` |
| [`B1152000`](#b1152000) | const | `B1152000` |
| [`B1500000`](#b1500000) | const | `B1500000` |
| [`B2000000`](#b2000000) | const | `B2000000` |
| [`B2500000`](#b2500000) | const | `B2500000` |
| [`B3000000`](#b3000000) | const | `B3000000` |
| [`B3500000`](#b3500000) | const | `B3500000` |
| [`B4000000`](#b4000000) | const | `B4000000` |

## Functions

### `decode`

```rust
const fn decode(encoded_speed: linux_raw_sys::general::speed_t) -> Option<u32>
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:844-978`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L844-L978)*

Translate from a `c::speed_t` code to an arbitrary integer speed value
`u32`.

On BSD platforms, integer speed values are already the same as their
encoded values.

On Linux on PowerPC, `TCGETS`/`TCSETS` support the `c_ispeed` and
`c_ospeed` fields.

On Linux on architectures other than PowerPC, `TCGETS`/`TCSETS` don't
support the `c_ispeed` and `c_ospeed` fields, so we have to fall back
to `TCGETS2`/`TCSETS2` to support them.

### `encode`

```rust
const fn encode(speed: u32) -> Option<linux_raw_sys::general::speed_t>
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:983-1117`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L983-L1117)*

Translate from an arbitrary `u32` arbitrary integer speed value to a
`c::speed_t` code.

## Constants

### `B0`
```rust
const B0: u32 = 0u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:679`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L679)*

`B0`

### `B50`
```rust
const B50: u32 = 50u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:682`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L682)*

`B50`

### `B75`
```rust
const B75: u32 = 75u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:685`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L685)*

`B75`

### `B110`
```rust
const B110: u32 = 110u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:688`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L688)*

`B110`

### `B134`
```rust
const B134: u32 = 134u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:691`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L691)*

`B134`

### `B150`
```rust
const B150: u32 = 150u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:694`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L694)*

`B150`

### `B200`
```rust
const B200: u32 = 200u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:697`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L697)*

`B200`

### `B300`
```rust
const B300: u32 = 300u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:700`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L700)*

`B300`

### `B600`
```rust
const B600: u32 = 600u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:703`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L703)*

`B600`

### `B1200`
```rust
const B1200: u32 = 1_200u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:706`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L706)*

`B1200`

### `B1800`
```rust
const B1800: u32 = 1_800u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:709`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L709)*

`B1800`

### `B2400`
```rust
const B2400: u32 = 2_400u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:712`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L712)*

`B2400`

### `B4800`
```rust
const B4800: u32 = 4_800u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:715`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L715)*

`B4800`

### `B9600`
```rust
const B9600: u32 = 9_600u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:718`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L718)*

`B9600`

### `B19200`
```rust
const B19200: u32 = 19_200u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:722`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L722)*

`B19200`

### `B38400`
```rust
const B38400: u32 = 38_400u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:726`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L726)*

`B38400`

### `B57600`
```rust
const B57600: u32 = 57_600u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:730`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L730)*

`B57600`

### `B115200`
```rust
const B115200: u32 = 115_200u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:734`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L734)*

`B115200`

### `B230400`
```rust
const B230400: u32 = 230_400u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:738`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L738)*

`B230400`

### `B460800`
```rust
const B460800: u32 = 460_800u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:748`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L748)*

`B460800`

### `B500000`
```rust
const B500000: u32 = 500_000u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:752`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L752)*

`B500000`

### `B576000`
```rust
const B576000: u32 = 576_000u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:756`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L756)*

`B576000`

### `B921600`
```rust
const B921600: u32 = 921_600u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:766`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L766)*

`B921600`

### `B1000000`
```rust
const B1000000: u32 = 1_000_000u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:770`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L770)*

`B1000000`

### `B1152000`
```rust
const B1152000: u32 = 1_152_000u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:774`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L774)*

`B1152000`

### `B1500000`
```rust
const B1500000: u32 = 1_500_000u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:778`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L778)*

`B1500000`

### `B2000000`
```rust
const B2000000: u32 = 2_000_000u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:782`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L782)*

`B2000000`

### `B2500000`
```rust
const B2500000: u32 = 2_500_000u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:793`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L793)*

`B2500000`

### `B3000000`
```rust
const B3000000: u32 = 3_000_000u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:804`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L804)*

`B3000000`

### `B3500000`
```rust
const B3500000: u32 = 3_500_000u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:815`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L815)*

`B3500000`

### `B4000000`
```rust
const B4000000: u32 = 4_000_000u32;
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:826`](../../../../../.source_1765210505/rustix-1.1.2/src/termios/types.rs#L826)*

`B4000000`

