*[libc](../../../../index.md) / [new](../../../index.md) / [linux_uapi](../../index.md) / [linux](../index.md) / [can](index.md)*

---

# Module `can`

Header: `uapi/linux/can.h`

## Contents

- [Modules](#modules)
  - [`bcm`](#bcm)
  - [`j1939`](#j1939)
  - [`raw`](#raw)
- [Structs](#structs)
  - [`can_frame`](#can-frame)
  - [`canfd_frame`](#canfd-frame)
  - [`canxl_frame`](#canxl-frame)
  - [`sockaddr_can`](#sockaddr-can)
  - [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp)
  - [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939)
  - [`can_filter`](#can-filter)
- [Type Aliases](#type-aliases)
  - [`canid_t`](#canid-t)
  - [`can_err_mask_t`](#can-err-mask-t)
- [Constants](#constants)
  - [`CAN_EFF_FLAG`](#can-eff-flag)
  - [`CAN_RTR_FLAG`](#can-rtr-flag)
  - [`CAN_ERR_FLAG`](#can-err-flag)
  - [`CAN_SFF_MASK`](#can-sff-mask)
  - [`CAN_EFF_MASK`](#can-eff-mask)
  - [`CAN_ERR_MASK`](#can-err-mask)
  - [`CANXL_PRIO_MASK`](#canxl-prio-mask)
  - [`CAN_SFF_ID_BITS`](#can-sff-id-bits)
  - [`CAN_EFF_ID_BITS`](#can-eff-id-bits)
  - [`CANXL_PRIO_BITS`](#canxl-prio-bits)
  - [`CAN_MAX_DLC`](#can-max-dlc)
  - [`CAN_MAX_DLEN`](#can-max-dlen)
  - [`CANFD_MAX_DLC`](#canfd-max-dlc)
  - [`CANFD_MAX_DLEN`](#canfd-max-dlen)
  - [`CANXL_MIN_DLC`](#canxl-min-dlc)
  - [`CANXL_MAX_DLC`](#canxl-max-dlc)
  - [`CANXL_MAX_DLC_MASK`](#canxl-max-dlc-mask)
  - [`CANXL_MIN_DLEN`](#canxl-min-dlen)
  - [`CANXL_MAX_DLEN`](#canxl-max-dlen)
  - [`CANFD_BRS`](#canfd-brs)
  - [`CANFD_ESI`](#canfd-esi)
  - [`CANFD_FDF`](#canfd-fdf)
  - [`CANXL_XLF`](#canxl-xlf)
  - [`CANXL_SEC`](#canxl-sec)
  - [`CAN_MTU`](#can-mtu)
  - [`CANFD_MTU`](#canfd-mtu)
  - [`CANXL_MTU`](#canxl-mtu)
  - [`CANXL_HDR_SIZE`](#canxl-hdr-size)
  - [`CANXL_MIN_MTU`](#canxl-min-mtu)
  - [`CANXL_MAX_MTU`](#canxl-max-mtu)
  - [`CAN_RAW`](#can-raw)
  - [`CAN_BCM`](#can-bcm)
  - [`CAN_TP16`](#can-tp16)
  - [`CAN_TP20`](#can-tp20)
  - [`CAN_MCNET`](#can-mcnet)
  - [`CAN_ISOTP`](#can-isotp)
  - [`CAN_J1939`](#can-j1939)
  - [`CAN_NPROTO`](#can-nproto)
  - [`SOL_CAN_BASE`](#sol-can-base)
  - [`CAN_INV_FILTER`](#can-inv-filter)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`bcm`](#bcm) | mod | Header: `linux/can/bcm.h` |
| [`j1939`](#j1939) | mod | `linux/can/j1939.h` |
| [`raw`](#raw) | mod | Header: `linux/can/raw.h` |
| [`can_frame`](#can-frame) | struct |  |
| [`canfd_frame`](#canfd-frame) | struct |  |
| [`canxl_frame`](#canxl-frame) | struct |  |
| [`sockaddr_can`](#sockaddr-can) | struct |  |
| [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp) | struct |  |
| [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939) | struct |  |
| [`can_filter`](#can-filter) | struct |  |
| [`canid_t`](#canid-t) | type |  |
| [`can_err_mask_t`](#can-err-mask-t) | type |  |
| [`CAN_EFF_FLAG`](#can-eff-flag) | const |  |
| [`CAN_RTR_FLAG`](#can-rtr-flag) | const |  |
| [`CAN_ERR_FLAG`](#can-err-flag) | const |  |
| [`CAN_SFF_MASK`](#can-sff-mask) | const |  |
| [`CAN_EFF_MASK`](#can-eff-mask) | const |  |
| [`CAN_ERR_MASK`](#can-err-mask) | const |  |
| [`CANXL_PRIO_MASK`](#canxl-prio-mask) | const |  |
| [`CAN_SFF_ID_BITS`](#can-sff-id-bits) | const |  |
| [`CAN_EFF_ID_BITS`](#can-eff-id-bits) | const |  |
| [`CANXL_PRIO_BITS`](#canxl-prio-bits) | const |  |
| [`CAN_MAX_DLC`](#can-max-dlc) | const |  |
| [`CAN_MAX_DLEN`](#can-max-dlen) | const |  |
| [`CANFD_MAX_DLC`](#canfd-max-dlc) | const |  |
| [`CANFD_MAX_DLEN`](#canfd-max-dlen) | const |  |
| [`CANXL_MIN_DLC`](#canxl-min-dlc) | const |  |
| [`CANXL_MAX_DLC`](#canxl-max-dlc) | const |  |
| [`CANXL_MAX_DLC_MASK`](#canxl-max-dlc-mask) | const |  |
| [`CANXL_MIN_DLEN`](#canxl-min-dlen) | const |  |
| [`CANXL_MAX_DLEN`](#canxl-max-dlen) | const |  |
| [`CANFD_BRS`](#canfd-brs) | const |  |
| [`CANFD_ESI`](#canfd-esi) | const |  |
| [`CANFD_FDF`](#canfd-fdf) | const |  |
| [`CANXL_XLF`](#canxl-xlf) | const |  |
| [`CANXL_SEC`](#canxl-sec) | const |  |
| [`CAN_MTU`](#can-mtu) | const |  |
| [`CANFD_MTU`](#canfd-mtu) | const |  |
| [`CANXL_MTU`](#canxl-mtu) | const |  |
| [`CANXL_HDR_SIZE`](#canxl-hdr-size) | const |  |
| [`CANXL_MIN_MTU`](#canxl-min-mtu) | const |  |
| [`CANXL_MAX_MTU`](#canxl-max-mtu) | const |  |
| [`CAN_RAW`](#can-raw) | const |  |
| [`CAN_BCM`](#can-bcm) | const |  |
| [`CAN_TP16`](#can-tp16) | const |  |
| [`CAN_TP20`](#can-tp20) | const |  |
| [`CAN_MCNET`](#can-mcnet) | const |  |
| [`CAN_ISOTP`](#can-isotp) | const |  |
| [`CAN_J1939`](#can-j1939) | const |  |
| [`CAN_NPROTO`](#can-nproto) | const |  |
| [`SOL_CAN_BASE`](#sol-can-base) | const |  |
| [`CAN_INV_FILTER`](#can-inv-filter) | const |  |

## Modules

- [`bcm`](bcm/index.md) — Header: `linux/can/bcm.h`
- [`j1939`](j1939/index.md) — `linux/can/j1939.h`
- [`raw`](raw/index.md) — Header: `linux/can/raw.h`

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:38-49`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L38-L49)*

#### Trait Implementations

##### `impl Any for can_frame`

- <span id="can-frame-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for can_frame`

- <span id="can-frame-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for can_frame`

- <span id="can-frame-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for can_frame`

- <span id="can-frame-clone"></span>`fn clone(&self) -> can_frame` — [`can_frame`](../../../index.md#can-frame)

##### `impl CloneToUninit for can_frame`

- <span id="can-frame-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for can_frame`

##### `impl Debug for can_frame`

- <span id="can-frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for can_frame`

- <span id="can-frame-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for can_frame`

- <span id="can-frame-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for can_frame`

- <span id="can-frame-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="can-frame-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for can_frame`

- <span id="can-frame-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="can-frame-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:55-65`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L55-L65)*

#### Trait Implementations

##### `impl Any for canfd_frame`

- <span id="canfd-frame-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for canfd_frame`

- <span id="canfd-frame-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for canfd_frame`

- <span id="canfd-frame-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for canfd_frame`

- <span id="canfd-frame-clone"></span>`fn clone(&self) -> canfd_frame` — [`canfd_frame`](../../../index.md#canfd-frame)

##### `impl CloneToUninit for canfd_frame`

- <span id="canfd-frame-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for canfd_frame`

##### `impl Debug for canfd_frame`

- <span id="canfd-frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for canfd_frame`

- <span id="canfd-frame-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for canfd_frame`

- <span id="canfd-frame-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for canfd_frame`

- <span id="canfd-frame-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="canfd-frame-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for canfd_frame`

- <span id="canfd-frame-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="canfd-frame-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:70-79`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L70-L79)*

#### Trait Implementations

##### `impl Any for canxl_frame`

- <span id="canxl-frame-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for canxl_frame`

- <span id="canxl-frame-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for canxl_frame`

- <span id="canxl-frame-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for canxl_frame`

- <span id="canxl-frame-clone"></span>`fn clone(&self) -> canxl_frame` — [`canxl_frame`](../../../index.md#canxl-frame)

##### `impl CloneToUninit for canxl_frame`

- <span id="canxl-frame-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for canxl_frame`

##### `impl Debug for canxl_frame`

- <span id="canxl-frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for canxl_frame`

- <span id="canxl-frame-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for canxl_frame`

- <span id="canxl-frame-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for canxl_frame`

- <span id="canxl-frame-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="canxl-frame-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for canxl_frame`

- <span id="canxl-frame-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="canxl-frame-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `sockaddr_can`

```rust
struct sockaddr_can {
    pub can_family: crate::sa_family_t,
    pub can_ifindex: crate::c_int,
    pub can_addr: __c_anonymous_sockaddr_can_can_addr,
}
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:102-113`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L102-L113)*

#### Trait Implementations

##### `impl Any for sockaddr_can`

- <span id="sockaddr-can-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for sockaddr_can`

- <span id="sockaddr-can-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for sockaddr_can`

- <span id="sockaddr-can-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for sockaddr_can`

- <span id="sockaddr-can-clone"></span>`fn clone(&self) -> sockaddr_can` — [`sockaddr_can`](../../../index.md#sockaddr-can)

##### `impl CloneToUninit for sockaddr_can`

- <span id="sockaddr-can-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for sockaddr_can`

##### `impl Debug for sockaddr_can`

- <span id="sockaddr-can-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for sockaddr_can`

- <span id="sockaddr-can-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for sockaddr_can`

- <span id="sockaddr-can-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for sockaddr_can`

- <span id="sockaddr-can-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sockaddr-can-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for sockaddr_can`

- <span id="sockaddr-can-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sockaddr-can-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__c_anonymous_sockaddr_can_tp`

```rust
struct __c_anonymous_sockaddr_can_tp {
    pub rx_id: canid_t,
    pub tx_id: canid_t,
}
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:115-131`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L115-L131)*

#### Trait Implementations

##### `impl Any for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-clone"></span>`fn clone(&self) -> __c_anonymous_sockaddr_can_tp` — [`__c_anonymous_sockaddr_can_tp`](../../../index.md#c-anonymous-sockaddr-can-tp)

##### `impl CloneToUninit for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __c_anonymous_sockaddr_can_tp`

##### `impl Debug for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="c-anonymous-sockaddr-can-tp-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="c-anonymous-sockaddr-can-tp-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__c_anonymous_sockaddr_can_j1939`

```rust
struct __c_anonymous_sockaddr_can_j1939 {
    pub name: u64,
    pub pgn: u32,
    pub addr: u8,
}
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:115-131`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L115-L131)*

#### Trait Implementations

##### `impl Any for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-clone"></span>`fn clone(&self) -> __c_anonymous_sockaddr_can_j1939` — [`__c_anonymous_sockaddr_can_j1939`](../../../index.md#c-anonymous-sockaddr-can-j1939)

##### `impl CloneToUninit for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __c_anonymous_sockaddr_can_j1939`

##### `impl Debug for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="c-anonymous-sockaddr-can-j1939-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="c-anonymous-sockaddr-can-j1939-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `can_filter`

```rust
struct can_filter {
    pub can_id: canid_t,
    pub can_mask: canid_t,
}
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:115-131`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L115-L131)*

#### Trait Implementations

##### `impl Any for can_filter`

- <span id="can-filter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for can_filter`

- <span id="can-filter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for can_filter`

- <span id="can-filter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for can_filter`

- <span id="can-filter-clone"></span>`fn clone(&self) -> can_filter` — [`can_filter`](../../../index.md#can-filter)

##### `impl CloneToUninit for can_filter`

- <span id="can-filter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for can_filter`

##### `impl Debug for can_filter`

- <span id="can-filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for can_filter`

- <span id="can-filter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for can_filter`

- <span id="can-filter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for can_filter`

- <span id="can-filter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="can-filter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for can_filter`

- <span id="can-filter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="can-filter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `canid_t`

```rust
type canid_t = u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:18`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L18)*

### `can_err_mask_t`

```rust
type can_err_mask_t = u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:24`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L24)*

## Constants

### `CAN_EFF_FLAG`
```rust
const CAN_EFF_FLAG: canid_t = 2_147_483_648u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:9`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L9)*

### `CAN_RTR_FLAG`
```rust
const CAN_RTR_FLAG: canid_t = 1_073_741_824u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:10`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L10)*

### `CAN_ERR_FLAG`
```rust
const CAN_ERR_FLAG: canid_t = 536_870_912u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:11`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L11)*

### `CAN_SFF_MASK`
```rust
const CAN_SFF_MASK: canid_t = 2_047u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:13`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L13)*

### `CAN_EFF_MASK`
```rust
const CAN_EFF_MASK: canid_t = 536_870_911u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:14`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L14)*

### `CAN_ERR_MASK`
```rust
const CAN_ERR_MASK: canid_t = 536_870_911u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:15`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L15)*

### `CANXL_PRIO_MASK`
```rust
const CANXL_PRIO_MASK: crate::canid_t = 2_047u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:16`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L16)*

### `CAN_SFF_ID_BITS`
```rust
const CAN_SFF_ID_BITS: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:20`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L20)*

### `CAN_EFF_ID_BITS`
```rust
const CAN_EFF_ID_BITS: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:21`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L21)*

### `CANXL_PRIO_BITS`
```rust
const CANXL_PRIO_BITS: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:22`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L22)*

### `CAN_MAX_DLC`
```rust
const CAN_MAX_DLC: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:26`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L26)*

### `CAN_MAX_DLEN`
```rust
const CAN_MAX_DLEN: usize = 8usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:27`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L27)*

### `CANFD_MAX_DLC`
```rust
const CANFD_MAX_DLC: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:29`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L29)*

### `CANFD_MAX_DLEN`
```rust
const CANFD_MAX_DLEN: usize = 64usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:30`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L30)*

### `CANXL_MIN_DLC`
```rust
const CANXL_MIN_DLC: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:32`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L32)*

### `CANXL_MAX_DLC`
```rust
const CANXL_MAX_DLC: crate::c_int = 2_047i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:33`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L33)*

### `CANXL_MAX_DLC_MASK`
```rust
const CANXL_MAX_DLC_MASK: crate::c_int = 2_047i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:34`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L34)*

### `CANXL_MIN_DLEN`
```rust
const CANXL_MIN_DLEN: usize = 1usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:35`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L35)*

### `CANXL_MAX_DLEN`
```rust
const CANXL_MAX_DLEN: usize = 2_048usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:36`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L36)*

### `CANFD_BRS`
```rust
const CANFD_BRS: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:51`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L51)*

### `CANFD_ESI`
```rust
const CANFD_ESI: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:52`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L52)*

### `CANFD_FDF`
```rust
const CANFD_FDF: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:53`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L53)*

### `CANXL_XLF`
```rust
const CANXL_XLF: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:67`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L67)*

### `CANXL_SEC`
```rust
const CANXL_SEC: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:68`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L68)*

### `CAN_MTU`
```rust
const CAN_MTU: usize = 16usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:81`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L81)*

### `CANFD_MTU`
```rust
const CANFD_MTU: usize = 72usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:82`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L82)*

### `CANXL_MTU`
```rust
const CANXL_MTU: usize = 2_060usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:83`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L83)*

### `CANXL_HDR_SIZE`
```rust
const CANXL_HDR_SIZE: usize = 12usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:87`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L87)*

### `CANXL_MIN_MTU`
```rust
const CANXL_MIN_MTU: usize = 76usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:88`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L88)*

### `CANXL_MAX_MTU`
```rust
const CANXL_MAX_MTU: usize = 2_060usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:89`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L89)*

### `CAN_RAW`
```rust
const CAN_RAW: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:91`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L91)*

### `CAN_BCM`
```rust
const CAN_BCM: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:92`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L92)*

### `CAN_TP16`
```rust
const CAN_TP16: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:93`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L93)*

### `CAN_TP20`
```rust
const CAN_TP20: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:94`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L94)*

### `CAN_MCNET`
```rust
const CAN_MCNET: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:95`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L95)*

### `CAN_ISOTP`
```rust
const CAN_ISOTP: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:96`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L96)*

### `CAN_J1939`
```rust
const CAN_J1939: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:97`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L97)*

### `CAN_NPROTO`
```rust
const CAN_NPROTO: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:98`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L98)*

### `SOL_CAN_BASE`
```rust
const SOL_CAN_BASE: crate::c_int = 100i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:100`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L100)*

### `CAN_INV_FILTER`
```rust
const CAN_INV_FILTER: canid_t = 536_870_912u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:133`](../../../../../../.source_1765633015/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L133)*

