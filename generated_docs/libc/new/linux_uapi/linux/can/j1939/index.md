*[libc](../../../../../index.md) / [new](../../../../index.md) / [linux_uapi](../../../index.md) / [linux](../../index.md) / [can](../index.md) / [j1939](index.md)*

---

# Module `j1939`

`linux/can/j1939.h`

## Contents

- [Modules](#modules)
  - [`bcm`](#bcm)
  - [`j1939`](#j1939)
  - [`raw`](#raw)
- [Structs](#structs)
  - [`j1939_filter`](#j1939_filter)
  - [`can_frame`](#can_frame)
  - [`canfd_frame`](#canfd_frame)
  - [`canxl_frame`](#canxl_frame)
  - [`sockaddr_can`](#sockaddr_can)
  - [`__c_anonymous_sockaddr_can_tp`](#__c_anonymous_sockaddr_can_tp)
  - [`__c_anonymous_sockaddr_can_j1939`](#__c_anonymous_sockaddr_can_j1939)
  - [`can_filter`](#can_filter)
- [Type Aliases](#type-aliases)
  - [`pgn_t`](#pgn_t)
  - [`priority_t`](#priority_t)
  - [`name_t`](#name_t)
  - [`canid_t`](#canid_t)
  - [`can_err_mask_t`](#can_err_mask_t)
- [Constants](#constants)
  - [`J1939_MAX_UNICAST_ADDR`](#j1939_max_unicast_addr)
  - [`J1939_IDLE_ADDR`](#j1939_idle_addr)
  - [`J1939_NO_ADDR`](#j1939_no_addr)
  - [`J1939_NO_NAME`](#j1939_no_name)
  - [`J1939_PGN_REQUEST`](#j1939_pgn_request)
  - [`J1939_PGN_ADDRESS_CLAIMED`](#j1939_pgn_address_claimed)
  - [`J1939_PGN_ADDRESS_COMMANDED`](#j1939_pgn_address_commanded)
  - [`J1939_PGN_PDU1_MAX`](#j1939_pgn_pdu1_max)
  - [`J1939_PGN_MAX`](#j1939_pgn_max)
  - [`J1939_NO_PGN`](#j1939_no_pgn)
  - [`SOL_CAN_J1939`](#sol_can_j1939)
  - [`SO_J1939_FILTER`](#so_j1939_filter)
  - [`SO_J1939_PROMISC`](#so_j1939_promisc)
  - [`SO_J1939_SEND_PRIO`](#so_j1939_send_prio)
  - [`SO_J1939_ERRQUEUE`](#so_j1939_errqueue)
  - [`SCM_J1939_DEST_ADDR`](#scm_j1939_dest_addr)
  - [`SCM_J1939_DEST_NAME`](#scm_j1939_dest_name)
  - [`SCM_J1939_PRIO`](#scm_j1939_prio)
  - [`SCM_J1939_ERRQUEUE`](#scm_j1939_errqueue)
  - [`J1939_NLA_PAD`](#j1939_nla_pad)
  - [`J1939_NLA_BYTES_ACKED`](#j1939_nla_bytes_acked)
  - [`J1939_NLA_TOTAL_SIZE`](#j1939_nla_total_size)
  - [`J1939_NLA_PGN`](#j1939_nla_pgn)
  - [`J1939_NLA_SRC_NAME`](#j1939_nla_src_name)
  - [`J1939_NLA_DEST_NAME`](#j1939_nla_dest_name)
  - [`J1939_NLA_SRC_ADDR`](#j1939_nla_src_addr)
  - [`J1939_NLA_DEST_ADDR`](#j1939_nla_dest_addr)
  - [`J1939_EE_INFO_NONE`](#j1939_ee_info_none)
  - [`J1939_EE_INFO_TX_ABORT`](#j1939_ee_info_tx_abort)
  - [`J1939_EE_INFO_RX_RTS`](#j1939_ee_info_rx_rts)
  - [`J1939_EE_INFO_RX_DPO`](#j1939_ee_info_rx_dpo)
  - [`J1939_EE_INFO_RX_ABORT`](#j1939_ee_info_rx_abort)
  - [`J1939_FILTER_MAX`](#j1939_filter_max)
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
| [`j1939_filter`](#j1939_filter) | struct |  |
| [`can_frame`](#can_frame) | struct |  |
| [`canfd_frame`](#canfd_frame) | struct |  |
| [`canxl_frame`](#canxl_frame) | struct |  |
| [`sockaddr_can`](#sockaddr_can) | struct |  |
| [`__c_anonymous_sockaddr_can_tp`](#__c_anonymous_sockaddr_can_tp) | struct |  |
| [`__c_anonymous_sockaddr_can_j1939`](#__c_anonymous_sockaddr_can_j1939) | struct |  |
| [`can_filter`](#can_filter) | struct |  |
| [`pgn_t`](#pgn_t) | type |  |
| [`priority_t`](#priority_t) | type |  |
| [`name_t`](#name_t) | type |  |
| [`canid_t`](#canid_t) | type |  |
| [`can_err_mask_t`](#can_err_mask_t) | type |  |
| [`J1939_MAX_UNICAST_ADDR`](#j1939_max_unicast_addr) | const |  |
| [`J1939_IDLE_ADDR`](#j1939_idle_addr) | const |  |
| [`J1939_NO_ADDR`](#j1939_no_addr) | const |  |
| [`J1939_NO_NAME`](#j1939_no_name) | const |  |
| [`J1939_PGN_REQUEST`](#j1939_pgn_request) | const |  |
| [`J1939_PGN_ADDRESS_CLAIMED`](#j1939_pgn_address_claimed) | const |  |
| [`J1939_PGN_ADDRESS_COMMANDED`](#j1939_pgn_address_commanded) | const |  |
| [`J1939_PGN_PDU1_MAX`](#j1939_pgn_pdu1_max) | const |  |
| [`J1939_PGN_MAX`](#j1939_pgn_max) | const |  |
| [`J1939_NO_PGN`](#j1939_no_pgn) | const |  |
| [`SOL_CAN_J1939`](#sol_can_j1939) | const |  |
| [`SO_J1939_FILTER`](#so_j1939_filter) | const |  |
| [`SO_J1939_PROMISC`](#so_j1939_promisc) | const |  |
| [`SO_J1939_SEND_PRIO`](#so_j1939_send_prio) | const |  |
| [`SO_J1939_ERRQUEUE`](#so_j1939_errqueue) | const |  |
| [`SCM_J1939_DEST_ADDR`](#scm_j1939_dest_addr) | const |  |
| [`SCM_J1939_DEST_NAME`](#scm_j1939_dest_name) | const |  |
| [`SCM_J1939_PRIO`](#scm_j1939_prio) | const |  |
| [`SCM_J1939_ERRQUEUE`](#scm_j1939_errqueue) | const |  |
| [`J1939_NLA_PAD`](#j1939_nla_pad) | const |  |
| [`J1939_NLA_BYTES_ACKED`](#j1939_nla_bytes_acked) | const |  |
| [`J1939_NLA_TOTAL_SIZE`](#j1939_nla_total_size) | const |  |
| [`J1939_NLA_PGN`](#j1939_nla_pgn) | const |  |
| [`J1939_NLA_SRC_NAME`](#j1939_nla_src_name) | const |  |
| [`J1939_NLA_DEST_NAME`](#j1939_nla_dest_name) | const |  |
| [`J1939_NLA_SRC_ADDR`](#j1939_nla_src_addr) | const |  |
| [`J1939_NLA_DEST_ADDR`](#j1939_nla_dest_addr) | const |  |
| [`J1939_EE_INFO_NONE`](#j1939_ee_info_none) | const |  |
| [`J1939_EE_INFO_TX_ABORT`](#j1939_ee_info_tx_abort) | const |  |
| [`J1939_EE_INFO_RX_RTS`](#j1939_ee_info_rx_rts) | const |  |
| [`J1939_EE_INFO_RX_DPO`](#j1939_ee_info_rx_dpo) | const |  |
| [`J1939_EE_INFO_RX_ABORT`](#j1939_ee_info_rx_abort) | const |  |
| [`J1939_FILTER_MAX`](#j1939_filter_max) | const |  |
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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:49-58`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L49-L58)*

#### Trait Implementations

##### `impl Clone for j1939_filter`

- <span id="j1939-filter-clone"></span>`fn clone(&self) -> j1939_filter` — [`j1939_filter`](../../../../index.md)

##### `impl Copy for j1939_filter`

##### `impl Debug for j1939_filter`

- <span id="j1939-filter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

### `pgn_t`

```rust
type pgn_t = u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:16`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L16)*

### `priority_t`

```rust
type priority_t = u8;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:17`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L17)*

### `name_t`

```rust
type name_t = u64;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:18`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L18)*

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

### `J1939_MAX_UNICAST_ADDR`
```rust
const J1939_MAX_UNICAST_ADDR: crate::c_uchar = 253u8;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:5`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L5)*

### `J1939_IDLE_ADDR`
```rust
const J1939_IDLE_ADDR: crate::c_uchar = 254u8;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:6`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L6)*

### `J1939_NO_ADDR`
```rust
const J1939_NO_ADDR: crate::c_uchar = 255u8;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:7`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L7)*

### `J1939_NO_NAME`
```rust
const J1939_NO_NAME: crate::c_ulong = 0u64;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:8`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L8)*

### `J1939_PGN_REQUEST`
```rust
const J1939_PGN_REQUEST: crate::c_uint = 59_904u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:9`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L9)*

### `J1939_PGN_ADDRESS_CLAIMED`
```rust
const J1939_PGN_ADDRESS_CLAIMED: crate::c_uint = 60_928u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:10`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L10)*

### `J1939_PGN_ADDRESS_COMMANDED`
```rust
const J1939_PGN_ADDRESS_COMMANDED: crate::c_uint = 65_240u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:11`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L11)*

### `J1939_PGN_PDU1_MAX`
```rust
const J1939_PGN_PDU1_MAX: crate::c_uint = 261_888u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:12`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L12)*

### `J1939_PGN_MAX`
```rust
const J1939_PGN_MAX: crate::c_uint = 262_143u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:13`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L13)*

### `J1939_NO_PGN`
```rust
const J1939_NO_PGN: crate::c_uint = 262_144u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:14`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L14)*

### `SOL_CAN_J1939`
```rust
const SOL_CAN_J1939: crate::c_int = 107i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:20`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L20)*

### `SO_J1939_FILTER`
```rust
const SO_J1939_FILTER: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:24`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L24)*

### `SO_J1939_PROMISC`
```rust
const SO_J1939_PROMISC: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:25`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L25)*

### `SO_J1939_SEND_PRIO`
```rust
const SO_J1939_SEND_PRIO: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:26`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L26)*

### `SO_J1939_ERRQUEUE`
```rust
const SO_J1939_ERRQUEUE: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:27`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L27)*

### `SCM_J1939_DEST_ADDR`
```rust
const SCM_J1939_DEST_ADDR: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:29`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L29)*

### `SCM_J1939_DEST_NAME`
```rust
const SCM_J1939_DEST_NAME: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:30`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L30)*

### `SCM_J1939_PRIO`
```rust
const SCM_J1939_PRIO: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:31`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L31)*

### `SCM_J1939_ERRQUEUE`
```rust
const SCM_J1939_ERRQUEUE: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:32`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L32)*

### `J1939_NLA_PAD`
```rust
const J1939_NLA_PAD: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:34`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L34)*

### `J1939_NLA_BYTES_ACKED`
```rust
const J1939_NLA_BYTES_ACKED: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:35`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L35)*

### `J1939_NLA_TOTAL_SIZE`
```rust
const J1939_NLA_TOTAL_SIZE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:36`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L36)*

### `J1939_NLA_PGN`
```rust
const J1939_NLA_PGN: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:37`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L37)*

### `J1939_NLA_SRC_NAME`
```rust
const J1939_NLA_SRC_NAME: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:38`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L38)*

### `J1939_NLA_DEST_NAME`
```rust
const J1939_NLA_DEST_NAME: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L39)*

### `J1939_NLA_SRC_ADDR`
```rust
const J1939_NLA_SRC_ADDR: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:40`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L40)*

### `J1939_NLA_DEST_ADDR`
```rust
const J1939_NLA_DEST_ADDR: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:41`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L41)*

### `J1939_EE_INFO_NONE`
```rust
const J1939_EE_INFO_NONE: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:43`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L43)*

### `J1939_EE_INFO_TX_ABORT`
```rust
const J1939_EE_INFO_TX_ABORT: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:44`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L44)*

### `J1939_EE_INFO_RX_RTS`
```rust
const J1939_EE_INFO_RX_RTS: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:45`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L45)*

### `J1939_EE_INFO_RX_DPO`
```rust
const J1939_EE_INFO_RX_DPO: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:46`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L46)*

### `J1939_EE_INFO_RX_ABORT`
```rust
const J1939_EE_INFO_RX_ABORT: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:47`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L47)*

### `J1939_FILTER_MAX`
```rust
const J1939_FILTER_MAX: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:60`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L60)*

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

