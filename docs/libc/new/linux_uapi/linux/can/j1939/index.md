*[libc](../../../../../index.md) / [new](../../../../index.md) / [linux_uapi](../../../index.md) / [linux](../../index.md) / [can](../index.md) / [j1939](index.md)*

---

# Module `j1939`

`linux/can/j1939.h`

## Modules

- [`bcm`](bcm/index.md) - Header: `linux/can/bcm.h`
- [`j1939`](j1939/index.md) - `linux/can/j1939.h`
- [`raw`](raw/index.md) - Header: `linux/can/raw.h`

## Structs

### `j1939_filter`

```rust
struct j1939_filter {
    pub name: name_t,
    pub name_mask: name_t,
    pub pgn: pgn_t,
    pub pgn_mask: pgn_t,
    pub addr: u8,
    pub addr_mask: u8,
}
```

#### Trait Implementations

##### `impl Clone for j1939_filter`

- `fn clone(self: &Self) -> j1939_filter` — [`j1939_filter`](../../../../index.md)

##### `impl Copy for j1939_filter`

##### `impl Debug for j1939_filter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `can_frame`

```rust
struct can_frame {
    pub can_id: canid_t,
    pub can_dlc: u8,
    __pad: crate::types::Padding<u8>,
    __res0: u8,
    pub len8_dlc: u8,
    pub data: [u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for can_frame`

- `fn clone(self: &Self) -> can_frame` — [`can_frame`](../../../../index.md)

##### `impl Copy for can_frame`

##### `impl Debug for can_frame`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `canfd_frame`

```rust
struct canfd_frame {
    pub can_id: canid_t,
    pub len: u8,
    pub flags: u8,
    __res0: u8,
    __res1: u8,
    pub data: [u8; 64],
}
```

#### Trait Implementations

##### `impl Clone for canfd_frame`

- `fn clone(self: &Self) -> canfd_frame` — [`canfd_frame`](../../../../index.md)

##### `impl Copy for canfd_frame`

##### `impl Debug for canfd_frame`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `canxl_frame`

```rust
struct canxl_frame {
    pub prio: canid_t,
    pub flags: u8,
    pub sdt: u8,
    pub len: u16,
    pub af: u32,
    pub data: [u8; 2048],
}
```

#### Trait Implementations

##### `impl Clone for canxl_frame`

- `fn clone(self: &Self) -> canxl_frame` — [`canxl_frame`](../../../../index.md)

##### `impl Copy for canxl_frame`

##### `impl Debug for canxl_frame`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr_can`

```rust
struct sockaddr_can {
    pub can_family: crate::sa_family_t,
    pub can_ifindex: crate::c_int,
    pub can_addr: __c_anonymous_sockaddr_can_can_addr,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_can`

- `fn clone(self: &Self) -> sockaddr_can` — [`sockaddr_can`](../../../../index.md)

##### `impl Copy for sockaddr_can`

##### `impl Debug for sockaddr_can`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__c_anonymous_sockaddr_can_tp`

```rust
struct __c_anonymous_sockaddr_can_tp {
    pub rx_id: canid_t,
    pub tx_id: canid_t,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_sockaddr_can_tp`

- `fn clone(self: &Self) -> __c_anonymous_sockaddr_can_tp` — [`__c_anonymous_sockaddr_can_tp`](../../../../index.md)

##### `impl Copy for __c_anonymous_sockaddr_can_tp`

##### `impl Debug for __c_anonymous_sockaddr_can_tp`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__c_anonymous_sockaddr_can_j1939`

```rust
struct __c_anonymous_sockaddr_can_j1939 {
    pub name: u64,
    pub pgn: u32,
    pub addr: u8,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_sockaddr_can_j1939`

- `fn clone(self: &Self) -> __c_anonymous_sockaddr_can_j1939` — [`__c_anonymous_sockaddr_can_j1939`](../../../../index.md)

##### `impl Copy for __c_anonymous_sockaddr_can_j1939`

##### `impl Debug for __c_anonymous_sockaddr_can_j1939`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `can_filter`

```rust
struct can_filter {
    pub can_id: canid_t,
    pub can_mask: canid_t,
}
```

#### Trait Implementations

##### `impl Clone for can_filter`

- `fn clone(self: &Self) -> can_filter` — [`can_filter`](../../../../index.md)

##### `impl Copy for can_filter`

##### `impl Debug for can_filter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Type Aliases

### `pgn_t`

```rust
type pgn_t = u32;
```

### `priority_t`

```rust
type priority_t = u8;
```

### `name_t`

```rust
type name_t = u64;
```

### `canid_t`

```rust
type canid_t = u32;
```

