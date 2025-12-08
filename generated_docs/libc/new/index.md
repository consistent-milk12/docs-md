*[libc](../index.md) / [new](index.md)*

---

# Module `new`

This module contains the future directory structure. If possible, new definitions should
get added here.

Eventually everything should be moved over, and we will move this directory to the top
level in `src`.

# Basic structure

Each child module here represents a library or group of libraries that we are binding. Each of
these has several submodules, representing either a directory or a header file in that library.

`#include`s turn into `pub use ...*;` statements. Then at the root level (here), we choose
which top-level headers we want to reexport the definitions for.

All modules are only crate-public since we don't reexport this structure.

## Modules

- [`common`](common/index.md) - Interfaces that are common across multiple platforms
- [`linux_uapi`](linux_uapi/index.md) - This directory maps to `include/uapi` in the Linux source tree.
- [`glibc`](glibc/index.md) - GNU libc.
- [`bcm`](bcm/index.md) - Header: `linux/can/bcm.h`
- [`j1939`](j1939/index.md) - `linux/can/j1939.h`
- [`raw`](raw/index.md) - Header: `linux/can/raw.h`

## Structs

### `bcm_timeval`

```rust
struct bcm_timeval {
    pub tv_sec: crate::c_long,
    pub tv_usec: crate::c_long,
}
```

#### Trait Implementations

##### `impl Clone for bcm_timeval`

