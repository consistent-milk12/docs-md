*[libc](../../../../../index.md) / [new](../../../../index.md) / [linux_uapi](../../../index.md) / [linux](../../index.md) / [can](../index.md) / [bcm](index.md)*

---

# Module `bcm`

Header: `linux/can/bcm.h`

## Contents

- [Modules](#modules)
  - [`bcm`](#bcm)
  - [`j1939`](#j1939)
  - [`raw`](#raw)
- [Structs](#structs)
  - [`bcm_timeval`](#bcm_timeval)
  - [`bcm_msg_head`](#bcm_msg_head)
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
  - [`TX_SETUP`](#tx_setup)
  - [`TX_DELETE`](#tx_delete)
  - [`TX_READ`](#tx_read)
  - [`TX_SEND`](#tx_send)
  - [`RX_SETUP`](#rx_setup)
  - [`RX_DELETE`](#rx_delete)
  - [`RX_READ`](#rx_read)
  - [`TX_STATUS`](#tx_status)
  - [`TX_EXPIRED`](#tx_expired)
  - [`RX_STATUS`](#rx_status)
  - [`RX_TIMEOUT`](#rx_timeout)
  - [`RX_CHANGED`](#rx_changed)
  - [`SETTIMER`](#settimer)
  - [`STARTTIMER`](#starttimer)
  - [`TX_COUNTEVT`](#tx_countevt)
  - [`TX_ANNOUNCE`](#tx_announce)
  - [`TX_CP_CAN_ID`](#tx_cp_can_id)
  - [`RX_FILTER_ID`](#rx_filter_id)
  - [`RX_CHECK_DLC`](#rx_check_dlc)
  - [`RX_NO_AUTOTIMER`](#rx_no_autotimer)
  - [`RX_ANNOUNCE_RESUME`](#rx_announce_resume)
  - [`TX_RESET_MULTI_IDX`](#tx_reset_multi_idx)
  - [`RX_RTR_FRAME`](#rx_rtr_frame)
  - [`CAN_FD_FRAME`](#can_fd_frame)
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
| [`bcm_timeval`](#bcm_timeval) | struct |  |
| [`bcm_msg_head`](#bcm_msg_head) | struct |  |
| [`can_frame`](#can_frame) | struct |  |
| [`canfd_frame`](#canfd_frame) | struct |  |
| [`canxl_frame`](#canxl_frame) | struct |  |
| [`sockaddr_can`](#sockaddr_can) | struct |  |
| [`__c_anonymous_sockaddr_can_tp`](#__c_anonymous_sockaddr_can_tp) | struct |  |
| [`__c_anonymous_sockaddr_can_j1939`](#__c_anonymous_sockaddr_can_j1939) | struct |  |
| [`can_filter`](#can_filter) | struct |  |
| [`canid_t`](#canid_t) | type |  |
| [`can_err_mask_t`](#can_err_mask_t) | type |  |
| [`TX_SETUP`](#tx_setup) | const |  |
| [`TX_DELETE`](#tx_delete) | const |  |
| [`TX_READ`](#tx_read) | const |  |
| [`TX_SEND`](#tx_send) | const |  |
| [`RX_SETUP`](#rx_setup) | const |  |
| [`RX_DELETE`](#rx_delete) | const |  |
| [`RX_READ`](#rx_read) | const |  |
| [`TX_STATUS`](#tx_status) | const |  |
| [`TX_EXPIRED`](#tx_expired) | const |  |
| [`RX_STATUS`](#rx_status) | const |  |
| [`RX_TIMEOUT`](#rx_timeout) | const |  |
| [`RX_CHANGED`](#rx_changed) | const |  |
| [`SETTIMER`](#settimer) | const |  |
| [`STARTTIMER`](#starttimer) | const |  |
| [`TX_COUNTEVT`](#tx_countevt) | const |  |
| [`TX_ANNOUNCE`](#tx_announce) | const |  |
| [`TX_CP_CAN_ID`](#tx_cp_can_id) | const |  |
| [`RX_FILTER_ID`](#rx_filter_id) | const |  |
| [`RX_CHECK_DLC`](#rx_check_dlc) | const |  |
| [`RX_NO_AUTOTIMER`](#rx_no_autotimer) | const |  |
| [`RX_ANNOUNCE_RESUME`](#rx_announce_resume) | const |  |
| [`TX_RESET_MULTI_IDX`](#tx_reset_multi_idx) | const |  |
| [`RX_RTR_FRAME`](#rx_rtr_frame) | const |  |
| [`CAN_FD_FRAME`](#can_fd_frame) | const |  |
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

- [`bcm`](bcm/index.md) — Header: `linux/can/bcm.h`
- [`j1939`](j1939/index.md) — `linux/can/j1939.h`
- [`raw`](raw/index.md) — Header: `linux/can/raw.h`

## Structs

### `bcm_timeval`

```rust
struct bcm_timeval {
    pub tv_sec: crate::c_long,
    pub tv_usec: crate::c_long,
}
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:5-21`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L5-L21)*

#### Trait Implementations

##### `impl Clone for bcm_timeval`

- <span id="bcm-timeval-clone"></span>`fn clone(&self) -> bcm_timeval` — [`bcm_timeval`](../../../../index.md)

##### `impl Copy for bcm_timeval`

##### `impl Debug for bcm_timeval`

- <span id="bcm-timeval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `bcm_msg_head`

```rust
struct bcm_msg_head {
    pub opcode: u32,
    pub flags: u32,
    pub count: u32,
    pub ival1: bcm_timeval,
    pub ival2: bcm_timeval,
    pub can_id: canid_t,
    pub nframes: u32,
    pub frames: [can_frame; 0],
}
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:5-21`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L5-L21)*

#### Trait Implementations

##### `impl Clone for bcm_msg_head`

- <span id="bcm-msg-head-clone"></span>`fn clone(&self) -> bcm_msg_head` — [`bcm_msg_head`](../../../../index.md)

##### `impl Copy for bcm_msg_head`

##### `impl Debug for bcm_msg_head`

- <span id="bcm-msg-head-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:38-49`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L38-L49)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:55-65`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L55-L65)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:70-79`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L70-L79)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:102-113`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L102-L113)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:115-131`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L115-L131)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:115-131`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L115-L131)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:115-131`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L115-L131)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:18`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L18)*

### `can_err_mask_t`

```rust
type can_err_mask_t = u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:24`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L24)*

## Constants

### `TX_SETUP`
```rust
const TX_SETUP: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `TX_DELETE`
```rust
const TX_DELETE: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `TX_READ`
```rust
const TX_READ: u32 = 3u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `TX_SEND`
```rust
const TX_SEND: u32 = 4u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_SETUP`
```rust
const RX_SETUP: u32 = 5u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_DELETE`
```rust
const RX_DELETE: u32 = 6u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_READ`
```rust
const RX_READ: u32 = 7u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `TX_STATUS`
```rust
const TX_STATUS: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `TX_EXPIRED`
```rust
const TX_EXPIRED: u32 = 9u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_STATUS`
```rust
const RX_STATUS: u32 = 10u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_TIMEOUT`
```rust
const RX_TIMEOUT: u32 = 11u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_CHANGED`
```rust
const RX_CHANGED: u32 = 12u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `SETTIMER`
```rust
const SETTIMER: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:41`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L41)*

### `STARTTIMER`
```rust
const STARTTIMER: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:42`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L42)*

### `TX_COUNTEVT`
```rust
const TX_COUNTEVT: u32 = 4u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:43`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L43)*

### `TX_ANNOUNCE`
```rust
const TX_ANNOUNCE: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:44`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L44)*

### `TX_CP_CAN_ID`
```rust
const TX_CP_CAN_ID: u32 = 16u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:45`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L45)*

### `RX_FILTER_ID`
```rust
const RX_FILTER_ID: u32 = 32u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:46`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L46)*

### `RX_CHECK_DLC`
```rust
const RX_CHECK_DLC: u32 = 64u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:47`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L47)*

### `RX_NO_AUTOTIMER`
```rust
const RX_NO_AUTOTIMER: u32 = 128u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:48`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L48)*

### `RX_ANNOUNCE_RESUME`
```rust
const RX_ANNOUNCE_RESUME: u32 = 256u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:49`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L49)*

### `TX_RESET_MULTI_IDX`
```rust
const TX_RESET_MULTI_IDX: u32 = 512u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:50`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L50)*

### `RX_RTR_FRAME`
```rust
const RX_RTR_FRAME: u32 = 1_024u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:51`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L51)*

### `CAN_FD_FRAME`
```rust
const CAN_FD_FRAME: u32 = 2_048u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:52`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L52)*

### `CAN_EFF_FLAG`
```rust
const CAN_EFF_FLAG: canid_t = 2_147_483_648u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:9`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L9)*

### `CAN_RTR_FLAG`
```rust
const CAN_RTR_FLAG: canid_t = 1_073_741_824u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:10`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L10)*

### `CAN_ERR_FLAG`
```rust
const CAN_ERR_FLAG: canid_t = 536_870_912u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:11`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L11)*

### `CAN_SFF_MASK`
```rust
const CAN_SFF_MASK: canid_t = 2_047u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:13`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L13)*

### `CAN_EFF_MASK`
```rust
const CAN_EFF_MASK: canid_t = 536_870_911u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:14`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L14)*

### `CAN_ERR_MASK`
```rust
const CAN_ERR_MASK: canid_t = 536_870_911u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:15`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L15)*

### `CANXL_PRIO_MASK`
```rust
const CANXL_PRIO_MASK: crate::canid_t = 2_047u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:16`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L16)*

### `CAN_SFF_ID_BITS`
```rust
const CAN_SFF_ID_BITS: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:20`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L20)*

### `CAN_EFF_ID_BITS`
```rust
const CAN_EFF_ID_BITS: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:21`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L21)*

### `CANXL_PRIO_BITS`
```rust
const CANXL_PRIO_BITS: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:22`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L22)*

### `CAN_MAX_DLC`
```rust
const CAN_MAX_DLC: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:26`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L26)*

### `CAN_MAX_DLEN`
```rust
const CAN_MAX_DLEN: usize = 8usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:27`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L27)*

### `CANFD_MAX_DLC`
```rust
const CANFD_MAX_DLC: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:29`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L29)*

### `CANFD_MAX_DLEN`
```rust
const CANFD_MAX_DLEN: usize = 64usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:30`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L30)*

### `CANXL_MIN_DLC`
```rust
const CANXL_MIN_DLC: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:32`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L32)*

### `CANXL_MAX_DLC`
```rust
const CANXL_MAX_DLC: crate::c_int = 2_047i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:33`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L33)*

### `CANXL_MAX_DLC_MASK`
```rust
const CANXL_MAX_DLC_MASK: crate::c_int = 2_047i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:34`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L34)*

### `CANXL_MIN_DLEN`
```rust
const CANXL_MIN_DLEN: usize = 1usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:35`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L35)*

### `CANXL_MAX_DLEN`
```rust
const CANXL_MAX_DLEN: usize = 2_048usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:36`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L36)*

### `CANFD_BRS`
```rust
const CANFD_BRS: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:51`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L51)*

### `CANFD_ESI`
```rust
const CANFD_ESI: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:52`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L52)*

### `CANFD_FDF`
```rust
const CANFD_FDF: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:53`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L53)*

### `CANXL_XLF`
```rust
const CANXL_XLF: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:67`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L67)*

### `CANXL_SEC`
```rust
const CANXL_SEC: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:68`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L68)*

### `CAN_MTU`
```rust
const CAN_MTU: usize = 16usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:81`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L81)*

### `CANFD_MTU`
```rust
const CANFD_MTU: usize = 72usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:82`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L82)*

### `CANXL_MTU`
```rust
const CANXL_MTU: usize = 2_060usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:83`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L83)*

### `CANXL_HDR_SIZE`
```rust
const CANXL_HDR_SIZE: usize = 12usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:87`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L87)*

### `CANXL_MIN_MTU`
```rust
const CANXL_MIN_MTU: usize = 76usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:88`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L88)*

### `CANXL_MAX_MTU`
```rust
const CANXL_MAX_MTU: usize = 2_060usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:89`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L89)*

### `CAN_RAW`
```rust
const CAN_RAW: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:91`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L91)*

### `CAN_BCM`
```rust
const CAN_BCM: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:92`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L92)*

### `CAN_TP16`
```rust
const CAN_TP16: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:93`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L93)*

### `CAN_TP20`
```rust
const CAN_TP20: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:94`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L94)*

### `CAN_MCNET`
```rust
const CAN_MCNET: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:95`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L95)*

### `CAN_ISOTP`
```rust
const CAN_ISOTP: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:96`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L96)*

### `CAN_J1939`
```rust
const CAN_J1939: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:97`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L97)*

### `CAN_NPROTO`
```rust
const CAN_NPROTO: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:98`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L98)*

### `SOL_CAN_BASE`
```rust
const SOL_CAN_BASE: crate::c_int = 100i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:100`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L100)*

### `CAN_INV_FILTER`
```rust
const CAN_INV_FILTER: canid_t = 536_870_912u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:133`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L133)*