### `can_err_mask_t`

```rust
type can_err_mask_t = u32;
```

## Constants

### `J1939_MAX_UNICAST_ADDR`

```rust
const J1939_MAX_UNICAST_ADDR: crate::c_uchar = 253u8;
```

### `J1939_IDLE_ADDR`

```rust
const J1939_IDLE_ADDR: crate::c_uchar = 254u8;
```

### `J1939_NO_ADDR`

```rust
const J1939_NO_ADDR: crate::c_uchar = 255u8;
```

### `J1939_NO_NAME`

```rust
const J1939_NO_NAME: crate::c_ulong = 0u64;
```

### `J1939_PGN_REQUEST`

```rust
const J1939_PGN_REQUEST: crate::c_uint = 59_904u32;
```

### `J1939_PGN_ADDRESS_CLAIMED`

```rust
const J1939_PGN_ADDRESS_CLAIMED: crate::c_uint = 60_928u32;
```

### `J1939_PGN_ADDRESS_COMMANDED`

```rust
const J1939_PGN_ADDRESS_COMMANDED: crate::c_uint = 65_240u32;
```

### `J1939_PGN_PDU1_MAX`

```rust
const J1939_PGN_PDU1_MAX: crate::c_uint = 261_888u32;
```

### `J1939_PGN_MAX`

```rust
const J1939_PGN_MAX: crate::c_uint = 262_143u32;
```

### `J1939_NO_PGN`

```rust
const J1939_NO_PGN: crate::c_uint = 262_144u32;
```

### `SOL_CAN_J1939`

```rust
const SOL_CAN_J1939: crate::c_int = 107i32;
```

### `SO_J1939_FILTER`

```rust
const SO_J1939_FILTER: crate::c_int = 1i32;
```

### `SO_J1939_PROMISC`

```rust
const SO_J1939_PROMISC: crate::c_int = 2i32;
```

### `SO_J1939_SEND_PRIO`

```rust
const SO_J1939_SEND_PRIO: crate::c_int = 3i32;
```

### `SO_J1939_ERRQUEUE`

```rust
const SO_J1939_ERRQUEUE: crate::c_int = 4i32;
```

### `SCM_J1939_DEST_ADDR`

```rust
const SCM_J1939_DEST_ADDR: crate::c_int = 1i32;
```

### `SCM_J1939_DEST_NAME`

```rust
const SCM_J1939_DEST_NAME: crate::c_int = 2i32;
```

### `SCM_J1939_PRIO`

```rust
const SCM_J1939_PRIO: crate::c_int = 3i32;
```

### `SCM_J1939_ERRQUEUE`

```rust
const SCM_J1939_ERRQUEUE: crate::c_int = 4i32;
```

### `J1939_NLA_PAD`

```rust
const J1939_NLA_PAD: crate::c_int = 0i32;
```

### `J1939_NLA_BYTES_ACKED`

```rust
const J1939_NLA_BYTES_ACKED: crate::c_int = 1i32;
```

### `J1939_NLA_TOTAL_SIZE`

```rust
const J1939_NLA_TOTAL_SIZE: crate::c_int = 2i32;
```

### `J1939_NLA_PGN`

```rust
const J1939_NLA_PGN: crate::c_int = 3i32;
```

### `J1939_NLA_SRC_NAME`

```rust
const J1939_NLA_SRC_NAME: crate::c_int = 4i32;
```

### `J1939_NLA_DEST_NAME`

```rust
const J1939_NLA_DEST_NAME: crate::c_int = 5i32;
```

### `J1939_NLA_SRC_ADDR`

```rust
const J1939_NLA_SRC_ADDR: crate::c_int = 6i32;
```

### `J1939_NLA_DEST_ADDR`

```rust
const J1939_NLA_DEST_ADDR: crate::c_int = 7i32;
```

### `J1939_EE_INFO_NONE`

```rust
const J1939_EE_INFO_NONE: crate::c_int = 0i32;
```

### `J1939_EE_INFO_TX_ABORT`

```rust
const J1939_EE_INFO_TX_ABORT: crate::c_int = 1i32;
```

### `J1939_EE_INFO_RX_RTS`

```rust
const J1939_EE_INFO_RX_RTS: crate::c_int = 2i32;
```

### `J1939_EE_INFO_RX_DPO`

```rust
const J1939_EE_INFO_RX_DPO: crate::c_int = 3i32;
```

### `J1939_EE_INFO_RX_ABORT`

```rust
const J1939_EE_INFO_RX_ABORT: crate::c_int = 4i32;
```

### `J1939_FILTER_MAX`

```rust
const J1939_FILTER_MAX: crate::c_int = 512i32;
```