- `fn clone(self: &Self) -> bcm_timeval` — [`bcm_timeval`](#bcm-timeval)

##### `impl Copy for bcm_timeval`

##### `impl Debug for bcm_timeval`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

#### Trait Implementations

##### `impl Clone for bcm_msg_head`

- `fn clone(self: &Self) -> bcm_msg_head` — [`bcm_msg_head`](#bcm-msg-head)

##### `impl Copy for bcm_msg_head`

##### `impl Debug for bcm_msg_head`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn clone(self: &Self) -> j1939_filter` — [`j1939_filter`](#j1939-filter)

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

- `fn clone(self: &Self) -> can_frame` — [`can_frame`](#can-frame)

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

- `fn clone(self: &Self) -> canfd_frame` — [`canfd_frame`](#canfd-frame)

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

- `fn clone(self: &Self) -> canxl_frame` — [`canxl_frame`](#canxl-frame)

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

- `fn clone(self: &Self) -> sockaddr_can` — [`sockaddr_can`](#sockaddr-can)

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

- `fn clone(self: &Self) -> __c_anonymous_sockaddr_can_tp` — [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp)

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

- `fn clone(self: &Self) -> __c_anonymous_sockaddr_can_j1939` — [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939)

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

- `fn clone(self: &Self) -> can_filter` — [`can_filter`](#can-filter)

##### `impl Copy for can_filter`

##### `impl Debug for can_filter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `rtentry`

```rust
struct rtentry {
    pub rt_pad1: crate::c_ulong,
    pub rt_dst: crate::sockaddr,
    pub rt_gateway: crate::sockaddr,
    pub rt_genmask: crate::sockaddr,
    pub rt_flags: crate::c_ushort,
    pub rt_pad2: crate::c_short,
    pub rt_pad3: crate::c_ulong,
    pub rt_tos: crate::c_uchar,
    pub rt_class: crate::c_uchar,
    pub rt_pad4: [crate::c_short; 3],
    pub rt_metric: crate::c_short,
    pub rt_dev: *mut crate::c_char,
    pub rt_mtu: crate::c_ulong,
    pub rt_window: crate::c_ulong,
    pub rt_irtt: crate::c_ushort,
}
```

#### Trait Implementations

##### `impl Clone for rtentry`

- `fn clone(self: &Self) -> rtentry` — [`rtentry`](#rtentry)

##### `impl Copy for rtentry`

##### `impl Debug for rtentry`

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

### `TX_SETUP`

```rust
const TX_SETUP: u32 = 1u32;
```

### `TX_DELETE`

```rust
const TX_DELETE: u32 = 2u32;
```

### `TX_READ`

```rust
const TX_READ: u32 = 3u32;
```

### `TX_SEND`

```rust
const TX_SEND: u32 = 4u32;
```

### `RX_SETUP`

```rust
const RX_SETUP: u32 = 5u32;
```

### `RX_DELETE`

```rust
const RX_DELETE: u32 = 6u32;
```

### `RX_READ`

```rust
const RX_READ: u32 = 7u32;
```

### `TX_STATUS`

```rust
const TX_STATUS: u32 = 8u32;
```

### `TX_EXPIRED`

```rust
const TX_EXPIRED: u32 = 9u32;
```

### `RX_STATUS`

```rust
const RX_STATUS: u32 = 10u32;
```

### `RX_TIMEOUT`

```rust
const RX_TIMEOUT: u32 = 11u32;
```

### `RX_CHANGED`

```rust
const RX_CHANGED: u32 = 12u32;
```

### `SETTIMER`

```rust
const SETTIMER: u32 = 1u32;
```

### `STARTTIMER`

```rust
const STARTTIMER: u32 = 2u32;
```

### `TX_COUNTEVT`

```rust
const TX_COUNTEVT: u32 = 4u32;
```

### `TX_ANNOUNCE`

```rust
const TX_ANNOUNCE: u32 = 8u32;
```

### `TX_CP_CAN_ID`

```rust
const TX_CP_CAN_ID: u32 = 16u32;
```

### `RX_FILTER_ID`

```rust
const RX_FILTER_ID: u32 = 32u32;
```

### `RX_CHECK_DLC`

```rust
const RX_CHECK_DLC: u32 = 64u32;
```

### `RX_NO_AUTOTIMER`

```rust
const RX_NO_AUTOTIMER: u32 = 128u32;
```

### `RX_ANNOUNCE_RESUME`

```rust
const RX_ANNOUNCE_RESUME: u32 = 256u32;
```

### `TX_RESET_MULTI_IDX`

```rust
const TX_RESET_MULTI_IDX: u32 = 512u32;
```

### `RX_RTR_FRAME`

```rust
const RX_RTR_FRAME: u32 = 1_024u32;
```

### `CAN_FD_FRAME`

```rust
const CAN_FD_FRAME: u32 = 2_048u32;
```

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

### `KEY_SPEC_THREAD_KEYRING`

```rust
const KEY_SPEC_THREAD_KEYRING: i32 = -1i32;
```

### `KEY_SPEC_PROCESS_KEYRING`

```rust
const KEY_SPEC_PROCESS_KEYRING: i32 = -2i32;
```

### `KEY_SPEC_SESSION_KEYRING`

```rust
const KEY_SPEC_SESSION_KEYRING: i32 = -3i32;
```

### `KEY_SPEC_USER_KEYRING`

```rust
const KEY_SPEC_USER_KEYRING: i32 = -4i32;
```

### `KEY_SPEC_USER_SESSION_KEYRING`

```rust
const KEY_SPEC_USER_SESSION_KEYRING: i32 = -5i32;
```

### `KEY_SPEC_GROUP_KEYRING`

```rust
const KEY_SPEC_GROUP_KEYRING: i32 = -6i32;
```

### `KEY_SPEC_REQKEY_AUTH_KEY`

```rust
const KEY_SPEC_REQKEY_AUTH_KEY: i32 = -7i32;
```

### `KEY_SPEC_REQUESTOR_KEYRING`

```rust
const KEY_SPEC_REQUESTOR_KEYRING: i32 = -8i32;
```

### `KEY_REQKEY_DEFL_NO_CHANGE`

```rust
const KEY_REQKEY_DEFL_NO_CHANGE: i32 = -1i32;
```

### `KEY_REQKEY_DEFL_DEFAULT`

```rust
const KEY_REQKEY_DEFL_DEFAULT: i32 = 0i32;
```

### `KEY_REQKEY_DEFL_THREAD_KEYRING`

```rust
const KEY_REQKEY_DEFL_THREAD_KEYRING: i32 = 1i32;
```

### `KEY_REQKEY_DEFL_PROCESS_KEYRING`

```rust
const KEY_REQKEY_DEFL_PROCESS_KEYRING: i32 = 2i32;
```

### `KEY_REQKEY_DEFL_SESSION_KEYRING`

```rust
const KEY_REQKEY_DEFL_SESSION_KEYRING: i32 = 3i32;
```

### `KEY_REQKEY_DEFL_USER_KEYRING`

```rust
const KEY_REQKEY_DEFL_USER_KEYRING: i32 = 4i32;
```

### `KEY_REQKEY_DEFL_USER_SESSION_KEYRING`

```rust
const KEY_REQKEY_DEFL_USER_SESSION_KEYRING: i32 = 5i32;
```

### `KEY_REQKEY_DEFL_GROUP_KEYRING`

```rust
const KEY_REQKEY_DEFL_GROUP_KEYRING: i32 = 6i32;
```

### `KEY_REQKEY_DEFL_REQUESTOR_KEYRING`

```rust
const KEY_REQKEY_DEFL_REQUESTOR_KEYRING: i32 = 7i32;
```

### `KEYCTL_GET_KEYRING_ID`

```rust
const KEYCTL_GET_KEYRING_ID: u32 = 0u32;
```

### `KEYCTL_JOIN_SESSION_KEYRING`

```rust
const KEYCTL_JOIN_SESSION_KEYRING: u32 = 1u32;
```

### `KEYCTL_UPDATE`

```rust
const KEYCTL_UPDATE: u32 = 2u32;
```

### `KEYCTL_REVOKE`

```rust
const KEYCTL_REVOKE: u32 = 3u32;
```

### `KEYCTL_CHOWN`

```rust
const KEYCTL_CHOWN: u32 = 4u32;
```

### `KEYCTL_SETPERM`

```rust
const KEYCTL_SETPERM: u32 = 5u32;
```

### `KEYCTL_DESCRIBE`

```rust
const KEYCTL_DESCRIBE: u32 = 6u32;
```

### `KEYCTL_CLEAR`

```rust
const KEYCTL_CLEAR: u32 = 7u32;
```

### `KEYCTL_LINK`

```rust
const KEYCTL_LINK: u32 = 8u32;
```

### `KEYCTL_UNLINK`

```rust
const KEYCTL_UNLINK: u32 = 9u32;
```

### `KEYCTL_SEARCH`

```rust
const KEYCTL_SEARCH: u32 = 10u32;
```

### `KEYCTL_READ`

```rust
const KEYCTL_READ: u32 = 11u32;
```

### `KEYCTL_INSTANTIATE`

```rust
const KEYCTL_INSTANTIATE: u32 = 12u32;
```

### `KEYCTL_NEGATE`

```rust
const KEYCTL_NEGATE: u32 = 13u32;
```

### `KEYCTL_SET_REQKEY_KEYRING`

```rust
const KEYCTL_SET_REQKEY_KEYRING: u32 = 14u32;
```

### `KEYCTL_SET_TIMEOUT`

```rust
const KEYCTL_SET_TIMEOUT: u32 = 15u32;
```

### `KEYCTL_ASSUME_AUTHORITY`

```rust
const KEYCTL_ASSUME_AUTHORITY: u32 = 16u32;
```

### `KEYCTL_GET_SECURITY`

```rust
const KEYCTL_GET_SECURITY: u32 = 17u32;
```

### `KEYCTL_SESSION_TO_PARENT`

```rust
const KEYCTL_SESSION_TO_PARENT: u32 = 18u32;
```

### `KEYCTL_REJECT`

```rust
const KEYCTL_REJECT: u32 = 19u32;
```

### `KEYCTL_INSTANTIATE_IOV`

```rust
const KEYCTL_INSTANTIATE_IOV: u32 = 20u32;
```

### `KEYCTL_INVALIDATE`

```rust
const KEYCTL_INVALIDATE: u32 = 21u32;
```

### `KEYCTL_GET_PERSISTENT`

```rust
const KEYCTL_GET_PERSISTENT: u32 = 22u32;
```

### `KEYCTL_DH_COMPUTE`

```rust
const KEYCTL_DH_COMPUTE: u32 = 23u32;
```

### `KEYCTL_PKEY_QUERY`

```rust
const KEYCTL_PKEY_QUERY: u32 = 24u32;
```

### `KEYCTL_PKEY_ENCRYPT`

```rust
const KEYCTL_PKEY_ENCRYPT: u32 = 25u32;
```

### `KEYCTL_PKEY_DECRYPT`

```rust
const KEYCTL_PKEY_DECRYPT: u32 = 26u32;
```

### `KEYCTL_PKEY_SIGN`

```rust
const KEYCTL_PKEY_SIGN: u32 = 27u32;
```

### `KEYCTL_PKEY_VERIFY`

```rust
const KEYCTL_PKEY_VERIFY: u32 = 28u32;
```

### `KEYCTL_RESTRICT_KEYRING`

```rust
const KEYCTL_RESTRICT_KEYRING: u32 = 29u32;
```

### `KEYCTL_MOVE`

```rust
const KEYCTL_MOVE: u32 = 30u32;
```

### `KEYCTL_CAPABILITIES`

```rust
const KEYCTL_CAPABILITIES: u32 = 31u32;
```

### `KEYCTL_SUPPORTS_ENCRYPT`

```rust
const KEYCTL_SUPPORTS_ENCRYPT: u32 = 1u32;
```

### `KEYCTL_SUPPORTS_DECRYPT`

```rust
const KEYCTL_SUPPORTS_DECRYPT: u32 = 2u32;
```

### `KEYCTL_SUPPORTS_SIGN`

```rust
const KEYCTL_SUPPORTS_SIGN: u32 = 4u32;
```

### `KEYCTL_SUPPORTS_VERIFY`

```rust
const KEYCTL_SUPPORTS_VERIFY: u32 = 8u32;
```

### `KEYCTL_CAPS0_CAPABILITIES`

```rust
const KEYCTL_CAPS0_CAPABILITIES: u32 = 1u32;
```

### `KEYCTL_CAPS0_PERSISTENT_KEYRINGS`

```rust
const KEYCTL_CAPS0_PERSISTENT_KEYRINGS: u32 = 2u32;
```

### `KEYCTL_CAPS0_DIFFIE_HELLMAN`

```rust
const KEYCTL_CAPS0_DIFFIE_HELLMAN: u32 = 4u32;
```

### `KEYCTL_CAPS0_PUBLIC_KEY`

```rust
const KEYCTL_CAPS0_PUBLIC_KEY: u32 = 8u32;
```

### `KEYCTL_CAPS0_BIG_KEY`

```rust
const KEYCTL_CAPS0_BIG_KEY: u32 = 16u32;
```

### `KEYCTL_CAPS0_INVALIDATE`

```rust
const KEYCTL_CAPS0_INVALIDATE: u32 = 32u32;
```

### `KEYCTL_CAPS0_RESTRICT_KEYRING`

```rust
const KEYCTL_CAPS0_RESTRICT_KEYRING: u32 = 64u32;
```

### `KEYCTL_CAPS0_MOVE`

```rust
const KEYCTL_CAPS0_MOVE: u32 = 128u32;
```

### `KEYCTL_CAPS1_NS_KEYRING_NAME`

```rust
const KEYCTL_CAPS1_NS_KEYRING_NAME: u32 = 1u32;
```

### `KEYCTL_CAPS1_NS_KEY_TAG`

```rust
const KEYCTL_CAPS1_NS_KEY_TAG: u32 = 2u32;
```

