*[libc](../../../../../index.md) / [new](../../../../index.md) / [linux_uapi](../../../index.md) / [linux](../../index.md) / [can](../index.md) / [raw](index.md)*

---

# Module `raw`

Header: `linux/can/raw.h`

## Contents

- [Modules](#modules)
  - [`bcm`](#bcm)
  - [`j1939`](#j1939)
  - [`raw`](#raw)
- [Structs](#structs)
  - [`can_frame`](#can_frame)
  - [`canfd_frame`](#canfd_frame)
  - [`canxl_frame`](#canxl_frame)
  - [`sockaddr_can`](#sockaddr_can)
  - [`__c_anonymous_sockaddr_can_tp`](#__c_anonymous_sockaddr_can_tp)
  - [`__c_anonymous_sockaddr_can_j1939`](#__c_anonymous_sockaddr_can_j1939)
  - [`can_filter`](#can_filter)
- [Type Aliases](#type-aliases)
  - [`canid_t`](#canid_t)
  - [`can_err_mask_t`](#can_err_mask_t)
- [Constants](#constants)
  - [`SOL_CAN_RAW`](#sol_can_raw)
  - [`CAN_RAW_FILTER_MAX`](#can_raw_filter_max)
  - [`CAN_RAW_FILTER`](#can_raw_filter)
  - [`CAN_RAW_ERR_FILTER`](#can_raw_err_filter)
  - [`CAN_RAW_LOOPBACK`](#can_raw_loopback)
  - [`CAN_RAW_RECV_OWN_MSGS`](#can_raw_recv_own_msgs)
  - [`CAN_RAW_FD_FRAMES`](#can_raw_fd_frames)
  - [`CAN_RAW_JOIN_FILTERS`](#can_raw_join_filters)
  - [`CAN_RAW_XL_FRAMES`](#can_raw_xl_frames)
  - [`CAN_EFF_FLAG`](#can_eff_flag)
  - [`CAN_RTR_FLAG`](#can_rtr_flag)
  - [`CAN_ERR_FLAG`](#can_err_flag)
  - [`CAN_SFF_MASK`](#can_sff_mask)
  - [`CAN_EFF_MASK`](#can_eff_mask)
  - [`CAN_ERR_MASK`](#can_err_mask)
  - [`CANXL_PRIO_MASK`](#canxl_prio_mask)
  - [`CAN_SFF_ID_BITS`](#can_sff_id_bits)
  - [`CAN_EFF_ID_BITS`](#can_eff_id_bits)
  - [`CANXL_PRIO_BITS`](#canxl_prio_bits)
  - [`CAN_MAX_DLC`](#can_max_dlc)
  - [`CAN_MAX_DLEN`](#can_max_dlen)
  - [`CANFD_MAX_DLC`](#canfd_max_dlc)
  - [`CANFD_MAX_DLEN`](#canfd_max_dlen)
  - [`CANXL_MIN_DLC`](#canxl_min_dlc)
  - [`CANXL_MAX_DLC`](#canxl_max_dlc)
  - [`CANXL_MAX_DLC_MASK`](#canxl_max_dlc_mask)
  - [`CANXL_MIN_DLEN`](#canxl_min_dlen)
  - [`CANXL_MAX_DLEN`](#canxl_max_dlen)
  - [`CANFD_BRS`](#canfd_brs)
  - [`CANFD_ESI`](#canfd_esi)
  - [`CANFD_FDF`](#canfd_fdf)
  - [`CANXL_XLF`](#canxl_xlf)
  - [`CANXL_SEC`](#canxl_sec)
  - [`CAN_MTU`](#can_mtu)
  - [`CANFD_MTU`](#canfd_mtu)
  - [`CANXL_MTU`](#canxl_mtu)
  - [`CANXL_HDR_SIZE`](#canxl_hdr_size)
  - [`CANXL_MIN_MTU`](#canxl_min_mtu)
  - [`CANXL_MAX_MTU`](#canxl_max_mtu)
  - [`CAN_RAW`](#can_raw)
  - [`CAN_BCM`](#can_bcm)
  - [`CAN_TP16`](#can_tp16)
  - [`CAN_TP20`](#can_tp20)
  - [`CAN_MCNET`](#can_mcnet)
  - [`CAN_ISOTP`](#can_isotp)
  - [`CAN_J1939`](#can_j1939)
  - [`CAN_NPROTO`](#can_nproto)
  - [`SOL_CAN_BASE`](#sol_can_base)
  - [`CAN_INV_FILTER`](#can_inv_filter)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`bcm`](#bcm) | mod | Header: `linux/can/bcm.h` |
| [`j1939`](#j1939) | mod | `linux/can/j1939.h` |
| [`raw`](#raw) | mod | Header: `linux/can/raw.h` |
| [`can_frame`](#can_frame) | struct |  |
| [`canfd_frame`](#canfd_frame) | struct |  |
| [`canxl_frame`](#canxl_frame) | struct |  |
| [`sockaddr_can`](#sockaddr_can) | struct |  |
| [`__c_anonymous_sockaddr_can_tp`](#__c_anonymous_sockaddr_can_tp) | struct |  |
| [`__c_anonymous_sockaddr_can_j1939`](#__c_anonymous_sockaddr_can_j1939) | struct |  |
| [`can_filter`](#can_filter) | struct |  |
| [`canid_t`](#canid_t) | type |  |
| [`can_err_mask_t`](#can_err_mask_t) | type |  |
| [`SOL_CAN_RAW`](#sol_can_raw) | const |  |
| [`CAN_RAW_FILTER_MAX`](#can_raw_filter_max) | const |  |
| [`CAN_RAW_FILTER`](#can_raw_filter) | const |  |
| [`CAN_RAW_ERR_FILTER`](#can_raw_err_filter) | const |  |
| [`CAN_RAW_LOOPBACK`](#can_raw_loopback) | const |  |
| [`CAN_RAW_RECV_OWN_MSGS`](#can_raw_recv_own_msgs) | const |  |
| [`CAN_RAW_FD_FRAMES`](#can_raw_fd_frames) | const |  |
| [`CAN_RAW_JOIN_FILTERS`](#can_raw_join_filters) | const |  |
| [`CAN_RAW_XL_FRAMES`](#can_raw_xl_frames) | const |  |
| [`CAN_EFF_FLAG`](#can_eff_flag) | const |  |
| [`CAN_RTR_FLAG`](#can_rtr_flag) | const |  |
| [`CAN_ERR_FLAG`](#can_err_flag) | const |  |
| [`CAN_SFF_MASK`](#can_sff_mask) | const |  |
| [`CAN_EFF_MASK`](#can_eff_mask) | const |  |
| [`CAN_ERR_MASK`](#can_err_mask) | const |  |
| [`CANXL_PRIO_MASK`](#canxl_prio_mask) | const |  |
| [`CAN_SFF_ID_BITS`](#can_sff_id_bits) | const |  |
| [`CAN_EFF_ID_BITS`](#can_eff_id_bits) | const |  |
| [`CANXL_PRIO_BITS`](#canxl_prio_bits) | const |  |
| [`CAN_MAX_DLC`](#can_max_dlc) | const |  |
| [`CAN_MAX_DLEN`](#can_max_dlen) | const |  |
| [`CANFD_MAX_DLC`](#canfd_max_dlc) | const |  |
| [`CANFD_MAX_DLEN`](#canfd_max_dlen) | const |  |
| [`CANXL_MIN_DLC`](#canxl_min_dlc) | const |  |
| [`CANXL_MAX_DLC`](#canxl_max_dlc) | const |  |
| [`CANXL_MAX_DLC_MASK`](#canxl_max_dlc_mask) | const |  |
| [`CANXL_MIN_DLEN`](#canxl_min_dlen) | const |  |
| [`CANXL_MAX_DLEN`](#canxl_max_dlen) | const |  |
| [`CANFD_BRS`](#canfd_brs) | const |  |
| [`CANFD_ESI`](#canfd_esi) | const |  |
| [`CANFD_FDF`](#canfd_fdf) | const |  |
| [`CANXL_XLF`](#canxl_xlf) | const |  |
| [`CANXL_SEC`](#canxl_sec) | const |  |
| [`CAN_MTU`](#can_mtu) | const |  |
| [`CANFD_MTU`](#canfd_mtu) | const |  |
| [`CANXL_MTU`](#canxl_mtu) | const |  |
| [`CANXL_HDR_SIZE`](#canxl_hdr_size) | const |  |
| [`CANXL_MIN_MTU`](#canxl_min_mtu) | const |  |
| [`CANXL_MAX_MTU`](#canxl_max_mtu) | const |  |
| [`CAN_RAW`](#can_raw) | const |  |
| [`CAN_BCM`](#can_bcm) | const |  |
| [`CAN_TP16`](#can_tp16) | const |  |
| [`CAN_TP20`](#can_tp20) | const |  |
| [`CAN_MCNET`](#can_mcnet) | const |  |
| [`CAN_ISOTP`](#can_isotp) | const |  |
| [`CAN_J1939`](#can_j1939) | const |  |
| [`CAN_NPROTO`](#can_nproto) | const |  |
| [`SOL_CAN_BASE`](#sol_can_base) | const |  |
| [`CAN_INV_FILTER`](#can_inv_filter) | const |  |

## Modules

- [`bcm`](bcm/index.md) - Header: `linux/can/bcm.h`
- [`j1939`](j1939/index.md) - `linux/can/j1939.h`
- [`raw`](raw/index.md) - Header: `linux/can/raw.h`

## Structs

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

- <span id="can-frame-clone"></span>`fn clone(&self) -> can_frame` — [`can_frame`](../../../../index.md)

##### `impl Copy for can_frame`

##### `impl Debug for can_frame`

- <span id="can-frame-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="canfd-frame-clone"></span>`fn clone(&self) -> canfd_frame` — [`canfd_frame`](../../../../index.md)

##### `impl Copy for canfd_frame`

##### `impl Debug for canfd_frame`

- <span id="canfd-frame-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="canxl-frame-clone"></span>`fn clone(&self) -> canxl_frame` — [`canxl_frame`](../../../../index.md)

##### `impl Copy for canxl_frame`

##### `impl Debug for canxl_frame`

- <span id="canxl-frame-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="sockaddr-can-clone"></span>`fn clone(&self) -> sockaddr_can` — [`sockaddr_can`](../../../../index.md)

##### `impl Copy for sockaddr_can`

##### `impl Debug for sockaddr_can`

- <span id="sockaddr-can-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_sockaddr_can_tp`

```rust
struct __c_anonymous_sockaddr_can_tp {
    pub rx_id: canid_t,
    pub tx_id: canid_t,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-clone"></span>`fn clone(&self) -> __c_anonymous_sockaddr_can_tp` — [`__c_anonymous_sockaddr_can_tp`](../../../../index.md)

##### `impl Copy for __c_anonymous_sockaddr_can_tp`

##### `impl Debug for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="c-anonymous-sockaddr-can-j1939-clone"></span>`fn clone(&self) -> __c_anonymous_sockaddr_can_j1939` — [`__c_anonymous_sockaddr_can_j1939`](../../../../index.md)

##### `impl Copy for __c_anonymous_sockaddr_can_j1939`

##### `impl Debug for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `can_filter`

```rust
struct can_filter {
    pub can_id: canid_t,
    pub can_mask: canid_t,
}
```

#### Trait Implementations

##### `impl Clone for can_filter`

- <span id="can-filter-clone"></span>`fn clone(&self) -> can_filter` — [`can_filter`](../../../../index.md)

##### `impl Copy for can_filter`

##### `impl Debug for can_filter`

- <span id="can-filter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `canid_t`

```rust
type canid_t = u32;
```

### `can_err_mask_t`

```rust
type can_err_mask_t = u32;
```

## Constants

### `SOL_CAN_RAW`

```rust
const SOL_CAN_RAW: crate::c_int = 101i32;
```

### `CAN_RAW_FILTER_MAX`

```rust
const CAN_RAW_FILTER_MAX: crate::c_int = 512i32;
```

### `CAN_RAW_FILTER`

```rust
const CAN_RAW_FILTER: crate::c_int = 1i32;
```

### `CAN_RAW_ERR_FILTER`

```rust
const CAN_RAW_ERR_FILTER: crate::c_int = 2i32;
```

### `CAN_RAW_LOOPBACK`

```rust
const CAN_RAW_LOOPBACK: crate::c_int = 3i32;
```

### `CAN_RAW_RECV_OWN_MSGS`

```rust
const CAN_RAW_RECV_OWN_MSGS: crate::c_int = 4i32;
```

### `CAN_RAW_FD_FRAMES`

```rust
const CAN_RAW_FD_FRAMES: crate::c_int = 5i32;
```

### `CAN_RAW_JOIN_FILTERS`

```rust
const CAN_RAW_JOIN_FILTERS: crate::c_int = 6i32;
```

### `CAN_RAW_XL_FRAMES`

```rust
const CAN_RAW_XL_FRAMES: crate::c_int = 7i32;
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