### `CAN_EFF_FLAG`

```rust
const CAN_EFF_FLAG: canid_t = 2_147_483_648u32;
```

### `CAN_RTR_FLAG`

```rust
const CAN_RTR_FLAG: canid_t = 1_073_741_824u32;
```

### `CAN_ERR_FLAG`

```rust
const CAN_ERR_FLAG: canid_t = 536_870_912u32;
```

### `CAN_SFF_MASK`

```rust
const CAN_SFF_MASK: canid_t = 2_047u32;
```

### `CAN_EFF_MASK`

```rust
const CAN_EFF_MASK: canid_t = 536_870_911u32;
```

### `CAN_ERR_MASK`

```rust
const CAN_ERR_MASK: canid_t = 536_870_911u32;
```

### `CANXL_PRIO_MASK`

```rust
const CANXL_PRIO_MASK: crate::canid_t = 2_047u32;
```

### `CAN_SFF_ID_BITS`

```rust
const CAN_SFF_ID_BITS: crate::c_int = 11i32;
```

### `CAN_EFF_ID_BITS`

```rust
const CAN_EFF_ID_BITS: crate::c_int = 29i32;
```

### `CANXL_PRIO_BITS`

```rust
const CANXL_PRIO_BITS: crate::c_int = 11i32;
```

### `CAN_MAX_DLC`

```rust
const CAN_MAX_DLC: crate::c_int = 8i32;
```

### `CAN_MAX_DLEN`

```rust
const CAN_MAX_DLEN: usize = 8usize;
```

### `CANFD_MAX_DLC`

```rust
const CANFD_MAX_DLC: crate::c_int = 15i32;
```

### `CANFD_MAX_DLEN`

```rust
const CANFD_MAX_DLEN: usize = 64usize;
```

### `CANXL_MIN_DLC`

```rust
const CANXL_MIN_DLC: crate::c_int = 0i32;
```

### `CANXL_MAX_DLC`

```rust
const CANXL_MAX_DLC: crate::c_int = 2_047i32;
```

### `CANXL_MAX_DLC_MASK`

```rust
const CANXL_MAX_DLC_MASK: crate::c_int = 2_047i32;
```

### `CANXL_MIN_DLEN`

```rust
const CANXL_MIN_DLEN: usize = 1usize;
```

### `CANXL_MAX_DLEN`

```rust
const CANXL_MAX_DLEN: usize = 2_048usize;
```

### `CANFD_BRS`

```rust
const CANFD_BRS: crate::c_int = 1i32;
```

### `CANFD_ESI`

```rust
const CANFD_ESI: crate::c_int = 2i32;
```

### `CANFD_FDF`

```rust
const CANFD_FDF: crate::c_int = 4i32;
```

### `CANXL_XLF`

```rust
const CANXL_XLF: crate::c_int = 128i32;
```

### `CANXL_SEC`

```rust
const CANXL_SEC: crate::c_int = 1i32;
```

### `CAN_MTU`

```rust
const CAN_MTU: usize = 16usize;
```

### `CANFD_MTU`

```rust
const CANFD_MTU: usize = 72usize;
```

### `CANXL_MTU`

```rust
const CANXL_MTU: usize = 2_060usize;
```

### `CANXL_HDR_SIZE`

```rust
const CANXL_HDR_SIZE: usize = 12usize;
```

### `CANXL_MIN_MTU`

```rust
const CANXL_MIN_MTU: usize = 76usize;
```

### `CANXL_MAX_MTU`

```rust
const CANXL_MAX_MTU: usize = 2_060usize;
```

### `CAN_RAW`

```rust
const CAN_RAW: crate::c_int = 1i32;
```

### `CAN_BCM`

```rust
const CAN_BCM: crate::c_int = 2i32;
```

### `CAN_TP16`

```rust
const CAN_TP16: crate::c_int = 3i32;
```

### `CAN_TP20`

```rust
const CAN_TP20: crate::c_int = 4i32;
```

### `CAN_MCNET`

```rust
const CAN_MCNET: crate::c_int = 5i32;
```

### `CAN_ISOTP`

```rust
const CAN_ISOTP: crate::c_int = 6i32;
```

### `CAN_J1939`

```rust
const CAN_J1939: crate::c_int = 7i32;
```

### `CAN_NPROTO`

```rust
const CAN_NPROTO: crate::c_int = 8i32;
```

### `SOL_CAN_BASE`

```rust
const SOL_CAN_BASE: crate::c_int = 100i32;
```

### `CAN_INV_FILTER`

```rust
const CAN_INV_FILTER: canid_t = 536_870_912u32;
```

